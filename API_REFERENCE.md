# RxTUI API Reference

Complete API documentation for the RxTUI framework.

## Module Structure

```
rxtui
├── prelude          // Common imports
├── component        // Component trait and types
├── node             // UI node types
├── style            // Styling types
├── app              // Application core
├── components       // Built-in components
├── macros           // Macro exports
└── effect           // Async effects (feature-gated)
```

## Prelude

```rust
use rxtui::prelude::*;
```

Imports all commonly used types:
- Core: `App`, `Context`, `Component`, `Node`, `Action`
- State: `State`, `StateExt`, `Message`, `MessageExt`
- Style: `Color`, `Style`, `Direction`, `Spacing`, `Border`, `BorderStyle`, `BorderEdges`
- Key: `Key`, `KeyWithModifiers`
- Macros: `node!`, `#[component]`, `#[update]`, `#[view]`, `#[effect]`

## Core Types

### Component

```rust
pub trait Component: 'static {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action;
    fn view(&self, ctx: &Context) -> Node;
    fn effects(&self, ctx: &Context) -> Vec<Effect>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

Derive with:
```rust
#[derive(Component)]
struct MyComponent;
```

### Action

```rust
pub enum Action {
    Update(Box<dyn State>),              // Update component state
    UpdateTopic(String, Box<dyn State>), // Update topic state
    None,                                // No action
    Exit,                                // Exit application
}
```

Helper methods:
```rust
Action::update(state)        // Shorthand for Update
Action::update_topic(topic, state)  // Shorthand for UpdateTopic
Action::none()               // Shorthand for None
Action::exit()               // Shorthand for Exit
```

### Context

```rust
impl Context {
    // Message handling
    pub fn handler<M: Message>(&self, msg: M) -> Box<dyn Fn()>;
    pub fn handler_with_value<F, M>(&self, f: F) -> Box<dyn Fn(T)>
        where F: Fn(T) -> M, M: Message;

    // State management
    pub fn get_state<S: State>(&self) -> S;
    pub fn get_state_or<S: State>(&self, default: S) -> S;

    // Topic messaging
    pub fn send_to_topic<M: Message>(&self, topic: &str, msg: M);
    pub fn read_topic<S: State>(&self, topic: &str) -> Option<S>;

    // Direct messaging
    pub fn send<M: Message>(&self, msg: M);
}
```

### Message

```rust
pub trait Message: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Message>;
}
```

Auto-implemented for types that are `Clone + Send + Sync + 'static`.

### State

```rust
pub trait State: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn State>;
}
```

Auto-implemented for types that are `Clone + Send + Sync + 'static`.

## Node Types

### Node

```rust
pub enum Node {
    Component(Arc<dyn Component>),
    Div(Div),
    Text(Text),
    RichText(RichText),
}
```

### Div

