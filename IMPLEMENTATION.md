# RxTUI - Implementation Details

## Overview

RxTUI is a reactive terminal user interface framework inspired by Elm's message-passing architecture and React's component model. It provides a declarative, component-based API for building interactive terminal applications with efficient rendering through virtual DOM diffing and advanced cross-component communication via topic-based messaging.

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                     Component System                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │  Components  │  │   Messages   │  │    Topics    │   │
│  │  - update()  │  │  - Direct    │  │  - Ownership │   │
│  │  - view()    │  │  - Topic     │  │  - Broadcast │   │
│  │  - effects() │  │  - Async     │  │  - State     │   │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘   │
│         │                 │                 │           │
│  ┌──────▼─────────────────▼─────────────────▼────────┐  │
│  │                     Context                       │  │
│  │  - StateMap: Component state storage              │  │
│  │  - Dispatcher: Message routing                    │  │
│  │  - TopicStore: Topic ownership & state            │  │
│  └──────────────────────┬────────────────────────────┘  │
└─────────────────────────┼───────────────────────────────┘
                          │
┌─────────────────────────▼──────────────────────────────┐
│                    Rendering Pipeline                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │     Node     │──│     VNode    │──│  RenderNode  │  │
│  │  (Component) │  │  (Virtual)   │  │ (Positioned) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                 │          │
│  ┌──────▼─────────────────▼─────────────────▼───────┐  │
│  │                   Virtual DOM (VDom)             │  │
│  │  - Diff: Compare old and new trees               │  │
│  │  - Patch: Generate minimal updates               │  │
│  │  - Layout: Calculate positions and sizes         │  │
│  └──────────────────────┬───────────────────────────┘  │
└─────────────────────────┼──────────────────────────────┘
                          │
┌─────────────────────────▼──────────────────────────────┐
│                     Terminal Output                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │Double Buffer │  │Cell Diffing  │  │  Optimized   │  │
│  │  Front/Back  │  │   Updates    │  │   Renderer   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Component System (`lib/component.rs`)

The component system is the heart of the framework, providing a React-like component model with state management and message passing.

#### Component Trait

```rust
pub trait Component: 'static {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        Action::default()
    }

    fn view(&self, ctx: &Context) -> Node;

    #[cfg(feature = "effects")]
    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        vec![]
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

**Key Design Decisions:**
- Components are stateless - all state is managed by Context
- Update method receives optional topic for cross-component messaging
- Components can be derived using `#[derive(Component)]` macro
- Effects support async background tasks (with feature flag)
- Default implementations provided for update and effects

#### Message and State Traits

Both Message and State traits are auto-implemented for any type that is `Clone + Send + Sync + 'static`:

```rust
pub trait Message: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Message>;
}

pub trait State: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn State>;
}
```

**Extension Traits for Downcasting:**
- `MessageExt` provides `downcast<T>()` for message type checking
- `StateExt` provides `downcast<T>()` for state type checking

#### Actions

Components return actions from their update method:

```rust
#[derive(Default)]
pub enum Action {
    Update(Box<dyn State>),              // Update component's local state
    UpdateTopic(String, Box<dyn State>), // Update topic state (first writer owns)
    None,                                // No action needed
    #[default]
    Exit,                                // Exit the application
}
```

Helper methods for ergonomic construction:
```rust
Action::update(state)        // Create Update action
Action::update_topic(topic, state)  // Create UpdateTopic action
Action::none()              // Create None action
Action::exit()              // Create Exit action
```

### 2. Context System (`lib/app/context.rs`)

The Context provides components with everything they need to function:

#### Core Components

**Context Structure:**
- `current_component_id`: Component being processed
- `dispatch`: Message dispatcher
- `states`: Component state storage (StateMap)
- `topics`: Topic-based messaging (Arc<TopicStore>)
- `message_queues`: Regular message queues (Arc<RwLock<HashMap>>)
- `topic_message_queues`: Topic message queues (Arc<RwLock<HashMap>>)

**StateMap:**
- Stores component states with interior mutability using `Arc<RwLock<HashMap>>`
- `get_or_init<T>()`: Get state or initialize with Default, handles type mismatches
- Type-safe state retrieval with automatic downcasting
- Thread-safe with RwLock protection

