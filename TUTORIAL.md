# RxTUI Tutorial

Learn RxTUI step by step, from basics to advanced features.

## Prerequisites

- Basic Rust knowledge
- A terminal that supports colors and mouse input
- Rust toolchain installed

## Chapter 1: Your First Component

Let's start with the simplest possible RxTUI app.

### Hello World

```rust
use rxtui::prelude::*;

// Define a component
#[derive(Component)]
struct HelloWorld;

impl HelloWorld {
    // Define how it looks
    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(bg: blue, pad: 2, @key_global(esc): ctx.handler(())) [
                text("Hello, RxTUI!", color: white, bold),
                text("Press Esc to exit", color: white)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(HelloWorld)
}
```

Run it:
```bash
cargo run
```

Press `Esc` to exit.

### What's happening?

1. **Component**: We define a struct and derive `Component`
2. **View**: The `#[view]` method returns what to display
3. **node! macro**: Creates the UI tree declaratively
4. **App**: Manages the terminal and event loop

## Chapter 2: Adding State

Most UIs need to manage data. Let's add state to our component.

### Counter with State

```rust
use rxtui::prelude::*;

// State holds our data
#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

#[derive(Component)]
struct Counter;

impl Counter {
    #[view]
    fn view(&self, _ctx: &Context, state: CounterState) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Count: {}", state.count), color: white)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Counter)
}
```

The `#[view]` macro automatically fetches the state and passes it as a parameter.

## Chapter 3: Handling Events

Now let's make it interactive by adding messages and updates.

### Interactive Counter

```rust
use rxtui::prelude::*;

// Messages represent events
#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

#[derive(Component)]
struct Counter;

impl Counter {
    // Handle messages and update state
    #[update]
    fn update(&self, _ctx: &Context, msg: CounterMsg, mut state: CounterState) -> Action {
        match msg {
            CounterMsg::Increment => {
                state.count += 1;
                Action::update(state)  // Save new state
            }
            CounterMsg::Decrement => {
                state.count -= 1;
                Action::update(state)
            }
            CounterMsg::Exit => Action::exit(),  // Exit app
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        node! {
            div(bg: black, pad: 2, @char_global('+'): ctx.handler(CounterMsg::Increment), @char_global('-'): ctx.handler(CounterMsg::Decrement), @char_global('q'): ctx.handler(CounterMsg::Exit)) [
                text(format!("Count: {}", state.count), color: white),
                text("Press +/- to change, q to quit", color: gray)
            ]
        }
    }
}
```

### The Update Cycle

1. User presses a key
2. Event handler creates a message
3. `update` processes the message
4. State changes
5. View re-renders with new state

## Chapter 4: Building UIs with node!

The `node!` macro is how you build UIs. Let's explore its features.

### Layout and Styling

```rust
#[view]
fn view(&self, ctx: &Context, state: MyState) -> Node {
    node! {
        // Vertical layout with padding
        div(bg: "#1a1a1a", pad: 2, gap: 1) [
            // Title with styling
            text("My App", color: cyan, bold, underline),

            // Horizontal layout
            hstack(gap: 2) [
                // Fixed width box
                div(w: 20, h: 5, border: white) [
                    text("Left panel")
                ],

                // Percentage width
                div(w_frac: 0.5, border: blue) [
                    text("Center (50%)")
                ],

                // Auto-sizing
                div(w_auto, border: green) [
                    text("Right (auto)")
                ]
            ],

            // Spacer
            spacer(2),

            // Footer
            text("Status: Ready", color: bright_green)
        ]
    }
}
```

### Rich Text

```rust
node! {
    div [
        // Inline styled text
        richtext [
            text("Status: "),
            text("SUCCESS", color: green, bold),
            text(" - "),
            text("5 items", color: yellow),
            text(" processed")
        ]
    ]
}
```

## Chapter 5: Interactive Elements

Let's make clickable buttons and focusable elements.

### Buttons and Focus

