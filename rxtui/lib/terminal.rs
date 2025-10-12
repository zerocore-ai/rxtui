//! Optimized terminal renderer that applies cell updates efficiently.
//!
//! This module is responsible for translating cell updates into terminal
//! commands, minimizing the number of escape sequences and I/O operations
//! to achieve optimal performance and eliminate flicker.

use crate::buffer::{Cell, CellStyle, CellUpdate};
use crate::style::Color;
use crossterm::{
    ExecutableCommand, cursor,
    style::{Attribute, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::{self, Write};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Optimized terminal renderer that tracks terminal state to minimize commands.
///
/// By tracking the current cursor position, colors, and attributes, we can
/// avoid sending redundant commands to the terminal, significantly improving
/// performance.
///
/// ```text
/// Terminal State Tracking:
/// ┌─────────────────────────────────────┐
/// │ TerminalRenderer                    │
/// │                                     │
/// │  current_pos: (10, 5)               │ ◄── Tracks cursor to avoid
/// │  current_fg: Some(Red)              │     redundant MoveTo commands
/// │  current_bg: Some(Blue)             │
/// │  current_style: Bold                │ ◄── Only sends style changes
/// │                                     │     when actually different
/// └─────────────────────────────────────┘
/// ```
pub struct TerminalRenderer {
    /// Output stream (usually stdout)
    stdout: io::Stdout,

    /// Current cursor position (x, y)
    current_pos: Option<(u16, u16)>,

    /// Current foreground color
    current_fg: Option<Color>,

    /// Current background color
    current_bg: Option<Color>,

    /// Current style attributes
    current_style: CellStyle,

    /// Whether synchronized output is supported
    supports_synchronized: bool,
}

/// A terminal command abstraction for batching operations.
#[derive(Debug)]
enum TerminalCommand {
    /// Move cursor to position
    MoveTo(u16, u16),

    /// Set foreground and background colors
    SetColors {
        fg: Option<Color>,
        bg: Option<Color>,
    },

    /// Print text at current position
    Print(String),

    /// Set style attributes
    SetStyle(CellStyle),

    /// Reset all styling
    Reset,
}

/// Batches cell updates into optimized terminal commands.
struct UpdateBatcher {
    /// The updates to process
    updates: Vec<CellUpdate>,
}

/// A run of consecutive cells with the same style.
struct Run {
    x: u16,
    y: u16,
    cells: Vec<Cell>,
    fg: Option<Color>,
    bg: Option<Color>,
    style: CellStyle,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl TerminalRenderer {
    /// Creates a new terminal renderer.
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            current_pos: None,
            current_fg: None,
            current_bg: None,
            current_style: CellStyle::default(),
            supports_synchronized: Self::detect_synchronized_output(),
        }
    }

    /// Detects if the terminal supports synchronized output mode.
    fn detect_synchronized_output() -> bool {
        // For now, we'll enable it for known terminals
        // In the future, we could do actual capability detection
        std::env::var("TERM_PROGRAM").is_ok_and(|term| {
            matches!(
                term.as_str(),
                "iTerm.app" | "kitty" | "alacritty" | "wezterm"
            )
        })
    }

    /// Applies a list of cell updates to the terminal.
    ///
    /// ```text
    /// Update Flow:
    /// ┌─────────────┐     ┌──────────────┐     ┌─────────────┐
    /// │ CellUpdates │ ──▶ │ UpdateBatcher│ ──▶ │  Terminal   │
    /// │ [(x,y,cell)]│     │  optimize()  │     │  Commands   │
    /// └─────────────┘     └──────────────┘     └─────────────┘
    ///                             │                     │
    ///                             ▼                     ▼
    ///                     ┌──────────────┐     ┌─────────────┐
    ///                     │ Sorted &     │     │ MoveTo(x,y) │
    ///                     │ Grouped Runs │     │ SetColors   │
    ///                     └──────────────┘     │ Print(text) │
    ///                                          └─────────────┘
    /// ```
    pub fn apply_updates(&mut self, updates: Vec<CellUpdate>) -> io::Result<()> {
        if updates.is_empty() {
            return Ok(());
        }

        // Use synchronized output if available
        if self.supports_synchronized {
            self.apply_updates_synchronized(updates)
        } else {
            self.apply_updates_direct(updates)
        }
    }

    /// Applies updates without terminal optimizations (for debugging).
    pub fn apply_updates_direct(&mut self, updates: Vec<CellUpdate>) -> io::Result<()> {
        // Process updates without optimization
        for update in updates {
            match update {
                CellUpdate::Single { x, y, cell } => {
                    self.stdout.execute(cursor::MoveTo(x, y))?;
                    self.apply_cell_style(&cell)?;
                    self.stdout.execute(Print(cell.char))?;
                }
            }
        }

        self.stdout.execute(ResetColor)?;
        self.stdout.execute(SetAttribute(Attribute::Reset))?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Clears the terminal display and resets renderer state tracking.
    pub fn clear_screen(&mut self) -> io::Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        self.stdout.execute(ResetColor)?;
        self.stdout.execute(SetAttribute(Attribute::Reset))?;
        self.stdout.flush()?;

        self.current_pos = None;
        self.current_fg = None;
        self.current_bg = None;
        self.current_style = CellStyle::default();

        Ok(())
    }

    /// Draws the entire buffer to terminal without optimization.
    pub fn draw_full_buffer(&mut self, buffer: &crate::buffer::ScreenBuffer) -> io::Result<()> {
        let (width, height) = buffer.dimensions();

        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buffer.get_cell(x, y) {
                    self.stdout.execute(cursor::MoveTo(x, y))?;
                    self.apply_cell_style(cell)?;
                    self.stdout.execute(Print(cell.char))?;
                }
            }
        }

        self.stdout.execute(ResetColor)?;
        self.stdout.execute(SetAttribute(Attribute::Reset))?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Converts our Color enum to crossterm color.
    pub fn color_to_crossterm(&self, color: Color) -> crossterm::style::Color {
        match color {
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::DarkRed,
            Color::Green => crossterm::style::Color::DarkGreen,
            Color::Yellow => crossterm::style::Color::DarkYellow,
            Color::Blue => crossterm::style::Color::DarkBlue,
            Color::Magenta => crossterm::style::Color::DarkMagenta,
            Color::Cyan => crossterm::style::Color::DarkCyan,
            Color::White => crossterm::style::Color::Grey,
            Color::BrightBlack => crossterm::style::Color::DarkGrey,
            Color::BrightRed => crossterm::style::Color::Red,
            Color::BrightGreen => crossterm::style::Color::Green,
            Color::BrightYellow => crossterm::style::Color::Yellow,
            Color::BrightBlue => crossterm::style::Color::Blue,
            Color::BrightMagenta => crossterm::style::Color::Magenta,
            Color::BrightCyan => crossterm::style::Color::Cyan,
            Color::BrightWhite => crossterm::style::Color::White,
            Color::Rgb(r, g, b) => crossterm::style::Color::Rgb { r, g, b },
        }
    }

    /// Applies cell styling to terminal.
    fn apply_cell_style(&mut self, cell: &Cell) -> io::Result<()> {
        // Always reset attributes first to prevent bleeding from previous cells
        self.stdout.execute(SetAttribute(Attribute::Reset))?;

        // Apply colors
        if let Some(fg) = &cell.fg {
            self.stdout
                .execute(SetForegroundColor(self.color_to_crossterm(*fg)))?;
        }
        if let Some(bg) = &cell.bg {
            self.stdout
                .execute(SetBackgroundColor(self.color_to_crossterm(*bg)))?;
        }

        // Apply text styling attributes
        if cell.style.bold {
            self.stdout.execute(SetAttribute(Attribute::Bold))?;
        }
        if cell.style.italic {
            self.stdout.execute(SetAttribute(Attribute::Italic))?;
        }
        if cell.style.underline {
            self.stdout.execute(SetAttribute(Attribute::Underlined))?;
        }
        if cell.style.strikethrough {
            self.stdout.execute(SetAttribute(Attribute::CrossedOut))?;
        }
        Ok(())
    }

    /// Applies updates with synchronized output mode for atomic rendering.
    ///
    /// ```text
    /// Without Synchronization:         With Synchronization:
    /// ┌──────────────────────┐        ┌──────────────────────┐
    /// │ MoveTo ──────────────┼───┐    │ Begin Sync (?2026h)  │
    /// │ SetColor ────────────┼─┐ │    │ ┌──────────────────┐ │
    /// │ Print "Hello" ───────┼┐│ │    │ │ MoveTo           │ │
    /// │ MoveTo ──────────────┼┼┼─┤    │ │ SetColor         │ │◄─ All updates
    /// │ Print "World" ───────┼┼┼┼┤    │ │ Print "Hello"    │ │  buffered
    /// └──────────────────────┘││││    │ │ MoveTo           │ │
    ///   Visible tearing ──────┘│││    │ │ Print "World"    │ │
    ///   as each command ───────┘││    │ └──────────────────┘ │
    ///   executes ───────────────┘│    │ End Sync (?2026l)    │
    ///   immediately ─────────────┘    └──────────────────────┘
    ///                                   Atomic update - no tearing
    /// ```
    fn apply_updates_synchronized(&mut self, updates: Vec<CellUpdate>) -> io::Result<()> {
        // Begin synchronized update
        self.stdout.execute(Print("\x1b[?2026h"))?;

        let result = self.apply_updates_optimized(updates);

        // End synchronized update
        self.stdout.execute(Print("\x1b[?2026l"))?;
        self.stdout.flush()?;

        result
    }

    /// Applies updates with full terminal optimizations.
    fn apply_updates_optimized(&mut self, updates: Vec<CellUpdate>) -> io::Result<()> {
        // Convert updates to optimized commands
        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        // Apply each command
        for cmd in commands {
            self.apply_command(cmd)?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    /// Applies a single terminal command.
    fn apply_command(&mut self, cmd: TerminalCommand) -> io::Result<()> {
        match cmd {
            TerminalCommand::MoveTo(x, y) => {
                if self.current_pos != Some((x, y)) {
                    self.stdout.execute(cursor::MoveTo(x, y))?;
                    self.current_pos = Some((x, y));
                }
            }
            TerminalCommand::SetColors { fg, bg } => {
                self.set_colors(fg, bg)?;
            }
            TerminalCommand::Print(text) => {
                self.stdout.execute(Print(&text))?;
                // Update cursor position
                if let Some((x, y)) = self.current_pos {
                    self.current_pos = Some((x + text.len() as u16, y));
                }
            }
            TerminalCommand::SetStyle(style) => {
                self.set_style(style)?;
            }
            TerminalCommand::Reset => {
                self.stdout.execute(ResetColor)?;
                self.stdout.execute(SetAttribute(Attribute::Reset))?;
                self.current_fg = None;
                self.current_bg = None;
                self.current_style = CellStyle::default();
            }
        }
        Ok(())
    }

    /// Sets colors only if they've changed.
    fn set_colors(&mut self, fg: Option<Color>, bg: Option<Color>) -> io::Result<()> {
        // Handle foreground color
        if fg != self.current_fg {
            match fg {
                Some(color) => {
                    self.stdout
                        .execute(SetForegroundColor(to_crossterm_color(color)))?;
                }
                None => {
                    // Reset to default foreground (usually white/gray)
                    // We use the terminal's default foreground explicitly
                    self.stdout
                        .execute(SetForegroundColor(crossterm::style::Color::Reset))?;
                }
            }
            self.current_fg = fg;
        }

        // Handle background color
        if bg != self.current_bg {
            match bg {
                Some(color) => {
                    self.stdout
                        .execute(SetBackgroundColor(to_crossterm_color(color)))?;
                }
                None => {
                    // Reset to default background (usually black/transparent)
                    // We use the terminal's default background explicitly
                    self.stdout
                        .execute(SetBackgroundColor(crossterm::style::Color::Reset))?;
                }
            }
            self.current_bg = bg;
        }

        Ok(())
    }

    /// Sets style attributes only if they've changed.
    fn set_style(&mut self, style: CellStyle) -> io::Result<()> {
        if style != self.current_style {
            // Always reset attributes when changing style to ensure clean state
            self.stdout.execute(SetAttribute(Attribute::Reset))?;

            // Apply new attributes if any are needed
            if style.bold {
                self.stdout.execute(SetAttribute(Attribute::Bold))?;
            }
            if style.italic {
                self.stdout.execute(SetAttribute(Attribute::Italic))?;
            }
            if style.underline {
                self.stdout.execute(SetAttribute(Attribute::Underlined))?;
            }
            if style.strikethrough {
                self.stdout.execute(SetAttribute(Attribute::CrossedOut))?;
            }

            self.current_style = style;
        }
        Ok(())
    }

    /// Resets the renderer state.
    #[allow(dead_code)]
    pub fn reset(&mut self) -> io::Result<()> {
        self.apply_command(TerminalCommand::Reset)
    }
}

impl UpdateBatcher {
    /// Creates a new update batcher.
    pub fn new(updates: Vec<CellUpdate>) -> Self {
        Self { updates }
    }

    /// Optimizes updates into efficient terminal commands.
    ///
    /// This performs several optimizations:
    /// 1. Sorts updates by position for better cursor movement
    /// 2. Groups consecutive cells with same style into runs
    /// 3. Minimizes style changes
    ///
    /// ```text
    /// Input Updates:              Optimization Process:           Output Commands:
    /// ┌─────────────┐            ┌─────────────────┐            ┌──────────────┐
    /// │ (5,2) 'C'   │            │ Sort by y,x:    │            │ MoveTo(0,0)  │
    /// │ (0,0) 'A'   │ ─────────▶ │ (0,0) → (1,0)   │ ─────────▶ │ SetColors    │
    /// │ (1,0) 'B'   │   Sort &   │ (3,1) → (5,2)   │  Generate  │ Print("AB")  │
    /// │ (3,1) 'D'   │   Group    │                 │  Commands  │ MoveTo(3,1)  │
    /// └─────────────┘            │ Group into runs │            │ Print("D")   │
    ///                            └─────────────────┘            └──────────────┘
    /// ```
    pub fn optimize(mut self) -> Vec<TerminalCommand> {
        let mut commands = Vec::new();

        // Sort updates by position (top-to-bottom, left-to-right)
        self.updates.sort_by_key(|update| match update {
            CellUpdate::Single { x, y, .. } => (*y, *x),
        });

        // Group updates into runs where possible
        let runs = self.group_into_runs();

        // Convert runs to commands
        for run in runs {
            commands.extend(run.into_commands());
        }

        commands
    }

    /// Groups updates into runs of consecutive cells with same style.
    ///
    /// ```text
    /// Single Updates:                     Grouped Runs:
    /// ┌────────────────┐                 ┌─────────────────────┐
    /// │ (0,0) 'H' Red  │                 │ Run @ (0,0):        │
    /// │ (1,0) 'e' Red  │ ──────────▶     │   "Hello" (Red)     │
    /// │ (2,0) 'l' Red  │  Group same     ├─────────────────────┤
    /// │ (3,0) 'l' Red  │  style cells    │ Run @ (0,1):        │
    /// │ (4,0) 'o' Red  │                 │   "World" (Blue)    │
    /// │ (0,1) 'W' Blue │                 └─────────────────────┘
    /// │ (1,1) 'o' Blue │
    /// │ (2,1) 'r' Blue │     ↑ Consecutive cells with same color/style
    /// │ (3,1) 'l' Blue │     │ are merged into single Print command
    /// │ (4,1) 'd' Blue │     │
    /// └────────────────┘     └── Reduces terminal commands significantly
    /// ```
    fn group_into_runs(self) -> Vec<Run> {
        let mut runs = Vec::new();
        let mut current_run: Option<Run> = None;

        for update in self.updates {
            match update {
                CellUpdate::Single { x, y, cell } => {
                    // Check if we can add to current run
                    if let Some(ref mut run) = current_run
                        && run.can_append(x, y, &cell)
                    {
                        run.cells.push(cell);
                        continue;
                    }

                    // Start new run
                    if let Some(run) = current_run.take() {
                        runs.push(run);
                    }
                    current_run = Some(Run::new(x, y, cell));
                }
            }
        }

        // Don't forget the last run
        if let Some(run) = current_run {
            runs.push(run);
        }

        runs
    }
}

impl Run {
    /// Creates a new run starting with a single cell.
    fn new(x: u16, y: u16, cell: Cell) -> Self {
        let fg = cell.fg;
        let bg = cell.bg;
        let style = cell.style.clone();
        Self {
            x,
            y,
            cells: vec![cell],
            fg,
            bg,
            style,
        }
    }

    /// Checks if a cell can be appended to this run.
    ///
    /// ```text
    /// Current Run:                   New Cell:              Can Append?
    /// ┌──────────────────┐          ┌─────────────┐
    /// │ @ (5,10)         │          │ @ (8,10)    │        ✓ YES - Consecutive
    /// │ "ABC" (len=3)    │    vs    │ 'D' Red/Blu │          position & same
    /// │ Red fg, Blue bg  │          │             │          style
    /// └──────────────────┘          └─────────────┘
    ///        ↓                             ↓
    ///   Next expected: (8,10)         Actual: (8,10) ✓
    ///
    /// ┌──────────────────┐          ┌─────────────┐
    /// │ @ (5,10)         │          │ @ (9,10)    │        ✗ NO - Not consecutive
    /// │ "ABC" (len=3)    │    vs    │ 'E' Red/Blu │          (gap in position)
    /// │ Red fg, Blue bg  │          │             │
    /// └──────────────────┘          └─────────────┘
    ///        ↓                             ↓
    ///   Next expected: (8,10)         Actual: (9,10) ✗
    /// ```
    fn can_append(&self, x: u16, y: u16, cell: &Cell) -> bool {
        // Must be on same line and consecutive
        if y != self.y || x != self.x + self.cells.len() as u16 {
            return false;
        }
        // Must have same style
        cell.fg == self.fg && cell.bg == self.bg && cell.style == self.style
    }

    /// Converts this run to terminal commands.
    fn into_commands(self) -> Vec<TerminalCommand> {
        let has_styles = self.style != CellStyle::default();

        let mut commands = vec![
            TerminalCommand::MoveTo(self.x, self.y),
            TerminalCommand::SetColors {
                fg: self.fg,
                bg: self.bg,
            },
            TerminalCommand::SetStyle(self.style),
        ];

        // Build string from cells
        let text: String = self.cells.iter().map(|c| c.char).collect();
        commands.push(TerminalCommand::Print(text));

        // Reset styles after printing if any non-default styles were applied
        if has_styles {
            commands.push(TerminalCommand::Reset);
        }

        commands
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Converts our Color enum to crossterm's Color type.
fn to_crossterm_color(color: Color) -> crossterm::style::Color {
    match color {
        Color::Black => crossterm::style::Color::Black,
        Color::Red => crossterm::style::Color::DarkRed,
        Color::Green => crossterm::style::Color::DarkGreen,
        Color::Yellow => crossterm::style::Color::DarkYellow,
        Color::Blue => crossterm::style::Color::DarkBlue,
        Color::Magenta => crossterm::style::Color::DarkMagenta,
        Color::Cyan => crossterm::style::Color::DarkCyan,
        Color::White => crossterm::style::Color::Grey,
        Color::BrightBlack => crossterm::style::Color::DarkGrey,
        Color::BrightRed => crossterm::style::Color::Red,
        Color::BrightGreen => crossterm::style::Color::Green,
        Color::BrightYellow => crossterm::style::Color::Yellow,
        Color::BrightBlue => crossterm::style::Color::Blue,
        Color::BrightMagenta => crossterm::style::Color::Magenta,
        Color::BrightCyan => crossterm::style::Color::Cyan,
        Color::BrightWhite => crossterm::style::Color::White,
        Color::Rgb(r, g, b) => crossterm::style::Color::Rgb { r, g, b },
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::{Cell, CellStyle, CellUpdate};
    use crate::style::Color;

    #[test]
    fn test_update_batcher_single_cell() {
        let updates = vec![CellUpdate::Single {
            x: 5,
            y: 10,
            cell: Cell {
                char: 'A',
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                style: CellStyle::default(),
            },
        }];

        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        assert_eq!(commands.len(), 4);
        assert!(matches!(commands[0], TerminalCommand::MoveTo(5, 10)));
        assert!(matches!(
            commands[1],
            TerminalCommand::SetColors {
                fg: Some(Color::Red),
                bg: Some(Color::Blue)
            }
        ));
        assert!(matches!(commands[2], TerminalCommand::SetStyle(_)));
        assert!(matches!(commands[3], TerminalCommand::Print(ref s) if s == "A"));
    }

    #[test]
    fn test_update_batcher_consecutive_cells_same_style() {
        let style = CellStyle::default();
        let updates = vec![
            CellUpdate::Single {
                x: 0,
                y: 0,
                cell: Cell {
                    char: 'H',
                    fg: Some(Color::Green),
                    bg: None,
                    style: style.clone(),
                },
            },
            CellUpdate::Single {
                x: 1,
                y: 0,
                cell: Cell {
                    char: 'e',
                    fg: Some(Color::Green),
                    bg: None,
                    style: style.clone(),
                },
            },
            CellUpdate::Single {
                x: 2,
                y: 0,
                cell: Cell {
                    char: 'l',
                    fg: Some(Color::Green),
                    bg: None,
                    style: style.clone(),
                },
            },
            CellUpdate::Single {
                x: 3,
                y: 0,
                cell: Cell {
                    char: 'l',
                    fg: Some(Color::Green),
                    bg: None,
                    style: style.clone(),
                },
            },
            CellUpdate::Single {
                x: 4,
                y: 0,
                cell: Cell {
                    char: 'o',
                    fg: Some(Color::Green),
                    bg: None,
                    style,
                },
            },
        ];

        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        // Should be optimized into a single run
        assert_eq!(commands.len(), 4);
        assert!(matches!(commands[0], TerminalCommand::MoveTo(0, 0)));
        assert!(matches!(commands[3], TerminalCommand::Print(ref s) if s == "Hello"));
    }

    #[test]
    fn test_update_batcher_different_styles() {
        let updates = vec![
            CellUpdate::Single {
                x: 0,
                y: 0,
                cell: Cell {
                    char: 'A',
                    fg: Some(Color::Red),
                    bg: None,
                    style: CellStyle::default(),
                },
            },
            CellUpdate::Single {
                x: 1,
                y: 0,
                cell: Cell {
                    char: 'B',
                    fg: Some(Color::Blue),
                    bg: None,
                    style: CellStyle::default(),
                },
            },
        ];

        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        // Should create two separate runs due to different colors
        // Each run needs: MoveTo, SetColors, SetStyle, Print = 4 commands
        // Total: 8 commands (4 for 'A' with Red, 4 for 'B' with Blue)
        assert_eq!(commands.len(), 8);

        // First run for 'A' with Red color
        assert!(matches!(commands[0], TerminalCommand::MoveTo(0, 0)));
        assert!(matches!(
            commands[1],
            TerminalCommand::SetColors {
                fg: Some(Color::Red),
                bg: None
            }
        ));
        assert!(matches!(commands[2], TerminalCommand::SetStyle(_)));
        assert!(matches!(commands[3], TerminalCommand::Print(ref s) if s == "A"));

        // Second run for 'B' with Blue color
        assert!(matches!(commands[4], TerminalCommand::MoveTo(1, 0)));
        assert!(matches!(
            commands[5],
            TerminalCommand::SetColors {
                fg: Some(Color::Blue),
                bg: None
            }
        ));
        assert!(matches!(commands[6], TerminalCommand::SetStyle(_)));
        assert!(matches!(commands[7], TerminalCommand::Print(ref s) if s == "B"));
    }

    #[test]
    fn test_update_batcher_sorting() {
        let updates = vec![
            CellUpdate::Single {
                x: 5,
                y: 2,
                cell: Cell::new('C'),
            },
            CellUpdate::Single {
                x: 0,
                y: 0,
                cell: Cell::new('A'),
            },
            CellUpdate::Single {
                x: 3,
                y: 1,
                cell: Cell::new('B'),
            },
        ];

        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        // First command should be MoveTo(0, 0) due to sorting
        assert!(matches!(commands[0], TerminalCommand::MoveTo(0, 0)));
    }

    #[test]
    fn test_run_can_append() {
        let cell1 = Cell {
            char: 'A',
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
            style: CellStyle::default(),
        };

        let run = Run::new(5, 10, cell1.clone());

        // Same style, consecutive position - should append
        let cell2 = Cell {
            char: 'B',
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
            style: CellStyle::default(),
        };
        assert!(run.can_append(6, 10, &cell2));

        // Different line - should not append
        assert!(!run.can_append(6, 11, &cell2));

        // Non-consecutive position - should not append
        assert!(!run.can_append(8, 10, &cell2));

        // Different color - should not append
        let cell3 = Cell {
            char: 'C',
            fg: Some(Color::Green),
            bg: Some(Color::Blue),
            style: CellStyle::default(),
        };
        assert!(!run.can_append(6, 10, &cell3));
    }

    #[test]
    fn test_run_with_bold_style() {
        let style = CellStyle {
            bold: true,
            ..Default::default()
        };

        let updates = vec![CellUpdate::Single {
            x: 0,
            y: 0,
            cell: Cell {
                char: 'B',
                fg: None,
                bg: None,
                style,
            },
        }];

        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();

        // Should include SetStyle command with bold
        let has_bold_style = commands
            .iter()
            .any(|cmd| matches!(cmd, TerminalCommand::SetStyle(style) if style.bold));
        assert!(has_bold_style);
    }

    #[test]
    fn test_terminal_command_types() {
        // Test MoveTo
        let move_cmd = TerminalCommand::MoveTo(10, 20);
        assert!(matches!(move_cmd, TerminalCommand::MoveTo(10, 20)));

        // Test SetColors
        let color_cmd = TerminalCommand::SetColors {
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
        };
        assert!(matches!(
            color_cmd,
            TerminalCommand::SetColors {
                fg: Some(Color::Red),
                bg: Some(Color::Blue)
            }
        ));

        // Test Print
        let print_cmd = TerminalCommand::Print("Hello".to_string());
        assert!(matches!(print_cmd, TerminalCommand::Print(ref s) if s == "Hello"));

        // Test Reset
        let reset_cmd = TerminalCommand::Reset;
        assert!(matches!(reset_cmd, TerminalCommand::Reset));
    }

    #[test]
    fn test_to_crossterm_color() {
        assert_eq!(
            to_crossterm_color(Color::Red),
            crossterm::style::Color::DarkRed
        );
        assert_eq!(
            to_crossterm_color(Color::BrightRed),
            crossterm::style::Color::Red
        );
        assert_eq!(
            to_crossterm_color(Color::Rgb(100, 150, 200)),
            crossterm::style::Color::Rgb {
                r: 100,
                g: 150,
                b: 200
            }
        );
    }

    #[test]
    fn test_empty_updates() {
        let updates = vec![];
        let batcher = UpdateBatcher::new(updates);
        let commands = batcher.optimize();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_multiple_runs_different_lines() {
        let updates = vec![
            CellUpdate::Single {
                x: 0,
                y: 0,
                cell: Cell::new('A'),
            },
            CellUpdate::Single {
                x: 1,
                y: 0,
                cell: Cell::new('B'),
            },
            CellUpdate::Single {
                x: 0,
                y: 1,
                cell: Cell::new('C'),
            },
            CellUpdate::Single {
                x: 1,
                y: 1,
                cell: Cell::new('D'),
            },
        ];

        let batcher = UpdateBatcher::new(updates);
        let runs = batcher.group_into_runs();

        // Should create two runs (one per line)
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].cells.len(), 2); // "AB"
        assert_eq!(runs[1].cells.len(), 2); // "CD"
    }
}