```rust
impl Div {
    pub fn new() -> Self;

    // Layout
    pub fn direction(self, dir: Direction) -> Self;
    pub fn gap(self, gap: u16) -> Self;
    pub fn wrap(self, mode: WrapMode) -> Self;

    // Alignment
    pub fn justify_content(self, justify: JustifyContent) -> Self;
    pub fn align_items(self, align: AlignItems) -> Self;
    pub fn align_self(self, align: AlignSelf) -> Self;

    // Sizing
    pub fn width(self, w: u16) -> Self;
    pub fn width_fraction(self, frac: f32) -> Self;
    pub fn width_auto(self) -> Self;
    pub fn width_content(self) -> Self;
    pub fn height(self, h: u16) -> Self;
    pub fn height_fraction(self, frac: f32) -> Self;
    pub fn height_auto(self) -> Self;
    pub fn height_content(self) -> Self;

    // Styling
    pub fn background(self, color: Color) -> Self;
    pub fn padding(self, spacing: Spacing) -> Self;
    pub fn style(self, style: Style) -> Self;

    // Borders
    pub fn border_color(self, color: Color) -> Self;
    pub fn border_style_with_color(self, style: BorderStyle, color: Color) -> Self;
    pub fn border_edges(self, edges: BorderEdges) -> Self;
    pub fn border_full(self, style: BorderStyle, color: Color, edges: BorderEdges) -> Self;

    // Positioning
    pub fn position(self, pos: Position) -> Self;
    pub fn top(self, offset: i16) -> Self;
    pub fn right(self, offset: i16) -> Self;
    pub fn bottom(self, offset: i16) -> Self;
    pub fn left(self, offset: i16) -> Self;
    pub fn z_index(self, z: i32) -> Self;

    // Scrolling
    pub fn overflow(self, overflow: Overflow) -> Self;
    pub fn show_scrollbar(self, show: bool) -> Self;

    // Focus
    pub fn focusable(self, focusable: bool) -> Self;
    pub fn focus_style(self, style: Style) -> Self;

    // Events
    pub fn on_click(self, handler: impl Fn()) -> Self;
    pub fn on_key(self, key: Key, handler: impl Fn()) -> Self;
    pub fn on_key_global(self, key: Key, handler: impl Fn()) -> Self;
    pub fn on_char(self, ch: char, handler: impl Fn()) -> Self;
    pub fn on_char_global(self, ch: char, handler: impl Fn()) -> Self;
    pub fn on_any_char(self, handler: impl Fn(char)) -> Self;
    pub fn on_focus(self, handler: impl Fn()) -> Self;
    pub fn on_blur(self, handler: impl Fn()) -> Self;

    // Children
    pub fn children(self, children: Vec<Node>) -> Self;
    pub fn child(self, child: Node) -> Self;
}
```

### Text

```rust
impl Text {
    pub fn new(content: impl Into<String>) -> Self;

    // Styling
    pub fn color(self, color: Color) -> Self;
    pub fn background(self, color: Color) -> Self;
    pub fn bold(self) -> Self;
    pub fn italic(self) -> Self;
    pub fn underline(self) -> Self;
    pub fn strikethrough(self) -> Self;
    pub fn style(self, style: TextStyle) -> Self;

    // Wrapping
    pub fn wrap(self, mode: TextWrap) -> Self;

    // Alignment
    pub fn align(self, align: TextAlign) -> Self;
}
```

### RichText

```rust
impl RichText {
    pub fn new() -> Self;

    // Add spans
    pub fn text(self, content: impl Into<String>) -> Self;
    pub fn styled(self, content: impl Into<String>, style: TextStyle) -> Self;
    pub fn colored(self, content: impl Into<String>, color: Color) -> Self;
    pub fn bold(self, content: impl Into<String>) -> Self;
    pub fn italic(self, content: impl Into<String>) -> Self;

    // Apply to all spans
    pub fn color(self, color: Color) -> Self;
    pub fn background(self, color: Color) -> Self;
    pub fn bold_all(self) -> Self;
    pub fn italic_all(self) -> Self;

    // Wrapping
    pub fn wrap(self, mode: TextWrap) -> Self;

    // Alignment
    pub fn align(self, align: TextAlign) -> Self;

    // Cursor support
    pub fn with_cursor(content: &str, position: usize, style: TextStyle) -> Self;
}
```

## Style Types

### Color

```rust
pub enum Color {
    // Basic colors
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,

    // Bright colors
    BrightBlack, BrightRed, BrightGreen, BrightYellow,
    BrightBlue, BrightMagenta, BrightCyan, BrightWhite,

    // RGB
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn from_hex(hex: &str) -> Result<Self, ParseError>;
}
```

### Style

```rust
pub struct Style {
    pub background: Option<Color>,
    pub direction: Option<Direction>,
    pub padding: Option<Spacing>,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub gap: Option<u16>,
    pub wrap: Option<WrapMode>,
    pub overflow: Option<Overflow>,
    pub border: Option<Border>,
    pub position: Option<Position>,
    pub top: Option<i16>,
    pub right: Option<i16>,
    pub bottom: Option<i16>,
    pub left: Option<i16>,
    pub z_index: Option<i32>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
}

impl Style {
    pub fn new() -> Self;
    pub fn background(self, color: Color) -> Self;
    pub fn padding(self, spacing: Spacing) -> Self;
    pub fn border(self, color: Color) -> Self;
    // ... builder methods for all fields
}
```