```rust
#[derive(Debug, Clone)]
enum AppMsg {
    ButtonClicked(String),
    InputFocused,
}

#[view]
fn view(&self, ctx: &Context, state: AppState) -> Node {
    node! {
        div(pad: 2) [
            text("Click buttons or use Tab to navigate:", color: yellow),

            hstack(gap: 2) [
                // Clickable button
                div(
                    border: white,
                    pad: 1,
                    focusable,  // Can receive focus
                    focus_style: ({
                        Style::default()
                            .background(Color::Blue)
                            .border(Color::Yellow)
                    }),
                    @click: ctx.handler(AppMsg::ButtonClicked("One".into())),
                    @key(enter): ctx.handler(AppMsg::ButtonClicked("One".into()))
                ) [
                    text("Button 1")
                ],

                // Another button
                div(border: white, pad: 1, focusable, @click: ctx.handler(AppMsg::ButtonClicked("Two".into())), @key(enter): ctx.handler(AppMsg::ButtonClicked("Two".into()))) [
                    text("Button 2")
                ]
            ],

            // Display which was clicked
            text(format!("Last clicked: {}", state.last_clicked))
        ]
    }
}
```

### Focus Navigation

- **Tab**: Move to next focusable element
- **Shift+Tab**: Move to previous
- **Enter**: Activate focused element

## Chapter 6: Text Input

RxTUI includes a built-in TextInput component.

### Using TextInput

```rust
use rxtui::prelude::*;
use rxtui::components::TextInput;

#[derive(Debug, Clone)]
enum FormMsg {
    Submit,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct FormState {
    submitted: bool,
}

#[derive(Component)]
struct Form;

impl Form {
    #[update]
    fn update(&self, _ctx: &Context, msg: FormMsg, mut state: FormState) -> Action {
        match msg {
            FormMsg::Submit => {
                state.submitted = true;
                Action::update(state)
            }
            FormMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: FormState) -> Node {
        node! {
            div(pad: 2, @key_global(esc): ctx.handler(FormMsg::Exit)) [
                text("User Form", bold, color: cyan),

                // Text inputs
                text("Name:"),
                input(placeholder: "Enter your name...", focusable),

                spacer(1),

                text("Email:"),
                input(
                    placeholder: "user@example.com",
                    w: 40,
                    border: cyan,
                    focusable
                ),

                spacer(1),

                // Password input
                text("Password:"),
                input(
                    placeholder: "********",
                    password,  // Masks input
                    border: yellow,
                    focusable
                ),

                spacer(2),

                // Submit button
                div(border: green, pad: 1, focusable, @click: ctx.handler(FormMsg::Submit), @key(enter): ctx.handler(FormMsg::Submit)) [
                    text("Submit")
                ],

                // Status
                if state.submitted {
                    text("Form submitted!", color: green)
                }
            ]
        }
    }
}
```

## Chapter 7: Component Communication

Components can communicate using topics - a pub/sub system.

### Parent-Child with Topics

```rust
// Shared message type
#[derive(Debug, Clone)]
struct UpdateRequest {
    value: String,
}

// Child component that sends updates
#[derive(Component)]
struct Child {
    id: String,
}

impl Child {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    #[update]
    fn update(&self, ctx: &Context, msg: ChildMsg, state: ChildState) -> Action {
        match msg {
            ChildMsg::SendUpdate => {
                // Send to parent via topic
                ctx.send_to_topic("parent.updates", UpdateRequest {
                    value: format!("Update from {}", self.id)
                });
                Action::none()
            }
        }
    }

    #[view]
    fn view(&self, ctx: &Context, _state: ChildState) -> Node {
        node! {
            div(border: white, pad: 1, @click: ctx.handler(ChildMsg::SendUpdate)) [
                text(format!("Child: {}", self.id))
            ]
        }
    }
}

// Parent that receives updates
#[derive(Component)]
struct Parent;

impl Parent {
    // Listen to topic messages
    #[update(msg = ParentMsg, topics = ["parent.updates" => UpdateRequest])]
    fn update(&self, _ctx: &Context, messages: Messages, mut state: ParentState) -> Action {
        match messages {
            Messages::ParentMsg(msg) => {
                // Handle own messages
                Action::none()
            }
            Messages::UpdateRequest(req) => {
                // Handle topic messages
                state.last_update = req.value;
                Action::update(state)  // Claim topic ownership
            }
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: ParentState) -> Node {
        node! {
            div(pad: 2) [
                text("Parent Component", bold),
                text(format!("Last update: {}", state.last_update)),

                hstack(gap: 2) [
                    node(Child::new("A")),
                    node(Child::new("B")),
                    node(Child::new("C"))
                ]
            ]
        }
    }
}
```

