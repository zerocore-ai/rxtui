use crate::bounds::Rect;
use crate::component::ComponentId;
use crate::key::Key;
use crate::node::{DivStyles, EventCallbacks, TextSpan};
use crate::style::{
    AlignItems, AlignSelf, Color, Dimension, Direction, JustifyContent, Overflow, Position,
    Spacing, Style, TextStyle, TextWrap,
};
use crate::utils::{display_width, wrap_text};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A node in the render tree with calculated position and dimensions.
///
/// RenderNodes are created from Nodes and contain all the information
/// needed to draw elements to the terminal screen.
///
/// ## Node Coordinates
///
/// ```text
///   Terminal Grid:
///   0 1 2 3 4 5 6 7 8 9
/// 0 ┌──────────────────┐
/// 1 │ RenderNode       │  x=1, y=1
/// 2 │ ┌─────────────┐  │  width=13
/// 3 │ │   Content   │  │  height=3
/// 4 │ └─────────────┘  │
/// 5 └──────────────────┘
/// ```
#[derive(Debug)]
pub struct RenderNode {
    /// The type of node (element container or text)
    pub node_type: RenderNodeType,

    /// X coordinate in terminal columns (0-based)
    pub x: u16,

    /// Y coordinate in terminal rows (0-based)
    pub y: u16,

    /// Width in terminal columns
    pub width: u16,

    /// Height in terminal rows
    pub height: u16,

    /// Style properties (colors, borders, padding)
    pub style: Option<Style>,

    /// Text color (only used for text nodes)
    pub text_color: Option<Color>,

    /// Full text style (only used for text nodes)
    pub text_style: Option<TextStyle>,

    /// Child nodes to render inside this node
    pub children: Vec<Rc<RefCell<RenderNode>>>,

    /// Parent node reference (for traversal)
    pub parent: Option<Weak<RefCell<RenderNode>>>,

    /// Visual styling for different states
    pub styles: DivStyles,

    /// Event callbacks
    pub events: EventCallbacks,

    /// Whether this element can receive focus
    pub focusable: bool,

    /// Whether this element is currently focused
    pub focused: bool,

    /// Whether this element is currently hovered
    pub hovered: bool,

    /// Whether this node needs to be redrawn
    pub dirty: bool,

    /// Z-index for layering (higher values render on top)
    pub z_index: i32,

    /// Position type (relative, absolute, fixed)
    pub position_type: Position,

    /// Vertical scroll offset in rows
    pub scroll_y: u16,

    /// Actual content width (may exceed container width)
    pub content_width: u16,

    /// Actual content height (may exceed container height)
    pub content_height: u16,

    /// Whether this node is scrollable (has overflow:scroll or auto)
    pub scrollable: bool,

    /// Component path that produced this node (used for focus targeting)
    pub component_path: Option<ComponentId>,
}

/// Types of nodes that can be rendered.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderNodeType {
    /// Div element that can have children and styling
    Element,

    /// Text content leaf node (single line)
    Text(String),

    /// Wrapped text content (multiple lines)
    TextWrapped(Vec<String>),

    /// Text with multiple styled segments
    RichText(Vec<TextSpan>),

    /// Wrapped styled text (multiple lines, each with styled segments)
    RichTextWrapped(Vec<Vec<TextSpan>>),
}

//--------------------------------------------------------------------------------------------------
// Helper Functions
//--------------------------------------------------------------------------------------------------

