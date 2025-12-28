# Inline Rendering Mode for rxtui

## Problem Statement

rxtui currently renders exclusively to the terminal's alternate screen buffer. This mode:
- Clears the terminal and takes over the full screen
- Content disappears when the app exits
- Not suitable for CLI tools that want persistent inline output

**Goal**: Add inline rendering mode that renders directly in the terminal without alternate screen, with proper terminal scrolling support.

---

## Terminal Scrolling Nuances

### The Core Challenge

When rendering inline (not in alternate screen), terminal scrolling creates coordinate system problems:

```
Before rendering (cursor at row 45 of 50-row terminal):
┌─────────────────────────┐
│ ... terminal history    │ row 1-44
│ cursor here ►           │ row 45
│                         │ row 46-50 (empty)
└─────────────────────────┘

After printing 10 lines:
┌─────────────────────────┐
│ ... terminal history    │ rows shifted up by 5
│ our line 1              │
│ our line 2              │
│ ...                     │
│ our line 10             │ row 50
└─────────────────────────┘

Problem: Our "starting position" (row 45) no longer exists at row 45!
         It scrolled up. Re-rendering to row 45 writes to wrong location.
```

### Strategy: Space Reservation

Reserve rendering space BEFORE drawing content. This guarantees we have room and establishes a stable coordinate system.

```
Step 1: Query cursor position → (col=0, row=45)
Step 2: Calculate content height → 10 lines
Step 3: Print 10 newlines (reserves space, causes scroll if needed)
Step 4: Move cursor back up 10 lines → now at stable "origin"
Step 5: Render content → writes into reserved space
Step 6: On re-render → move to origin, overwrite in place
```

**Why this works**: After reservation, all scrolling has already happened. The origin position is stable for the lifetime of the app.

---

## Comprehensive Design

### 1. New Types

**File**: `lib/app/config.rs`

```rust
/// Terminal rendering mode
pub enum TerminalMode {
    /// Full-screen alternate buffer (current behavior)
    AlternateScreen,
    /// Inline rendering in main terminal buffer
    Inline(InlineConfig),
}

/// Configuration for inline rendering mode
pub struct InlineConfig {
    /// How to determine rendering height
    pub height: InlineHeight,
    /// Whether to show cursor during rendering
    pub cursor_visible: bool,
    /// Whether to preserve output after app exits
    pub preserve_on_exit: bool,
}

/// Height determination strategy for inline mode
pub enum InlineHeight {
    /// Fixed number of lines
    Fixed(u16),
    /// Grow to fit content, with optional maximum
    Content { max: Option<u16> },
    /// Fill remaining terminal space below cursor
    Fill { min: u16 },
}

impl Default for InlineConfig {
    fn default() -> Self {
        Self {
            height: InlineHeight::Content { max: Some(24) },
            cursor_visible: false,
            preserve_on_exit: true,
        }
    }
}
```

**File**: `lib/app/inline.rs` (new file)

```rust
/// Runtime state for inline rendering
pub(crate) struct InlineState {
    /// Row where our rendering area starts (after space reservation)
    pub origin_row: u16,
    /// Column where rendering starts (usually 0)
    pub origin_col: u16,
    /// Current reserved height
    pub reserved_height: u16,
    /// Terminal dimensions at initialization
    pub terminal_size: (u16, u16),
    /// Whether space has been reserved
    pub initialized: bool,
}

impl InlineState {
    pub fn new() -> Self {
        Self {
            origin_row: 0,
            origin_col: 0,
            reserved_height: 0,
            terminal_size: (80, 24),
            initialized: false,
        }
    }
}
```

### 2. Space Reservation Algorithm

**File**: `lib/app/inline.rs`

