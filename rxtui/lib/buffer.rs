//! Double buffering and cell-level diffing for flicker-free rendering.
//!
//! This module implements a double-buffering system that maintains two complete
//! representations of the terminal screen. By comparing these buffers cell-by-cell,
//! we can generate minimal updates that eliminate flicker entirely.
//!
//! ## Architecture
//!
//! ```text
//!     Current Screen          Next Frame           Diff Result
//!     ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
//!     │ Hello World │      │ Hello Rust! │      │      ^^^^   │
//!     │ Terminal UI │      │ Terminal UI │      │ (no change) │
//!     └─────────────┘      └─────────────┘      └─────────────┘
//!        Front Buffer         Back Buffer          Cell Updates
//! ```

use crate::style::{Color, TextStyle};
use crate::utils::char_width;
use std::fmt;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a single cell in the terminal with its visual properties.
///
/// Each cell contains a character and its associated styling information.
/// This granular representation allows for precise tracking of what has changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// The character displayed in this cell
    pub char: char,

    /// Foreground color (text color)
    pub fg: Option<Color>,

    /// Background color
    pub bg: Option<Color>,

    /// Additional styling attributes
    pub style: CellStyle,
}

/// Style attributes that can be applied to a cell.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CellStyle {
    /// Bold text
    pub bold: bool,

    /// Italic text
    pub italic: bool,

    /// Underlined text
    pub underline: bool,

    /// Strikethrough text
    pub strikethrough: bool,
}

/// A buffer representing the entire terminal screen as a 2D grid of cells.
///
/// This buffer maintains a complete snapshot of what should be displayed
/// on the terminal, allowing for efficient diffing between frames.
pub struct ScreenBuffer {
    /// 2D grid of cells [row ⨉ column]
    cells: Vec<Vec<Cell>>,

    /// Width in columns
    width: u16,

    /// Height in rows
    height: u16,
}

/// Double buffer system for flicker-free rendering.
///
/// Maintains two buffers:
/// - `front`: What's currently displayed on the terminal
/// - `back`: What we're rendering for the next frame
///
/// After rendering to the back buffer and applying updates,
/// the buffers are swapped.
pub struct DoubleBuffer {
    /// The buffer representing what's currently on screen
    front: ScreenBuffer,

    /// The buffer we're rendering to for the next frame
    back: ScreenBuffer,
}

/// Represents an update to a single cell.
#[derive(Debug)]
pub enum CellUpdate {
    /// Update a single cell
    Single { x: u16, y: u16, cell: Cell },
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl CellStyle {
    /// Creates a CellStyle from a TextStyle, applying only the style attributes.
    pub fn from_text_style(text_style: &TextStyle) -> Self {
        Self {
            bold: text_style.bold.unwrap_or(false),
            italic: text_style.italic.unwrap_or(false),
            underline: text_style.underline.unwrap_or(false),
            strikethrough: text_style.strikethrough.unwrap_or(false),
        }
    }

    /// Merges this CellStyle with another, taking the other's values where they differ from defaults.
    pub fn merge_with(self, other: &CellStyle) -> Self {
        Self {
            bold: self.bold || other.bold,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            strikethrough: self.strikethrough || other.strikethrough,
        }
    }
}

impl Cell {
    /// Creates a new cell with default styling.
    pub fn new(char: char) -> Self {
        Self {
            char,
            fg: None,
            bg: None,
            style: CellStyle::default(),
        }
    }

    /// Creates an empty cell (space with no styling).
    pub fn empty() -> Self {
        Self::new(' ')
    }

    /// Sets the foreground color.
    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn with_bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets the style attributes.
    pub fn with_style(mut self, style: CellStyle) -> Self {
        self.style = style;
        self
    }
}

impl ScreenBuffer {
    /// Creates a new screen buffer with the given dimensions.
    ///
    /// All cells are initialized as empty (spaces with no styling).
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![vec![Cell::empty(); width as usize]; height as usize];
        Self {
            cells,
            width,
            height,
        }
    }

