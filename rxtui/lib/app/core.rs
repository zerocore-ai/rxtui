use crate::app::Context;
use crate::bounds::Rect;
use crate::buffer::{DoubleBuffer, ScreenBuffer};
use crate::component::{Action, Component, ComponentId};
use crate::node::Div;
use crate::node::Node;
use crate::terminal::TerminalRenderer;
use crate::vdom::VDom;
use crate::vnode::VNode;
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event},
    execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;

use super::config::{InlineConfig, InlineHeight, RenderConfig, TerminalMode};
use super::context::{FocusRequest, FocusTarget};
use super::events::{handle_key_event, handle_mouse_event};
use super::inline::InlineState;
use super::renderer::render_node_to_buffer;
use std::collections::HashMap;
#[cfg(feature = "effects")]
use std::collections::HashSet;

#[cfg(feature = "effects")]
use crate::effect::EffectRuntime;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Type alias for the render log callback function.
type RenderLogFn = Box<dyn Fn(&str)>;

/// Signal to indicate that the application should exit.
/// Used to propagate exit requests through the component tree.
pub struct ExitSignal;

/// Main application controller for terminal UI applications.
///
/// Manages the lifecycle of a terminal application including:
/// - Terminal initialization and cleanup
/// - Event loop processing (keyboard, mouse, resize)
/// - Virtual DOM rendering and updates
/// - Model state management through init-view-update pattern
///
/// ## Application Flow
///
/// ```text
///     ┌─────────────┐
///     │   App::new  │ ← Initialize terminal, enable raw mode
///     └──────┬──────┘
///            │
///            ▼
///     ┌─────────────┐
///     │  App::run   │ ← Start event loop with root model
///     └──────┬──────┘
///            │
///            ▼
///    ┌───────────────┐
///    │  Event Loop   │ ◄─┐
///    └───────┬───────┘   │
///            │           │
///     ┌──────▼──────┐    │
///     │   Render    │    │
///     │   Model     │    │
///     └──────┬──────┘    │
///            │           │
///     ┌──────▼──────┐    │
///     │ Update VDom │    │
///     └──────┬──────┘    │
///            │           │
///     ┌──────▼──────┐    │
///     │    Draw     │    │
///     │  Terminal   │    │
///     └──────┬──────┘    │
///            │           │
///     ┌──────▼──────┐    │
///     │Handle Events│────┘
///     └─────────────┘
/// ```
pub struct App {
    /// Virtual DOM instance that manages the UI tree
    vdom: VDom,

    /// Shared flag to control the application lifecycle
    running: Rc<RefCell<bool>>,

    /// Flag indicating whether a render is needed
    needs_render: Rc<RefCell<bool>>,

    /// Double buffer for flicker-free rendering
    double_buffer: DoubleBuffer,

    /// Optional function to call after each render for logging
    render_log_fn: Option<RenderLogFn>,

    /// Terminal renderer for optimized output
    terminal_renderer: TerminalRenderer,

    /// Rendering configuration for debugging and optimization control
    config: RenderConfig,

    /// Terminal rendering mode (alternate screen or inline)
    terminal_mode: TerminalMode,

    /// State for inline rendering mode
    inline_state: InlineState,

