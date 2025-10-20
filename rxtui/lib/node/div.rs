use crate::component::ComponentId;
use crate::key::{Key, KeyWithModifiers};
use crate::style::{
    AlignItems, AlignSelf, Border, BorderEdges, BorderStyle, Color, Dimension, Direction,
    JustifyContent, Overflow, Position, Spacing, Style, WrapMode,
};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Type alias for keyboard event handler tuple: (key, handler, is_global)
pub type KeyHandler = (Key, Rc<dyn Fn()>, bool);

/// Type alias for keyboard event handler with modifiers: (key_with_modifiers, handler, is_global)
pub type KeyWithModifiersHandler = (KeyWithModifiers, Rc<dyn Fn()>, bool);

/// A container that can hold child elements
#[derive(Clone)]
pub struct Div<T> {
    /// Child elements
    pub children: Vec<T>,

    /// Visual styling for different states
    pub styles: DivStyles,

    /// Event callbacks
    pub events: EventCallbacks,

    /// Whether this container can receive focus
    pub focusable: bool,

    /// Whether this container is currently focused
    pub focused: bool,

    /// Whether this container is currently hovered
    pub hovered: bool,

    /// Component path that owns this div (used for focus targeting)
    pub component_path: Option<ComponentId>,
}

/// Style configuration for a div in different states.
#[derive(Clone, Default)]
pub struct DivStyles {
    /// Base style when div is in normal state
    pub base: Option<Style>,

    /// Style to apply when div is focused
    pub focus: Option<Style>,

    /// Style to apply when div is hovered
    pub hover: Option<Style>,
}

/// Event callbacks for a div.
#[derive(Clone, Default)]
pub struct EventCallbacks {
    /// Click event handler
    pub on_click: Option<Rc<dyn Fn()>>,

    /// Keyboard event handlers: (key, handler, is_global)
    /// Global handlers work regardless of focus state
    pub on_key: Vec<KeyHandler>,

    /// Keyboard event handlers with modifiers: (key_with_modifiers, handler, is_global)
    /// These are checked before simple on_key handlers
    pub on_key_with_modifiers: Vec<KeyWithModifiersHandler>,

    /// Handler for any character input (receives the character)
    pub on_any_char: Option<Rc<dyn Fn(char)>>,

    /// Handler for any key press (receives the full Key enum)
    pub on_any_key: Option<Rc<dyn Fn(Key)>>,

    /// Called when div gains focus
    pub on_focus: Option<Rc<dyn Fn()>>,