    /// Gets a reference to the cell at the given position.
    ///
    /// Returns None if the position is out of bounds.
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get(y as usize)?.get(x as usize)
    }

    /// Gets a mutable reference to the cell at the given position.
    ///
    /// Returns None if the position is out of bounds.
    pub fn get_cell_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get_mut(y as usize)?.get_mut(x as usize)
    }

    /// Sets the cell at the given position.
    ///
    /// Does nothing if the position is out of bounds.
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(target) = self.get_cell_mut(x, y) {
            *target = cell;
        }
    }

    /// Clears the buffer by setting all cells to empty.
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::empty();
            }
        }
    }

    /// Resizes the buffer to new dimensions.
    ///
    /// If the new size is larger, new cells are filled with empty cells.
    /// If the new size is smaller, cells are truncated.
    pub fn resize(&mut self, width: u16, height: u16) {
        let height_usize = height as usize;
        let width_usize = width as usize;

        // Resize height
        self.cells
            .resize(height_usize, vec![Cell::empty(); width_usize]);

        // Resize width of each row
        for row in &mut self.cells {
            row.resize(width_usize, Cell::empty());
        }

        self.width = width;
        self.height = height;
    }

    /// Gets the dimensions of the buffer.
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Fills a rectangular region with the given cell.
    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, cell: Cell) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_cell(x + dx, y + dy, cell.clone());
            }
        }
    }

    /// Writes a string starting at the given position.
    ///
    /// The string is written horizontally. If it extends beyond the buffer width,
    /// it is truncated. Properly handles wide characters (CJK, emoji) that take 2 columns.
    pub fn write_str(&mut self, x: u16, y: u16, text: &str, fg: Option<Color>, bg: Option<Color>) {
        let mut current_x = x;

        for ch in text.chars() {
            let ch_width = char_width(ch);

            // Check if character fits in remaining space
            if current_x + ch_width as u16 > self.width {
                break;
            }

            // Set the main cell
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            self.set_cell(current_x, y, cell);

            // For wide characters, fill the next cell with a space
            // This ensures proper rendering in terminals
            if ch_width == 2 && current_x + 1 < self.width {
                let mut space_cell = Cell::new(' ');
                space_cell.fg = fg;
                space_cell.bg = bg;
                self.set_cell(current_x + 1, y, space_cell);
            }

            current_x += ch_width as u16;
        }
    }

    /// Writes a string with full text styling starting at the given position.
    ///
    /// The string is written horizontally. If it extends beyond the buffer width,
    /// it is truncated. Properly handles wide characters (CJK, emoji) that take 2 columns.
    pub fn write_styled_str(&mut self, x: u16, y: u16, text: &str, text_style: Option<&TextStyle>) {
        let (fg, bg, cell_style) = if let Some(style) = text_style {
            (
                style.color,
                style.background,
                CellStyle::from_text_style(style),
            )
        } else {
            (None, None, CellStyle::default())
        };

        let mut current_x = x;

        for ch in text.chars() {
            let ch_width = char_width(ch);

            // Check if character fits in remaining space
            if current_x + ch_width as u16 > self.width {
                break;
            }

            // Set the main cell
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            cell.style = cell_style.clone();
            self.set_cell(current_x, y, cell);

            // For wide characters, fill the next cell with a space
            // This ensures proper rendering in terminals
            if ch_width == 2 && current_x + 1 < self.width {
                let mut space_cell = Cell::new(' ');
                space_cell.fg = fg;
                space_cell.bg = bg;
                space_cell.style = cell_style.clone();
                self.set_cell(current_x + 1, y, space_cell);
            }

            current_x += ch_width as u16;
        }
    }
}