**Dispatcher:**
- Routes messages to components or topics
- `send_to_id(component_id, message)`: Direct component messaging
- `send_to_topic(topic, message)`: Topic-based messaging
- Shared message queue storage with Context

**TopicStore:**
- Manages topic ownership (first writer becomes owner)
- Stores topic states separately from component states
- Tracks which component owns which topic via `owners: RwLock<HashMap<String, ComponentId>>`
- Thread-safe with RwLock protection
- `update_topic()`: Returns bool indicating if update was successful

#### Context Public API

```rust
impl Context {
    // Message handling
    pub fn handler<M: Message>(&self, msg: M) -> Box<dyn Fn() + 'static>;
    pub fn handler_with_value<F, M, T>(&self, f: F) -> Box<dyn Fn(T) + 'static>;

    // State management
    pub fn get_state<S: State + Default + Clone>(&self) -> S;
    pub fn get_state_or<S: State + Clone>(&self, default: S) -> S;

    // Direct messaging
    pub fn send<M: Message>(&self, msg: M);

    // Topic messaging
    pub fn send_to_topic<M: Message>(&self, topic: &str, msg: M);
    pub fn read_topic<S: State + Clone>(&self, topic: &str) -> Option<S>;
}
```

#### Message Flow

1. **Direct Messages**: Sent to specific component via `ctx.handler(msg)` which creates closures
2. **Topic Messages**: Sent via `ctx.send_to_topic(topic, msg)`
   - If topic has owner → delivered only to owner
   - If no owner → broadcast to all components until one claims it

### 3. Topic-Based Messaging System

A unique feature for cross-component communication without direct references:

#### Concepts

- **Topics**: Named channels for messages (e.g., "counter_a", "global_state")
- **Ownership**: First component to write to a topic becomes its owner
- **Unassigned Messages**: Messages to unclaimed topics are broadcast to all components

#### How It Works

1. **Sending Messages:**
   ```rust
   ctx.send_to_topic("my-topic", MyMessage);
   ```

2. **Claiming Ownership:**
   ```rust
   // First component to return this action owns the topic
   Action::UpdateTopic("my-topic".to_string(), Box::new(MyState))
   ```

3. **Handling Topic Messages (Using Macros):**
   ```rust
   // With the #[update] macro:
   #[update(msg = MyMsg, topics = ["my-topic" => TopicMsg])]
   fn update(&self, ctx: &Context, messages: Messages, mut state: MyState) -> Action {
       match messages {
           Messages::MyMsg(msg) => { /* handle regular message */ }
           Messages::TopicMsg(msg) => { /* handle topic message */ }
       }
   }

   // Dynamic topics from component fields:
   #[update(msg = MyMsg, topics = [self.topic_name => TopicMsg])]
   fn update(&self, ctx: &Context, messages: Messages, state: MyState) -> Action {
       // Topic name from self.topic_name field
   }
   ```

4. **Reading Topic State:**
   ```rust
   let state: Option<MyState> = ctx.read_topic("my-topic");
   ```

**Design Rationale:**
- Enables decoupled component communication
- Supports both single-writer/multiple-reader and broadcast patterns
- Automatic ownership management prevents conflicts
- Idempotent updates - multiple attempts to claim ownership are safe

### 4. Application Core (`lib/app/core.rs`)

The App struct manages the entire application lifecycle:

#### Initialization
```rust
App::new()  // Standard initialization
App::with_config(RenderConfig { ... })  // With custom config
```
- Enables terminal raw mode and alternate screen
- Hides cursor and enables mouse capture
- Initializes double buffer for flicker-free rendering
- Sets up event handling with crossterm
- Creates effect runtime (if feature enabled) using Tokio

#### Event Loop

The main loop (`run_loop`) follows this sequence:

1. **Component Tree Expansion**:
   - Start with root component
   - Recursively expand components to VNodes
   - Assign component IDs based on tree position using `ComponentId::child(index)` method (e.g., "0", "0.0", "0.1")

2. **Message Processing**:
   - Components drain all pending messages (regular + topic)
   - Messages trigger state updates via component's `update` method
   - Handle actions (Update, UpdateTopic, Exit, None)