## Chapter 8: Scrollable Content

Handle content that exceeds the viewport.

### Scrollable List

```rust
#[view]
fn view(&self, ctx: &Context, state: ListState) -> Node {
    node! {
        div(pad: 2) [
            text("Scrollable List (use arrows when focused):", color: yellow),

            // Scrollable container
            div(
                h: 10,               // Fixed height
                border: white,
                overflow: scroll,    // Enable scrolling
                show_scrollbar: true,
                focusable           // Must be focusable for keyboard control
            ) [
                // Generate many items
                {state.items.iter().enumerate().map(|(i, item)| {
                    node! {
                        div [
                            text(format!("{}: {}", i + 1, item))
                        ]
                    }
                }).collect::<Vec<_>>()}
            ],

            text("Arrow keys: scroll, Page Up/Down: page, Home/End: jump", color: gray)
        ]
    }
}
```

## Chapter 9: Async Effects

Use effects for background tasks like timers or API calls.

### Timer with Effects

```rust
use rxtui::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone)]
enum ClockMsg {
    Tick,
    Reset,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct ClockState {
    seconds: u32,
}

#[derive(Component)]
struct Clock;

// Enable the #[effect] macro by adding `#[component]` attribute
#[component]
impl Clock {
    #[update]
    fn update(&self, _ctx: &Context, msg: ClockMsg, mut state: ClockState) -> Action {
        match msg {
            ClockMsg::Tick => {
                state.seconds += 1;
                Action::update(state)
            }
            ClockMsg::Reset => Action::update(ClockState::default()),
            ClockMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: ClockState) -> Node {
        let time = format!("{:02}:{:02}", state.seconds / 60, state.seconds % 60);

        node! {
            div(bg: black, pad: 2, @char_global('r'): ctx.handler(ClockMsg::Reset), @char_global('q'): ctx.handler(ClockMsg::Exit)) [
                text(&time, color: cyan, bold),
                text("Press r to reset, q to quit", color: gray)
            ]
        }
    }

    // Async effect that runs in background
    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send(ClockMsg::Tick);
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Clock)
}
```

Remember to add tokio to your dependencies:
```toml
[dependencies]
rxtui = { path = "rxtui", features = ["effects"] }
tokio = { version = "1.0", features = ["full"] }
```

## Chapter 10: Complete Application

Let's build a todo list app combining everything we've learned.

### Todo List App

```rust
use rxtui::prelude::*;
use rxtui::components::TextInput;

#[derive(Debug, Clone)]
enum TodoMsg {
    AddTodo,
    ToggleTodo(usize),
    DeleteTodo(usize),
    ClearCompleted,
    Exit,
}

#[derive(Debug, Clone)]
struct Todo {
    text: String,
    completed: bool,
}

#[derive(Debug, Clone, Default)]
struct TodoState {
    todos: Vec<Todo>,
    filter: Filter,
}

#[derive(Debug, Clone, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self { Filter::All }
}

#[derive(Component)]
struct TodoApp;

