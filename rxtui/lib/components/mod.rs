//! Reusable UI components for rxtui
//!
//! This module provides pre-built components that can be easily composed
//! to create more complex user interfaces.

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

/// Animated shimmer text effect
pub mod shimmer_text;

/// Text input component for user text entry
pub mod text_input;

/// Spinner component for loading animations
pub mod spinner;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use shimmer_text::{ShimmerSpeed, ShimmerText};
pub use spinner::{Spinner, SpinnerMsg, SpinnerSpeed, SpinnerType};
pub use text_input::TextInput;