```rust
impl InlineState {
    /// Reserve space for inline rendering
    /// Must be called before first render
    pub fn reserve_space(
        &mut self,
        stdout: &mut impl Write,
        height: u16,
    ) -> io::Result<()> {
        // 1. Get terminal dimensions
        let (term_width, term_height) = terminal::size()?;
        self.terminal_size = (term_width, term_height);

        // 2. Query current cursor position
        let (cursor_col, cursor_row) = cursor::position()?;

        // 3. Calculate how many lines we can use
        let available_below = term_height.saturating_sub(cursor_row);
        let need_to_scroll = height.saturating_sub(available_below);

        // 4. Print newlines to reserve space (causes scroll if needed)
        for _ in 0..height {
            stdout.execute(Print("\n"))?;
        }

        // 5. Move cursor back up to origin
        stdout.execute(cursor::MoveUp(height))?;

        // 6. Query new position (this is our stable origin)
        let (new_col, new_row) = cursor::position()?;
        self.origin_row = new_row;
        self.origin_col = new_col;
        self.reserved_height = height;
        self.initialized = true;

        Ok(())
    }

    /// Expand reserved space if content grew
    pub fn expand_space(
        &mut self,
        stdout: &mut impl Write,
        new_height: u16,
    ) -> io::Result<()> {
        if new_height <= self.reserved_height {
            return Ok(());
        }

        let additional = new_height - self.reserved_height;

        // Move to end of current reserved area
        stdout.execute(cursor::MoveTo(
            0,
            self.origin_row + self.reserved_height,
        ))?;

        // Print additional newlines
        for _ in 0..additional {
            stdout.execute(Print("\n"))?;
        }

        // Origin shifts up if we scrolled
        let (_, term_height) = self.terminal_size;
        let bottom_row = self.origin_row + new_height;
        if bottom_row > term_height {
            let scroll_amount = bottom_row - term_height;
            self.origin_row = self.origin_row.saturating_sub(scroll_amount);
        }

        self.reserved_height = new_height;
        Ok(())
    }

    /// Move cursor to origin for rendering
    pub fn move_to_origin(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(self.origin_col, self.origin_row))?;
        Ok(())
    }

    /// Move cursor to end of rendered content (for exit)
    pub fn move_to_end(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.execute(cursor::MoveTo(0, self.origin_row + self.reserved_height))?;
        stdout.execute(Print("\n"))?; // Ensure prompt appears below
        Ok(())
    }
}
```

### 3. Height Calculation for Inline Mode

**File**: `lib/render_tree/tree.rs`

The key change: don't clamp height to viewport for inline Content mode.

```rust
pub fn layout(&mut self, viewport_width: u16, viewport_height: u16, unclamped_height: bool) {
    // ... existing code ...

    // Height resolution with optional unclamping
    match height_dim {
        Some(Dimension::Fixed(h)) => {
            root_ref.height = if unclamped_height { h } else { h.min(viewport_height) };
        }
        Some(Dimension::Percentage(pct)) => {
            let calculated = (viewport_height as f32 * pct) as u16;
            root_ref.height = calculated.max(1);
            if !unclamped_height {
                root_ref.height = root_ref.height.min(viewport_height);
            }
        }
        Some(Dimension::Content) | None => {
            // For Content or None, use intrinsic height
            // Only clamp if not in unclamped mode
            root_ref.height = if unclamped_height {
                intrinsic_height
            } else {
                intrinsic_height.min(viewport_height)
            };
        }
        Some(Dimension::Auto) => {
            root_ref.height = viewport_height; // Auto still fills available space
        }
    }

    // ... continue with layout_with_parent ...
}
```

### 4. Modified App Initialization

**File**: `lib/app/core.rs`

```rust
impl App {
    /// Create app with specified terminal mode
    pub fn with_mode<C: Component + 'static>(
        root: C,
        mode: TerminalMode,
    ) -> io::Result<Self> {
        let mut stdout = io::stdout();

        // Always enable raw mode for event handling
        terminal::enable_raw_mode()?;

        match &mode {
            TerminalMode::AlternateScreen => {
                stdout.execute(terminal::EnterAlternateScreen)?;
                stdout.execute(cursor::Hide)?;
            }
            TerminalMode::Inline(config) => {
                if !config.cursor_visible {
                    stdout.execute(cursor::Hide)?;
                }
                // Space reservation happens on first render
            }
        }

        stdout.execute(event::EnableMouseCapture)?;

        let (width, height) = terminal::size()?;

        Ok(Self {
            // ... existing fields ...
            terminal_mode: mode,
            inline_state: InlineState::new(),
        })
    }

    /// Convenience constructor for inline mode with defaults
    pub fn inline<C: Component + 'static>(root: C) -> io::Result<Self> {
        Self::with_mode(root, TerminalMode::Inline(InlineConfig::default()))
    }
}
```

### 5. Modified Rendering Pipeline

**File**: `lib/app/core.rs`