3. **Virtual DOM Update**:
   - VDom diffs new tree against current
   - Generates patches for changes
   - Updates render tree

4. **Layout & Rendering**:
   - Calculate positions and sizes based on Dimension types
   - Render to back buffer
   - Diff buffers and apply changes to terminal

5. **Event Handling**:
   - Process keyboard/mouse events (poll with 16ms timeout by default)
   - Events trigger new messages via event handlers
   - Handle terminal resize events

6. **Effect Management** (if feature enabled):
   - Spawn effects for newly mounted components
   - Cleanup effects for unmounted components
   - Effects run in Tokio runtime with JoinHandle tracking

#### Component Tree Expansion

The `expand_component_tree` method is crucial:

1. Drains all messages for the component (both regular and topic messages)
2. Processes each message:
   - Regular messages → component's update
   - Topic messages → check if component handles topic
3. Handles actions:
   - `Update` → update component state via StateMap
   - `UpdateTopic` → update topic state, claim ownership if first
   - `Exit` → propagate exit signal
   - `None` → no operation
4. Calls component's `view` to get UI tree
5. Recursively expands child components

### 5. Node Types

Three levels of node representation:

#### Node (`lib/node/mod.rs`)
High-level component tree:
```rust
pub enum Node {
    Component(Arc<dyn Component>),  // Component instance (Arc for sharing)
    Div(Div<Node>),                 // Container with children
    Text(Text),                     // Text content
    RichText(RichText),            // Styled text with multiple spans
}
```

#### VNode (`lib/vnode.rs`)
Virtual DOM nodes after component expansion:
```rust
pub enum VNode {
    Div(Div<VNode>),        // Expanded div (generic over child type)
    Text(Text),             // Text node
    RichText(RichText),     // Rich text node
}
```

#### RenderNode (`lib/render_tree/node.rs`)
Positioned nodes ready for drawing:
```rust
pub struct RenderNode {
    pub node_type: RenderNodeType,
    pub x: u16, pub y: u16,           // Position
    pub width: u16, pub height: u16,   // Size
    pub content_width: u16,            // Actual content size
    pub content_height: u16,
    pub scroll_y: u16,                 // Vertical scroll offset
    pub scrollable: bool,              // Has overflow:scroll/auto
    pub style: Option<Style>,          // Visual style
    pub children: Vec<Rc<RefCell<RenderNode>>>,
    pub parent: Option<Weak<RefCell<RenderNode>>>,
    pub focusable: bool,
    pub focused: bool,
    pub dirty: bool,
    pub z_index: i32,
    // Event handlers stored as Rc<dyn Fn()>
}
```

### 6. Div System (`lib/node/div.rs`)

Divs are generic containers that can hold different child types:

```rust
pub struct Div<T> {
    pub children: Vec<T>,
    pub styles: DivStyles,           // Base, focus, hover styles
    pub gap: Option<u16>,
    pub wrap: Option<WrapMode>,
    pub focusable: bool,
    pub overflow: Option<Overflow>,
    pub show_scrollbar: Option<bool>,
    pub callbacks: EventCallbacks,   // Click, focus, blur handlers
    pub key_handlers: Vec<KeyHandler>,
    pub global_key_handlers: Vec<KeyHandler>,
    pub key_with_modifiers_handlers: Vec<KeyWithModifiersHandler>,
    pub any_char_handler: Option<Rc<dyn Fn(char) -> Box<dyn Message>>>,
}
```

#### DivStyles
```rust
pub struct DivStyles {
    pub base: Option<Style>,    // Normal style
    pub focus: Option<Style>,   // When focused
    pub hover: Option<Style>,   // When hovered (future)
}
```

#### Builder Pattern

Both the builder pattern and the `node!` macro are fully supported ways to create UIs. Choose based on your preference and use case.

```rust
// Using the builder pattern
Div::new()
    .background(Color::Blue)
    .padding(Spacing::all(2))
    .direction(Direction::Horizontal)
    .width(20)
    .height_fraction(0.5)
    .focusable(true)
    .overflow(Overflow::Scroll)
    .show_scrollbar(true)
    .on_click(handler)
    .on_key(Key::Enter, handler)
    .on_key_with_modifiers(KeyWithModifiers::with_ctrl(Key::Char('a')), handler)
    .children(vec![...])

// Using the node! macro
node! {
    div(
        bg: blue,
        pad: 2,
        dir: horizontal,
        w: 20,
        h_frac: 0.5,
        focusable,
        overflow: scroll,
        show_scrollbar: true
    ) [
        // Children using expressions or spread
        (child_node),
        ...(child_nodes)
    ]
}
```

