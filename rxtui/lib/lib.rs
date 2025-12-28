//! # RxTUI - Reactive Terminal User Interface Framework
//!
//! A modern terminal UI framework for Rust that brings React-style component architecture
//! and declarative UI building to the terminal. Build interactive, stateful terminal
//! applications with ease using familiar patterns.
//!
//! ## Features
//!
//! - **Component-based architecture** - Build reusable, composable UI components
//! - **Declarative UI with `node!` macro** - Express your UI structure clearly
//! - **Virtual DOM with diffing** - Efficient, minimal terminal updates
//! - **Message-based state management** - Predictable state updates like Elm
//! - **Async effects** - Handle background tasks, timers, and I/O operations
//! - **Rich styling** - Colors, borders, text styles, and layout control
//! - **Built-in components** - TextInput, forms, and more
//!
//! ## Architecture Overview
//!
//! ```text
//!     ┌────────────┐     ┌─────────────┐     ┌─────────────┐
//!     │  Component │────▶│  node!      │────▶│    Node     │
//!     │   (trait)  │     │   macro     │     │    Tree     │
//!     └────────────┘     └─────────────┘     └─────────────┘
//!            │                  │                   │
//!            │                  ▼                   ▼
//!     ┌─────────────┐    ┌─────────────┐     ┌─────────────┐
//!     │   Update    │    │    View     │────▶│    VDom     │
//!     │  (messages) │    │  (render)   │     │   (state)   │
//!     └─────────────┘    └─────────────┘     └─────────────┘
//!            │                                     │
//!            ▼                                     ▼
//!     ┌─────────────┐                       ┌─────────────┐
//!     │   Action    │                       │    Diff     │
//!     │  (state)    │                       │   Engine    │
//!     └─────────────┘                       └─────────────┘
//!                                                  │
//!                                                  ▼
//!                                           ┌─────────────┐
//!                                           │  Terminal   │
//!                                           │   Render    │
//!                                           └─────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rxtui::prelude::*;
//!
//! #[derive(Component)]
//! struct Counter;
//!
//! impl Counter {
//!     #[update]
//!     fn update(&self, _ctx: &Context, msg: &str, mut count: i32) -> Action {
//!         match msg {
//!             "inc" => Action::update(count + 1),
//!             "dec" => Action::update(count - 1),
//!             _ => Action::exit(),
//!         }
//!     }
//!
//!     #[view]
//!     fn view(&self, ctx: &Context, count: i32) -> Node {
//!         node! {
//!             div(
//!                 pad: 2,
//!                 align: center,
//!                 w_frac: 1.0,
//!                 gap: 1,
//!                 @key(up): ctx.handler("inc"),
//!                 @key(down): ctx.handler("dec"),
//!                 @key(esc): ctx.handler("exit")
//!             ) [
//!                 text(format!("Count: {count}"), color: white, bold),
//!                 text("use ↑/↓ to change, esc to exit", color: bright_black)
//!             ]
//!         }
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     App::new()?.run(Counter)
//! }
//! ```
//!
//! ## Key Concepts
//!
//! - **Component**: Main trait for building UI components with state management
//! - **node! macro**: Declarative macro for building UI trees with JSX-like syntax
//! - **Node**: Virtual representation of UI elements (divs, text, components)
//! - **Context**: Provides access to message handlers and component communication
//! - **Action**: Return type from update methods (update state, exit, etc.)
//! - **App**: Main application runner that manages the event loop
//! - **VDom**: Virtual DOM that tracks UI state and calculates diffs
//!
//! ## Examples
//!
//! ### Basic Hello World
//!
//! ```rust,no_run
//! use rxtui::prelude::*;
//!
//! #[derive(Component)]
//! struct HelloWorld;
//!
//! impl HelloWorld {
//!     #[view]
//!     fn view(&self, ctx: &Context) -> Node {
//!         node! {
//!             div(bg: blue, pad: 2, @key_global(esc): ctx.handler(())) [
//!                 text("Hello, Terminal!", color: white, bold),
//!                 text("Press Esc to exit", color: white)
//!             ]
//!         }
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     App::new()?.run(HelloWorld)
//! }
//! ```
//!
//! ### Using the node! Macro
//!
//! The `node!` macro provides a declarative way to build UI trees:
//!
//! ```rust
//! use rxtui::prelude::*;
//!
//! fn build_ui(ctx: &Context) -> Node {
//!     node! {
//!         div(
//!             bg: black,              // Background color
//!             border_color: white,    // Border color
//!             border_style: rounded,  // Border style
//!             pad: 2,                 // Padding
//!             gap: 1,                 // Gap between children
//!             dir: vertical           // Layout direction
//!         ) [
//!             text("Title", color: yellow, bold),
//!
//!             div(dir: horizontal, gap: 2) [
//!                 div(bg: blue, w: 20, h: 5) [
//!                     text("Left Panel", color: white)
//!                 ],
//!                 div(bg: green, w: 20, h: 5) [
//!                     text("Right Panel", color: white)
//!                 ]
//!             ],
//!
//!             text("Status: Ready", color: bright_black)
//!         ]
//!     }
//! }
//! ```
//!
//! ### State Management
//!
//! Components manage state through messages and updates:
//!
//! ```rust
//! use rxtui::prelude::*;
//!
//! #[derive(Clone, Default)]
//! struct TodoState {
//!     items: Vec<String>,
//!     input: String,
//! }
//!
//! #[derive(Component)]
//! struct TodoApp;
//!
//! impl TodoApp {
//!     #[update]
//!     fn update(&self, _ctx: &Context, msg: &str, mut state: TodoState) -> Action {
//!         match msg {
//!             "add" => {
//!                 if !state.input.is_empty() {
//!                     state.items.push(state.input.clone());
//!                     state.input.clear();
//!                 }
//!                 Action::update(state)
//!             }
//!             "clear" => {
//!                 state.items.clear();
//!                 Action::update(state)
//!             }
//!             _ => Action::exit()
//!         }
//!     }
//!
//!     #[view]
//!     fn view(&self, ctx: &Context, state: TodoState) -> Node {
//!         node! {
//!             div(pad: 2, gap: 1) [
//!                 text("Todo List", color: yellow, bold),
//!                 div(gap: 1) [
//!                     ...(state.items.iter().map(|item| {
//!                         node! { text(format!("• {}", item), color: white) }
//!                     }).collect::<Vec<_>>())
//!                 ],
//!                 text(format!("{} items", state.items.len()), color: bright_black)
//!             ]
//!         }
//!     }
//! }
//! ```
//!
//! ### Async Effects
//!
//! Handle background tasks with effects (requires `effects` feature):
//!
//! ```rust
//! use rxtui::prelude::*;
//!
//! #[derive(Component)]
//! struct Timer;
//!
//! impl Timer {
//!     #[update]
//!     fn update(&self, _ctx: &Context, tick: bool, seconds: u32) -> Action {
//!         if !tick {
//!             return Action::exit();
//!         }
//!         Action::update(seconds + 1)
//!     }
//!
//!     #[view]
//!     fn view(&self, ctx: &Context, seconds: u32) -> Node {
//!         node! {
//!             div(pad: 2, align: center, @key(esc): ctx.handler(false)) [
//!                 text(format!("Timer: {}s", seconds), color: white, bold),
//!                 text("Press Esc to stop", color: bright_black)
//!             ]
//!         }
//!     }
//!
//!     #[cfg(feature = "effects")]
//!     #[effect]
//!     async fn tick(&self, ctx: &Context) {
//!         loop {
//!             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//!             ctx.send(true);
//!         }
//!     }
//! }
//! ```
//!
//! ### Rich Text Formatting
//!
//! Create styled text with multiple segments:
//!
//! ```rust
//! use rxtui::prelude::*;
//!
//! fn status_line() -> Node {
//!     node! {
//!         richtext [
//!             text("Status: ", color: white),
//!             text("Connected", color: green, bold),
//!             text(" | ", color: bright_black),
//!             text("CPU: ", color: white),
//!             text("42%", color: yellow),
//!             text(" | ", color: bright_black),
//!             text("Mem: ", color: white),
//!             text("2.1GB", color: cyan)
//!         ]
//!     }
//! }
//! ```
//!
//! ### Input Handling
//!
//! Use the built-in TextInput component:
//!
//! ```rust
//! use rxtui::prelude::*;
//!
//! fn input_form(ctx: &Context) -> Node {
//!     node! {
//!         div(pad: 2, gap: 1) [
//!             text("Enter your name:", color: white),
//!             input(
//!                 placeholder: "Type here...",
//!                 border_color: cyan,
//!                 w: 30,
//!                 focusable,
//!                 @submit: ctx.handler("submit")
//!             )
//!         ]
//!     }
//! }
//! ```