```rust
impl App {
    fn draw(&mut self) -> io::Result<()> {
        match &self.terminal_mode {
            TerminalMode::AlternateScreen => {
                self.draw_alternate_screen()
            }
            TerminalMode::Inline(config) => {
                self.draw_inline(config)
            }
        }
    }

    fn draw_inline(&mut self, config: &InlineConfig) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Calculate content dimensions
        let (term_width, term_height) = terminal::size()?;
        let unclamped = matches!(config.height, InlineHeight::Content { .. });

        // Layout with potentially unclamped height
        let layout_height = match &config.height {
            InlineHeight::Fixed(h) => *h,
            InlineHeight::Content { max } => max.unwrap_or(term_height),
            InlineHeight::Fill { min } => {
                let available = term_height.saturating_sub(self.inline_state.origin_row);
                available.max(*min)
            }
        };

        self.vdom.layout(term_width, layout_height, unclamped);

        // Get actual content height from rendered tree
        let content_height = self.vdom.render_tree()
            .and_then(|rt| rt.root.as_ref().map(|r| r.borrow().height))
            .unwrap_or(1);

        // Apply height limits
        let render_height = match &config.height {
            InlineHeight::Fixed(h) => *h,
            InlineHeight::Content { max } => {
                max.map(|m| content_height.min(m)).unwrap_or(content_height)
            }
            InlineHeight::Fill { min } => content_height.max(*min),
        };

        // Initialize or expand space reservation
        if !self.inline_state.initialized {
            self.inline_state.reserve_space(&mut stdout, render_height)?;
        } else if render_height > self.inline_state.reserved_height {
            self.inline_state.expand_space(&mut stdout, render_height)?;
        }

        // Move to origin
        self.inline_state.move_to_origin(&mut stdout)?;

        // Render to buffer
        self.double_buffer.clear_back();
        let root = self.vdom.render_tree().unwrap().root.clone().unwrap();
        let clip_rect = Rect {
            x: 0,
            y: 0,
            width: term_width,
            height: render_height,
        };
        render_node_to_buffer(&root.borrow(), &mut self.double_buffer.back_mut(), &clip_rect, None);

        // Apply updates with line-based clearing
        let updates = self.double_buffer.diff();
        self.terminal_renderer.apply_updates_inline(updates, self.inline_state.origin_row)?;

        self.double_buffer.swap();
        stdout.flush()?;

        Ok(())
    }
}
```

### 6. Modified Terminal Renderer

**File**: `lib/terminal.rs`

```rust
impl TerminalRenderer {
    /// Apply updates for inline mode with origin offset
    pub fn apply_updates_inline(
        &mut self,
        updates: Vec<CellUpdate>,
        origin_row: u16,
    ) -> io::Result<()> {
        // Transform coordinates to account for origin
        let transformed: Vec<CellUpdate> = updates
            .into_iter()
            .map(|update| match update {
                CellUpdate::Single { x, y, cell } => CellUpdate::Single {
                    x,
                    y: y + origin_row,
                    cell,
                },
            })
            .collect();

        // Use existing optimized update path
        self.apply_updates_optimized(transformed)?;
        Ok(())
    }

    /// Clear specific lines (for inline mode)
    pub fn clear_lines(&mut self, start_row: u16, count: u16) -> io::Result<()> {
        for row in start_row..(start_row + count) {
            self.stdout.execute(cursor::MoveTo(0, row))?;
            self.stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        }
        self.current_pos = None;
        Ok(())
    }
}
```

### 7. Modified Cleanup

**File**: `lib/app/core.rs`

```rust
impl Drop for App {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        let _ = stdout.execute(event::DisableMouseCapture);
        let _ = stdout.execute(cursor::Show);

        match &self.terminal_mode {
            TerminalMode::AlternateScreen => {
                let _ = stdout.execute(terminal::LeaveAlternateScreen);
            }
            TerminalMode::Inline(config) => {
                if config.preserve_on_exit {
                    // Move cursor below rendered content
                    let _ = self.inline_state.move_to_end(&mut stdout);
                } else {
                    // Clear the inline rendering area
                    let _ = self.terminal_renderer.clear_lines(
                        self.inline_state.origin_row,
                        self.inline_state.reserved_height,
                    );
                    let _ = stdout.execute(cursor::MoveTo(
                        self.inline_state.origin_col,
                        self.inline_state.origin_row,
                    ));
                }
            }
        }

        let _ = stdout.flush();
        let _ = terminal::disable_raw_mode();
    }
}
```

### 8. Event Handling Changes

**File**: `lib/app/core.rs`

