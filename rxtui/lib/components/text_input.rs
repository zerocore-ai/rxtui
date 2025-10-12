use crate::component::{Action, Component, Message, MessageExt};
use crate::key::{Key, KeyWithModifiers};
use crate::node::Node;
use crate::node::{DivStyles, RichText, Text};
use crate::style::{
    Border, BorderEdges, BorderStyle, Color, Dimension, Overflow, Position, Spacing, Style,
    TextStyle, TextWrap,
};
use crate::{Context, Div};
use std::any::Any;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Messages for TextInput component
#[derive(Debug, Clone)]
pub enum TextInputMsg {
    /// Component gained focus
    Focused,

    /// Component lost focus
    Blurred,

    /// Character input received
    CharInput(char),

    /// Backspace key pressed
    Backspace,

    /// Delete key pressed
    Delete,

    /// Delete word backward (Ctrl+W, Alt+Backspace)
    DeleteWordBackward,

    /// Delete word forward (Alt+D)
    DeleteWordForward,

    /// Delete to beginning of line (Ctrl+U)
    DeleteToLineStart,

    /// Delete to end of line (Ctrl+K)
    DeleteToLineEnd,

    /// Cursor movement
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    CursorWordLeft,
    CursorWordRight,

    /// Selection operations
    SelectLeft,
    SelectRight,
    SelectAll,
    SelectWord,
    ClearSelection,

    /// Edit operations
    Cut,
    Copy,
    Paste(String),

    /// Submit (Enter key)
    Submit,

    /// Clear the input content
    Clear,
}

/// State for TextInput component
#[derive(Debug, Clone, Default)]
pub struct TextInputState {
    /// Whether the input is currently focused
    pub focused: bool,

    /// The current input content
    pub content: String,

    /// Current cursor position (in characters, not bytes)
    pub cursor_position: usize,

    /// Start of selection (None if no selection)
    pub selection_start: Option<usize>,

    /// End of selection (None if no selection)
    pub selection_end: Option<usize>,
}

/// A text input component for user text entry with sensible defaults
///
/// TextInput comes with default styling:
/// - Cyan single-line border
/// - Width of 30 cells
/// - Height of 3 cells (to accommodate border)
/// - Horizontal padding of 1 cell
/// - Placeholder text in italic grey (BrightBlack)
/// - Content text in normal white
///
/// All defaults can be overridden using the builder methods.
/// Supports full text editing with keyboard input, placeholder styling,
/// content styling, and focus styling.
///
/// # Basic Example
///
/// ```ignore
/// use rxtui::prelude::*;
///
/// // Uses default cyan border and 30x3 size
/// let input = TextInput::new()
///     .placeholder("Enter your name...");
/// ```
///
/// # Customization Example
///
/// ```ignore
/// use rxtui::prelude::*;
///
/// // Override specific properties while keeping other defaults
/// let input = TextInput::new()
///     .placeholder("Custom input...")
///     .border(Color::Yellow)  // Override border color
///     .width(50)              // Override width
///     .focus_border(Color::Green)
///     .focus_background(Color::Rgb(0, 50, 0));
/// ```
///
/// # Placeholder Styling Example
///
/// ```ignore
/// use rxtui::prelude::*;
///
/// // Customize placeholder text appearance
/// let input = TextInput::new()
///     .placeholder("Enter email...")
///     .placeholder_color(Color::Blue)    // Override default grey
///     .placeholder_italic(false)         // Remove italic style
///     .placeholder_bold(true);            // Make it bold instead
/// ```
///
/// # Content Styling Example
///
/// ```ignore
/// use rxtui::prelude::*;
///
/// // Customize the typed content appearance
/// let input = TextInput::new()
///     .content_color(Color::Green)       // Green text when typing
///     .content_bold(true)                // Bold typed text
///     .placeholder("Type here...");      // Placeholder remains default style
/// ```
pub struct TextInput {
    placeholder: Option<String>,
    placeholder_style: Option<TextStyle>,
    content_style: Option<TextStyle>,
    cursor_style: Option<TextStyle>,
    selection_style: Option<TextStyle>,
    styles: DivStyles,
    focusable: bool,
    wrap: Option<TextWrap>,
    password_mode: bool,
    clear_on_submit: bool,
    on_change: Option<Box<dyn Fn(String)>>,
    on_submit: Option<Box<dyn Fn()>>,
    on_blur: Option<Box<dyn Fn()>>,
    key_handlers: Vec<(Key, Rc<dyn Fn()>)>,
    key_global_handlers: Vec<(Key, Rc<dyn Fn()>)>,
    key_with_modifiers_handlers: Vec<(KeyWithModifiers, Rc<dyn Fn()>)>,
    key_with_modifiers_global_handlers: Vec<(KeyWithModifiers, Rc<dyn Fn()>)>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl TextInput {
    /// Helper to delete selected text
    fn delete_selection(&self, state: &mut TextInputState) {
        if let (Some(start), Some(end)) = (state.selection_start, state.selection_end) {
            let (start, end) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            let mut chars: Vec<char> = state.content.chars().collect();
            chars.drain(start..end);
            state.content = chars.into_iter().collect();
            state.cursor_position = start;
            state.selection_start = None;
            state.selection_end = None;
        }
    }

    /// Find the previous word boundary from the given position
    fn find_word_boundary_left(&self, text: &str, pos: usize) -> usize {
        let chars: Vec<char> = text.chars().collect();
        if pos == 0 {
            return 0;
        }

        let mut new_pos = pos - 1;

        // Skip whitespace
        while new_pos > 0 && chars[new_pos].is_whitespace() {
            new_pos -= 1;
        }

        // Skip word characters
        while new_pos > 0
            && !chars[new_pos - 1].is_whitespace()
            && !chars[new_pos - 1].is_ascii_punctuation()
        {
            new_pos -= 1;
        }

        new_pos
    }

    /// Find the next word boundary from the given position
    fn find_word_boundary_right(&self, text: &str, pos: usize) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();

        if pos >= len {
            return len;
        }

        let mut new_pos = pos;

        // Skip current word
        while new_pos < len
            && !chars[new_pos].is_whitespace()
            && !chars[new_pos].is_ascii_punctuation()
        {
            new_pos += 1;
        }

        // Skip whitespace and punctuation
        while new_pos < len
            && (chars[new_pos].is_whitespace() || chars[new_pos].is_ascii_punctuation())
        {
            new_pos += 1;
        }

        new_pos
    }