impl TodoApp {
    #[update]
    fn update(&self, _ctx: &Context, msg: TodoMsg, mut state: TodoState) -> Action {
        match msg {
            TodoMsg::AddTodo => {
                // Note: In real app, get text from input component
                state.todos.push(Todo {
                    text: "New todo".into(),
                    completed: false,
                });
                Action::update(state)
            }
            TodoMsg::ToggleTodo(idx) => {
                if let Some(todo) = state.todos.get_mut(idx) {
                    todo.completed = !todo.completed;
                }
                Action::update(state)
            }
            TodoMsg::DeleteTodo(idx) => {
                state.todos.remove(idx);
                Action::update(state)
            }
            TodoMsg::ClearCompleted => {
                state.todos.retain(|t| !t.completed);
                Action::update(state)
            }
            TodoMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: TodoState) -> Node {
        let visible_todos: Vec<_> = state.todos.iter().enumerate()
            .filter(|(_, t)| match state.filter {
                Filter::All => true,
                Filter::Active => !t.completed,
                Filter::Completed => t.completed,
            })
            .collect();

        let active_count = state.todos.iter().filter(|t| !t.completed).count();

        node! {
            div(bg: black, pad: 2, @key_global(esc): ctx.handler(TodoMsg::Exit)) [
                // Header
                div(bg: blue, pad: 1, w_frac: 1.0) [
                    text("TODO LIST", color: white, bold)
                ],

                spacer(1),

                // Input area
                hstack(gap: 1) [
                    input(
                        placeholder: "What needs to be done?",
                        w: 40,
                        focusable
                    ),
                    div(border: green, pad: 1, focusable, @click: ctx.handler(TodoMsg::AddTodo)) [
                        text("Add")
                    ]
                ],

                spacer(1),

                // Todo list
                div(
                    h: 15,
                    overflow: scroll,
                    show_scrollbar: true,
                    border: white
                ) [
                    ...(visible_todos.iter().map(|(idx, todo)| {
                        let idx = *idx;
                        let style = if todo.completed {
                            TextStyle::default()
                                .color(Color::Gray)
                                .strikethrough(true)
                        } else {
                            TextStyle::default()
                        };

                        node! {
                            hstack(gap: 1) [
                                // Checkbox
                                div(
                                    w: 3,
                                    focusable,
                                    border: (if todo.completed { green } else { white }),
                                    @click: ctx.handler(TodoMsg::ToggleTodo(idx))
                                ) [
                                    text(if todo.completed { "✓" } else { " " })
                                ],

                                // Todo text
                                text(&todo.text, style: style),

                                // Delete button
                                div(border: red, pad_h: 1, focusable, @click: ctx.handler(TodoMsg::DeleteTodo(idx))) [
                                    text("×")
                                ]
                            ]
                        }
                    }).collect::<Vec<Node>>())
                ],

                spacer(1),

                // Footer
                hstack(gap: 2) [
                    text(format!("{} items left", active_count)),

                    div(border: yellow, pad: 1, focusable, @click: ctx.handler(TodoMsg::ClearCompleted)) [
                        text("Clear completed")
                    ]
                ],

                spacer(1),
                text("Press ESC to exit", color: gray)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(TodoApp)
}
```

## Next Steps

You've learned the fundamentals of RxTUI! Here's what to explore next:

1. **Read the full documentation**: Check `DOCS.md` for complete API details
2. **Study the examples**: Run and modify the example apps
3. **Build something**: Create your own TUI application
4. **Explore advanced features**:
   - Custom components
   - Complex layouts
   - Performance optimization
   - Custom rendering

## Tips for Success

1. **Start simple**: Build basic components first, then combine them
2. **Use the type system**: Let Rust's types guide your design
3. **Think in components**: Break complex UIs into reusable pieces
4. **Handle errors gracefully**: Always provide feedback to users
5. **Test interactively**: TUIs are best tested by using them

## Common Patterns

### Loading States

```rust
#[view]
fn view(&self, ctx: &Context, state: DataState) -> Node {
    node! {
        div [
            match &state.data {
                Loading => text("Loading...", color: yellow),
                Error(e) => text(format!("Error: {}", e), color: red),
                Success(data) => text(format!("Data: {}", data)),
            }
        ]
    }
}
```

### Modal Dialogs

```rust
#[view]
fn view(&self, ctx: &Context, state: AppState) -> Node {
    node! {
        div [
            // Main content
            div [ /* ... */ ],

            // Modal overlay
            if state.show_modal {
                div(
                    absolute,
                    top: 0, left: 0,
                    w_frac: 1.0, h_frac: 1.0,
                    bg: (Color::Black.with_alpha(128)),  // Semi-transparent
                    z: 1000
                ) [
                    // Modal content
                    div(
                        w: 40, h: 10,
                        bg: white,
                        border: black,
                        pad: 2
                    ) [
                        text("Modal Dialog", color: black, bold),
                        // Modal content...
                    ]
                ]
            }
        ]
    }
}
```

Happy coding with RxTUI!