```rust
// In run_loop(), modify resize handling:
Event::Resize(width, height) => {
    match &self.terminal_mode {
        TerminalMode::AlternateScreen => {
            // Existing behavior: full re-layout and clear
            self.vdom.layout(width, height, false);
            self.double_buffer.resize(width, height);
            self.double_buffer.reset();
            self.terminal_renderer.clear_screen()?;
        }
        TerminalMode::Inline(_) => {
            // Inline mode: only handle width changes
            // Height is managed by space reservation
            self.inline_state.terminal_size = (width, height);
            // Re-render will handle layout
        }
    }
    *self.needs_render.borrow_mut() = true;
}

// Mouse event coordinate translation for inline mode:
Event::Mouse(mouse_event) => {
    let adjusted_event = match &self.terminal_mode {
        TerminalMode::Inline(_) => {
            // Translate coordinates relative to origin
            let adjusted_row = mouse_event.row
                .saturating_sub(self.inline_state.origin_row);
            MouseEvent {
                row: adjusted_row,
                ..mouse_event
            }
        }
        _ => mouse_event,
    };
    // ... existing mouse handling with adjusted_event ...
}
```

---

## Implementation Order

### Phase 1: Foundation (Core Types & Config)
1. Add `TerminalMode`, `InlineConfig`, `InlineHeight` enums to `config.rs`
2. Create `inline.rs` with `InlineState` struct
3. Add `terminal_mode` and `inline_state` fields to `App` struct
4. Update `App::new()` to accept mode, create `App::with_mode()` and `App::inline()`

### Phase 2: Space Reservation
1. Implement `InlineState::reserve_space()` with cursor position query
2. Implement `InlineState::expand_space()` for dynamic height growth
3. Implement `InlineState::move_to_origin()` and `move_to_end()`
4. Add `TerminalRenderer::clear_lines()` for line-based clearing

### Phase 3: Layout Changes
1. Add `unclamped_height: bool` parameter to `RenderTree::layout()`
2. Modify height clamping logic to respect unclamped flag
3. Ensure intrinsic height calculation works correctly for Content mode

### Phase 4: Rendering Pipeline
1. Implement `App::draw_inline()` method
2. Add `TerminalRenderer::apply_updates_inline()` with origin offset
3. Integrate space reservation into first render
4. Handle height expansion on subsequent renders

### Phase 5: Cleanup & Events
1. Modify `Drop` implementation for inline mode cleanup
2. Add mouse coordinate translation for inline mode
3. Modify resize event handling (width-only for inline)
4. Add `preserve_on_exit` behavior

### Phase 6: Testing & Polish
1. Create inline mode examples
2. Test with various terminal emulators
3. Handle edge cases (very small terminals, rapid resizing)
4. Document public API

---

## Files to Modify

| File | Changes |
|------|---------|
| `lib/app/config.rs` | Add `TerminalMode`, `InlineConfig`, `InlineHeight` |
| `lib/app/inline.rs` | New file: `InlineState` implementation |
| `lib/app/core.rs` | `App::with_mode()`, `App::inline()`, `draw_inline()`, modified `Drop` |
| `lib/app/mod.rs` | Export new types and inline module |
| `lib/render_tree/tree.rs` | Add `unclamped_height` parameter to `layout()` |
| `lib/terminal.rs` | `apply_updates_inline()`, `clear_lines()` |
| `lib/lib.rs` | Re-export `TerminalMode`, `InlineConfig`, `InlineHeight` |

---

## API Usage Example

```rust
use rxtui::{App, InlineConfig, InlineHeight, TerminalMode};

// Simple inline mode with defaults
let app = App::inline(MyComponent)?;
app.run()?;

// Custom inline configuration
let config = InlineConfig {
    height: InlineHeight::Fixed(10),
    cursor_visible: true,
    preserve_on_exit: true,
};
let app = App::with_mode(MyComponent, TerminalMode::Inline(config))?;
app.run()?;

// Content-based height with max
let config = InlineConfig {
    height: InlineHeight::Content { max: Some(20) },
    ..Default::default()
};
let app = App::with_mode(MyComponent, TerminalMode::Inline(config))?;
app.run()?;
```

---

## Edge Cases & Considerations

1. **Terminal too small**: If terminal height < requested inline height, clamp to available space
2. **Cursor position query fails**: Fall back to assuming cursor is at terminal height (worst case, reserve full space)
3. **Content shrinks**: Don't reduce reserved space (could leave gaps), just render less
4. **Very rapid updates**: Existing double-buffering and diffing still applies
5. **Mouse clicks outside area**: Ignore or pass through based on config
6. **No synchronized output**: Inline mode should work without it (most terminals support)
7. **Piped output**: Detect `!isatty()` and use simplified output mode