    /// Delete word backward from cursor position
    fn delete_word_backward(&self, state: &mut TextInputState) {
        if state.cursor_position == 0 {
            return;
        }

        let new_pos = self.find_word_boundary_left(&state.content, state.cursor_position);
        let mut chars: Vec<char> = state.content.chars().collect();
        chars.drain(new_pos..state.cursor_position);
        state.content = chars.into_iter().collect();
        state.cursor_position = new_pos;
    }

    /// Delete word forward from cursor position
    fn delete_word_forward(&self, state: &mut TextInputState) {
        let char_count = state.content.chars().count();
        if state.cursor_position >= char_count {
            return;
        }

        let new_pos = self.find_word_boundary_right(&state.content, state.cursor_position);
        let mut chars: Vec<char> = state.content.chars().collect();
        chars.drain(state.cursor_position..new_pos);
        state.content = chars.into_iter().collect();
    }

    /// Delete from cursor to beginning of line
    fn delete_to_line_start(&self, state: &mut TextInputState) {
        if state.cursor_position == 0 {
            return;
        }

        let mut chars: Vec<char> = state.content.chars().collect();
        chars.drain(0..state.cursor_position);
        state.content = chars.into_iter().collect();
        state.cursor_position = 0;
    }

    /// Delete from cursor to end of line
    fn delete_to_line_end(&self, state: &mut TextInputState) {
        let char_count = state.content.chars().count();
        if state.cursor_position >= char_count {
            return;
        }

        let mut chars: Vec<char> = state.content.chars().collect();
        chars.drain(state.cursor_position..);
        state.content = chars.into_iter().collect();
    }
    /// Creates the default style for TextInput components
    fn default_style() -> Style {
        Style {
            padding: Some(Spacing::horizontal(1)),
            width: Some(Dimension::Fixed(30)),
            height: Some(Dimension::Fixed(3)),
            border: Some(Border {
                enabled: true,
                style: BorderStyle::Single,
                color: Color::Cyan,
                edges: BorderEdges::ALL,
            }),
            overflow: Some(Overflow::Hidden),
            ..Default::default()
        }
    }

    /// Creates the default placeholder text style (italic, grey)
    fn default_placeholder_style() -> TextStyle {
        TextStyle {
            color: Some(Color::BrightBlack), // Grey color
            background: None,
            bold: None,
            italic: Some(true), // Italic style
            underline: None,
            strikethrough: None,
            wrap: None,
            align: None,
        }
    }

    /// Creates the default content text style (normal white text)
    fn default_content_style() -> TextStyle {
        TextStyle {
            color: Some(Color::White), // Normal white text
            background: None,
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
            wrap: None,
            align: None,
        }
    }