    /// Called when div loses focus
    pub on_blur: Option<Rc<dyn Fn()>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<T> Div<T> {
    /// Creates a new empty div
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            styles: DivStyles::default(),
            events: EventCallbacks::default(),
            focusable: false,
            focused: false,
            hovered: false,
            component_path: None,
        }
    }

    /// Sets the children of this div
    pub fn children(mut self, children: Vec<T>) -> Self {
        self.children = children;
        self
    }

    /// Adds a single child
    pub fn child(mut self, child: T) -> Self {
        self.children.push(child);
        self
    }

    /// Makes this div focusable
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    /// Sets the display direction
    pub fn direction(mut self, direction: Direction) -> Self {
        self.styles.base.get_or_insert(Style::default()).direction = Some(direction);
        self
    }

    /// Sets the position type
    pub fn position(mut self, position: Position) -> Self {
        self.styles.base.get_or_insert(Style::default()).position = Some(position);
        self
    }

    /// Sets the overflow behavior
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.styles.base.get_or_insert(Style::default()).overflow = Some(overflow);
        self
    }

    /// Sets the padding
    pub fn padding(mut self, padding: Spacing) -> Self {
        self.styles.base.get_or_insert(Style::default()).padding = Some(padding);
        self
    }

    /// Sets the margin
    pub fn margin(mut self, margin: Spacing) -> Self {
        self.styles.base.get_or_insert(Style::default()).margin = Some(margin);
        self
    }

    /// Sets the gap between children
    pub fn gap(mut self, gap: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).gap = Some(gap);
        self
    }

    /// Sets the wrap mode for children
    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.styles.base.get_or_insert(Style::default()).wrap = Some(wrap);
        self
    }

    /// Sets the width
    pub fn width(mut self, width: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).width = Some(Dimension::Fixed(width));
        self
    }

    /// Sets the width with a dimension
    pub fn width_dim(mut self, width: Dimension) -> Self {
        self.styles.base.get_or_insert(Style::default()).width = Some(width);
        self
    }

    /// Sets the height
    pub fn height(mut self, height: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).height = Some(Dimension::Fixed(height));
        self
    }

    /// Sets the height with a dimension
    pub fn height_dim(mut self, height: Dimension) -> Self {
        self.styles.base.get_or_insert(Style::default()).height = Some(height);
        self
    }

    /// Sets the width as fraction of parent (0.0 to 1.0)
    pub fn width_fraction(mut self, fraction: f32) -> Self {
        let normalized = fraction.clamp(0.0, 1.0);
        self.styles.base.get_or_insert(Style::default()).width =
            Some(Dimension::Percentage(normalized));
        self
    }

    /// Sets the height as fraction of parent (0.0 to 1.0)
    pub fn height_fraction(mut self, fraction: f32) -> Self {
        let normalized = fraction.clamp(0.0, 1.0);
        self.styles.base.get_or_insert(Style::default()).height =
            Some(Dimension::Percentage(normalized));
        self
    }

    /// Sets the width to auto
    pub fn width_auto(mut self) -> Self {
        self.styles.base.get_or_insert(Style::default()).width = Some(Dimension::Auto);
        self
    }

    /// Sets the height to auto
    pub fn height_auto(mut self) -> Self {
        self.styles.base.get_or_insert(Style::default()).height = Some(Dimension::Auto);
        self
    }

    /// Sets the height to content
    pub fn height_content(mut self) -> Self {
        self.styles.base.get_or_insert(Style::default()).height = Some(Dimension::Content);
        self
    }

    /// Sets the width to content
    pub fn width_content(mut self) -> Self {
        self.styles.base.get_or_insert(Style::default()).width = Some(Dimension::Content);
        self
    }

    /// Sets the minimum width
    pub fn min_width(mut self, width: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).min_width = Some(width);
        self
    }

    /// Sets the minimum height
    pub fn min_height(mut self, height: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).min_height = Some(height);
        self
    }

    /// Sets the maximum width
    pub fn max_width(mut self, width: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).max_width = Some(width);
        self
    }

    /// Sets the maximum height
    pub fn max_height(mut self, height: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).max_height = Some(height);
        self
    }

    /// Sets the background color
    pub fn background(mut self, color: Color) -> Self {
        self.styles.base.get_or_insert(Style::default()).background = Some(color);
        self
    }

    /// Sets the border style
    pub fn border(mut self, border: BorderStyle) -> Self {
        self.styles.base.get_or_insert(Style::default()).border = Some(Border {
            enabled: true,
            style: border,
            color: Color::White,
            edges: BorderEdges::ALL,
        });
        self
    }

    /// Sets the border using an explicit Border configuration.
    pub fn border_with(mut self, border: Border) -> Self {
        self.styles.base.get_or_insert(Style::default()).border = Some(border);
        self
    }

    /// Sets the border color with default Single style
    pub fn border_color(mut self, color: Color) -> Self {
        self.styles.base.get_or_insert(Style::default()).border = Some(Border {
            enabled: true,
            style: BorderStyle::Single,
            color,
            edges: BorderEdges::ALL,
        });
        self
    }

    /// Sets the border style (convenience for border with default color)
    pub fn border_style(self, style: BorderStyle) -> Self {
        self.border(style)
    }

    /// Sets the border style with color (for macro compatibility)
    pub fn border_style_with_color(mut self, style: BorderStyle, color: Color) -> Self {
        self.styles.base.get_or_insert(Style::default()).border = Some(Border {
            enabled: true,
            style,
            color,
            edges: BorderEdges::ALL,
        });
        self
    }

    /// Sets which border edges to render
    pub fn border_edges(mut self, edges: BorderEdges) -> Self {
        let style = self.styles.base.get_or_insert(Style::default());
        if let Some(border) = &mut style.border {
            border.edges = edges;
        } else {
            style.border = Some(Border {
                enabled: true,
                style: BorderStyle::Single,
                color: Color::White,
                edges,
            });
        }
        self
    }

    /// Sets whether to show scrollbar for scrollable content
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.styles
            .base
            .get_or_insert(Style::default())
            .show_scrollbar = Some(show);
        self
    }

    /// Sets position to absolute (for macro compatibility when used as flag)
    pub fn absolute_position(mut self) -> Self {
        self.styles.base.get_or_insert(Style::default()).position = Some(Position::Absolute);
        self
    }

    /// Sets the absolute position coordinates
    pub fn absolute(mut self, x: u16, y: u16) -> Self {
        let style = self.styles.base.get_or_insert(Style::default());
        style.position = Some(Position::Absolute);
        style.x = Some(x);
        style.y = Some(y);
        self
    }

    /// Sets the x coordinate (for absolute positioning)
    pub fn x(mut self, x: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).x = Some(x);
        self
    }

    /// Sets the y coordinate (for absolute positioning)
    pub fn y(mut self, y: u16) -> Self {
        self.styles.base.get_or_insert(Style::default()).y = Some(y);
        self
    }

    /// Sets the top position
    pub fn top(mut self, top: i16) -> Self {
        self.styles.base.get_or_insert(Style::default()).top = Some(top);
        self
    }

    /// Sets the right position
    pub fn right(mut self, right: i16) -> Self {
        self.styles.base.get_or_insert(Style::default()).right = Some(right);
        self
    }

    /// Sets the bottom position
    pub fn bottom(mut self, bottom: i16) -> Self {
        self.styles.base.get_or_insert(Style::default()).bottom = Some(bottom);
        self
    }

    /// Sets the left position
    pub fn left(mut self, left: i16) -> Self {
        self.styles.base.get_or_insert(Style::default()).left = Some(left);
        self
    }

    /// Sets the z-index
    pub fn z_index(mut self, z: i32) -> Self {
        self.styles.base.get_or_insert(Style::default()).z_index = Some(z);
        self
    }

    /// Sets how content is distributed along the main axis
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.styles
            .base
            .get_or_insert(Style::default())
            .justify_content = Some(justify);
        self
    }

    /// Sets how items are aligned on the cross axis
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.styles.base.get_or_insert(Style::default()).align_items = Some(align);
        self
    }

    /// Sets this element's alignment, overriding parent's align_items
    pub fn align_self(mut self, align: AlignSelf) -> Self {
        self.styles.base.get_or_insert(Style::default()).align_self = Some(align);
        self
    }

    /// Sets the focus style
    pub fn focus_style(mut self, style: Style) -> Self {
        self.styles.focus = Some(style);
        self
    }

    /// Sets the border color when focused
    pub fn focus_border(self, color: Color) -> Self {
        self.focus_border_with(Border::new(color))
    }

    /// Sets the border style and color when focused
    pub fn focus_border_style(self, border_style: BorderStyle, color: Color) -> Self {
        self.focus_border_with(Border::with_style(border_style, color))
    }

    /// Sets the focus border using an explicit configuration
    pub fn focus_border_with(mut self, border: Border) -> Self {
        let mut style = self.styles.focus.clone().unwrap_or_default();
        style.border = Some(border);
        self.styles.focus = Some(style);
        self
    }

    /// Sets the hover style
    pub fn hover_style(mut self, style: Style) -> Self {
        self.styles.hover = Some(style);
        self
    }

    /// Sets the base style directly
    pub fn style(mut self, style: Style) -> Self {
        self.styles.base = Some(style);
        self
    }

    /// Registers a key handler
    pub fn on_key(mut self, key: Key, handler: impl Fn() + 'static) -> Self {
        self.events.on_key.push((key, Rc::new(handler), false));
        self
    }

    /// Registers a character key handler
    pub fn on_char(mut self, ch: char, handler: impl Fn() + 'static) -> Self {
        self.events
            .on_key
            .push((Key::Char(ch), Rc::new(handler), false));
        self
    }

    /// Registers a global key handler (works even when not focused)
    pub fn on_key_global(mut self, key: Key, handler: impl Fn() + 'static) -> Self {
        self.events.on_key.push((key, Rc::new(handler), true));
        self
    }

    /// Registers a global character key handler (works even when not focused)
    pub fn on_char_global(mut self, ch: char, handler: impl Fn() + 'static) -> Self {
        self.events
            .on_key
            .push((Key::Char(ch), Rc::new(handler), true));
        self
    }

    /// Registers a key handler with modifiers
    pub fn on_key_with_modifiers(
        mut self,
        key_with_modifiers: KeyWithModifiers,
        handler: impl Fn() + 'static,
    ) -> Self {
        self.events
            .on_key_with_modifiers
            .push((key_with_modifiers, Rc::new(handler), false));
        self
    }

    /// Registers a global key handler with modifiers (works even when not focused)
    pub fn on_key_with_modifiers_global(
        mut self,
        key_with_modifiers: KeyWithModifiers,
        handler: impl Fn() + 'static,
    ) -> Self {
        self.events
            .on_key_with_modifiers
            .push((key_with_modifiers, Rc::new(handler), true));
        self
    }

    /// Registers a handler for any character input
    pub fn on_any_char(mut self, handler: impl Fn(char) + 'static) -> Self {
        self.events.on_any_char = Some(Rc::new(handler));
        self
    }

    /// Registers a handler for any key press
    pub fn on_any_key(mut self, handler: impl Fn(Key) + 'static) -> Self {
        self.events.on_any_key = Some(Rc::new(handler));
        self
    }

    /// Registers a click handler
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.events.on_click = Some(Rc::new(handler));
        self
    }

    /// Registers a focus handler
    pub fn on_focus(mut self, handler: impl Fn() + 'static) -> Self {
        self.events.on_focus = Some(Rc::new(handler));
        self
    }

    /// Registers a blur handler
    pub fn on_blur(mut self, handler: impl Fn() + 'static) -> Self {
        self.events.on_blur = Some(Rc::new(handler));
        self
    }

    /// Converts a Div to a new type using a mapping function
    pub fn map<U, F>(self, f: F) -> Div<U>
    where
        F: FnMut(T) -> U,
    {
        Div {
            children: self.children.into_iter().map(f).collect(),
            styles: self.styles,
            events: self.events,
            focusable: self.focusable,
            focused: self.focused,
            hovered: self.hovered,
            component_path: self.component_path,
        }
    }

    /// Gets the active style based on the current state
    pub fn active_style(&self) -> Option<&Style> {
        if self.focused && self.styles.focus.is_some() {
            self.styles.focus.as_ref()
        } else if self.hovered && self.styles.hover.is_some() {
            self.styles.hover.as_ref()
        } else {
            self.styles.base.as_ref()
        }
    }
}

