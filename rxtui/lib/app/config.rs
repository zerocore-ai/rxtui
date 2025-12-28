//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Terminal rendering mode.
#[derive(Clone, Default)]
pub enum TerminalMode {
    /// Full-screen alternate buffer (default behavior).
    /// Content disappears when app exits.
    #[default]
    AlternateScreen,

    /// Inline rendering in main terminal buffer.
    /// Content persists in terminal history after app exits.
    Inline(InlineConfig),
}

/// Configuration for inline rendering mode.
#[derive(Clone)]
pub struct InlineConfig {
    /// How to determine rendering height.
    pub height: InlineHeight,

    /// Whether to show cursor during rendering.
    pub cursor_visible: bool,

    /// Whether to preserve output after app exits.
    pub preserve_on_exit: bool,

    /// Whether to capture mouse events.
    ///
    /// Default is `false` to allow natural terminal scrolling.
    /// Set to `true` if you need mouse interaction (clicks, hover)
    /// within the inline UI, but note this will prevent terminal
    /// scrollbar and scroll gestures from working.
    pub mouse_capture: bool,
}

/// Height determination strategy for inline mode.
#[derive(Clone)]
pub enum InlineHeight {
    /// Fixed number of lines.
    Fixed(u16),

    /// Grow to fit content, with optional maximum.
    Content { max: Option<u16> },

    /// Fill remaining terminal space below cursor.
    Fill { min: u16 },
}

/// Configuration options for debugging and optimization control.
#[derive(Clone)]
pub struct RenderConfig {
    /// Enable double buffering for flicker-free rendering (default: true)
    pub double_buffering: bool,

    /// Enable terminal-specific optimizations (default: true)
    pub terminal_optimizations: bool,

    /// Enable cell-level diffing (default: true)
    pub cell_diffing: bool,

    /// Event polling duration in milliseconds (default: 100ms)
    /// Lower values make the app more responsive but use more CPU
    pub poll_duration_ms: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RenderConfig {
    /// Creates a debug configuration with all optimizations disabled.
    pub fn debug() -> Self {
        Self {
            double_buffering: false,
            terminal_optimizations: false,
            cell_diffing: false,
            poll_duration_ms: 50,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for InlineConfig {
    fn default() -> Self {
        Self {
            // Default to content-based height with no max limit.
            // Users can set a max if they want to constrain height.
            height: InlineHeight::Content { max: None },
            cursor_visible: false,
            preserve_on_exit: true,
            mouse_capture: false,
        }
    }
}

impl Default for InlineHeight {
    fn default() -> Self {
        Self::Content { max: None }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            double_buffering: true,
            terminal_optimizations: true,
            cell_diffing: true,
            poll_duration_ms: 50,
        }
    }
}