    /// Creates the default cursor style (inverted background)
    fn default_cursor_style() -> TextStyle {
        TextStyle {
            color: Some(Color::Black),
            background: Some(Color::White),
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
            wrap: None,
            align: None,
        }
    }

    /// Creates the default selection style (blue background)
    fn default_selection_style() -> TextStyle {
        TextStyle {
            color: Some(Color::White),
            background: Some(Color::Blue),
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
            wrap: None,
            align: None,
        }
    }

    /// Creates a new TextInput component with default styling
    pub fn new() -> Self {
        Self {
            placeholder: None,
            placeholder_style: Some(Self::default_placeholder_style()),
            content_style: Some(Self::default_content_style()),
            cursor_style: Some(Self::default_cursor_style()),
            selection_style: Some(Self::default_selection_style()),
            styles: DivStyles {
                base: Some(Self::default_style()),
                focus: None,
                hover: None,
            },
            focusable: true,                 // Text inputs are focusable by default
            wrap: Some(TextWrap::WordBreak), // Default to WordBreak for better text wrapping
            password_mode: false,            // Default to normal text mode
            clear_on_submit: false,          // Default to not clearing on submit
            on_change: None,
            on_submit: None,
            on_blur: None,
            key_handlers: Vec::new(),
            key_global_handlers: Vec::new(),
            key_with_modifiers_handlers: Vec::new(),
            key_with_modifiers_global_handlers: Vec::new(),
        }
    }

    /// Sets the placeholder text to display when the input is empty
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Sets whether this input can receive focus
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    /// Enables password mode which masks the input content
    pub fn password(mut self, password: bool) -> Self {
        self.password_mode = password;
        self
    }

    /// Enables automatic clearing of input content on submit (Enter key)
    pub fn clear_on_submit(mut self, clear: bool) -> Self {
        self.clear_on_submit = clear;
        self
    }