impl<T> Default for Div<T> {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl<T: PartialEq> PartialEq for Div<T> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.styles == other.styles
            && self.focusable == other.focusable
            && self.focused == other.focused
            && self.hovered == other.hovered
            && self.component_path == other.component_path
    }
}

impl PartialEq for DivStyles {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.focus == other.focus && self.hover == other.hover
    }
}

impl Debug for DivStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DivStyles")
            .field("base", &self.base)
            .field("focus", &self.focus)
            .field("hover", &self.hover)
            .finish()
    }
}

impl Debug for EventCallbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCallbacks")
            .field("on_click", &self.on_click.is_some())
            .field("on_key_count", &self.on_key.len())
            .field(
                "on_key_with_modifiers_count",
                &self.on_key_with_modifiers.len(),
            )
            .field("on_any_char", &self.on_any_char.is_some())
            .field("on_any_key", &self.on_any_key.is_some())
            .field("on_focus", &self.on_focus.is_some())
            .field("on_blur", &self.on_blur.is_some())
            .finish()
    }
}

impl<T: Debug> Debug for Div<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Div")
            .field("children", &self.children)
            .field("styles", &self.styles)
            .field("events", &self.events)
            .field("focusable", &self.focusable)
            .field("focused", &self.focused)
            .field("hovered", &self.hovered)
            .finish()
    }
}

/// ElementBuilder is used for type-safe method chaining
pub struct ElementBuilder<T> {
    element: T,
    _phantom: PhantomData<T>,
}

impl<T> ElementBuilder<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> T {
        self.element
    }
}