### 7. Virtual DOM (`lib/vdom.rs`)

Manages UI state and efficient updates:

#### Core Operations

1. **Render**: Accept new VNode tree
2. **Diff**: Compare with current tree
3. **Patch**: Apply changes to render tree
4. **Layout**: Calculate positions based on constraints
5. **Draw**: Output to terminal

#### Diffing Algorithm (`lib/diff.rs`)

Generates minimal patches:
```rust
pub enum Patch {
    Replace {
        old: Rc<RefCell<RenderNode>>,
        new: VNode,
    },
    UpdateText {
        node: Rc<RefCell<RenderNode>>,
        new_text: String,
        new_style: Option<TextStyle>,
    },
    UpdateRichText {
        node: Rc<RefCell<RenderNode>>,
        new_spans: Vec<TextSpan>,
    },
    UpdateProps {
        node: Rc<RefCell<RenderNode>>,
        div: Div<VNode>,
    },
    AddChild {
        parent: Rc<RefCell<RenderNode>>,
        child: VNode,
        index: usize,
    },
    RemoveChild {
        parent: Rc<RefCell<RenderNode>>,
        index: usize,
    },
    ReorderChildren {
        parent: Rc<RefCell<RenderNode>>,
        moves: Vec<Move>,
    },
}
```

### 8. Layout System (`lib/render_tree/tree.rs`)

Sophisticated layout engine supporting multiple sizing modes:

#### Dimension Types
```rust
pub enum Dimension {
    Fixed(u16),       // Exact size in cells
    Percentage(f32),  // Percentage of parent (stored 0.0-1.0)
    Auto,            // Share remaining space equally
    Content,         // Size based on children
}
```

#### Layout Algorithm

1. **Fixed**: Use exact size
2. **Percentage**: Calculate from parent size (content box after padding)
3. **Content**:
   - Horizontal: width = sum of children + gaps, height = max child
   - Vertical: width = max child, height = sum of children + gaps
4. **Auto**: Divide remaining space equally among auto-sized elements

#### Text Wrapping
Multiple wrapping modes supported:
- `None`: No wrapping
- `Character`: Break at any character
- `Word`: Break at word boundaries
- `WordBreak`: Try words, break if necessary

#### Scrolling Support
- **Vertical scrolling**: Implemented with scroll_y offset
- **Scrollbar rendering**: Optional visual indicator showing position
- **Keyboard navigation**: Up/Down arrows, PageUp/PageDown, Home/End
- **Mouse wheel**: ScrollUp/ScrollDown events
- **Content tracking**: content_height vs container height
- **Focus requirement**: Container must be focusable for keyboard scrolling
- **Note**: Horizontal scrolling not yet implemented

### 9. Rendering Pipeline (`lib/app/renderer.rs`)

Converts render tree to terminal output:

#### Rendering Steps

1. **Clear Background**: Fill with parent background color or inherit
2. **Draw Borders**: Render border characters if present (single, double, rounded, thick)
3. **Apply Padding**: Adjust content area based on Spacing
4. **Handle Scrolling**: Apply scroll_y offset for scrollable containers
5. **Render Content**:
   - For containers: Recurse into children respecting z-index
   - For text: Draw text with wrapping and style
   - For rich text: Draw styled segments preserving individual styles
6. **Apply Clipping**: Ensure content stays within bounds using clip_rect
7. **Draw Scrollbar**: Show position indicator if enabled and scrollable

#### Style Inheritance
- Text nodes inherit parent's background if not specified
- Focus styles override normal styles when focused
- Children can override parent styles
- Rich text spans maintain individual styles

### 10. Terminal Output System

#### Double Buffering (`lib/buffer.rs`)

Eliminates flicker completely:

```rust
pub struct DoubleBuffer {
    front_buffer: ScreenBuffer,  // Currently displayed
    back_buffer: ScreenBuffer,   // Next frame
    width: u16,
    height: u16,
}
```