    /// Sets the callback to be called when the input content changes
    pub fn on_change(mut self, callback: impl Fn(String) + 'static) -> Self {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Sets the callback to be called when Enter is pressed
    pub fn on_submit(mut self, callback: impl Fn() + 'static) -> Self {
        self.on_submit = Some(Box::new(callback));
        self
    }

    /// Sets the callback to be called when the input loses focus
    pub fn on_blur(mut self, callback: impl Fn() + 'static) -> Self {
        self.on_blur = Some(Box::new(callback));
        self
    }

    /// Registers a key handler that fires when the input is focused.
    pub fn on_key(mut self, key: Key, handler: impl Fn() + 'static) -> Self {
        self.key_handlers.push((key, Rc::new(handler)));
        self
    }

    /// Registers a global key handler that fires regardless of focus state.
    pub fn on_key_global(mut self, key: Key, handler: impl Fn() + 'static) -> Self {
        self.key_global_handlers.push((key, Rc::new(handler)));
        self
    }

    /// Registers a key handler that includes modifier state (Ctrl/Alt/Shift/Meta).
    pub fn on_key_with_modifiers(
        mut self,
        key: KeyWithModifiers,
        handler: impl Fn() + 'static,
    ) -> Self {
        self.key_with_modifiers_handlers
            .push((key, Rc::new(handler)));
        self
    }

    /// Registers a global modifier-aware key handler that fires regardless of focus state.
    pub fn on_key_with_modifiers_global(
        mut self,
        key: KeyWithModifiers,
        handler: impl Fn() + 'static,
    ) -> Self {
        self.key_with_modifiers_global_handlers
            .push((key, Rc::new(handler)));
        self
    }

    fn update(&self, ctx: &Context, msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        if let Some(msg) = msg.downcast::<TextInputMsg>() {
            let mut state = ctx.get_state::<TextInputState>();

            match msg {
                TextInputMsg::Focused => {
                    state.focused = true;
                    // Move cursor to end when gaining focus
                    state.cursor_position = state.content.chars().count();
                }
                TextInputMsg::Blurred => {
                    state.focused = false;
                    // Clear selection when losing focus
                    state.selection_start = None;
                    state.selection_end = None;

                    if let Some(callback) = &self.on_blur {
                        callback();
                    }
                }
                TextInputMsg::CharInput(ch) => {
                    // Only accept input when focused
                    if state.focused {
                        // Check for ESC sequences that indicate Alt+key combinations
                        // Alt+b and Alt+f are common word navigation shortcuts
                        // These often come through as ESC followed by the character

                        // For now, just handle regular character input
                        // Clear selection and insert at cursor
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        }

                        // Convert to char indices
                        let char_pos = state.cursor_position;
                        let mut chars: Vec<char> = state.content.chars().collect();

                        // Insert character at cursor position
                        if char_pos <= chars.len() {
                            chars.insert(char_pos, *ch);
                            state.content = chars.into_iter().collect();
                            state.cursor_position += 1;

                            // Call on_change callback
                            if let Some(callback) = &self.on_change {
                                callback(state.content.clone());
                            }
                        }
                    }
                }
                TextInputMsg::Backspace => {
                    // Only process backspace when focused
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else if state.cursor_position > 0 {
                            // Delete character before cursor
                            let mut chars: Vec<char> = state.content.chars().collect();
                            chars.remove(state.cursor_position - 1);
                            state.content = chars.into_iter().collect();
                            state.cursor_position -= 1;

                            // Call on_change callback
                            if let Some(callback) = &self.on_change {
                                callback(state.content.clone());
                            }
                        }
                    }
                }
                TextInputMsg::Delete => {
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else {
                            // Delete character after cursor
                            let mut chars: Vec<char> = state.content.chars().collect();
                            if state.cursor_position < chars.len() {
                                chars.remove(state.cursor_position);
                                state.content = chars.into_iter().collect();

                                // Call on_change callback
                                if let Some(callback) = &self.on_change {
                                    callback(state.content.clone());
                                }
                            }
                        }
                    }
                }
                TextInputMsg::DeleteWordBackward => {
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else {
                            self.delete_word_backward(&mut state);
                            // Call on_change callback
                            if let Some(callback) = &self.on_change {
                                callback(state.content.clone());
                            }
                        }
                    }
                }
                TextInputMsg::DeleteWordForward => {
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else {
                            self.delete_word_forward(&mut state);
                        }
                    }
                }
                TextInputMsg::DeleteToLineStart => {
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else {
                            self.delete_to_line_start(&mut state);
                        }
                    }
                }
                TextInputMsg::DeleteToLineEnd => {
                    if state.focused {
                        if state.selection_start.is_some() {
                            self.delete_selection(&mut state);
                        } else {
                            self.delete_to_line_end(&mut state);
                        }
                    }
                }
                TextInputMsg::CursorLeft => {
                    if state.focused && state.cursor_position > 0 {
                        state.cursor_position -= 1;
                        // Clear selection when moving cursor
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                TextInputMsg::CursorRight => {
                    if state.focused {
                        let char_count = state.content.chars().count();
                        if state.cursor_position < char_count {
                            state.cursor_position += 1;
                        }
                        // Clear selection when moving cursor
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                TextInputMsg::CursorHome => {
                    if state.focused {
                        state.cursor_position = 0;
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                TextInputMsg::CursorEnd => {
                    if state.focused {
                        state.cursor_position = state.content.chars().count();
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                TextInputMsg::CursorWordLeft => {
                    if state.focused {
                        state.cursor_position =
                            self.find_word_boundary_left(&state.content, state.cursor_position);
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                TextInputMsg::CursorWordRight => {
                    if state.focused {
                        state.cursor_position =
                            self.find_word_boundary_right(&state.content, state.cursor_position);
                        state.selection_start = None;
                        state.selection_end = None;
                    }
                }
                // TODO: Implement selection operations
                TextInputMsg::SelectLeft
                | TextInputMsg::SelectRight
                | TextInputMsg::SelectAll
                | TextInputMsg::SelectWord
                | TextInputMsg::ClearSelection => {
                    // Will be implemented when we add selection support
                }
                // TODO: Implement clipboard operations
                TextInputMsg::Cut | TextInputMsg::Copy | TextInputMsg::Paste(_) => {
                    // Will be implemented when we add clipboard support
                }
                TextInputMsg::Submit => {
                    // Call on_submit callback when Enter is pressed
                    if let Some(callback) = &self.on_submit {
                        callback();
                    }

                    // Clear content if clear_on_submit is enabled
                    if self.clear_on_submit {
                        state.content.clear();
                        state.cursor_position = 0;
                        state.selection_start = None;
                        state.selection_end = None;

                        // Call on_change callback to notify of cleared content
                        if let Some(callback) = &self.on_change {
                            callback(state.content.clone());
                        }
                    }
                }
                TextInputMsg::Clear => {
                    state.content.clear();
                    state.cursor_position = 0;
                    state.selection_start = None;
                    state.selection_end = None;

                    // Call on_change callback
                    if let Some(callback) = &self.on_change {
                        callback(state.content.clone());
                    }
                }
            }

            return Action::update(state);
        }

        Action::none()
    }

    fn view(&self, ctx: &Context) -> Node {
        let state = ctx.get_state::<TextInputState>();

        // Create a div and apply our stored styles
        let mut container = Div::new();

        // Apply base style if we have one
        if let Some(base) = &self.styles.base {
            container = container.style(base.clone());
        }

        // Apply focus style if we have one
        if let Some(focus) = &self.styles.focus {
            container = container.focus_style(focus.clone());
        }

        // Apply hover style if we have one
        if let Some(hover) = &self.styles.hover {
            container = container.hover_style(hover.clone());
        }

        // Set focusable
        if self.focusable {
            container = container.focusable(true);
        }

        // Add event handlers
        container = container
            .on_focus(ctx.handler(TextInputMsg::Focused))
            .on_blur(ctx.handler(TextInputMsg::Blurred))
            .on_key(Key::Backspace, ctx.handler(TextInputMsg::Backspace))
            .on_key(Key::Delete, ctx.handler(TextInputMsg::Delete))
            .on_key(Key::Left, ctx.handler(TextInputMsg::CursorLeft))
            .on_key(Key::Right, ctx.handler(TextInputMsg::CursorRight))
            .on_key(Key::Home, ctx.handler(TextInputMsg::CursorHome))
            .on_key(Key::End, ctx.handler(TextInputMsg::CursorEnd))
            .on_key(Key::Enter, ctx.handler(TextInputMsg::Submit))
            // Add modifier-aware handlers for word navigation
            // Terminals send Alt+B/F as Char('b'/'f') with ALT, not Arrow keys
            .on_key_with_modifiers(
                KeyWithModifiers::with_alt(Key::Char('b')),
                ctx.handler(TextInputMsg::CursorWordLeft),
            )
            .on_key_with_modifiers(
                KeyWithModifiers::with_alt(Key::Char('f')),
                ctx.handler(TextInputMsg::CursorWordRight),
            )
            // On macOS, Cmd+Left/Right don't send modifier info properly
            // Use Ctrl+A/E which work cross-platform
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('a')),
                ctx.handler(TextInputMsg::CursorHome),
            )
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('e')),
                ctx.handler(TextInputMsg::CursorEnd),
            )
            // On other platforms, Ctrl+B/F for cursor navigation (Emacs-style)
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('b')),
                ctx.handler(TextInputMsg::CursorLeft),
            )
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('f')),
                ctx.handler(TextInputMsg::CursorRight),
            )
            // Word deletion handlers
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('w')),
                ctx.handler(TextInputMsg::DeleteWordBackward),
            )
            .on_key_with_modifiers(
                KeyWithModifiers::with_alt(Key::Char('d')),
                ctx.handler(TextInputMsg::DeleteWordForward),
            )
            // Alt+Backspace for word backward deletion (macOS Option+Delete)
            .on_key_with_modifiers(
                KeyWithModifiers::with_alt(Key::Backspace),
                ctx.handler(TextInputMsg::DeleteWordBackward),
            )
            // Alt+Delete for word forward deletion (macOS Option+Fn+Delete if supported)
            .on_key_with_modifiers(
                KeyWithModifiers::with_alt(Key::Delete),
                ctx.handler(TextInputMsg::DeleteWordForward),
            )
            // Command+Delete for delete to line start (macOS)
            .on_key_with_modifiers(
                KeyWithModifiers {
                    key: Key::Backspace,
                    ctrl: false,
                    alt: false,
                    shift: false,
                    meta: true,
                },
                ctx.handler(TextInputMsg::DeleteToLineStart),
            )
            // Line deletion handlers
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('u')),
                ctx.handler(TextInputMsg::DeleteToLineStart),
            )
            .on_key_with_modifiers(
                KeyWithModifiers::with_ctrl(Key::Char('k')),
                ctx.handler(TextInputMsg::DeleteToLineEnd),
            )
            .on_any_char(ctx.handler_with_value(|ch| {
                // Only handle regular character input
                // Control sequences are handled by on_key_with_modifiers above
                TextInputMsg::CharInput(ch)
            }));

        for (key_with_modifiers, handler) in &self.key_with_modifiers_handlers {
            let handler = handler.clone();
            container =
                container.on_key_with_modifiers(*key_with_modifiers, move || (handler.as_ref())());
        }

        for (key, handler) in &self.key_handlers {
            let handler = handler.clone();
            container = container.on_key(*key, move || (handler.as_ref())());
        }

        for (key_with_modifiers, handler) in &self.key_with_modifiers_global_handlers {
            let handler = handler.clone();
            container = container
                .on_key_with_modifiers_global(*key_with_modifiers, move || (handler.as_ref())());
        }

        for (key, handler) in &self.key_global_handlers {
            let handler = handler.clone();
            container = container.on_key_global(*key, move || (handler.as_ref())());
        }

        // Display content if present, otherwise show placeholder
        if !state.content.is_empty() || state.focused {
            // Mask content if in password mode
            let display_content = if self.password_mode {
                "•".repeat(state.content.chars().count())
            } else {
                state.content.clone()
            };

            // Show the actual content with cursor when focused
            let node = if state.focused {
                // Use RichText with cursor
                let cursor_style = self
                    .cursor_style
                    .clone()
                    .unwrap_or_else(Self::default_cursor_style);
                let mut rich_text =
                    RichText::with_cursor(&display_content, state.cursor_position, cursor_style);

                // Apply wrapping if specified
                if let Some(wrap) = self.wrap {
                    rich_text = rich_text.wrap(wrap);
                }

                // Apply content style to non-cursor spans
                if let Some(content_style) = &self.content_style {
                    for span in &mut rich_text.spans {
                        if span.style.is_none() || span.style.as_ref().unwrap().background.is_none()
                        {
                            span.style = Some(content_style.clone());
                        }
                    }
                }

                rich_text.into()
            } else if !state.content.is_empty() {
                // Show content without cursor when not focused
                let mut text = Text::new(display_content.clone());
                let default_style = Self::default_content_style();
                let final_style = TextStyle::merge(Some(default_style), self.content_style.clone());

                if let Some(style) = final_style {
                    text.style = Some(style.clone());
                }

                // Apply wrapping
                if let Some(wrap) = self.wrap {
                    text.style.get_or_insert(TextStyle::default()).wrap = Some(wrap);
                }

                text.into()
            } else {
                // Empty but focused - show just cursor
                let cursor_style = self
                    .cursor_style
                    .clone()
                    .unwrap_or_else(Self::default_cursor_style);
                let rich_text = RichText::with_cursor("", 0, cursor_style);
                rich_text.into()
            };

            container = container.children(vec![node]);
        } else if let Some(placeholder) = &self.placeholder {
            // Show placeholder when content is empty and not focused
            let mut text = Text::new(placeholder.clone());
            let default_style = Self::default_placeholder_style();
            let final_style = TextStyle::merge(Some(default_style), self.placeholder_style.clone());

            if let Some(style) = final_style {
                text.style = Some(style.clone());
            }

            container = container.children(vec![text.into()]);
        }

        container.into()
    }
}