//--------------------------------------------------------------------------------------------------
// Modules: Core Components
//--------------------------------------------------------------------------------------------------

/// Prelude module for convenient imports
pub mod prelude;

/// New component-based system (parallel implementation)
pub mod component;

/// Node types for component tree (includes div, text, rich_text)
pub mod node;

/// Virtual node types for the VDOM
mod vnode;

//--------------------------------------------------------------------------------------------------
// Modules: Rendering
//--------------------------------------------------------------------------------------------------

/// Virtual DOM implementation for managing the UI state.
/// Maintains the current UI tree and applies patches from the diff engine.
mod vdom;

/// Diffing algorithm for efficiently updating the UI.
/// Compares old and new virtual DOM trees to generate minimal change patches.
mod diff;

/// Rendering engine that converts virtual nodes into terminal output.
/// Handles the actual drawing of elements to the screen.
mod render_tree;

/// Double buffering and cell-level diffing for flicker-free rendering.
/// Maintains screen state to enable precise, minimal updates.
mod buffer;

/// Optimized terminal renderer for applying cell updates.
/// Minimizes escape sequences and I/O operations for best performance.
mod terminal;

//--------------------------------------------------------------------------------------------------
// Modules: Application
//--------------------------------------------------------------------------------------------------

/// Application framework for building terminal UIs.
/// Provides the main application lifecycle and event handling.
pub mod app;