    /// Effect runtime for managing async tasks
    #[cfg(feature = "effects")]
    effect_runtime: Option<EffectRuntime>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl App {
    /// Creates a new terminal UI application using alternate screen mode (default).
    ///
    /// Initializes the terminal by:
    /// - Enabling raw mode for character-by-character input
    /// - Switching to alternate screen buffer
    /// - Hiding the cursor
    /// - Enabling mouse capture for click events
    ///
    /// The terminal state is automatically restored when the app is dropped.
    pub fn new() -> io::Result<Self> {
        Self::with_mode(TerminalMode::AlternateScreen)
    }

    /// Creates a new terminal UI application with inline rendering mode.
    ///
    /// In inline mode:
    /// - Content renders directly in the terminal (no alternate screen)
    /// - Content persists in terminal history after app exits
    /// - Height is content-based by default (grows to fit, max 24 lines)
    ///
    /// The terminal state is automatically restored when the app is dropped.
    pub fn inline() -> io::Result<Self> {
        Self::with_mode(TerminalMode::Inline(InlineConfig::default()))
    }

    /// Creates a new terminal UI application with custom inline configuration.
    ///
    /// Use this for fine-grained control over inline rendering behavior.
    ///
    /// # Example
    /// ```rust,ignore
    /// use rxtui::{App, InlineConfig, InlineHeight};
    ///
    /// let config = InlineConfig {
    ///     height: InlineHeight::Fixed(10),
    ///     cursor_visible: true,
    ///     preserve_on_exit: true,
    /// };
    /// let app = App::inline_with_config(config)?;
    /// ```
    pub fn inline_with_config(config: InlineConfig) -> io::Result<Self> {
        Self::with_mode(TerminalMode::Inline(config))
    }

    /// Creates a new terminal UI application with the specified terminal mode.
    ///
    /// This is the core constructor that handles both alternate screen and inline modes.
    pub fn with_mode(mode: TerminalMode) -> io::Result<Self> {
        let mut stdout = io::stdout();

        // Always enable raw mode for event handling
        terminal::enable_raw_mode()?;

        // Mode-specific terminal setup
        match &mode {
            TerminalMode::AlternateScreen => {
                stdout.execute(terminal::EnterAlternateScreen)?;
                stdout.execute(cursor::Hide)?;
                stdout.execute(event::EnableMouseCapture)?;
            }
            TerminalMode::Inline(config) => {
                if !config.cursor_visible {
                    stdout.execute(cursor::Hide)?;
                }
                // Only enable mouse capture if explicitly requested
                // Default is false to allow natural terminal scrolling
                if config.mouse_capture {
                    stdout.execute(event::EnableMouseCapture)?;
                }
                // Space reservation happens on first render
            }
        }

        let running = Rc::new(RefCell::new(true));
        let needs_render = Rc::new(RefCell::new(true));

        // Get initial terminal size for double buffer
        let (width, height) = terminal::size()?;

        // Initialize effect runtime if feature is enabled
        #[cfg(feature = "effects")]
        let effect_runtime = Some(EffectRuntime::new());

        Ok(Self {
            vdom: VDom::new(),
            running,
            needs_render,
            double_buffer: DoubleBuffer::new(width, height),
            render_log_fn: None,
            terminal_renderer: TerminalRenderer::new(),
            config: RenderConfig::default(),
            terminal_mode: mode,
            inline_state: InlineState::new(),
            #[cfg(feature = "effects")]
            effect_runtime,
        })
    }

    /// Runs the application with a component instance.
    ///
    /// This uses the component system that provides:
    /// - Component-based architecture
    /// - Message-driven state updates
    /// - Tree expansion from components to VNodes
    ///
    /// ## Example
    /// ```rust,ignore
    /// let mut app = App::new()?;
    /// let root = MyRootComponent::default();
    /// app.run(root)?;
    /// ```
    ///
    /// This method blocks until the application exits.
    pub fn run<C>(&mut self, root_component: C) -> io::Result<()>
    where
        C: Component,
    {
        self.run_loop(root_component)
    }

    /// Sets the render configuration for debugging and optimization control.
    pub fn render_config(mut self, config: RenderConfig) -> Self {
        self.config = config;
        self
    }

    /// Disables all rendering optimizations for debugging.
    /// This is equivalent to calling all disable_* methods.
    pub fn disable_all_optimizations(mut self) -> Self {
        self.config = RenderConfig::debug();
        self
    }

    /// Disables double buffering, causing direct terminal rendering.
    /// Warning: This may cause visible flicker during updates.
    pub fn disable_double_buffering(mut self) -> Self {
        self.config.double_buffering = false;
        self
    }

    /// Disables terminal-specific optimizations.
    /// This uses simpler, more compatible terminal commands.
    pub fn disable_terminal_optimizations(mut self) -> Self {
        self.config.terminal_optimizations = false;
        self
    }

    /// Disables cell-level diffing.
    /// This causes the entire screen to be redrawn on each update.
    pub fn disable_cell_diffing(mut self) -> Self {
        self.config.cell_diffing = false;
        self
    }

    /// Sets the event polling duration in milliseconds.
    /// Lower values make the app more responsive but use more CPU.
    /// Default is 100ms.
    pub fn poll_duration(mut self, duration_ms: u64) -> Self {
        self.config.poll_duration_ms = duration_ms;
        self
    }

    /// Sets the app to use a fast polling rate (10ms).
    /// This makes the app very responsive but uses more CPU.
    pub fn fast_polling(mut self) -> Self {
        self.config.poll_duration_ms = 10;
        self
    }

    /// Sets the app to use a slow polling rate (500ms).
    /// This reduces CPU usage but may feel less responsive.
    pub fn slow_polling(mut self) -> Self {
        self.config.poll_duration_ms = 500;
        self
    }

    /// Main event loop using component-based architecture.
    ///
    /// Manages component state through messages and actions,
    /// expanding component trees into VNode trees for rendering.
    ///
    /// Only renders when:
    /// 1. Initial render
    /// 2. Messages are processed and state changes
    /// 3. External events trigger render
    /// 4. Terminal is resized
    fn run_loop<C>(&mut self, root_component: C) -> io::Result<()>
    where
        C: Component,
    {
        let focus_clear_flag = self.vdom.focus_clear_flag();
        let mut context = Context::new(focus_clear_flag);
        let mut components: HashMap<ComponentId, Arc<dyn Component>> = HashMap::new();

        // Store the root component
        let root_id = ComponentId::default();
        let root_arc = Arc::new(root_component) as Arc<dyn Component>;
        let root_type_id = root_arc.type_id();
        components.insert(root_id.clone(), root_arc.clone());

        let mut needs_render = true; // Initial render

        // Spawn effects for root component ONCE before entering the loop
        #[cfg(feature = "effects")]
        if let Some(runtime) = &self.effect_runtime
            && !context.effect_tracker.has_effects(&root_id, root_type_id)
        {
            let effects = root_arc.effects(&context);
            if !effects.is_empty() {
                runtime.spawn(root_id.clone(), effects);
                context
                    .effect_tracker
                    .mark_spawned(root_id.clone(), root_type_id);
            }
        }

        while *self.running.borrow() {
            // Check if we have pending messages that need processing
            if context.has_pending_messages() {
                needs_render = true;
            }

            // Expand component tree to VNode tree
            let vnode_tree = if let Some(root_component) = components.get(&root_id) {
                context.current_component_id = root_id.clone();
                // Create a temporary clone of components to avoid borrow issues
                let mut temp_components = HashMap::new();

                // Expand the tree, processing messages and handling exit signals
                match self.expand_component_tree(
                    root_component.as_ref(),
                    &mut context,
                    &mut temp_components,
                ) {
                    Ok(vnode) => {
                        // Handle effects for dynamically mounted/unmounted components
                        #[cfg(feature = "effects")]
                        if let Some(runtime) = &self.effect_runtime {
                            // Build a set of current component instances with their types
                            let mut current_instances: HashSet<(ComponentId, std::any::TypeId)> =
                                HashSet::new();
                            for (comp_id, component) in &temp_components {
                                if comp_id != &root_id {
                                    // Skip root, already handled
                                    current_instances
                                        .insert((comp_id.clone(), component.type_id()));
                                }
                            }

                            // Spawn effects for newly mounted components (not root)
                            for (comp_id, component) in &temp_components {
                                // Skip root component as it's already handled
                                if comp_id != &root_id {
                                    let type_id = component.type_id();

                                    // Check if this exact component instance (ID + Type) has effects
                                    if !context.effect_tracker.has_effects(comp_id, type_id) {
                                        // This is a truly new component instance
                                        // CRITICAL: Set the context's component ID so effects send messages to the right component
                                        let original_id = context.current_component_id.clone();
                                        context.current_component_id = comp_id.clone();

                                        let effects = component.effects(&context);
                                        if !effects.is_empty() {
                                            runtime.spawn(comp_id.clone(), effects);
                                            context
                                                .effect_tracker
                                                .mark_spawned(comp_id.clone(), type_id);
                                        }

                                        // Restore original ID
                                        context.current_component_id = original_id;
                                    }
                                }
                            }

                            // Cleanup effects for unmounted components (excluding root)
                            let tracked = context.effect_tracker.get_all();
                            for (comp_id, type_id) in tracked {
                                // Never cleanup root component effects
                                if comp_id == root_id {
                                    continue;
                                }

                                // Check if this component instance is still in the tree
                                if !current_instances.contains(&(comp_id.clone(), type_id)) {
                                    // Component was unmounted or type changed
                                    runtime.cleanup(&comp_id);
                                    context.effect_tracker.remove(&comp_id, type_id);
                                }
                            }
                        }

                        // Merge temp_components back into main components map
                        // This is critical for nested components to receive messages
                        components.extend(temp_components);
                        vnode
                    }
                    Err(ExitSignal) => {
                        *self.running.borrow_mut() = false;
                        break;
                    }
                }
            } else {
                VNode::div()
            };

            // Render if needed
            if needs_render || *self.needs_render.borrow() {
                // Render VNode tree
                self.vdom.render(vnode_tree);

                let focus_requests = context.take_focus_requests();
                self.apply_focus_requests(&context, focus_requests);

                let (width, height) = terminal::size()?;
                self.vdom.layout(width, height);

                self.draw()?;

                // Log render tree if callback is set
                if let Some(log_fn) = &self.render_log_fn {
                    let debug_string = self.render_tree_debug_string();
                    log_fn(&debug_string);
                }

                // Clear render flags
                *self.needs_render.borrow_mut() = false;
                needs_render = false;
            }

            // Poll for events with configurable timeout
            if event::poll(std::time::Duration::from_millis(
                self.config.poll_duration_ms,
            ))? {
                match event::read()? {
                    Event::Key(key_event) => {
                        handle_key_event(&self.vdom, key_event);
                        // Key events may have triggered messages via event handlers
                        needs_render = true;
                    }
                    Event::Mouse(mouse_event) => {
                        handle_mouse_event(&self.vdom, mouse_event);
                        // Mouse events may have triggered messages via event handlers
                        needs_render = true;
                    }
                    Event::Resize(width, height) => {
                        match &self.terminal_mode {
                            TerminalMode::AlternateScreen => {
                                // Full re-layout and screen clear for alternate screen
                                self.vdom.layout(width, height);
                                self.double_buffer.resize(width, height);
                                self.double_buffer.reset();
                                self.terminal_renderer.clear_screen()?;
                            }
                            TerminalMode::Inline(_) => {
                                // For inline mode, just update terminal size tracking
                                // Height is managed by space reservation, width changes trigger re-render
                                self.inline_state.terminal_size = (width, height);
                                // Don't clear screen - we're rendering in reserved space
                            }
                        }
                        *self.needs_render.borrow_mut() = true;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Expands a component tree into a VNode tree recursively
    fn expand_component_tree(
        &self,
        component: &dyn Component,
        context: &mut Context,
        components: &mut HashMap<ComponentId, Arc<dyn Component>>,
    ) -> Result<VNode, ExitSignal> {
        // Process all pending messages (regular, owned topics, and unassigned topics)
        let messages = context.drain_all_messages();
        for (msg, topic) in messages {
            let action = component.update(context, msg, topic.as_deref());

            match action {
                Action::Update(new_state) => {
                    context
                        .states
                        .insert(context.current_component_id.clone(), new_state);

                    // If this was an unassigned topic message and we handled it, claim the topic
                    if let Some(topic_name) = topic
                        && context
                            .topics
                            .claim_topic(topic_name.clone(), context.current_component_id.clone())
                    {
                        // We just claimed this topic, drain its remaining messages
                        context.drain_topic_if_claimed(&topic_name, &context.current_component_id);
                    }
                }
                Action::UpdateTopic(topic_name, new_state) => {
                    // Update topic state (idempotent - first writer becomes owner)
                    context.topics.update_topic(
                        topic_name.clone(),
                        new_state,
                        context.current_component_id.clone(),
                    );

                    // If this was an unassigned topic message for the same topic, drain it
                    if let Some(msg_topic) = topic
                        && msg_topic == topic_name
                    {
                        context.drain_topic_if_claimed(&topic_name, &context.current_component_id);
                    }
                }
                Action::Exit => {
                    return Err(ExitSignal);
                }
                Action::None => {
                    // Component didn't handle this message, leave topic unassigned
                }
            }
        }

        // Get the node from the component's view
        context.begin_component_render();
        let node = component.view(context);
        context.end_component_render();

        // Convert Node to VNode, expanding any nested components
        self.node_to_vnode(node, context, components, 0)
    }

    /// Converts a Node to a VNode, expanding components recursively
    fn node_to_vnode(
        &self,
        node: Node,
        context: &mut Context,
        components: &mut HashMap<ComponentId, Arc<dyn Component>>,
        child_index: usize,
    ) -> Result<VNode, ExitSignal> {
        match node {
            Node::Component(component) => {
                // Update context for this component
                let parent_id = context.current_component_id.clone();
                context.current_component_id = parent_id.child(child_index);

                // Store component in the map
                let component_id = context.current_component_id.clone();

                // Expand the component recursively, propagating any exit signal
                let vnode = self.expand_component_tree(component.as_ref(), context, components)?;

                // Store the component for future updates
                components.insert(component_id, Arc::clone(&component));

                // Restore parent context
                context.current_component_id = parent_id;

                Ok(vnode)
            }
            Node::Div(div) => {
                // Track the path through divs to ensure unique component IDs
                let parent_id = context.current_component_id.clone();
                context.current_component_id = parent_id.child(child_index);

                // Convert div children
                let mut vnode_children = Vec::new();
                for (i, child) in div.children.into_iter().enumerate() {
                    // Propagate any exit signal from children
                    vnode_children.push(self.node_to_vnode(child, context, components, i)?);
                }

                // Restore parent context after processing div children
                context.current_component_id = parent_id.clone();

                // Create VNode div with converted children
                let mut vnode_div = Div::new();
                vnode_div.children = vnode_children;

                // Copy over the style and event properties
                vnode_div.styles = div.styles;
                vnode_div.events = div.events;
                vnode_div.focusable = div.focusable;
                vnode_div.focused = div.focused;
                vnode_div.hovered = div.hovered;
                vnode_div.component_path = Some(parent_id);

                Ok(VNode::Div(vnode_div))
            }
            Node::Text(text) => {
                // Text nodes are directly converted
                Ok(VNode::Text(text))
            }
            Node::RichText(rich) => {
                // RichText nodes are directly converted
                Ok(VNode::RichText(rich))
            }
        }
    }

    /// Returns a debug string representation of the current render tree.
    ///
    /// This is useful for debugging and logging the UI structure.
    pub fn render_tree_debug_string(&self) -> String {
        self.vdom.get_render_tree().debug_string()
    }

    /// Sets a callback function to be called after each render with the render tree debug string.
    ///
    /// This is useful for logging the render tree state for debugging purposes.
    pub fn set_render_log_fn<F: Fn(&str) + 'static>(&mut self, log_fn: F) {
        self.render_log_fn = Some(Box::new(log_fn));
    }

    /// Applies any focus requests that were queued during the render cycle.
    fn apply_focus_requests(&self, context: &Context, requests: Vec<FocusRequest>) {
        let render_tree = self.vdom.get_render_tree();
        let mut focus_applied = false;

        for request in requests {
            match request.target {
                FocusTarget::Component(component_id) => {
                    if let Some(root) = render_tree.find_component_root(&component_id)
                        && let Some(target) = render_tree.find_first_focusable_in(&root)
                    {
                        render_tree.set_focused_node(Some(target));
                        focus_applied = true;
                    }
                }
                FocusTarget::GlobalFirst => {
                    if let Some(target) = render_tree.find_first_focusable_global() {
                        render_tree.set_focused_node(Some(target));
                        focus_applied = true;
                    }
                }
            }
        }

        if focus_applied {
            context.cancel_focus_clear();
        }

        if context.take_focus_clear_request() {
            render_tree.set_focused_node(None);
        }
    }

    /// Renders the current UI tree to the terminal.
    ///
    /// Dispatches to the appropriate rendering method based on terminal mode:
    /// - AlternateScreen: Uses double buffering for flicker-free full-screen rendering
    /// - Inline: Renders to a reserved region in the main terminal buffer
    fn draw(&mut self) -> io::Result<()> {
        match &self.terminal_mode {
            TerminalMode::AlternateScreen => {
                if self.config.double_buffering {
                    self.draw_with_double_buffer()
                } else {
                    self.draw_direct()
                }
            }
            TerminalMode::Inline(config) => {
                // Clone config to avoid borrow issues
                let config = config.clone();
                self.draw_inline(&config)
            }
        }
    }

    /// Draws in inline mode with space reservation.
    fn draw_inline(&mut self, config: &InlineConfig) -> io::Result<()> {
        use std::io::Write;
        let mut stdout = io::stdout();

        // Get terminal dimensions
        let (term_width, term_height) = terminal::size()?;

        // Determine if we should use unclamped height
        let unclamped = matches!(config.height, InlineHeight::Content { .. });

        // For layout, always use full terminal height to ensure proper child layout.
        // The render_height (below) will handle the actual clipping.
        // For Fixed mode, use the fixed height for layout too.
        let layout_height = match &config.height {
            InlineHeight::Fixed(h) => *h,
            InlineHeight::Content { .. } | InlineHeight::Fill { .. } => term_height,
        };

        // Layout with full dimensions - unclamped allows root to grow beyond viewport
        self.vdom
            .layout_with_options(term_width, layout_height, unclamped);

        // Get actual content height from rendered tree
        let content_height = self
            .vdom
            .get_render_tree()
            .root
            .as_ref()
            .map(|r| r.borrow().height)
            .unwrap_or(1);

        // Apply height limits from config
        let render_height = match &config.height {
            InlineHeight::Fixed(h) => *h,
            InlineHeight::Content { max } => {
                max.map(|m| content_height.min(m)).unwrap_or(content_height)
            }
            InlineHeight::Fill { min } => content_height.max(*min),
        };

        // Ensure we have at least 1 line
        let render_height = render_height.max(1);

        // Initialize or expand space reservation
        if !self.inline_state.initialized {
            self.inline_state
                .reserve_space(&mut stdout, render_height)?;
        } else if render_height > self.inline_state.reserved_height {
            self.inline_state.expand_space(&mut stdout, render_height)?;
        }

        // Resize double buffer to match render dimensions
        if self.double_buffer.back_buffer_mut().dimensions() != (term_width, render_height) {
            self.double_buffer.resize(term_width, render_height);
            self.double_buffer.reset();
        }

        // Clear the back buffer
        self.double_buffer.clear_back();

        // Render the tree to the back buffer
        if let Some(root) = &self.vdom.get_render_tree().root {
            let root_ref = root.borrow();
            let buffer = self.double_buffer.back_buffer_mut();
            let clip_rect = Rect::new(0, 0, term_width, render_height);
            render_node_to_buffer(&root_ref, buffer, &clip_rect, None);
        }

        // Diff and apply updates with origin offset
        let updates = self.double_buffer.diff();
        self.terminal_renderer
            .apply_updates_inline(updates, self.inline_state.origin_row)?;

        // Swap buffers
        self.double_buffer.swap();

        // Clear dirty flags
        self.vdom.get_render_tree().clear_all_dirty();

        stdout.flush()?;
        Ok(())
    }

    /// Draws using double buffering and cell diffing for optimal performance.
    fn draw_with_double_buffer(&mut self) -> io::Result<()> {
        // Clear the back buffer
        self.double_buffer.clear_back();

        // Render the tree to the back buffer
        if let Some(root) = &self.vdom.get_render_tree().root {
            let root_ref = root.borrow();
            let buffer = self.double_buffer.back_buffer_mut();
            let (width, height) = buffer.dimensions();
            let clip_rect = Rect::new(0, 0, width, height);
            render_node_to_buffer(&root_ref, buffer, &clip_rect, None);
        }

        if self.config.cell_diffing {
            // Diff the buffers to find changes
            let updates = self.double_buffer.diff();

            // Apply updates to terminal
            if self.config.terminal_optimizations {
                self.terminal_renderer.apply_updates(updates)?;
            } else {
                // Apply updates without optimizations
                self.terminal_renderer.apply_updates_direct(updates)?;
            }
        } else {
            // Redraw entire screen without diffing
            let buffer = self.double_buffer.back_buffer_mut();
            self.terminal_renderer.draw_full_buffer(buffer)?;
        }

        // Swap buffers for next frame
        self.double_buffer.swap();

        // Clear all dirty flags after drawing
        self.vdom.get_render_tree().clear_all_dirty();

        Ok(())
    }

    /// Draws directly to terminal without double buffering (for debugging).
    fn draw_direct(&mut self) -> io::Result<()> {
        // Clear screen
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        // Create a temporary buffer for direct rendering
        let (width, height) = terminal::size()?;
        let mut buffer = ScreenBuffer::new(width, height);

        // Render the tree to the temporary buffer
        if let Some(root) = &self.vdom.get_render_tree().root {
            let root_ref = root.borrow();
            let clip_rect = Rect::new(0, 0, width, height);
            render_node_to_buffer(&root_ref, &mut buffer, &clip_rect, None);
        }

        // Draw each cell directly to terminal
        let mut stdout = io::stdout();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    execute!(stdout, cursor::MoveTo(x, y))?;

                    // Set colors if present
                    if let Some(fg) = &cell.fg {
                        execute!(
                            stdout,
                            SetForegroundColor(self.terminal_renderer.color_to_crossterm(*fg))
                        )?;
                    }
                    if let Some(bg) = &cell.bg {
                        execute!(
                            stdout,
                            SetBackgroundColor(self.terminal_renderer.color_to_crossterm(*bg))
                        )?;
                    }

                    // Print character
                    execute!(stdout, Print(cell.char))?;

                    // Reset colors
                    if cell.fg.is_some() || cell.bg.is_some() {
                        execute!(stdout, ResetColor)?;
                    }
                }
            }
        }

        // Clear all dirty flags after drawing
        self.vdom.get_render_tree().clear_all_dirty();

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

/// Cleanup handler that restores terminal state on application exit.
///
/// Automatically:
/// - Disables mouse capture
/// - Shows the cursor
/// - Returns to main screen buffer (alternate screen mode only)
/// - Moves cursor below content (inline mode with preserve_on_exit)
/// - Disables raw mode
impl Drop for App {
    fn drop(&mut self) {
        use std::io::Write;

        let mut stdout = io::stdout();

        // Show cursor for both modes
        let _ = stdout.execute(cursor::Show);

        // Mode-specific cleanup
        match &self.terminal_mode {
            TerminalMode::AlternateScreen => {
                let _ = stdout.execute(event::DisableMouseCapture);
                let _ = stdout.execute(terminal::LeaveAlternateScreen);
            }
            TerminalMode::Inline(config) => {
                // Disable mouse capture if it was enabled
                if config.mouse_capture {
                    let _ = stdout.execute(event::DisableMouseCapture);
                }
                if config.preserve_on_exit {
                    // Move cursor below rendered content so shell prompt appears after
                    let _ = self.inline_state.move_to_end(&mut stdout);
                } else {
                    // Clear the inline rendering area
                    let _ = self.terminal_renderer.clear_lines(
                        self.inline_state.origin_row,
                        self.inline_state.reserved_height,
                    );
                    // Move cursor back to origin
                    let _ = stdout.execute(cursor::MoveTo(
                        self.inline_state.origin_col,
                        self.inline_state.origin_row,
                    ));
                }
            }
        }

        // Flush to ensure all commands are sent before disabling raw mode
        let _ = stdout.flush();

        // Finally disable raw mode
        let _ = terminal::disable_raw_mode();
    }
}