//--------------------------------------------------------------------------------------------------
// Container-style builder methods
//--------------------------------------------------------------------------------------------------

impl TextInput {
    /// Sets the background color
    pub fn background(mut self, color: Color) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.background = Some(color);
        self.styles.base = Some(style);
        self
    }

    /// Sets the border color (creates a default border if none exists)
    pub fn border(mut self, color: Color) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        if style.border.is_none() {
            style.border = Some(Border::new(color));
        }
        if let Some(ref mut border) = style.border {
            border.color = color;
            border.enabled = true;
        }
        self.styles.base = Some(style);
        self
    }

    /// Sets the border style and color
    pub fn border_style(mut self, border_style: BorderStyle, color: Color) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.border = Some(Border {
            enabled: true,
            style: border_style,
            color,
            edges: BorderEdges::ALL,
        });
        self.styles.base = Some(style);
        self
    }

    /// Sets border edges to display
    pub fn border_edges(mut self, edges: BorderEdges) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        if style.border.is_none() {
            style.border = Some(Border::new(Color::White));
        }
        if let Some(ref mut border) = style.border {
            border.edges = edges;
        }
        self.styles.base = Some(style);
        self
    }

    /// Sets full border configuration
    pub fn border_full(
        mut self,
        border_style: BorderStyle,
        color: Color,
        edges: BorderEdges,
    ) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.border = Some(Border {
            enabled: true,
            style: border_style,
            color,
            edges,
        });
        self.styles.base = Some(style);
        self
    }

    /// Sets the inner padding around content
    pub fn padding(mut self, padding: Spacing) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.padding = Some(padding);
        self.styles.base = Some(style);
        self
    }

    /// Sets the width
    pub fn width(mut self, width: u16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.width = Some(Dimension::Fixed(width));
        self.styles.base = Some(style);
        self
    }

    /// Sets the width as a fraction of the parent (0.0 to 1.0)
    pub fn width_fraction(mut self, fraction: f32) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.width = Some(Dimension::Percentage(fraction.clamp(0.0, 1.0)));
        self.styles.base = Some(style);
        self
    }

    /// Sets the width to auto
    pub fn width_auto(mut self) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.width = Some(Dimension::Auto);
        self.styles.base = Some(style);
        self
    }

    /// Sets the width to content-based sizing
    pub fn width_content(mut self) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.width = Some(Dimension::Content);
        self.styles.base = Some(style);
        self
    }

    /// Sets the height
    pub fn height(mut self, height: u16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.height = Some(Dimension::Fixed(height));
        self.styles.base = Some(style);
        self
    }

    /// Sets the height as a fraction of the parent (0.0 to 1.0)
    pub fn height_fraction(mut self, fraction: f32) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.height = Some(Dimension::Percentage(fraction.clamp(0.0, 1.0)));
        self.styles.base = Some(style);
        self
    }

    /// Sets the height to auto
    pub fn height_auto(mut self) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.height = Some(Dimension::Auto);
        self.styles.base = Some(style);
        self
    }

    /// Sets the height to content-based sizing
    pub fn height_content(mut self) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.height = Some(Dimension::Content);
        self.styles.base = Some(style);
        self
    }

    /// Sets the focus style
    pub fn focus_style(mut self, style: Style) -> Self {
        self.styles.focus = Some(style);
        self
    }

    /// Sets the hover style
    pub fn hover_style(mut self, style: Style) -> Self {
        self.styles.hover = Some(style);
        self
    }

    /// Sets the border color when focused
    pub fn focus_border(mut self, color: Color) -> Self {
        let mut style = self.styles.focus.clone().unwrap_or_default();
        if style.border.is_none() {
            style.border = Some(Border::new(color));
        }
        if let Some(ref mut border) = style.border {
            border.color = color;
            border.enabled = true;
        }
        self.styles.focus = Some(style);
        self
    }

    /// Sets the border style and color when focused
    pub fn focus_border_style(mut self, border_style: BorderStyle, color: Color) -> Self {
        let mut style = self.styles.focus.clone().unwrap_or_default();
        style.border = Some(Border {
            enabled: true,
            style: border_style,
            color,
            edges: BorderEdges::ALL,
        });
        self.styles.focus = Some(style);
        self
    }

    /// Sets the background color when focused
    pub fn focus_background(mut self, color: Color) -> Self {
        let mut style = self.styles.focus.clone().unwrap_or_default();
        style.background = Some(color);
        self.styles.focus = Some(style);
        self
    }

    /// Sets the padding when focused
    pub fn focus_padding(mut self, padding: Spacing) -> Self {
        let mut style = self.styles.focus.clone().unwrap_or_default();
        style.padding = Some(padding);
        self.styles.focus = Some(style);
        self
    }

    /// Sets the border color when hovered
    pub fn hover_border(mut self, color: Color) -> Self {
        let mut style = self.styles.hover.clone().unwrap_or_default();
        if style.border.is_none() {
            style.border = Some(Border::new(color));
        }
        if let Some(ref mut border) = style.border {
            border.color = color;
            border.enabled = true;
        }
        self.styles.hover = Some(style);
        self
    }

    /// Sets the border style and color when hovered
    pub fn hover_border_style(mut self, border_style: BorderStyle, color: Color) -> Self {
        let mut style = self.styles.hover.clone().unwrap_or_default();
        style.border = Some(Border {
            enabled: true,
            style: border_style,
            color,
            edges: BorderEdges::ALL,
        });
        self.styles.hover = Some(style);
        self
    }

    /// Sets the background color when hovered
    pub fn hover_background(mut self, color: Color) -> Self {
        let mut style = self.styles.hover.clone().unwrap_or_default();
        style.background = Some(color);
        self.styles.hover = Some(style);
        self
    }

    /// Sets the padding when hovered
    pub fn hover_padding(mut self, padding: Spacing) -> Self {
        let mut style = self.styles.hover.clone().unwrap_or_default();
        style.padding = Some(padding);
        self.styles.hover = Some(style);
        self
    }

    /// Sets the position type
    pub fn position(mut self, position: Position) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.position = Some(position);
        self.styles.base = Some(style);
        self
    }

    /// Sets absolute positioning
    pub fn absolute(self) -> Self {
        self.position(Position::Absolute)
    }

    /// Sets the top offset
    pub fn top(mut self, top: i16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.top = Some(top);
        self.styles.base = Some(style);
        self
    }

    /// Sets the right offset
    pub fn right(mut self, right: i16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.right = Some(right);
        self.styles.base = Some(style);
        self
    }

    /// Sets the bottom offset
    pub fn bottom(mut self, bottom: i16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.bottom = Some(bottom);
        self.styles.base = Some(style);
        self
    }

    /// Sets the left offset
    pub fn left(mut self, left: i16) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.left = Some(left);
        self.styles.base = Some(style);
        self
    }

    /// Sets the z-index for layering
    pub fn z_index(mut self, z_index: i32) -> Self {
        let mut style = self.styles.base.clone().unwrap_or_else(Self::default_style);
        style.z_index = Some(z_index);
        self.styles.base = Some(style);
        self
    }

    /// Sets the complete placeholder text style
    pub fn placeholder_style(mut self, style: TextStyle) -> Self {
        self.placeholder_style = Some(style);
        self
    }

    /// Sets the placeholder text color
    pub fn placeholder_color(mut self, color: Color) -> Self {
        let mut style = self
            .placeholder_style
            .clone()
            .unwrap_or_else(Self::default_placeholder_style);
        style.color = Some(color);
        self.placeholder_style = Some(style);
        self
    }

    /// Sets the placeholder text background color
    pub fn placeholder_background(mut self, color: Color) -> Self {
        let mut style = self
            .placeholder_style
            .clone()
            .unwrap_or_else(Self::default_placeholder_style);
        style.background = Some(color);
        self.placeholder_style = Some(style);
        self
    }

    /// Makes the placeholder text bold
    pub fn placeholder_bold(mut self, bold: bool) -> Self {
        let mut style = self
            .placeholder_style
            .clone()
            .unwrap_or_else(Self::default_placeholder_style);
        style.bold = Some(bold);
        self.placeholder_style = Some(style);
        self
    }

    /// Makes the placeholder text italic
    pub fn placeholder_italic(mut self, italic: bool) -> Self {
        let mut style = self
            .placeholder_style
            .clone()
            .unwrap_or_else(Self::default_placeholder_style);
        style.italic = Some(italic);
        self.placeholder_style = Some(style);
        self
    }

    /// Makes the placeholder text underlined
    pub fn placeholder_underline(mut self, underline: bool) -> Self {
        let mut style = self
            .placeholder_style
            .clone()
            .unwrap_or_else(Self::default_placeholder_style);
        style.underline = Some(underline);
        self.placeholder_style = Some(style);
        self
    }

    /// Sets the complete content text style
    pub fn content_style(mut self, style: TextStyle) -> Self {
        self.content_style = Some(style);
        self
    }

    /// Sets the content text color
    pub fn content_color(mut self, color: Color) -> Self {
        let mut style = self
            .content_style
            .clone()
            .unwrap_or_else(Self::default_content_style);
        style.color = Some(color);
        self.content_style = Some(style);
        self
    }

    /// Sets the content text background color
    pub fn content_background(mut self, color: Color) -> Self {
        let mut style = self
            .content_style
            .clone()
            .unwrap_or_else(Self::default_content_style);
        style.background = Some(color);
        self.content_style = Some(style);
        self
    }

    /// Makes the content text bold
    pub fn content_bold(mut self, bold: bool) -> Self {
        let mut style = self
            .content_style
            .clone()
            .unwrap_or_else(Self::default_content_style);
        style.bold = Some(bold);
        self.content_style = Some(style);
        self
    }

    /// Makes the content text italic
    pub fn content_italic(mut self, italic: bool) -> Self {
        let mut style = self
            .content_style
            .clone()
            .unwrap_or_else(Self::default_content_style);
        style.italic = Some(italic);
        self.content_style = Some(style);
        self
    }

    /// Makes the content text underlined
    pub fn content_underline(mut self, underline: bool) -> Self {
        let mut style = self
            .content_style
            .clone()
            .unwrap_or_else(Self::default_content_style);
        style.underline = Some(underline);
        self.content_style = Some(style);
        self
    }

    /// Sets the cursor style
    pub fn cursor_style(mut self, style: TextStyle) -> Self {
        self.cursor_style = Some(style);
        self
    }

    /// Sets the cursor color (background when cursor is shown)
    pub fn cursor_color(mut self, color: Color) -> Self {
        let mut style = self
            .cursor_style
            .clone()
            .unwrap_or_else(Self::default_cursor_style);
        style.background = Some(color);
        // Automatically set text color for contrast
        style.color = Some(match color {
            Color::Black | Color::Blue | Color::Red | Color::Magenta => Color::White,
            _ => Color::Black,
        });
        self.cursor_style = Some(style);
        self
    }

    /// Sets the selection style
    pub fn selection_style(mut self, style: TextStyle) -> Self {
        self.selection_style = Some(style);
        self
    }

    /// Sets the selection background color
    pub fn selection_color(mut self, color: Color) -> Self {
        let mut style = self
            .selection_style
            .clone()
            .unwrap_or_else(Self::default_selection_style);
        style.background = Some(color);
        self.selection_style = Some(style);
        self
    }

    /// Enables text wrapping with the specified mode
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Component for TextInput {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        TextInput::update(self, ctx, msg, topic)
    }

    fn view(&self, ctx: &Context) -> Node {
        TextInput::view(self, ctx)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}