### Key

```rust
pub enum Key {
    // Regular character
    Char(char),

    // Special keys
    Esc, Enter, Tab, BackTab, Backspace, Delete,

    // Arrow keys
    Up, Down, Left, Right,

    // Navigation
    PageUp, PageDown, Home, End,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

pub struct KeyWithModifiers {
    pub key: Key,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,  // Cmd on macOS, Win on Windows
}

impl KeyWithModifiers {
    pub fn new(key: Key) -> Self;
    pub fn with_ctrl(key: Key) -> Self;
    pub fn with_alt(key: Key) -> Self;
    pub fn with_shift(key: Key) -> Self;
    pub fn is_primary_modifier(&self) -> bool;  // Platform-aware (Cmd on macOS, Ctrl elsewhere)
}
```

### TextAlign

```rust
pub enum TextAlign {
    Left,    // Align text to the left edge (default)
    Center,  // Center text horizontally
    Right,   // Align text to the right edge
}
```

### JustifyContent

```rust
pub enum JustifyContent {
    Start,         // Pack items at the start of the main axis (default)
    Center,        // Center items along the main axis
    End,           // Pack items at the end of the main axis
    SpaceBetween,  // Distribute items evenly, first at start, last at end
    SpaceAround,   // Distribute items evenly with equal space around each item
    SpaceEvenly,   // Distribute items evenly with equal space between and around items
}
```

### AlignItems

```rust
pub enum AlignItems {
    Start,   // Align items at the start of the cross axis (default)
    Center,  // Center items along the cross axis
    End,     // Align items at the end of the cross axis
}
```

### AlignSelf

```rust
pub enum AlignSelf {
    Auto,    // Use the parent's AlignItems value (default)
    Start,   // Align at the start of the cross axis
    Center,  // Center along the cross axis
    End,     // Align at the end of the cross axis
}
```

### TextStyle

```rust
pub struct TextStyle {
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub strikethrough: Option<bool>,
    pub wrap: Option<TextWrap>,
    pub align: Option<TextAlign>,
}

impl TextStyle {
    pub fn new() -> Self;
    pub fn color(self, color: Color) -> Self;
    pub fn background(self, color: Color) -> Self;
    pub fn bold(self) -> Self;
    pub fn italic(self) -> Self;
    pub fn underline(self) -> Self;
    pub fn strikethrough(self) -> Self;
    pub fn merge(base: Option<Self>, overlay: Option<Self>) -> Option<Self>;
}
```

### Dimension

```rust
pub enum Dimension {
    Fixed(u16),       // Exact size
    Percentage(f32),  // Normalized (0.0 to 1.0)
    Auto,             // Share remaining
    Content,          // Fit content
}
```

### Direction

```rust
pub enum Direction {
    Horizontal,
    Vertical,
}
```

### Spacing

```rust
pub struct Spacing {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Spacing {
    pub fn all(value: u16) -> Self;
    pub fn horizontal(value: u16) -> Self;
    pub fn vertical(value: u16) -> Self;
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self;
}
```

### BorderStyle

```rust
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    Thick,
}
```

### BorderEdges

```rust
bitflags! {
    pub struct BorderEdges: u8 {
        const TOP = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT = 0b1000;
        const ALL = 0b1111;
    }
}
```

### Border

```rust
pub struct Border {
    pub enabled: bool,
    pub style: BorderStyle,
    pub color: Color,
    pub edges: BorderEdges,
}

impl Border {
    pub fn new(color: Color) -> Self;
    pub fn style(self, style: BorderStyle) -> Self;
    pub fn edges(self, edges: BorderEdges) -> Self;
}
```

### Position

```rust
pub enum Position {
    Relative,
    Absolute,
}
```