//--------------------------------------------------------------------------------------------------
// Modules: Styling & Layout
//--------------------------------------------------------------------------------------------------

/// Styling system for terminal UI components.
/// Defines colors, spacing, borders, and other visual properties.
pub mod style;

/// Bounds and rectangle operations for dirty region tracking.
/// Provides types for tracking screen regions that need redrawing.
pub mod bounds;

//--------------------------------------------------------------------------------------------------
// Modules: Input & Utilities
//--------------------------------------------------------------------------------------------------

/// Key representation for keyboard input.
/// Provides an enum for representing both characters and special keys.
pub mod key;

/// Utilities for terminal rendering, Unicode width calculations, and text wrapping.
/// Provides helpers for display width, text manipulation, and wrapping algorithms.
mod utils;

/// Provider traits for Component macro system (internal use)
/// Enables safe defaults via method shadowing for update/view/effects
#[doc(hidden)]
pub mod providers;

//--------------------------------------------------------------------------------------------------
// Modules: Macros
//--------------------------------------------------------------------------------------------------

/// Macro-based DSL for building TUI components
/// Provides ergonomic macros for composing components with less boilerplate
pub mod macros;

//--------------------------------------------------------------------------------------------------
// Modules: Components
//--------------------------------------------------------------------------------------------------

/// Reusable UI components for building forms and interfaces
/// Provides pre-built components like TextInput, Button, etc.
#[cfg(feature = "components")]
pub mod components;

/// Stub components module when the `components` feature is disabled.
#[cfg(not(feature = "components"))]
pub mod components {}

/// Async effects system for running background tasks
#[cfg(feature = "effects")]
pub mod effect;

/// Stub effect module when effects feature is disabled
#[cfg(not(feature = "effects"))]
pub mod effect {
    /// Stub Effect type when effects feature is disabled
    #[derive(Debug, Clone)]
    pub struct Effect;

    impl Effect {
        /// Create an empty effect vector
        pub fn none() -> Vec<Self> {
            vec![]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

// Re-export the derive macro with the same name
#[doc(hidden)]
pub use rxtui_macros::Component as ComponentMacro;
pub use rxtui_macros::{component, update, view};

// Conditionally export the effect macro only when the feature is enabled
#[cfg(feature = "effects")]
pub use rxtui_macros::effect;

pub use app::{App, Context, InlineConfig, InlineHeight, TerminalMode};
pub use bounds::Rect;
pub use component::{Action, Component, Message, MessageExt, State};
#[cfg(feature = "components")]
pub use components::{ShimmerSpeed, ShimmerText, TextInput};
pub use key::{Key, KeyWithModifiers};
pub use node::{Div, Node, RichText, Text, TextSpan};
pub use style::{
    BorderEdges, BorderStyle, Color, Dimension, Direction, Overflow, Position, Spacing, Style,
    TextStyle, TextWrap, WrapMode,
};

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    mod rich_text_tests;
}