/// Calculate offset and item spacing based on JustifyContent mode
fn calculate_justify_offsets(
    justify: JustifyContent,
    available_space: u16,
    item_count: usize,
    gap: u16,
) -> (u16, u16) {
    match justify {
        JustifyContent::Start => (0, gap),
        JustifyContent::End => (available_space, gap),
        JustifyContent::Center => (available_space / 2, gap),
        JustifyContent::SpaceBetween => {
            if item_count > 1 {
                let spacing = available_space / (item_count as u16 - 1);
                (0, spacing)
            } else {
                (0, gap)
            }
        }
        JustifyContent::SpaceAround => {
            if item_count > 0 {
                let spacing = available_space / item_count as u16;
                (spacing / 2, spacing)
            } else {
                (0, gap)
            }
        }
        JustifyContent::SpaceEvenly => {
            if item_count > 0 {
                let spacing = available_space / (item_count as u16 + 1);
                (spacing, spacing)
            } else {
                (0, gap)
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RenderNode {
    /// Creates a new render node with the specified type.
    ///
    /// Initializes position and dimensions to 0.
    pub fn new(node_type: RenderNodeType) -> Self {
        Self {
            node_type,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            style: None,
            text_color: None,
            text_style: None,
            children: Vec::new(),
            parent: None,
            styles: DivStyles::default(),
            events: EventCallbacks::default(),
            focusable: false,
            focused: false,
            hovered: false,
            dirty: true,
            z_index: 0,
            position_type: Position::Relative,
            scroll_y: 0,
            content_width: 0,
            content_height: 0,
            scrollable: false,
            component_path: None,
        }
    }

    /// Creates a new element container node.
    pub fn element() -> Self {
        Self::new(RenderNodeType::Element)
    }

    /// Creates a new text node with the given content.
    pub fn text(content: impl Into<String>) -> Self {
        Self::new(RenderNodeType::Text(content.into()))
    }

    /// Creates a new wrapped text node with multiple lines.
    pub fn text_wrapped(lines: Vec<String>) -> Self {
        Self::new(RenderNodeType::TextWrapped(lines))
    }

    /// Sets the absolute position of this node in terminal coordinates.
    pub fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Sets the dimensions of this node in terminal cells.
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Adds a child node to this container and sets up parent reference.
    pub fn add_child_with_parent(
        self_rc: &Rc<RefCell<RenderNode>>,
        child: Rc<RefCell<RenderNode>>,
    ) {
        child.borrow_mut().parent = Some(Rc::downgrade(self_rc));
        self_rc.borrow_mut().children.push(child);
    }

    /// Gets the bounds rectangle for this node.
    pub fn bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Marks this node as dirty, requiring a redraw.
    ///
    /// Also marks all parent nodes as dirty since they contain
    /// this dirty region.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        // Note: Parent propagation would require upgrading weak ref
        // For now, we'll handle this at the tree level
    }

    /// Clears the dirty flag after rendering.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Computes the effective style for the current focus/hover state.
    pub fn compose_state_style(
        styles: &DivStyles,
        focusable: bool,
        focused: bool,
        hovered: bool,
    ) -> Option<Style> {
        let base = styles.base.clone();

        let focus_overlay = if focused {
            let default_focus = if focusable {
                Some(Style::default_focus())
            } else {
                None
            };
            Style::merge(default_focus, styles.focus.clone())
        } else {
            None
        };

        let hover_overlay = if hovered { styles.hover.clone() } else { None };

        let with_focus = Style::merge(base, focus_overlay);
        Style::merge(with_focus, hover_overlay)
    }

    /// Applies the provided style to this node, updating derived properties.
    fn apply_computed_style(&mut self, style: Option<Style>) {
        if let Some(ref style) = style {
            if let Some(Dimension::Fixed(width)) = style.width {
                self.width = width;
            }
            if let Some(Dimension::Fixed(height)) = style.height {
                self.height = height;
            }
            self.position_type = style.position.unwrap_or(Position::Relative);
            self.z_index = style.z_index.unwrap_or(0);
        } else {
            self.position_type = Position::Relative;
            self.z_index = 0;
        }

        self.style = style;
    }

    /// Recomputes the node style based on focus/hover state and marks dirty if needed.
    pub fn refresh_state_style(&mut self) {
        let new_style =
            Self::compose_state_style(&self.styles, self.focusable, self.focused, self.hovered);
        let needs_dirty = self.style != new_style;
        self.apply_computed_style(new_style);
        if needs_dirty {
            self.mark_dirty();
        }
    }

    /// Returns true if this node creates a positioning context for absolute children.
    /// A node is "positioned" if it has position: absolute or fixed (not relative).
    pub fn is_positioned(&self) -> bool {
        matches!(self.position_type, Position::Absolute | Position::Fixed)
    }

    /// Updates the vertical scroll position by the given delta, clamping to valid range.
    ///
    /// Returns true if the scroll position changed.
    pub fn update_scroll(&mut self, delta_y: i16) -> bool {
        if !self.scrollable {
            return false;
        }

        let old_scroll_y = self.scroll_y;

        // Calculate maximum scroll value
        let max_scroll_y = self.content_height.saturating_sub(self.height);

        // Update scroll position with clamping
        self.scroll_y = (self.scroll_y as i16 + delta_y)
            .max(0)
            .min(max_scroll_y as i16) as u16;

        // Return whether position changed
        self.scroll_y != old_scroll_y
    }

    /// Sets the vertical scroll position to a specific value, clamping to valid range.
    pub fn set_scroll_y(&mut self, y: u16) {
        if !self.scrollable {
            return;
        }

        let max_scroll_y = self.content_height.saturating_sub(self.height);
        self.scroll_y = y.min(max_scroll_y);
    }

    /// Returns the maximum scrollable range for vertical axis.
    pub fn get_max_scroll_y(&self) -> u16 {
        self.content_height.saturating_sub(self.height)
    }

    /// Calculates the intrinsic (content-based) size of this node and its children.
    /// Returns (width, height) based on the node's content.
    pub fn calculate_intrinsic_size(&self) -> (u16, u16) {
        // Use multi-pass calculation for complex scenarios
        self.calculate_intrinsic_size_multipass(3, None)
    }

    /// Multi-pass intrinsic size calculation with convergence detection.
    /// Handles complex scenarios like percentage children in content-sized parents.
    fn calculate_intrinsic_size_multipass(
        &self,
        max_passes: usize,
        hint: Option<(u16, u16)>,
    ) -> (u16, u16) {
        let mut size = self.calculate_intrinsic_size_single_pass(hint);
        let mut prev_size = size;

        for _pass in 1..max_passes {
            // Use previous size as hint for next pass
            size = self.calculate_intrinsic_size_single_pass(Some(prev_size));

            // Check for convergence
            if size == prev_size {
                break;
            }
            prev_size = size;
        }

        size
    }

    /// Single pass of intrinsic size calculation.
    /// Uses hint for resolving percentages and simulating wrapping.
    fn calculate_intrinsic_size_single_pass(&self, hint: Option<(u16, u16)>) -> (u16, u16) {
        match &self.node_type {
            RenderNodeType::Text(text) => {
                // Check if this text node has wrapping enabled
                if let Some(text_style) = &self.text_style
                    && let Some(wrap_mode) = text_style.wrap
                    && wrap_mode != TextWrap::None
                {
                    // Determine wrap width from either:
                    // 1. Text node's own fixed width
                    // 2. Hint from parent (for text in fixed-width containers)
                    let wrap_width = if let Some(style) = &self.style {
                        if let Some(Dimension::Fixed(width)) = style.width {
                            Some(width)
                        } else {
                            // Use hint width if available
                            hint.map(|(w, _)| w)
                        }
                    } else {
                        // No style, use hint width if available
                        hint.map(|(w, _)| w)
                    };

                    if let Some(width) = wrap_width {
                        // Apply wrapping at the determined width to get accurate height
                        let wrapped_lines = wrap_text(text, width, wrap_mode);
                        let height = wrapped_lines.len() as u16;
                        let actual_width = wrapped_lines
                            .iter()
                            .map(|l| display_width(l))
                            .max()
                            .unwrap_or(0) as u16;
                        return (actual_width.min(width), height);
                    }
                }
                // Default: unwrapped text size
                (display_width(text) as u16, 1)
            }
            RenderNodeType::TextWrapped(lines) => {
                // Already wrapped text: width is longest line, height is line count
                let width = lines.iter().map(|l| display_width(l)).max().unwrap_or(0) as u16;
                let height = lines.len() as u16;
                (width, height)
            }
            RenderNodeType::RichText(spans) => {
                // Calculate total width by summing all span content widths
                let width = spans
                    .iter()
                    .map(|span| display_width(&span.content) as u16)
                    .sum();
                // RichText is single line for now
                (width, 1)
            }
            RenderNodeType::RichTextWrapped(lines) => {
                // Already wrapped styled text: width is longest line, height is line count
                let width = lines
                    .iter()
                    .map(|line| {
                        line.iter()
                            .map(|span| display_width(&span.content) as u16)
                            .sum::<u16>()
                    })
                    .max()
                    .unwrap_or(0);
                let height = lines.len() as u16;
                (width, height)
            }
            RenderNodeType::Element => {
                // Element nodes calculate size based on children and layout direction
                if self.children.is_empty() {
                    return (0, 0);
                }

                let style = self.style.as_ref();
                let direction = style
                    .and_then(|s| s.direction)
                    .unwrap_or(Direction::Vertical);
                let padding = style.and_then(|s| s.padding).unwrap_or(Spacing::all(0));
                let border_size = if style
                    .and_then(|s| s.border.as_ref())
                    .is_some_and(|b| b.enabled)
                {
                    2 // 1 cell on each side
                } else {
                    0
                };

                // Check for wrapping mode and constraints
                let wrap_mode = style.and_then(|s| s.wrap);
                let gap = style.and_then(|s| s.gap).unwrap_or(0);

                // Check if we should simulate wrapping
                let should_wrap = if let Some(crate::style::WrapMode::Wrap) = wrap_mode {
                    match direction {
                        Direction::Horizontal => {
                            // Wrap horizontally if we have a fixed width constraint
                            style
                                .and_then(|s| s.width)
                                .is_some_and(|w| matches!(w, Dimension::Fixed(_)))
                        }
                        Direction::Vertical => {
                            // Wrap vertically if we have a fixed height constraint
                            style
                                .and_then(|s| s.height)
                                .is_some_and(|h| matches!(h, Dimension::Fixed(_)))
                        }
                    }
                } else {
                    false
                };

                if should_wrap {
                    // Simulate wrapping layout to calculate intrinsic size
                    self.calculate_wrapped_intrinsic_size(
                        direction,
                        padding,
                        border_size,
                        gap,
                        hint,
                    )
                } else {
                    // Standard layout calculation (no wrapping)
                    self.calculate_standard_intrinsic_size(
                        direction,
                        padding,
                        border_size,
                        gap,
                        hint,
                    )
                }
            }
        }
    }

    /// Calculate intrinsic size for standard (non-wrapped) layout.
    fn calculate_standard_intrinsic_size(
        &self,
        direction: Direction,
        padding: Spacing,
        border_size: u16,
        gap: u16,
        hint: Option<(u16, u16)>,
    ) -> (u16, u16) {
        let mut total_width = 0u16;
        let mut total_height = 0u16;
        let mut max_width = 0u16;
        let mut max_height = 0u16;
        let mut relative_children = 0u16;

        // Calculate hint to pass to children based on parent's constraints
        let child_hint = if let Some(style) = &self.style {
            match (style.width, style.height) {
                (Some(Dimension::Fixed(w)), Some(Dimension::Fixed(h))) => {
                    // Both dimensions fixed: pass content area as hint
                    let content_width =
                        w.saturating_sub(padding.left + padding.right + border_size);
                    let content_height =
                        h.saturating_sub(padding.top + padding.bottom + border_size);
                    Some((content_width, content_height))
                }
                (Some(Dimension::Fixed(w)), _) => {
                    // Width fixed: pass content width, keep height from original hint
                    let content_width =
                        w.saturating_sub(padding.left + padding.right + border_size);
                    Some((content_width, hint.map(|(_, h)| h).unwrap_or(0)))
                }
                (_, Some(Dimension::Fixed(h))) => {
                    // Height fixed: pass content height, keep width from original hint
                    let content_height =
                        h.saturating_sub(padding.top + padding.bottom + border_size);
                    Some((hint.map(|(w, _)| w).unwrap_or(0), content_height))
                }
                _ => hint, // No fixed dimensions, pass hint through
            }
        } else {
            hint
        };

        for child in &self.children {
            let child_ref = child.borrow();

            let participates_in_flow = !child_ref
                .style
                .as_ref()
                .and_then(|s| s.position)
                .is_some_and(|position| matches!(position, Position::Absolute | Position::Fixed));

            // Calculate child's size, considering hints for percentages
            let (child_width, child_height) = {
                let intrinsic = child_ref.calculate_intrinsic_size_multipass(2, child_hint);
                let mut width = intrinsic.0;
                let mut height = intrinsic.1;

                // Apply fixed dimensions if specified
                if let Some(style) = &child_ref.style {
                    if let Some(Dimension::Fixed(w)) = style.width {
                        width = w;
                    } else if let Some(Dimension::Percentage(pct)) = style.width {
                        // Use hint to resolve percentage if available
                        if let Some((hint_w, _)) = hint {
                            width = (hint_w as f32 * pct) as u16;
                        }
                    }

                    if let Some(Dimension::Fixed(h)) = style.height {
                        height = h;
                    } else if let Some(Dimension::Percentage(pct)) = style.height {
                        // Use hint to resolve percentage if available
                        if let Some((_, hint_h)) = hint {
                            height = (hint_h as f32 * pct) as u16;
                        }
                    }
                }

                (width, height)
            };

            if participates_in_flow {
                relative_children = relative_children.saturating_add(1);
            }

            match direction {
                Direction::Horizontal => {
                    total_width = total_width.saturating_add(child_width);
                    max_height = max_height.max(child_height);
                }
                Direction::Vertical => {
                    total_height = total_height.saturating_add(child_height);
                    max_width = max_width.max(child_width);
                }
            }
        }

        let gap_total = if gap > 0 && relative_children > 1 {
            gap.saturating_mul(relative_children.saturating_sub(1))
        } else {
            0
        };

        let content_width = match direction {
            Direction::Horizontal => total_width.saturating_add(gap_total),
            Direction::Vertical => max_width,
        };

        let content_height = match direction {
            Direction::Horizontal => max_height,
            Direction::Vertical => total_height.saturating_add(gap_total),
        };

        let final_width = content_width
            .saturating_add(padding.left + padding.right)
            .saturating_add(border_size);

        let final_height = content_height
            .saturating_add(padding.top + padding.bottom)
            .saturating_add(border_size);

        (final_width, final_height)
    }

    /// Calculate intrinsic size for wrapped layout.
    fn calculate_wrapped_intrinsic_size(
        &self,
        direction: Direction,
        padding: Spacing,
        border_size: u16,
        gap: u16,
        hint: Option<(u16, u16)>,
    ) -> (u16, u16) {
        // Get the fixed constraint dimension
        let constraint = match direction {
            Direction::Horizontal => {
                // For horizontal wrap, we need fixed width
                if let Some(Dimension::Fixed(w)) = self.style.as_ref().and_then(|s| s.width) {
                    w.saturating_sub(padding.left + padding.right + border_size)
                } else {
                    // Shouldn't happen due to should_wrap check, but fallback to hint or large value
                    hint.map(|(w, _)| w).unwrap_or(u16::MAX)
                }
            }
            Direction::Vertical => {
                // For vertical wrap, we need fixed height
                if let Some(Dimension::Fixed(h)) = self.style.as_ref().and_then(|s| s.height) {
                    h.saturating_sub(padding.top + padding.bottom + border_size)
                } else {
                    // Shouldn't happen due to should_wrap check, but fallback to hint or large value
                    hint.map(|(_, h)| h).unwrap_or(u16::MAX)
                }
            }
        };

        // Calculate hint to pass to children based on constraint
        let child_hint = match direction {
            Direction::Horizontal => {
                // Pass the constrained width as hint
                Some((constraint, hint.map(|(_, h)| h).unwrap_or(0)))
            }
            Direction::Vertical => {
                // Pass the constrained height as hint
                Some((hint.map(|(w, _)| w).unwrap_or(0), constraint))
            }
        };

        // Collect children sizes
        let mut child_sizes = Vec::new();
        for child in &self.children {
            let child_ref = child.borrow();
            let (child_width, child_height) = {
                let intrinsic = child_ref.calculate_intrinsic_size_multipass(2, child_hint);
                let mut width = intrinsic.0;
                let mut height = intrinsic.1;

                // Apply fixed dimensions if specified
                if let Some(style) = &child_ref.style {
                    if let Some(Dimension::Fixed(w)) = style.width {
                        width = w;
                    } else if let Some(Dimension::Percentage(pct)) = style.width
                        && let Some((hint_w, _)) = hint
                    {
                        width = (hint_w as f32 * pct) as u16;
                    }

                    if let Some(Dimension::Fixed(h)) = style.height {
                        height = h;
                    } else if let Some(Dimension::Percentage(pct)) = style.height
                        && let Some((_, hint_h)) = hint
                    {
                        height = (hint_h as f32 * pct) as u16;
                    }
                }

                (width, height)
            };
            child_sizes.push((child_width, child_height));
        }

        // Simulate wrapping layout
        match direction {
            Direction::Horizontal => {
                // Horizontal wrap: children flow left to right, wrap to new rows
                let mut rows = Vec::new();
                let mut current_row = Vec::new();
                let mut current_row_width = 0u16;

                for (child_width, child_height) in child_sizes {
                    if current_row_width > 0 && current_row_width + gap + child_width > constraint {
                        // Start new row
                        rows.push(current_row);
                        current_row = vec![(child_width, child_height)];
                        current_row_width = child_width;
                    } else {
                        if !current_row.is_empty() {
                            current_row_width += gap;
                        }
                        current_row_width += child_width;
                        current_row.push((child_width, child_height));
                    }
                }
                if !current_row.is_empty() {
                    rows.push(current_row);
                }

                // Calculate total size from rows
                let total_width = constraint; // Width is fixed
                let total_height = rows
                    .iter()
                    .map(|row| row.iter().map(|(_, h)| *h).max().unwrap_or(0))
                    .sum::<u16>()
                    + (rows.len().saturating_sub(1) as u16 * gap);

                let final_width = total_width
                    .saturating_add(padding.left + padding.right)
                    .saturating_add(border_size);

                let final_height = total_height
                    .saturating_add(padding.top + padding.bottom)
                    .saturating_add(border_size);

                (final_width, final_height)
            }
            Direction::Vertical => {
                // Vertical wrap: children flow top to bottom, wrap to new columns
                let mut columns = Vec::new();
                let mut current_column = Vec::new();
                let mut current_column_height = 0u16;

                for (child_width, child_height) in child_sizes {
                    if current_column_height > 0
                        && current_column_height + gap + child_height > constraint
                    {
                        // Start new column
                        columns.push(current_column);
                        current_column = vec![(child_width, child_height)];
                        current_column_height = child_height;
                    } else {
                        if !current_column.is_empty() {
                            current_column_height += gap;
                        }
                        current_column_height += child_height;
                        current_column.push((child_width, child_height));
                    }
                }
                if !current_column.is_empty() {
                    columns.push(current_column);
                }

                // Calculate total size from columns
                let total_width = columns
                    .iter()
                    .map(|col| col.iter().map(|(w, _)| *w).max().unwrap_or(0))
                    .sum::<u16>()
                    + (columns.len().saturating_sub(1) as u16 * gap);
                let total_height = constraint; // Height is fixed

                let final_width = total_width
                    .saturating_add(padding.left + padding.right)
                    .saturating_add(border_size);

                let final_height = total_height
                    .saturating_add(padding.top + padding.bottom)
                    .saturating_add(border_size);

                (final_width, final_height)
            }
        }
    }

    /// Applies text wrapping to a text node if needed based on width and text style.
    /// Converts Text node to TextWrapped if wrapping is enabled.
    pub fn apply_text_wrapping(&mut self, available_width: u16) {
        match &self.node_type {
            RenderNodeType::Text(text) => {
                // Only apply to single-line text nodes with text style
                if let Some(text_style) = &self.text_style
                    && let Some(wrap_mode) = text_style.wrap
                    && wrap_mode != TextWrap::None
                    && available_width > 0
                {
                    // Apply wrapping
                    let wrapped_lines = wrap_text(text, available_width, wrap_mode);

                    // Update node type and dimensions
                    self.node_type = RenderNodeType::TextWrapped(wrapped_lines.clone());
                    self.height = wrapped_lines.len() as u16;
                    self.width = wrapped_lines
                        .iter()
                        .map(|l| display_width(l))
                        .max()
                        .unwrap_or(0) as u16;
                }
            }
            RenderNodeType::RichText(spans) => {
                // Check if we have wrapping enabled in text_style
                if let Some(text_style) = &self.text_style
                    && let Some(wrap_mode) = text_style.wrap
                    && wrap_mode != TextWrap::None
                    && available_width > 0
                {
                    // Build a mapping of character positions to span indices, styles, and cursor flag
                    let mut char_to_span = Vec::new();
                    let full_text: String = spans
                        .iter()
                        .enumerate()
                        .map(|(idx, span)| {
                            // Store which span each character belongs to
                            for _ in 0..span.content.chars().count() {
                                char_to_span.push((idx, span.style.clone(), span.is_cursor));
                            }
                            span.content.as_str()
                        })
                        .collect();

                    // Apply wrapping to the full text
                    let wrapped_lines = wrap_text(&full_text, available_width, wrap_mode);

                    // Build wrapped lines with correct span information
                    let mut wrapped_styled_lines = Vec::new();
                    let mut char_offset = 0;

                    for line in wrapped_lines {
                        let mut line_spans = Vec::new();
                        let mut current_span_idx = None;
                        let mut current_content = String::new();
                        let mut current_style = None;
                        let mut current_is_cursor = false;

                        // Process each character in the line
                        for ch in line.chars() {
                            if char_offset < char_to_span.len() {
                                let (span_idx, style, is_cursor) = &char_to_span[char_offset];

                                // Check if we're starting a new span (different index, style, or cursor flag)
                                if current_span_idx != Some(*span_idx)
                                    || current_style != *style
                                    || current_is_cursor != *is_cursor
                                {
                                    // Save previous span if it exists
                                    if !current_content.is_empty() {
                                        line_spans.push(TextSpan {
                                            content: current_content.clone(),
                                            style: current_style.clone(),
                                            is_cursor: current_is_cursor,
                                        });
                                    }
                                    // Start new span
                                    current_content = String::new();
                                    current_span_idx = Some(*span_idx);
                                    current_style = style.clone();
                                    current_is_cursor = *is_cursor;
                                }

                                current_content.push(ch);
                            }
                            char_offset += 1;
                        }

                        // Add the last span in the line
                        if !current_content.is_empty() {
                            line_spans.push(TextSpan {
                                content: current_content,
                                style: current_style,
                                is_cursor: current_is_cursor,
                            });
                        }

                        if !line_spans.is_empty() {
                            wrapped_styled_lines.push(line_spans);
                        }
                    }

                    // Update node type and dimensions
                    if !wrapped_styled_lines.is_empty() {
                        self.height = wrapped_styled_lines.len() as u16;
                        self.width = wrapped_styled_lines
                            .iter()
                            .map(|line| {
                                line.iter()
                                    .map(|span| display_width(&span.content) as u16)
                                    .sum::<u16>()
                            })
                            .max()
                            .unwrap_or(0);
                        self.node_type = RenderNodeType::RichTextWrapped(wrapped_styled_lines);
                    }
                }
            }
            _ => {}
        }
    }

    /// Performs layout calculation for this node and its children.
    ///
    /// Layout determines the position of child nodes based on
    /// the layout direction (vertical or horizontal) and padding.
    pub fn layout(&mut self) {
        match &self.style {
            Some(style) => {
                let direction = style.direction.unwrap_or(Direction::Vertical);
                self.layout_children(direction);
            }
            None => {
                self.layout_children(Direction::Vertical);
            }
        }
    }

    /// Performs layout calculation with parent dimensions for percentage resolution.
    ///
    /// This method resolves percentage-based dimensions before laying out children.
    pub fn layout_with_parent(&mut self, parent_width: u16, parent_height: u16) {
        // First, calculate intrinsic size if we need it
        let (intrinsic_width, intrinsic_height) = self.calculate_intrinsic_size();

        // Resolve percentage and fixed dimensions first (auto handled in layout_children_with_parent)
        if let Some(style) = &self.style {
            // Resolve width
            match style.width {
                Some(Dimension::Percentage(pct)) => {
                    // Calculate percentage of parent width
                    let calculated_width = (parent_width as f32 * pct) as u16;
                    // Ensure at least 1 cell width
                    self.width = calculated_width.max(1);
                }
                Some(Dimension::Fixed(w)) => {
                    self.width = w;
                }
                Some(Dimension::Content) => {
                    // Use intrinsic width, but cap at parent width
                    self.width = intrinsic_width.min(parent_width);
                }
                Some(Dimension::Auto) => {
                    // Auto should have been resolved by parent's layout
                    // Don't override if already set
                }
                None => {
                    // If no width specified, use intrinsic size (content-based)
                    // UNLESS this is a text node with alignment that was already given width by parent
                    let has_alignment = self.text_style.as_ref().and_then(|ts| ts.align).is_some();

                    if has_alignment && self.width >= intrinsic_width {
                        // Text with alignment already has width from parent, don't override
                    } else {
                        self.width = intrinsic_width.min(parent_width);
                    }
                }
            }

            // Resolve height
            match style.height {
                Some(Dimension::Percentage(pct)) => {
                    // Calculate percentage of parent height
                    let calculated_height = (parent_height as f32 * pct) as u16;
                    // Ensure at least 1 cell height
                    self.height = calculated_height.max(1);
                }
                Some(Dimension::Fixed(h)) => {
                    self.height = h;
                }
                Some(Dimension::Content) => {
                    // Use intrinsic height, but cap at parent height
                    self.height = intrinsic_height.min(parent_height);
                }
                Some(Dimension::Auto) => {
                    // Auto should have been resolved by parent's layout
                    // Don't override if already set
                }
                None => {
                    // If no height specified, use intrinsic size (content-based)
                    self.height = intrinsic_height.min(parent_height);
                }
            }
        } else {
            // No style - use intrinsic (content) size
            // UNLESS this is a text node with alignment that was already given width by parent
            let has_alignment = self.text_style.as_ref().and_then(|ts| ts.align).is_some();

            if has_alignment && self.width >= intrinsic_width {
                // Text with alignment already has width from parent, don't override
            } else {
                self.width = intrinsic_width.min(parent_width);
            }
            self.height = intrinsic_height.min(parent_height);
        }

        // Apply text wrapping if this is a text node with wrapping enabled
        // Use the node's own width (which may have been set to Fixed) as the constraint
        // Note: Skip if already wrapped (TextWrapped or RichTextWrapped)
        if matches!(
            self.node_type,
            RenderNodeType::Text(_) | RenderNodeType::RichText(_)
        ) {
            // If we have a fixed width, use that; otherwise use parent width
            let wrap_width = if let Some(style) = &self.style {
                match style.width {
                    Some(Dimension::Fixed(w)) => w,
                    _ => self.width.min(parent_width),
                }
            } else {
                self.width.min(parent_width)
            };
            self.apply_text_wrapping(wrap_width);
        }

        // Now layout children with resolved dimensions
        let direction = self
            .style
            .as_ref()
            .and_then(|s| s.direction)
            .unwrap_or(Direction::Vertical);
        self.layout_children_with_parent(direction);
    }

    /// Lays out child nodes according to the specified direction.
    ///
    /// ## Vertical Layout
    /// ```text
    /// ┌──────────┐
    /// │ Child 1  │ ← y = parent.y + padding.top
    /// │──────────│
    /// │ Child 2  │ ← y = child1.y + child1.height
    /// │──────────│
    /// │ Child 3  │ ← y = child2.y + child2.height
    /// └──────────┘
    /// ```
    ///
    /// ## Horizontal Layout
    /// ```text
    /// ┌──────┬──────┬──────┐
    /// │ Ch1  │ Ch2  │ Ch3  │
    /// └──────┴──────┴──────┘
    ///    ↑      ↑      ↑
    ///   x=0    x=6    x=12
    /// ```
    fn layout_children(&mut self, direction: Direction) {
        let padding = self
            .style
            .as_ref()
            .and_then(|s| s.padding)
            .unwrap_or(Spacing::all(0));

        // Check if border is enabled and adjust content area accordingly
        let border_offset = if self
            .style
            .as_ref()
            .and_then(|s| s.border.as_ref())
            .is_some_and(|b| b.enabled)
        {
            1
        } else {
            0
        };

        let mut offset = 0u16;

        for child in &self.children {
            let mut child_ref = child.borrow_mut();

            match direction {
                Direction::Vertical => {
                    child_ref.set_position(
                        self.x + padding.left + border_offset,
                        self.y + padding.top + border_offset + offset,
                    );
                    offset += child_ref.height;
                }
                Direction::Horizontal => {
                    child_ref.set_position(
                        self.x + padding.left + border_offset + offset,
                        self.y + padding.top + border_offset,
                    );
                    offset += child_ref.width;
                }
            }

            child_ref.layout();
        }
    }

    /// Lays out child nodes with wrapping enabled.
    fn layout_children_with_wrap(
        &mut self,
        direction: Direction,
        content_width: u16,
        content_height: u16,
        padding: Spacing,
        border_offset: u16,
        gap: u16,
    ) {
        let start_x = self.x + padding.left + border_offset;
        let start_y = self.y + padding.top + border_offset;

        // Get alignment settings
        let justify_content = self
            .style
            .as_ref()
            .and_then(|s| s.justify_content)
            .unwrap_or(JustifyContent::Start);

        let align_items = self
            .style
            .as_ref()
            .and_then(|s| s.align_items)
            .unwrap_or(AlignItems::Start);

        match direction {
            Direction::Horizontal => {
                // Horizontal wrapping: items flow left to right, wrap to next row

                // First pass: Calculate dimensions and group into rows
                struct RowInfo {
                    start_index: usize,
                    end_index: usize,
                    width: u16, // Total width of items WITHOUT gaps
                    height: u16,
                }

                let mut rows = Vec::new();
                let mut current_row_width = 0u16; // Width without gaps
                let mut current_row_width_with_gaps = 0u16; // Width including gaps for fitting check
                let mut current_row_height = 0u16;
                let mut row_start_index = 0;

                // Resolve all child dimensions first
                for child in &self.children {
                    let mut child_ref = child.borrow_mut();
                    child_ref.layout_with_parent(content_width, content_height);
                }

                // Group children into rows
                for (i, child) in self.children.iter().enumerate() {
                    let child_ref = child.borrow();
                    let child_width = child_ref.width;
                    let child_height = child_ref.height;

                    // Check if child fits in current row (considering gaps)
                    let width_if_added = if current_row_width > 0 {
                        current_row_width_with_gaps + gap + child_width
                    } else {
                        child_width
                    };

                    if current_row_width > 0 && width_if_added > content_width {
                        // Save current row and start new one
                        rows.push(RowInfo {
                            start_index: row_start_index,
                            end_index: i,
                            width: current_row_width, // Store width WITHOUT gaps
                            height: current_row_height,
                        });

                        row_start_index = i;
                        current_row_width = child_width;
                        current_row_width_with_gaps = child_width;
                        current_row_height = child_height;
                    } else {
                        // Add to current row
                        current_row_width += child_width;
                        current_row_width_with_gaps = width_if_added;
                        current_row_height = current_row_height.max(child_height);
                    }
                }

                // Don't forget the last row
                if row_start_index < self.children.len() {
                    rows.push(RowInfo {
                        start_index: row_start_index,
                        end_index: self.children.len(),
                        width: current_row_width, // Store width WITHOUT gaps
                        height: current_row_height,
                    });
                }

                // Second pass: Position children with alignment
                let mut current_y = start_y;

                for row in &rows {
                    // Calculate horizontal positioning for this row based on justify_content
                    let row_item_count = row.end_index - row.start_index;
                    // Calculate total width including gaps
                    let total_gaps_width = if row_item_count > 1 {
                        gap * (row_item_count as u16 - 1)
                    } else {
                        0
                    };
                    let row_width_with_gaps = row.width + total_gaps_width;
                    let available_width = content_width.saturating_sub(row_width_with_gaps);

                    // Calculate starting X and spacing for this row
                    let (row_start_x, item_spacing) = match justify_content {
                        JustifyContent::Start => (start_x, gap),
                        JustifyContent::End => (start_x + available_width, gap),
                        JustifyContent::Center => (start_x + available_width / 2, gap),
                        JustifyContent::SpaceBetween => {
                            if row_item_count > 1 {
                                let total_gaps = row_item_count - 1;
                                let spacing =
                                    (available_width + gap * total_gaps as u16) / total_gaps as u16;
                                (start_x, spacing)
                            } else {
                                (start_x, gap)
                            }
                        }
                        JustifyContent::SpaceAround => {
                            if row_item_count > 0 {
                                let spacing = available_width / row_item_count as u16;
                                (start_x + spacing / 2, gap + spacing)
                            } else {
                                (start_x, gap)
                            }
                        }
                        JustifyContent::SpaceEvenly => {
                            if row_item_count > 0 {
                                let spacing = available_width / (row_item_count as u16 + 1);
                                (start_x + spacing, gap + spacing)
                            } else {
                                (start_x, gap)
                            }
                        }
                    };

                    // Position each child in this row
                    let mut current_x = row_start_x;

                    for i in row.start_index..row.end_index {
                        let mut child_ref = self.children[i].borrow_mut();

                        // Apply AlignItems for vertical positioning within the row
                        let child_align = child_ref
                            .style
                            .as_ref()
                            .and_then(|s| s.align_self)
                            .unwrap_or(AlignSelf::Auto);

                        let effective_align = match child_align {
                            AlignSelf::Auto => align_items,
                            AlignSelf::Start => AlignItems::Start,
                            AlignSelf::Center => AlignItems::Center,
                            AlignSelf::End => AlignItems::End,
                        };

                        let y_position = match effective_align {
                            AlignItems::Start => current_y,
                            AlignItems::Center => {
                                let child_space = row.height.saturating_sub(child_ref.height);
                                current_y + (child_space / 2)
                            }
                            AlignItems::End => {
                                let child_space = row.height.saturating_sub(child_ref.height);
                                current_y + child_space
                            }
                        };

                        child_ref.set_position(current_x, y_position);
                        current_x += child_ref.width;

                        // Add spacing after each item except the last in row
                        if i < row.end_index - 1 {
                            current_x += item_spacing;
                        }
                    }

                    // Move to next row
                    current_y += row.height + gap;
                }
            }
            Direction::Vertical => {
                // Vertical wrapping: items flow top to bottom, wrap to next column

                // First pass: Calculate dimensions and group into columns
                struct ColInfo {
                    start_index: usize,
                    end_index: usize,
                    width: u16,
                    height: u16, // Total height of items WITHOUT gaps
                }

                let mut cols = Vec::new();
                let mut current_col_width = 0u16;
                let mut current_col_height = 0u16; // Height without gaps
                let mut current_col_height_with_gaps = 0u16; // Height including gaps for fitting check
                let mut col_start_index = 0;

                // Resolve all child dimensions first
                for child in &self.children {
                    let mut child_ref = child.borrow_mut();
                    child_ref.layout_with_parent(content_width, content_height);
                }

                // Group children into columns
                for (i, child) in self.children.iter().enumerate() {
                    let child_ref = child.borrow();
                    let child_width = child_ref.width;
                    let child_height = child_ref.height;

                    // Check if child fits in current column (considering gaps)
                    let height_if_added = if current_col_height > 0 {
                        current_col_height_with_gaps + gap + child_height
                    } else {
                        child_height
                    };

                    if current_col_height > 0 && height_if_added > content_height {
                        // Save current column and start new one
                        cols.push(ColInfo {
                            start_index: col_start_index,
                            end_index: i,
                            width: current_col_width,
                            height: current_col_height, // Store height WITHOUT gaps
                        });

                        col_start_index = i;
                        current_col_width = child_width;
                        current_col_height = child_height;
                        current_col_height_with_gaps = child_height;
                    } else {
                        // Add to current column
                        current_col_height += child_height;
                        current_col_height_with_gaps = height_if_added;
                        current_col_width = current_col_width.max(child_width);
                    }
                }

                // Don't forget the last column
                if col_start_index < self.children.len() {
                    cols.push(ColInfo {
                        start_index: col_start_index,
                        end_index: self.children.len(),
                        width: current_col_width,
                        height: current_col_height, // Store height WITHOUT gaps
                    });
                }

                // Second pass: Position children with alignment
                let mut current_x = start_x;

                for col in &cols {
                    // Calculate vertical positioning for this column based on justify_content
                    let col_item_count = col.end_index - col.start_index;
                    // Calculate total height including gaps
                    let total_gaps_height = if col_item_count > 1 {
                        gap * (col_item_count as u16 - 1)
                    } else {
                        0
                    };
                    let col_height_with_gaps = col.height + total_gaps_height;
                    let available_height = content_height.saturating_sub(col_height_with_gaps);

                    // Calculate starting Y and spacing for this column
                    let (col_start_y, item_spacing) = match justify_content {
                        JustifyContent::Start => (start_y, gap),
                        JustifyContent::End => (start_y + available_height, gap),
                        JustifyContent::Center => (start_y + available_height / 2, gap),
                        JustifyContent::SpaceBetween => {
                            if col_item_count > 1 {
                                let total_gaps = col_item_count - 1;
                                let spacing = (available_height + gap * total_gaps as u16)
                                    / total_gaps as u16;
                                (start_y, spacing)
                            } else {
                                (start_y, gap)
                            }
                        }
                        JustifyContent::SpaceAround => {
                            if col_item_count > 0 {
                                let spacing = available_height / col_item_count as u16;
                                (start_y + spacing / 2, gap + spacing)
                            } else {
                                (start_y, gap)
                            }
                        }
                        JustifyContent::SpaceEvenly => {
                            if col_item_count > 0 {
                                let spacing = available_height / (col_item_count as u16 + 1);
                                (start_y + spacing, gap + spacing)
                            } else {
                                (start_y, gap)
                            }
                        }
                    };

                    // Position each child in this column
                    let mut current_y = col_start_y;

                    for i in col.start_index..col.end_index {
                        let mut child_ref = self.children[i].borrow_mut();

                        // Apply AlignItems for horizontal positioning within the column
                        let child_align = child_ref
                            .style
                            .as_ref()
                            .and_then(|s| s.align_self)
                            .unwrap_or(AlignSelf::Auto);

                        let effective_align = match child_align {
                            AlignSelf::Auto => align_items,
                            AlignSelf::Start => AlignItems::Start,
                            AlignSelf::Center => AlignItems::Center,
                            AlignSelf::End => AlignItems::End,
                        };

                        let x_position = match effective_align {
                            AlignItems::Start => current_x,
                            AlignItems::Center => {
                                let child_space = col.width.saturating_sub(child_ref.width);
                                current_x + (child_space / 2)
                            }
                            AlignItems::End => {
                                let child_space = col.width.saturating_sub(child_ref.width);
                                current_x + child_space
                            }
                        };

                        child_ref.set_position(x_position, current_y);
                        current_y += child_ref.height;

                        // Add spacing after each item except the last in column
                        if i < col.end_index - 1 {
                            current_y += item_spacing;
                        }
                    }

                    // Move to next column
                    current_x += col.width + gap;
                }
            }
        }

        // Layout children of each child
        for child in &self.children {
            let mut child_ref = child.borrow_mut();
            if let Some(child_style) = &child_ref.style {
                let child_direction = child_style.direction.unwrap_or(Direction::Vertical);
                child_ref.layout_children_with_parent(child_direction);
            } else {
                child_ref.layout_children_with_parent(Direction::Vertical);
            }
        }
    }

    /// Lays out child nodes with parent dimension context for percentage resolution.
    pub(crate) fn layout_children_with_parent(&mut self, direction: Direction) {
        let padding = self
            .style
            .as_ref()
            .and_then(|s| s.padding)
            .unwrap_or(Spacing::all(0));

        // Check if border is enabled and adjust content area accordingly
        let border_offset = if self
            .style
            .as_ref()
            .and_then(|s| s.border.as_ref())
            .is_some_and(|b| b.enabled)
        {
            1
        } else {
            0
        };

        // Calculate content box dimensions (after padding and border)
        let content_width = self
            .width
            .saturating_sub(padding.left + padding.right + (border_offset * 2));
        let content_height = self
            .height
            .saturating_sub(padding.top + padding.bottom + (border_offset * 2));

        // Check if wrapping is enabled
        let wrap_mode = self.style.as_ref().and_then(|s| s.wrap);
        let gap = self.style.as_ref().and_then(|s| s.gap).unwrap_or(0);

        // If wrapping is enabled, use wrapping layout
        if let Some(crate::style::WrapMode::Wrap) = wrap_mode {
            self.layout_children_with_wrap(
                direction,
                content_width,
                content_height,
                padding,
                border_offset,
                gap,
            );
            return;
        }

        // First pass: Identify child types and calculate fixed/percentage sizes
        let mut absolute_children = Vec::new();
        let mut auto_children = Vec::new();
        let mut used_space = 0u16;
        let mut child_sizes = Vec::new();

        for (index, child) in self.children.iter().enumerate() {
            let mut child_ref = child.borrow_mut();

            // Extract position info from style
            let (position_type, z_index) = if let Some(style) = &child_ref.style {
                (
                    style.position.unwrap_or(Position::Relative),
                    style.z_index.unwrap_or(0),
                )
            } else {
                (Position::Relative, 0)
            };

            child_ref.position_type = position_type;
            child_ref.z_index = z_index;

            // Skip absolute/fixed positioned children in normal flow
            if matches!(
                child_ref.position_type,
                Position::Absolute | Position::Fixed
            ) {
                absolute_children.push(index);
                child_sizes.push(0);
                continue;
            }

            // Apply text wrapping early for text/richtext nodes if they have wrapping enabled
            // This must happen before size calculation to get correct heights
            if matches!(
                child_ref.node_type,
                RenderNodeType::Text(_) | RenderNodeType::RichText(_)
            ) && let Some(text_style) = &child_ref.text_style
                && let Some(wrap_mode) = text_style.wrap
                && wrap_mode != TextWrap::None
            {
                // Determine the available width for wrapping
                let wrap_width = if let Some(style) = &child_ref.style {
                    match style.width {
                        Some(Dimension::Fixed(w)) => w,
                        Some(Dimension::Percentage(pct)) => (content_width as f32 * pct) as u16,
                        _ => content_width,
                    }
                } else {
                    content_width
                };
                child_ref.apply_text_wrapping(wrap_width);
            }

            // Determine child size based on dimension type
            let dimension = match direction {
                Direction::Vertical => child_ref.style.as_ref().and_then(|s| s.height),
                Direction::Horizontal => child_ref.style.as_ref().and_then(|s| s.width),
            };

            let child_size = match dimension {
                Some(Dimension::Fixed(size)) => {
                    used_space = used_space.saturating_add(size);
                    size
                }
                Some(Dimension::Percentage(pct)) => {
                    let parent_size = match direction {
                        Direction::Vertical => content_height,
                        Direction::Horizontal => content_width,
                    };
                    let size = (parent_size as f32 * pct) as u16;
                    used_space = used_space.saturating_add(size);
                    size
                }
                Some(Dimension::Content) => {
                    // Calculate intrinsic size for content-based dimension
                    let (intrinsic_w, intrinsic_h) = child_ref.calculate_intrinsic_size();
                    let size = match direction {
                        Direction::Horizontal => intrinsic_w,
                        Direction::Vertical => intrinsic_h,
                    };
                    used_space = used_space.saturating_add(size);
                    size
                }
                Some(Dimension::Auto) => {
                    auto_children.push(index);
                    // For text nodes with auto sizing, use content size
                    match &child_ref.node_type {
                        RenderNodeType::Text(text) => match direction {
                            Direction::Horizontal => {
                                let size = display_width(text) as u16;
                                used_space = used_space.saturating_add(size);
                                size
                            }
                            Direction::Vertical => {
                                used_space = used_space.saturating_add(1);
                                1
                            }
                        },
                        RenderNodeType::RichText(spans) => match direction {
                            Direction::Horizontal => {
                                let size: u16 = spans
                                    .iter()
                                    .map(|span| display_width(&span.content) as u16)
                                    .sum();
                                used_space = used_space.saturating_add(size);
                                size
                            }
                            Direction::Vertical => {
                                used_space = used_space.saturating_add(1);
                                1
                            }
                        },
                        RenderNodeType::TextWrapped(lines) => match direction {
                            Direction::Horizontal => {
                                let size = lines.iter().map(|l| display_width(l)).max().unwrap_or(0)
                                    as u16;
                                used_space = used_space.saturating_add(size);
                                size
                            }
                            Direction::Vertical => {
                                let size = lines.len() as u16;
                                used_space = used_space.saturating_add(size);
                                size
                            }
                        },
                        RenderNodeType::RichTextWrapped(lines) => match direction {
                            Direction::Horizontal => {
                                let size = lines
                                    .iter()
                                    .map(|line| {
                                        line.iter()
                                            .map(|span| display_width(&span.content) as u16)
                                            .sum::<u16>()
                                    })
                                    .max()
                                    .unwrap_or(0);
                                used_space = used_space.saturating_add(size);
                                size
                            }
                            Direction::Vertical => {
                                let size = lines.len() as u16;
                                used_space = used_space.saturating_add(size);
                                size
                            }
                        },
                        _ => 0, // Will be calculated in second pass
                    }
                }
                None => {
                    // If no dimension specified, use content-based sizing
                    let (intrinsic_w, intrinsic_h) = child_ref.calculate_intrinsic_size();
                    let size = match direction {
                        Direction::Horizontal => intrinsic_w,
                        Direction::Vertical => intrinsic_h,
                    };
                    used_space = used_space.saturating_add(size);
                    size
                }
            };

            child_sizes.push(child_size);
        }

        // Second pass: Calculate auto sizes
        let available_space = match direction {
            Direction::Vertical => content_height.saturating_sub(used_space),
            Direction::Horizontal => content_width.saturating_sub(used_space),
        };

        let auto_size = if !auto_children.is_empty() {
            available_space / auto_children.len() as u16
        } else {
            0
        };

        // Update auto-sized children
        for &index in &auto_children {
            let is_text = {
                let child_ref = self.children[index].borrow();
                matches!(
                    child_ref.node_type,
                    RenderNodeType::Text(_)
                        | RenderNodeType::TextWrapped(_)
                        | RenderNodeType::RichText(_)
                        | RenderNodeType::RichTextWrapped(_)
                )
            };
            // Skip text nodes as they already have their size
            if !is_text {
                child_sizes[index] = auto_size;
            }
        }

        // Calculate total space used by children and gaps
        let relative_children_count = self.children.len() - absolute_children.len();
        let total_gaps = if relative_children_count > 1 {
            gap * (relative_children_count as u16 - 1)
        } else {
            0
        };

        // Calculate total size of relative children in main axis
        let total_children_size: u16 = child_sizes
            .iter()
            .enumerate()
            .filter(|(i, _)| !absolute_children.contains(i))
            .map(|(_, size)| *size)
            .sum();

        let total_used_space = total_children_size + total_gaps;

        // Get justify content setting
        let justify_content = self
            .style
            .as_ref()
            .and_then(|s| s.justify_content)
            .unwrap_or(JustifyContent::Start);

        // Calculate starting offset and spacing based on JustifyContent
        let (mut offset, item_spacing) = match direction {
            Direction::Vertical => {
                let available_space = content_height.saturating_sub(total_used_space);
                calculate_justify_offsets(
                    justify_content,
                    available_space,
                    relative_children_count,
                    gap,
                )
            }
            Direction::Horizontal => {
                let available_space = content_width.saturating_sub(total_used_space);
                calculate_justify_offsets(
                    justify_content,
                    available_space,
                    relative_children_count,
                    gap,
                )
            }
        };

        // Third pass: Position and layout all children
        for (index, child) in self.children.iter().enumerate() {
            let mut child_ref = child.borrow_mut();

            // Skip absolute/fixed positioned children
            if absolute_children.contains(&index) {
                continue;
            }

            // Set child dimensions based on calculated sizes
            match direction {
                Direction::Vertical => {
                    child_ref.height = child_sizes[index];
                    // Set width for the child (respecting its own width setting)
                    if let Some(style) = &child_ref.style {
                        match style.width {
                            Some(Dimension::Fixed(w)) => child_ref.width = w,
                            Some(Dimension::Percentage(pct)) => {
                                child_ref.width = (content_width as f32 * pct) as u16;
                            }
                            Some(Dimension::Content) => {
                                // Content-based width
                                let (intrinsic_w, _) = child_ref.calculate_intrinsic_size();
                                child_ref.width = intrinsic_w.min(content_width);
                            }
                            Some(Dimension::Auto) => {
                                // Auto in perpendicular direction means fill available space
                                match &child_ref.node_type {
                                    RenderNodeType::Text(text) => {
                                        // If text has alignment, fill parent width for alignment to work
                                        if child_ref
                                            .text_style
                                            .as_ref()
                                            .and_then(|ts| ts.align)
                                            .is_some()
                                        {
                                            child_ref.width = content_width;
                                        } else {
                                            child_ref.width = display_width(text) as u16;
                                        }
                                    }
                                    RenderNodeType::RichText(spans) => {
                                        // If RichText has alignment, fill parent width for alignment to work
                                        let has_alignment =
                                            child_ref.text_style.as_ref().and_then(|ts| ts.align);
                                        if has_alignment.is_some() {
                                            child_ref.width = content_width;
                                        } else {
                                            child_ref.width = spans
                                                .iter()
                                                .map(|span| display_width(&span.content) as u16)
                                                .sum();
                                        }
                                    }
                                    RenderNodeType::TextWrapped(lines) => {
                                        // If wrapped text has alignment, fill parent width for alignment to work
                                        if child_ref
                                            .text_style
                                            .as_ref()
                                            .and_then(|ts| ts.align)
                                            .is_some()
                                        {
                                            child_ref.width = content_width;
                                        } else {
                                            child_ref.width = lines
                                                .iter()
                                                .map(|l| display_width(l))
                                                .max()
                                                .unwrap_or(0)
                                                as u16;
                                        }
                                    }
                                    RenderNodeType::RichTextWrapped(lines) => {
                                        // If wrapped RichText has alignment, fill parent width for alignment to work
                                        if child_ref
                                            .text_style
                                            .as_ref()
                                            .and_then(|ts| ts.align)
                                            .is_some()
                                        {
                                            child_ref.width = content_width;
                                        } else {
                                            child_ref.width = lines
                                                .iter()
                                                .map(|line| {
                                                    line.iter()
                                                        .map(|span| {
                                                            display_width(&span.content) as u16
                                                        })
                                                        .sum::<u16>()
                                                })
                                                .max()
                                                .unwrap_or(0);
                                        }
                                    }
                                    _ => {
                                        child_ref.width = content_width;
                                    }
                                }
                            }
                            None => {
                                // None means use content-based sizing
                                // UNLESS this is a text node with alignment, then use full available width
                                let has_alignment =
                                    child_ref.text_style.as_ref().and_then(|ts| ts.align);

                                if has_alignment.is_some() {
                                    // Text with alignment needs full width to align within
                                    child_ref.width = content_width;
                                } else {
                                    let (intrinsic_w, _) = child_ref.calculate_intrinsic_size();
                                    child_ref.width = intrinsic_w.min(content_width);
                                }
                            }
                        }
                    } else {
                        // No style - use intrinsic width
                        // UNLESS this is a text node with alignment, then use full available width
                        let has_alignment = child_ref.text_style.as_ref().and_then(|ts| ts.align);

                        if has_alignment.is_some() {
                            // Text with alignment needs full width to align within
                            child_ref.width = content_width;
                        } else {
                            let (intrinsic_w, _) = child_ref.calculate_intrinsic_size();
                            child_ref.width = intrinsic_w.min(content_width);
                        }
                    }

                    // Apply AlignItems for cross-axis alignment (horizontal axis in vertical layout)
                    let align_items = self
                        .style
                        .as_ref()
                        .and_then(|s| s.align_items)
                        .unwrap_or(AlignItems::Start);

                    // Check if child overrides with align_self
                    let child_align = child_ref
                        .style
                        .as_ref()
                        .and_then(|s| s.align_self)
                        .unwrap_or(AlignSelf::Auto);

                    let effective_align = match child_align {
                        AlignSelf::Auto => align_items,
                        AlignSelf::Start => AlignItems::Start,
                        AlignSelf::Center => AlignItems::Center,
                        AlignSelf::End => AlignItems::End,
                    };

                    let x_position = match effective_align {
                        AlignItems::Start => self.x + padding.left + border_offset,
                        AlignItems::Center => {
                            let child_space = content_width.saturating_sub(child_ref.width);
                            self.x + padding.left + border_offset + (child_space / 2)
                        }
                        AlignItems::End => {
                            let child_space = content_width.saturating_sub(child_ref.width);
                            self.x + padding.left + border_offset + child_space
                        }
                    };

                    child_ref
                        .set_position(x_position, self.y + padding.top + border_offset + offset);
                    offset += child_sizes[index];
                    // Add spacing after each child based on justify mode
                    // For SpaceBetween, add spacing after all children except the last
                    // For SpaceAround and SpaceEvenly, add spacing after all children
                    // For Start, Center, End, use regular gap spacing
                    let is_last_relative_child = {
                        // Find if this is the last non-absolute child
                        let mut last_idx = index;
                        for i in (index + 1)..self.children.len() {
                            if !absolute_children.contains(&i) {
                                last_idx = i;
                            }
                        }
                        last_idx == index
                    };

                    // Add spacing based on justify mode
                    if !is_last_relative_child {
                        offset += item_spacing;
                    } else if matches!(
                        justify_content,
                        JustifyContent::SpaceAround | JustifyContent::SpaceEvenly
                    ) {
                        // These modes need spacing after the last item too
                        offset += item_spacing;
                    }
                }
                Direction::Horizontal => {
                    // Set width from calculated size (includes auto-sizing)
                    child_ref.width = child_sizes[index];

                    // Set height for the child (respecting its own height setting)
                    if let Some(style) = &child_ref.style {
                        match style.height {
                            Some(Dimension::Fixed(h)) => child_ref.height = h,
                            Some(Dimension::Percentage(pct)) => {
                                child_ref.height = (content_height as f32 * pct) as u16;
                            }
                            Some(Dimension::Content) => {
                                // Content-based height
                                let (_, intrinsic_h) = child_ref.calculate_intrinsic_size();
                                child_ref.height = intrinsic_h.min(content_height);
                            }
                            Some(Dimension::Auto) => match &child_ref.node_type {
                                RenderNodeType::Text(_) | RenderNodeType::RichText(_) => {
                                    child_ref.height = 1;
                                }
                                RenderNodeType::TextWrapped(lines) => {
                                    child_ref.height = lines.len() as u16;
                                }
                                RenderNodeType::RichTextWrapped(lines) => {
                                    child_ref.height = lines.len() as u16;
                                }
                                _ => {
                                    child_ref.height = content_height;
                                }
                            },
                            None => {
                                // None means use content-based sizing
                                let (_, intrinsic_h) = child_ref.calculate_intrinsic_size();
                                child_ref.height = intrinsic_h.min(content_height);
                            }
                        }
                    } else {
                        // No style - use intrinsic height
                        let (_, intrinsic_h) = child_ref.calculate_intrinsic_size();
                        child_ref.height = intrinsic_h.min(content_height);
                    }

                    // Apply AlignItems for cross-axis alignment (vertical axis in horizontal layout)
                    let align_items = self
                        .style
                        .as_ref()
                        .and_then(|s| s.align_items)
                        .unwrap_or(AlignItems::Start);

                    // Check if child overrides with align_self
                    let child_align = child_ref
                        .style
                        .as_ref()
                        .and_then(|s| s.align_self)
                        .unwrap_or(AlignSelf::Auto);

                    let effective_align = match child_align {
                        AlignSelf::Auto => align_items,
                        AlignSelf::Start => AlignItems::Start,
                        AlignSelf::Center => AlignItems::Center,
                        AlignSelf::End => AlignItems::End,
                    };

                    let y_position = match effective_align {
                        AlignItems::Start => self.y + padding.top + border_offset,
                        AlignItems::Center => {
                            let child_space = content_height.saturating_sub(child_ref.height);
                            self.y + padding.top + border_offset + (child_space / 2)
                        }
                        AlignItems::End => {
                            let child_space = content_height.saturating_sub(child_ref.height);
                            self.y + padding.top + border_offset + child_space
                        }
                    };

                    child_ref
                        .set_position(self.x + padding.left + border_offset + offset, y_position);
                    offset += child_sizes[index];
                    // Add spacing after each child based on justify mode
                    // For SpaceBetween, add spacing after all children except the last
                    // For SpaceAround and SpaceEvenly, add spacing after all children
                    // For Start, Center, End, use regular gap spacing
                    let is_last_relative_child = {
                        // Find if this is the last non-absolute child
                        let mut last_idx = index;
                        for i in (index + 1)..self.children.len() {
                            if !absolute_children.contains(&i) {
                                last_idx = i;
                            }
                        }
                        last_idx == index
                    };

                    // Add spacing based on justify mode
                    if !is_last_relative_child {
                        offset += item_spacing;
                    } else if matches!(
                        justify_content,
                        JustifyContent::SpaceAround | JustifyContent::SpaceEvenly
                    ) {
                        // These modes need spacing after the last item too
                        offset += item_spacing;
                    }
                }
            }

            // Layout child's children
            child_ref.layout_with_parent(content_width, content_height);
        }

        // Second pass: position absolute/fixed children
        for index in absolute_children {
            let child = &self.children[index];
            let mut child_ref = child.borrow_mut();

            match child_ref.position_type {
                Position::Fixed => {
                    // Fixed positioning: relative to viewport (0, 0)
                    self.position_absolute_child(
                        &mut child_ref,
                        0,
                        0,
                        content_width,
                        content_height,
                    );
                }
                Position::Absolute => {
                    // Absolute positioning: relative to this container
                    self.position_absolute_child(
                        &mut child_ref,
                        self.x,
                        self.y,
                        self.width,
                        self.height,
                    );
                }
                _ => {} // Already handled
            }

            // Layout the absolutely positioned child
            child_ref.layout_with_parent(content_width, content_height);
        }

        // Track content dimensions for scrolling
        self.calculate_content_dimensions();

        // Set scrollable flag based on overflow style
        if let Some(style) = &self.style {
            match style.overflow {
                Some(Overflow::Scroll) | Some(Overflow::Auto) => {
                    self.scrollable = true;
                    // Make scrollable elements focusable by default
                    if !self.focusable && self.events.on_click.is_none() {
                        self.focusable = true;
                    }
                }
                _ => {
                    self.scrollable = false;
                }
            }
        }
    }

    /// Calculates the actual content dimensions (may exceed container bounds).
    /// This is used to determine scrollable area.
    fn calculate_content_dimensions(&mut self) {
        if self.children.is_empty() {
            // For leaf nodes, content dimensions equal node dimensions
            self.content_width = self.width;
            self.content_height = self.height;
            return;
        }

        // Get padding values to account for them in content dimensions
        let padding = self
            .style
            .as_ref()
            .and_then(|s| s.padding)
            .unwrap_or(Spacing::all(0));

        // Check if border is enabled
        let border_offset = if self
            .style
            .as_ref()
            .and_then(|s| s.border.as_ref())
            .is_some_and(|b| b.enabled)
        {
            1
        } else {
            0
        };

        // Find the maximum extent of all children
        let mut max_x = 0u16;
        let mut max_y = 0u16;

        for child in &self.children {
            let child_ref = child.borrow();
            // Skip absolute/fixed positioned children as they don't affect content size
            if matches!(
                child_ref.position_type,
                Position::Absolute | Position::Fixed
            ) {
                continue;
            }

            let child_right = child_ref.x + child_ref.width;
            let child_bottom = child_ref.y + child_ref.height;

            // Update max extents relative to this node's position
            if child_ref.x >= self.x && child_right > self.x {
                max_x = max_x.max(child_right - self.x);
            }
            if child_ref.y >= self.y && child_bottom > self.y {
                max_y = max_y.max(child_bottom - self.y);
            }
        }

        // Add padding to the content dimensions if children extend beyond the container
        // This ensures scrollable content includes padding after the last child
        if max_x > self.width {
            max_x = max_x + padding.right + border_offset;
        }
        if max_y > self.height {
            max_y = max_y + padding.bottom + border_offset;
        }

        // Content dimensions are the maximum of container size and children extent with padding
        self.content_width = self.width.max(max_x);
        self.content_height = self.height.max(max_y);
    }

    /// Positions an absolutely positioned child based on its offset properties.
    fn position_absolute_child(
        &self,
        child: &mut RenderNode,
        container_x: u16,
        container_y: u16,
        container_width: u16,
        container_height: u16,
    ) {
        if let Some(style) = &child.style {
            let mut x = container_x;
            let mut y = container_y;

            // Apply position offsets
            if let Some(left) = style.left {
                x = container_x.saturating_add_signed(left);
            } else if let Some(right) = style.right {
                // Position from right edge
                x = (container_x + container_width)
                    .saturating_sub(child.width)
                    .saturating_add_signed(-right);
            }

            if let Some(top) = style.top {
                y = container_y.saturating_add_signed(top);
            } else if let Some(bottom) = style.bottom {
                // Position from bottom edge
                y = (container_y + container_height)
                    .saturating_sub(child.height)
                    .saturating_add_signed(-bottom);
            }

            child.set_position(x, y);
        }
    }

    /// Handles a click event on this node.
    ///
    /// Calls the registered click handler if one exists.
    pub fn handle_click(&self) {
        if let Some(on_click) = &self.events.on_click {
            on_click();
        }
    }

    /// Handles a key press event on this node.
    ///
    /// Checks if a handler is registered for the pressed key
    /// and calls it if found. Only processes non-global handlers.
    pub fn handle_key(&self, key: Key) {
        // First check on_any_key handler
        if let Some(ref handler) = self.events.on_any_key {
            handler(key);
        }

        // Check on_any_char handler for character keys
        if let Key::Char(ch) = key
            && let Some(ref handler) = self.events.on_any_char
        {
            handler(ch);
        }

        // Then check specific key handlers
        for (k, handler, is_global) in &self.events.on_key {
            if *k == key && !is_global {
                handler();
                break;
            }
        }
    }

    /// Handles a key press for global handlers only.
    ///
    /// Global handlers work regardless of focus state.
    pub fn handle_global_key(&self, key: Key) {
        for (k, handler, is_global) in &self.events.on_key {
            if *k == key && *is_global {
                handler();
                // Don't break - allow multiple global handlers for same key
            }
        }
    }

    /// Checks if a handler is registered for the pressed key with modifiers
    /// and calls it if found. Only processes non-global handlers.
    pub fn handle_key_with_modifiers(&self, key_with_modifiers: crate::key::KeyWithModifiers) {
        // Check specific key with modifiers handlers
        for (k, handler, is_global) in &self.events.on_key_with_modifiers {
            if *k == key_with_modifiers && !is_global {
                handler();
                break;
            }
        }
    }

    /// Checks if a global handler is registered for the pressed key with modifiers and calls it.
    /// Global handlers work regardless of focus state.
    pub fn handle_global_key_with_modifiers(
        &self,
        key_with_modifiers: crate::key::KeyWithModifiers,
    ) {
        for (k, handler, is_global) in &self.events.on_key_with_modifiers {
            if *k == key_with_modifiers && *is_global {
                handler();
                // Don't break - allow multiple global handlers for same key
            }
        }
    }
}
