pub mod config;
pub mod context;
pub mod core;
pub mod events;
pub(crate) mod inline;
pub mod renderer;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use config::{InlineConfig, InlineHeight, TerminalMode};
pub use context::Context;
pub use core::App;
