use crossterm::{ExecutableCommand, cursor, style::Print, terminal};
use std::io::{self, Write};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Runtime state for inline rendering.
///
/// Tracks the position and dimensions of the inline rendering area
/// within the terminal's main buffer.
pub(crate) struct InlineState {
    /// Row where our rendering area starts (after space reservation).
    pub origin_row: u16,
    /// Column where rendering starts (usually 0).
    pub origin_col: u16,
    /// Current reserved height.
    pub reserved_height: u16,
    /// Terminal dimensions at initialization.
    pub terminal_size: (u16, u16),
    /// Whether space has been reserved.
    pub initialized: bool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl InlineState {
    /// Creates a new uninitialized inline state.
    pub fn new() -> Self {
        Self {
            origin_row: 0,
            origin_col: 0,
            reserved_height: 0,
            terminal_size: (80, 24),
            initialized: false,
        }
    }

    /// Reserve space for inline rendering.
    ///
    /// This must be called before the first render. It:
    /// 1. Queries current cursor position
    /// 2. Prints newlines to reserve space (causing scroll if needed)
    /// 3. Moves cursor back up to establish a stable origin
    /// 4. Clears the reserved area to prevent artifacts from existing terminal content
    ///
    /// After this call, `origin_row` and `origin_col` define a stable
    /// coordinate system for rendering.
    pub fn reserve_space(&mut self, stdout: &mut impl Write, height: u16) -> io::Result<()> {
        // Get terminal dimensions
        let (term_width, term_height) = terminal::size()?;
        self.terminal_size = (term_width, term_height);

        // Query current cursor position (used for debugging, but we rely on new_row/new_col below)
        let (_cursor_col, _cursor_row) = cursor::position()?;

        // Clamp height to terminal height if needed
        let height = height.min(term_height);

        // Print newlines to reserve space (causes scroll if needed)
        for _ in 0..height {
            stdout.execute(Print("\n"))?;
        }

        // Move cursor back up to origin
        if height > 0 {
            stdout.execute(cursor::MoveUp(height))?;
        }

        // Query new position - this is our stable origin
        let (new_col, new_row) = cursor::position()?;
        self.origin_row = new_row;
        self.origin_col = new_col;
        self.reserved_height = height;
        self.initialized = true;

        // Clear the reserved area to remove any existing terminal content.
        // This ensures our front buffer (all empty cells) matches the actual terminal state.
        // Without this, artifacts can appear where existing content wasn't overwritten.
        self.clear_area(stdout, height)?;

        Ok(())
    }

    /// Clear the reserved inline area.
    ///
    /// Clears each line in the reserved area to ensure no existing terminal
    /// content interferes with rendering.
    fn clear_area(&self, stdout: &mut impl Write, height: u16) -> io::Result<()> {
        for row in 0..height {
            stdout.execute(cursor::MoveTo(0, self.origin_row + row))?;
            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        }
        // Move cursor back to origin after clearing
        stdout.execute(cursor::MoveTo(self.origin_col, self.origin_row))?;
        Ok(())
    }

    /// Expand reserved space if content grew.
    ///
    /// If the new height is greater than current reserved height,
    /// adds more newlines, clears the new area, and adjusts origin if scrolling occurred.
    pub fn expand_space(&mut self, stdout: &mut impl Write, new_height: u16) -> io::Result<()> {
        if new_height <= self.reserved_height {
            return Ok(());
        }

        let additional = new_height - self.reserved_height;
        let old_height = self.reserved_height;

        // Move to end of current reserved area
        stdout.execute(cursor::MoveTo(0, self.origin_row + self.reserved_height))?;

        // Print additional newlines
        for _ in 0..additional {
            stdout.execute(Print("\n"))?;
        }

        // Check if we scrolled - origin shifts up if bottom exceeds terminal
        let (_, term_height) = self.terminal_size;
        let bottom_row = self.origin_row + new_height;
        if bottom_row > term_height {
            let scroll_amount = bottom_row - term_height;
            self.origin_row = self.origin_row.saturating_sub(scroll_amount);
        }

        self.reserved_height = new_height;

        // Clear only the newly added lines
        for row in old_height..new_height {
            stdout.execute(cursor::MoveTo(0, self.origin_row + row))?;
            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        }

        Ok(())
    }

    /// Move cursor to origin for rendering.
    #[allow(dead_code)]
    pub fn move_to_origin(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(self.origin_col, self.origin_row))?;
        Ok(())
    }

    /// Move cursor to end of rendered content (for exit).
    ///
    /// Positions cursor below the rendered area and prints a newline
    /// to ensure the shell prompt appears on a fresh line.
    pub fn move_to_end(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(0, self.origin_row + self.reserved_height))?;
        stdout.execute(Print("\n"))?;
        Ok(())
    }

    /// Translate a terminal row to a row relative to our origin.
    ///
    /// Used for translating mouse event coordinates.
    #[allow(dead_code)]
    pub fn translate_row(&self, terminal_row: u16) -> Option<u16> {
        if terminal_row >= self.origin_row && terminal_row < self.origin_row + self.reserved_height
        {
            Some(terminal_row - self.origin_row)
        } else {
            None
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for InlineState {
    fn default() -> Self {
        Self::new()
    }
}