### Overflow

```rust
pub enum Overflow {
    None,    // No clipping
    Hidden,  // Clip content
    Scroll,  // Scrollable
    Auto,    // Auto scrollbars
}
```

### WrapMode

```rust
pub enum WrapMode {
    NoWrap,
    Wrap,
}
```

### TextWrap

```rust
pub enum TextWrap {
    None,
    Character,
    Word,
    WordBreak,
}
```

## App

```rust
pub struct App {
    // Private fields
}

impl App {
    pub fn new() -> Result<Self>;
    pub fn with_config(config: RenderConfig) -> Result<Self>;
    pub fn run<C: Component>(&mut self, root: C) -> Result<()>;
}
```

### RenderConfig

```rust
pub struct RenderConfig {
    pub poll_duration_ms: u64,  // Event poll timeout (default: 16)
    pub use_double_buffer: bool, // Enable double buffering (default: true)
    pub use_diffing: bool,       // Enable cell diffing (default: true)
    pub use_alternate_screen: bool, // Use alternate screen (default: true)
}
```

## Key

```rust
pub enum Key {
    // Special keys
    Backspace, Enter, Left, Right, Up, Down,
    Home, End, PageUp, PageDown,
    Tab, Delete, Insert, Esc,

    // Function keys
    F(u8),  // F1-F12

    // Character
    Char(char),

    // Null
    Null,
}
```

### KeyWithModifiers

```rust
pub struct KeyWithModifiers {
    pub key: Key,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

impl KeyWithModifiers {
    pub fn with_ctrl(key: Key) -> Self;
    pub fn with_alt(key: Key) -> Self;
    pub fn with_shift(key: Key) -> Self;
    pub fn with_meta(key: Key) -> Self;
}
```

## Built-in Components

### TextInput

```rust
use rxtui::components::TextInput;

impl TextInput {
    pub fn new() -> Self;

    // Content
    pub fn placeholder(self, text: impl Into<String>) -> Self;
    pub fn password(self, enabled: bool) -> Self;

    // Container styling
    pub fn background(self, color: Color) -> Self;
    pub fn border(self, color: Color) -> Self;
    pub fn border_style(self, style: BorderStyle, color: Color) -> Self;
    pub fn border_edges(self, edges: BorderEdges) -> Self;
    pub fn border_full(self, style: BorderStyle, color: Color, edges: BorderEdges) -> Self;
    pub fn padding(self, spacing: Spacing) -> Self;
    pub fn z_index(self, z: i32) -> Self;
    pub fn position(self, pos: Position) -> Self;
    pub fn absolute(self) -> Self;
    pub fn top(self, offset: i16) -> Self;
    pub fn right(self, offset: i16) -> Self;
    pub fn bottom(self, offset: i16) -> Self;
    pub fn left(self, offset: i16) -> Self;

    // Sizing
    pub fn width(self, w: u16) -> Self;
    pub fn width_fraction(self, frac: f32) -> Self;
    pub fn width_auto(self) -> Self;
    pub fn width_content(self) -> Self;
    pub fn height(self, h: u16) -> Self;
    pub fn height_fraction(self, frac: f32) -> Self;
    pub fn height_auto(self) -> Self;
    pub fn height_content(self) -> Self;

    // Text styling
    pub fn content_color(self, color: Color) -> Self;
    pub fn content_bold(self, bold: bool) -> Self;
    pub fn content_background(self, color: Color) -> Self;
    pub fn placeholder_color(self, color: Color) -> Self;
    pub fn placeholder_background(self, color: Color) -> Self;
    pub fn placeholder_bold(self, bold: bool) -> Self;
    pub fn placeholder_italic(self, italic: bool) -> Self;
    pub fn placeholder_underline(self, underline: bool) -> Self;
    pub fn placeholder_style(self, style: TextStyle) -> Self;
    pub fn content_style(self, style: TextStyle) -> Self;
    pub fn cursor_color(self, color: Color) -> Self;

    // Focus
    pub fn focusable(self, enabled: bool) -> Self;
    pub fn focus_border(self, color: Color) -> Self;
    pub fn focus_border_style(self, style: BorderStyle, color: Color) -> Self;
    pub fn focus_background(self, color: Color) -> Self;
    pub fn focus_style(self, style: Style) -> Self;
    pub fn focus_padding(self, spacing: Spacing) -> Self;

    // Wrapping
    pub fn wrap(self, mode: TextWrap) -> Self;

    // Events
    pub fn on_change(self, callback: impl Fn(String) + 'static) -> Self;
    pub fn on_submit(self, callback: impl Fn() + 'static) -> Self;
    pub fn on_blur(self, callback: impl Fn() + 'static) -> Self;
    pub fn on_key(self, key: Key, handler: impl Fn() + 'static) -> Self;
    pub fn on_key_global(self, key: Key, handler: impl Fn() + 'static) -> Self;
    pub fn on_key_with_modifiers(self, key: KeyWithModifiers, handler: impl Fn() + 'static) -> Self;
    pub fn on_key_with_modifiers_global(
        self,
        key: KeyWithModifiers,
        handler: impl Fn() + 'static,
    ) -> Self;
}
```