**Cell Structure:**
```rust
pub struct Cell {
    pub char: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub style: TextStyle,  // Bitflags for bold, italic, etc.
}
```

**Diff Process:**
1. Render to back buffer
2. Compare with front buffer cell-by-cell
3. Generate list of changed cells with positions
4. Apply updates to terminal in optimal order
5. Swap buffers

#### Terminal Renderer (`lib/terminal.rs`)

Optimized output with multiple strategies:

1. **Batch Updates**: Group cells with same colors
2. **Skip Unchanged**: Only update modified cells
3. **Optimize Movements**: Minimize cursor jumps using manhattan distance
4. **Style Batching**: Combine style changes
5. **ANSI Escape Sequences**: Direct terminal control

### 11. Event System (`lib/app/events.rs`)

Comprehensive input handling using crossterm:

#### Keyboard Events

**Focus Navigation:**
- Tab: Next focusable element
- Shift+Tab: Previous focusable element

**Scrolling (for focused scrollable elements):**
- Up/Down arrows: Scroll by 1 line
- PageUp/PageDown: Scroll by container height
- Home/End: Jump to top/bottom

**Event Routing:**
1. Global handlers always receive events (marked with `_global`)
2. Focused element receives local events
3. Character and key handlers triggered with modifiers support

#### Mouse Events

**Click Handling:**
1. Find node at click position using tree traversal
2. Set focus if focusable
3. Trigger click handler

**Scroll Handling:**
1. Find scrollable node under cursor
2. Apply scroll delta (3 lines per wheel event)
3. Clamp to content bounds

### 12. RichText System (`lib/node/rich_text.rs`)

Provides inline text styling with multiple spans:

#### Core Structure
```rust
pub struct RichText {
    pub spans: Vec<TextSpan>,
    pub style: Option<TextStyle>,  // Top-level style for wrapping, etc.
}

pub struct TextSpan {
    pub content: String,
    pub style: Option<TextStyle>,
}
```

#### Builder API
```rust
RichText::new()
    .text("Normal text ")
    .colored("red text", Color::Red)
    .bold("bold text")
    .italic("italic text")
    .styled("custom", TextStyle { ... })
    .wrap(TextWrap::Word)
```

#### Special Features
- **Multiple Spans**: Each span can have different styling
- **Top-Level Styling**: Apply wrapping or common styles to all spans
- **Helper Methods**: `bold_all()`, `color()` for all spans
- **Text Wrapping**: Preserves span styles across wrapped lines
- **Cursor Support**: `with_cursor()` for text input components

### 13. Style System (`lib/style.rs`)

Rich styling capabilities:

#### Colors
- 16 standard terminal colors (Black, Red, Green, Yellow, Blue, Magenta, Cyan, White)
- Bright variants (BrightBlack through BrightWhite)
- RGB support (24-bit color) via `Rgb(u8, u8, u8)`
- Hex color parsing support

#### Text Styles
```rust
pub struct TextStyle {
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub strikethrough: Option<bool>,
    pub wrap: Option<TextWrap>,
}
```

Style merging support with `TextStyle::merge()` for inheritance.

#### Borders
```rust
pub struct Border {
    pub enabled: bool,
    pub style: BorderStyle,  // Single, Double, Rounded, Thick
    pub color: Color,
    pub edges: BorderEdges,  // Bitflags for TOP, RIGHT, BOTTOM, LEFT
}
```

#### Position
```rust
pub enum Position {
    Relative,  // Normal flow
    Absolute,  // Positioned relative to parent
}
```

With top, right, bottom, left offsets for absolute positioning.

#### Overflow
```rust
pub enum Overflow {
    None,   // Content not clipped
    Hidden, // Content clipped at boundaries
    Scroll, // Content clipped but scrollable
    Auto,   // Auto show scrollbars
}
```

### 14. Macro System

Provides ergonomic APIs for building UIs:

#### node! Macro (`lib/macros/node.rs`)
Declarative syntax for building UI trees:
```rust
node! {
    div(bg: blue, pad: 2, @click: ctx.handler(Msg::Click), @key(enter): ctx.handler(Msg::Enter)) [
        text("Hello", color: white),
        div(border: white) [
            text("Nested")
        ]
    ]
}
```

