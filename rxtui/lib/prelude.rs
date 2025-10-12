//! Prelude module for convenient imports.
//!
//! This module re-exports commonly used types and traits for easier usage.
//!
//! # Example
//!
//! ```rust
//! use rxtui::prelude::*;
//! ```

// Core app types
pub use crate::app::{App, Context};

// Component system
pub use crate::component::{Action, Message, MessageExt, State};

// Effects system
pub use crate::effect::Effect;

// Re-export both the trait and the derive macro
pub use crate::Component;
pub use crate::ComponentMacro as Component;

// Re-export attribute macros
#[cfg(feature = "effects")]
pub use crate::effect;
pub use crate::{component, update, view};

// UI elements
pub use crate::node::{Div, Node, RichText, Text};

// Components
#[cfg(feature = "components")]
pub use crate::components::{ShimmerSpeed, ShimmerText, TextInput};

// Style types
pub use crate::style::*;

// Key handling
pub use crate::key::{Key, KeyWithModifiers};

// Layout types
pub use crate::bounds::Rect;

// Main macro for building TUI components
pub use crate::node;