Messages:
```rust
pub enum TextInputMsg {
    Focused,
    Blurred,
    CharInput(char),
    Backspace,
    Delete,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    // ... more
}
```

## Attribute Macros

### #[derive(Component)]

Automatically implements the Component trait:

```rust
#[derive(Component)]
struct MyComponent {
    // Fields
}
```

### #[component]

Enables collection of `#[effect]` methods:

```rust
#[derive(Component)]
struct Timer;

#[component]  // Required for #[effect]
impl Timer {
    // Methods
}
```

### #[update]

Simplifies update method with automatic state handling:

```rust
// Basic
#[update]
fn update(&self, ctx: &Context, msg: MyMsg) -> Action {
    // No state parameter = stateless
}

// With state
#[update]
fn update(&self, ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
    // State automatically fetched and passed
}

// With topics
#[update(msg = MyMsg, topics = ["topic" => TopicMsg])]
fn update(&self, ctx: &Context, messages: Messages, mut state: MyState) -> Action {
    match messages {
        Messages::MyMsg(msg) => { /* ... */ }
        Messages::TopicMsg(msg) => { /* ... */ }
    }
}

// Dynamic topics
#[update(msg = MyMsg, topics = [self.topic_field => TopicMsg])]
fn update(&self, ctx: &Context, messages: Messages, state: MyState) -> Action {
    // Topic name from component field
}
```

### #[view]

Simplifies view method with automatic state handling:

```rust
// Without state
#[view]
fn view(&self, ctx: &Context) -> Node {
    // No state needed
}

// With state
#[view]
fn view(&self, ctx: &Context, state: MyState) -> Node {
    // State automatically fetched and passed
}
```

### #[effect]

Marks async methods as effects (requires `effects` feature):

```rust
#[component]
impl MyComponent {
    #[effect]
    async fn background_task(&self, ctx: &Context) {
        // Async code
    }

    #[effect]
    async fn with_state(&self, ctx: &Context, state: MyState) {
        // Can access state
    }
}
```

## Effects (Feature-gated)

Enable with:
```toml
rxtui = { path = "rxtui", features = ["effects"] }
```

### Effect Type

```rust
pub type Effect = Pin<Box<dyn Future<Output = ()> + Send>>;
```

### Manual Implementation

```rust
impl Component for MyComponent {
    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        vec![
            Box::pin(async move {
                // Async task
            })
        ]
    }
}
```

## node! Macro

### Syntax Reference

```rust
node! {
    // Element types
    div(...) [...],
    text(...),
    richtext(...) [...],
    vstack(...) [...],
    hstack(...) [...],
    input(...),
    spacer(n),
    node(component),

    // Properties (in parentheses)
    prop: value,
    flag,  // Boolean flags

    // Children (in brackets)
    [
        child1,
        child2,
    ],

    // Event handlers (start with @)
    @event: handler,
}
```