impl DoubleBuffer {
    /// Creates a new double buffer with the given dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            front: ScreenBuffer::new(width, height),
            back: ScreenBuffer::new(width, height),
        }
    }

    /// Swaps the front and back buffers.
    ///
    /// After this operation:
    /// - The back buffer becomes the front buffer (what's on screen)
    /// - The front buffer becomes the back buffer (ready for next frame)
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Provides mutable access to the back buffer for rendering.
    pub fn back_buffer_mut(&mut self) -> &mut ScreenBuffer {
        &mut self.back
    }

    /// Clears both front and back buffers, keeping dimensions intact.
    pub fn reset(&mut self) {
        self.front.clear();
        self.back.clear();
    }

    /// Resizes both buffers to the new dimensions.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.front.resize(width, height);
        self.back.resize(width, height);
    }

    /// Compares the front and back buffers and returns a list of cell updates.
    ///
    /// This is the core of the flicker-free rendering system. By comparing
    /// buffers cell-by-cell, we can determine exactly what needs to be updated
    /// on the terminal.
    pub fn diff(&self) -> Vec<CellUpdate> {
        let mut updates = Vec::new();
        let (width, height) = self.front.dimensions();

        for y in 0..height {
            for x in 0..width {
                let front_cell = self.front.get_cell(x, y);
                let back_cell = self.back.get_cell(x, y);

                match (front_cell, back_cell) {
                    (Some(front), Some(back)) if front != back => {
                        updates.push(CellUpdate::Single {
                            x,
                            y,
                            cell: back.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        updates
    }

    /// Clears the back buffer.
    pub fn clear_back(&mut self) {
        self.back.clear();
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_buffer_diff_empty() {
        let db = DoubleBuffer::new(10, 5);
        let updates = db.diff();
        // Initially both buffers are empty, so no updates
        assert!(updates.is_empty());
    }

    #[test]
    fn test_double_buffer_diff_single_change() {
        let mut db = DoubleBuffer::new(10, 5);

        // First set both buffers to have the same content
        db.back_buffer_mut().set_cell(2, 1, Cell::new('A'));
        db.swap();

        // Copy front to back to start with identical buffers
        db.back_buffer_mut().set_cell(2, 1, Cell::new('A'));

        // Now modify just one cell in back buffer
        db.back_buffer_mut()
            .set_cell(2, 1, Cell::new('H').with_fg(Color::Red));

        // Should detect one update
        let updates = db.diff();
        assert_eq!(updates.len(), 1);

        // Swap buffers
        db.swap();

        // The key insight: after swap, if we render the exact same content
        // to the back buffer, there should be no updates!
        db.back_buffer_mut()
            .set_cell(2, 1, Cell::new('H').with_fg(Color::Red));

        let updates = db.diff();
        assert_eq!(updates.len(), 0); // No changes!
    }

    #[test]
    fn test_screen_buffer_write_str() {
        let mut buffer = ScreenBuffer::new(20, 5);
        buffer.write_str(2, 1, "Hello", Some(Color::Green), Some(Color::Black));

        assert_eq!(buffer.get_cell(2, 1).unwrap().char, 'H');
        assert_eq!(buffer.get_cell(3, 1).unwrap().char, 'e');
        assert_eq!(buffer.get_cell(6, 1).unwrap().char, 'o');
        assert_eq!(buffer.get_cell(2, 1).unwrap().fg, Some(Color::Green));
        assert_eq!(buffer.get_cell(2, 1).unwrap().bg, Some(Color::Black));
    }

    #[test]
    fn test_no_flicker_scenario() {
        let mut db = DoubleBuffer::new(20, 5);

        // Initial render: "Hello World" with blue background
        for i in 0..11 {
            db.back_buffer_mut()
                .set_cell(i, 0, Cell::new(' ').with_bg(Color::Blue));
        }
        db.back_buffer_mut()
            .write_str(0, 0, "Hello World", Some(Color::White), Some(Color::Blue));
        let updates1 = db.diff();
        assert_eq!(updates1.len(), 11); // 11 characters changed from empty
        db.swap();

        // Clear back buffer to simulate the app's behavior
        db.clear_back();

        // Write new content with same background
        for i in 0..12 {
            db.back_buffer_mut()
                .set_cell(i, 0, Cell::new(' ').with_bg(Color::Blue));
        }
        db.back_buffer_mut()
            .write_str(0, 0, "Hello Rust!", Some(Color::White), Some(Color::Blue));

        let updates2 = db.diff();

        // Even though we cleared and rewrote everything, the double buffer
        // system ensures only actual changes are sent to terminal
        // This eliminates flicker because terminal never sees the "cleared" state

        // Count actual changes:
        // - "Hello World" and "Hello Rust!" both have 11 characters
        // - But we set 12 cells with blue background (0..12)
        // - Position 11 is an extra blue background space
        let mut actual_changes = 0;
        for update in &updates2 {
            match update {
                CellUpdate::Single { .. } => actual_changes += 1,
            }
        }

        // Changes:
        // - Positions 6-10: "World" → "Rust!" (5 changes)
        // - Position 11: empty → blue background space (1 change)
        // Total: 6 changes
        assert!(actual_changes == 6);
    }
}
