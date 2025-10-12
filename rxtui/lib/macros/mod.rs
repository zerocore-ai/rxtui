//! Macro-based DSL for building TUI components
//!
//! This module provides the `node!` macro for composing rxtui components
//! with an ergonomic, declarative syntax.
//!
//! # Overview
//!
//! The `node!` macro reduces boilerplate by 50-70% compared to the builder pattern
//! while maintaining type safety and readability.
//!
//! # Syntax
//!
//! The macro uses a clean syntax:
//! - Containers: `container(props) [children]`
//! - Text: `text("content", props)`
//! - Components use parentheses `()` for properties
//! - Containers use brackets `[]` for children
//! - Event handlers use the `@` prefix
//!
//! # Quick Example
//!
//! ```ignore
//! use rxtui::prelude::*;
//!
//! fn view(&self, ctx: &Context) -> Node {
//!     node! {
//!         container(bg: black, pad: 2) [
//!             text("Hello World", color: white, bold),
//!             spacer(1),
//!
//!             container(bg: blue, w: 50) [
//!                 text("Click me!", color: white),
//!                 @click: ctx.handler(Msg::Clicked),
//!             ]
//!         ]
//!     }
//! }
//! ```
//!
//! # Color Support
//!
//! Colors can be specified in multiple ways:
//! - **Named**: `red`, `blue`, `green`, `white`, `black`, etc.
//! - **Bright variants**: `bright_red`, `bright_blue`, etc.
//! - **Hex**: `"#FF5733"`, `"#FFF"`
//! - **Conditional**: `(if dark { white } else { black })`
//!
//! # Property Shortcuts
//!
//! Common properties have short aliases:
//! - `bg` → background color
//! - `dir` → direction (vertical/v, horizontal/h)
//! - `pad` → padding
//! - `w` → width
//! - `h` → height
//! - `w_frac` → width fraction (0.0–1.0)
//! - `h_frac` → height fraction (0.0–1.0)
//!
//! # Event Handlers
//!
//! Events use the `@` prefix:
//! - `@click: handler` - Mouse click
//! - `@char('q'): handler` - Character key
//! - `@key(enter): handler` - Special key
//! - `@char_global('q'): handler` - Global character
//! - `@key_global(esc): handler` - Global key
//! - `@focus: handler` - Focus gained
//! - `@blur: handler` - Focus lost

mod internal;
mod node;
