<div align="center">
  <a href="./#gh-dark-mode-only" target="_blank">
    <img width="500" alt="rxtui-dark" src="https://github.com/user-attachments/assets/3e3235bc-3792-44eb-88d5-e847631c0086" />
  </a>
  <a href="./#gh-light-mode-only" target="_blank">
    <img width="500" alt="rxtui-light" src="https://github.com/user-attachments/assets/3d1e00f4-39ac-4053-b45b-c4bab7de1361" />
  </a>

  <br />

<b>———&nbsp;&nbsp;&nbsp;reactive terminal UI framework for rust&nbsp;&nbsp;&nbsp;———</b>

</div>

<br />

<div align='center'>
  <a href="https://crates.io/crates/rxtui">
    <img src="https://img.shields.io/crates/v/rxtui?style=for-the-badge&logo=rust&logoColor=white" alt="crates.io version"/>
  </a>
  <a href="https://docs.rs/rxtui">
    <img src="https://img.shields.io/badge/docs.rs-rxtui-blue?style=for-the-badge&logo=docs.rs" alt="docs.rs"/>
  </a>
  <a href="https://github.com/microsandbox/rxtui/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue?style=for-the-badge" alt="license"/>
  </a>
</div>

<br />

## Why RxTUI?

Terminal UIs have traditionally been painful to build. You either work with low-level escape sequences or use immediate-mode libraries that require manual state management. **RxTUI** brings the retained-mode, component-based architecture that revolutionized web development to the terminal.

- **Declarative UI** - Describe what your UI should look like, not how to change it
- **Component Architecture** - Build complex apps from simple, reusable components
- **Message-Based State** - Elm-inspired architecture for predictable state updates
- **Efficient Rendering** - Virtual DOM with intelligent diffing minimizes redraws
- **Rich Styling** - Colors, borders, flexbox-style layout, text wrapping, and more
- **Built-in Components** - TextInput, forms, and other common UI elements
- **Async Effects** - First-class support for timers, API calls, and background tasks

## Quick Start

```rust
use rxtui::prelude::*;

#[derive(Component)]
struct Counter;

impl Counter {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str, mut count: i32) -> Action {
        match msg {
            "inc" => Action::update(count + 1),
            "dec" => Action::update(count - 1),
            _ => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, count: i32) -> Node {
        node! {
            div(
                pad: 2,
                align: center,
                w_frac: 1.0,
                gap: 1,
                @key(up): ctx.handler("inc"),
                @key(down): ctx.handler("dec"),
                @key(esc): ctx.handler("exit")
            ) [
                text(format!("Count: {count}"), color: white, bold),
                text("use ↑/↓ to change, esc to exit", color: bright_black)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Counter)
}
```

## Features

### The `node!` Macro

Build UIs declaratively with a JSX-like syntax:

```rust
node! {
    div(bg: black, pad: 2, border_color: white) [
        text("Hello, Terminal!", color: yellow, bold),

        div(dir: horizontal, gap: 2) [
            div(bg: blue, w: 20, h: 5) [
                text("Left Panel", color: white)
            ],
            div(bg: green, w: 20, h: 5) [
                text("Right Panel", color: white)
            ]
        ]
    ]
}
```

### Component System

Components manage their own state and handle messages:

```rust
#[derive(Component)]
struct TodoList;

impl TodoList {
    #[update]
    fn update(&self, ctx: &Context, msg: TodoMsg, mut state: TodoState) -> Action {
        match msg {
            TodoMsg::Add(item) => {
                state.items.push(item);
                Action::update(state)
            }
            TodoMsg::Remove(idx) => {
                state.items.remove(idx);
                Action::update(state)
            }
            _ => Action::none()
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: TodoState) -> Node {
        // Build your UI using the current state
    }
}
```

### Layout System

Flexbox-inspired layout with:
- Direction control (horizontal/vertical)
- Justify content (start, center, end, space-between, etc.)
- Align items (start, center, end)
- Wrapping support
- Percentage and fixed sizing
- Auto-sizing based on content

### Rich Text Support

Create styled text with multiple segments:

```rust
node! {
    richtext [
        text("Status: ", color: white),
        text("Connected", color: green, bold),
        text(" | ", color: bright_black),
        text("CPU: ", color: white),
        text("42%", color: yellow)
    ]
}
```

### Async Effects

Handle background tasks with the effects system:

```rust
use std::time::Duration;

#[derive(Component)]
struct Timer;

impl Timer {
    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send("tick");
        }
    }
}
```

## Examples

The [examples directory](https://github.com/microsandbox/rxtui/tree/main/examples) contains comprehensive demonstrations:

- **counter** - Minimal interactive counter
- **form** - Text input and form handling
- **stopwatch** - Timer with effects
- **align** - Flexbox-style alignment
- **components** - Component composition
- **demo** - Full feature showcase

Run examples with:
```bash
cargo run --example counter
```

## Documentation

- [API Documentation](https://docs.rs/rxtui) - Complete API reference
- [GitHub Repository](https://github.com/microsandbox/rxtui) - Source code and issues
- [Examples](https://github.com/microsandbox/rxtui/tree/main/examples) - Learn by example

## Requirements

- Rust 1.70 or later
- Terminal with UTF-8 support
- Unix-like system (Linux, macOS) or Windows 10+

## Optional Features

- `effects` (default) - Async effects support with tokio

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](https://github.com/microsandbox/rxtui/blob/main/LICENSE) file for details.