Features:
- Property shortcuts (bg, pad, w, h, etc.)
- Color literals (red, blue, "#FF5733")
- Event handler syntax with @ prefix
- Optional properties with ! suffix
- Stack helpers (hstack, vstack)

#### Attribute Macros (from rxtui-macros crate)

**#[derive(Component)]**: Auto-implements Component trait with providers pattern

**#[component]**: Collects #[effect] methods for async support, implements `__component_effects_impl`

**#[update]**: Handles message downcasting and state management, supports topic routing

**#[view]**: Automatically fetches component state via `ctx.get_state()`

**#[effect]**: Marks async methods as effects, collected by #[component]

### 15. Effects System (`lib/effect/`, requires feature flag)

Supports async background tasks:

#### Effect Runtime (`lib/effect/runtime.rs`)
- Spawns Tokio runtime for async execution
- Manages effect lifecycle per component with JoinHandle tracking
- Automatic cleanup on component unmount via abort()
- Effects stored in HashMap<ComponentId, Vec<JoinHandle>>

#### Effect Type
```rust
pub type Effect = Pin<Box<dyn Future<Output = ()> + Send>>;
```

#### Effect Definition with Macros
```rust
#[component]
impl MyComponent {
    #[effect]
    async fn background_task(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send(MyMsg::Tick);
        }
    }

    #[effect]
    async fn with_state(&self, ctx: &Context, state: MyState) {
        // State automatically fetched via ctx.get_state()
        if state.should_process {
            // Do work
        }
    }
}
```

#### Common Use Cases
- Timers and periodic updates
- Network requests with reqwest/hyper
- File system monitoring with notify
- WebSocket connections
- Background computations

### 16. Built-in Components

#### TextInput (`lib/components/text_input.rs`)

Full-featured text input component with:
- Text editing operations (insert, delete, backspace)
- Cursor movement (arrows, Home/End, word navigation)
- Word operations (Ctrl+W delete word, Alt+B/F word movement)
- Line operations (Ctrl+U/K delete to start/end)
- Password mode for masked input
- Placeholder text with customizable styling
- Focus management and styling
- Selection support (partial implementation)
- Builder pattern for configuration

State management:
```rust
pub struct TextInputState {
    pub focused: bool,
    pub content: String,
    pub cursor_position: usize,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
}
```

## Performance Optimizations

### 1. Virtual DOM
- Minimal patch generation with path-based updates
- Short-circuit unchanged subtrees via equality checks
- Efficient tree traversal with indexed paths

### 2. Double Buffering
- Zero flicker guaranteed
- Cell-level diffing reduces writes
- Only changed cells updated to terminal

### 3. Terminal Renderer
- Batch color changes to reduce escape sequences
- Optimize cursor movements with distance calculations
- Skip unchanged regions entirely

### 4. Message System
- Zero-copy message routing where possible using Arc
- Lazy state cloning only on modification
- Efficient topic distribution with ownership tracking

### 5. Memory Management
- Rc/RefCell for shared ownership in render tree
- Weak references prevent cycles
- Minimal allocations during render
- Arc for thread-safe component sharing

## Configuration

### RenderConfig
Controls rendering behavior for debugging:
```rust
pub struct RenderConfig {
    pub poll_duration_ms: u64,        // Event poll timeout (default 16ms)
    pub use_double_buffer: bool,      // Enable/disable double buffering
    pub use_diffing: bool,            // Enable/disable cell diffing
    pub use_alternate_screen: bool,   // Use alternate screen buffer
}
```

## Testing Support

### Unit Testing
- Components can be tested in isolation
- Mock Context for state and message testing
- VNode equality for view testing

### Integration Testing
- Test harness for full app testing (planned)
- Event simulation support
- Buffer inspection for render verification

## Platform Compatibility

- **Unix/Linux**: Full support via crossterm
- **macOS**: Full support including iTerm2 features
- **Windows**: Support via Windows Terminal and ConPTY

## Future Enhancements

### Planned Features
- Horizontal scrolling support
- More built-in components (Button, Select, Table, List)
- Animation system with interpolation
- Layout constraints and flexbox-like model
- Accessibility features (screen reader support)
- Hot reload for development