### Property Shortcuts

| Short | Full | Type |
|-------|------|------|
| `bg` | `background` | Color |
| `dir` | `direction` | Direction |
| `pad` | `padding` | u16 (all sides) |
| `pad_h` | - | u16 (horizontal) |
| `pad_v` | - | u16 (vertical) |
| `w` | `width` | u16 |
| `h` | `height` | u16 |
| `w_frac` | - | f32 (0.0-1.0) |
| `h_frac` | - | f32 (0.0-1.0) |
| `w_auto` | - | flag |
| `h_auto` | - | flag |
| `w_content` | - | flag |
| `h_content` | - | flag |
| `justify` | `justify_content` | JustifyContent |
| `align` | `align_items` | AlignItems |
| `align_self` | - | AlignSelf |

### Color Values

```rust
// Named colors (no prefix needed)
color: red
color: bright_blue

// Hex strings
color: "#FF5733"
color: "#F50"

// Expressions (need parentheses)
color: (Color::Rgb(255, 0, 0))
color: (my_color_variable)

// Conditional
color: (if condition { red } else { blue })

// Optional (with ! suffix)
color: (optional_color)!
```

### Event Handlers

| Syntax | Description |
|--------|-------------|
| `@click: handler` | Mouse click |
| `@char('x'): handler` | Character key |
| `@key(enter): handler` | Special key |
| `@key(Char('-')): handler` | Character via Key enum |
| `@key(ctrl + 'c'): handler` | Key with modifiers |
| `@char_global('q'): handler` | Global character |
| `@key_global(esc): handler` | Global special key |
| `@key_global(ctrl + enter): handler` | Global key with modifiers |
| `@focus: handler` | Gained focus |
| `@blur: handler` | Lost focus |
| `@any_char: \|ch\| handler` | Any character |

## Helper Macros

### color_value!

Internal macro for parsing color values in node!:

```rust
color_value!(red)           // Named color
color_value!("#FF0000")     // Hex color
color_value!((expr))        // Expression
```

### direction_value!

Internal macro for parsing directions:

```rust
direction_value!(horizontal)
direction_value!(vertical)
direction_value!(h)  // Short for horizontal
direction_value!(v)  // Short for vertical
```

### justify_value!

Internal macro for parsing justify content values:

```rust
justify_value!(start)
justify_value!(center)
justify_value!(end)
justify_value!(space_between)
justify_value!(space_around)
justify_value!(space_evenly)
```

### align_items_value!

Internal macro for parsing align items values:

```rust
align_items_value!(start)
align_items_value!(center)
align_items_value!(end)
```

### align_self_value!

Internal macro for parsing align self values:

```rust
align_self_value!(auto)
align_self_value!(start)
align_self_value!(center)
align_self_value!(end)
```

## Type Aliases

```rust
pub type ComponentId = String;
pub type TopicName = String;
```

## Traits

### MessageExt

Extension trait for message downcasting:

```rust
pub trait MessageExt {
    fn downcast<T: Any>(&self) -> Option<&T>;
}
```

### StateExt

Extension trait for state downcasting:

```rust
pub trait StateExt {
    fn downcast<T: Any>(&self) -> Option<&T>;
}
```

## Error Types

RxTUI uses `std::io::Result` for most operations that can fail (terminal I/O).

## Platform Support

- **Unix/Linux**: Full support
- **macOS**: Full support
- **Windows**: Supported via crossterm backend

## Feature Flags

| Flag | Description |
|------|-------------|
| `effects` | Enable async effects system (requires tokio) |

## Thread Safety

- Components must be `Send + Sync + 'static`
- State and Messages must be `Send + Sync + 'static`
- Effects run on a separate Tokio runtime

## Performance Considerations

- Virtual DOM diffing minimizes updates
- Double buffering eliminates flicker
- Cell-level diffing reduces terminal I/O
- Lazy state cloning only when modified
- Topic messages use zero-copy routing when possible
