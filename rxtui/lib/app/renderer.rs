use crate::bounds::Rect;
use crate::buffer::{Cell, ScreenBuffer};
use crate::render_tree::RenderNode;
use crate::render_tree::RenderNodeType;
use crate::style::{Color, Overflow};
use crate::utils::{display_width, substring_by_columns};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Renders a node and its children to the screen buffer with clipping and background inheritance.
///
/// ## Clipping Strategy
///
/// This function uses two different clip rectangles:
///
/// 1. **element_clip**: Used for rendering the element itself (border, background)
///    - Always the intersection of node bounds and incoming clip_rect
///    - Ensures the element doesn't render outside its parent's clip area
///
/// 2. **children_clip**: Used for clipping child elements
///    - When overflow:hidden, clips to padding box (CSS behavior)
///    - When overflow:none, uses parent's clip_rect unchanged
///
/// ```text
/// CSS Box Model & Clipping:
/// ┌─────────────────────────┐ ← node_bounds
/// │ Border                  │
/// │ ┌─────────────────────┐ │ ← padding_box (overflow:hidden clips here)
/// │ │ Padding             │ │
/// │ │ ┌─────────────────┐ │ │ ← content_box
/// │ │ │                 │ │ │
/// │ │ │    Content      │ │ │
/// │ │ │                 │ │ │
/// │ │ └─────────────────┘ │ │
/// │ └─────────────────────┘ │
/// └─────────────────────────┘
///
/// overflow:hidden clips at padding edge (includes padding, excludes border)
/// overflow:none allows children to render outside all bounds
/// ```
pub fn render_node_to_buffer(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_bg: Option<Color>,
) {
    render_node_with_offset(node, buffer, clip_rect, parent_bg, 0);
}

/// Internal function that handles rendering with accumulated scroll offset
fn render_node_with_offset(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_bg: Option<Color>,
    parent_scroll_offset: i16,
) {
    // Calculate the rendered position with parent scroll offset applied
    // Using i32 to allow negative positions for proper clipping
    let rendered_y_i32 = node.y as i32 - parent_scroll_offset as i32;
    let rendered_x = node.x; // No horizontal scrolling

    // Determine the effective vertical extent for clipping.
    // Text nodes can have more content than their laid-out height represents,
    // so we want to keep rendering as long as there are remaining lines.
    let node_visual_height = match &node.node_type {
        RenderNodeType::TextWrapped(lines) => node.height.max(lines.len() as u16),
        RenderNodeType::RichTextWrapped(lines) => node.height.max(lines.len() as u16),
        _ => node.height,
    };

    // For bounds checking, we need to handle negative positions
    // Elements with negative y are partially or fully above the viewport
    let node_bounds = if rendered_y_i32 < 0 {
        // Node starts above viewport - check if it extends into view
        if rendered_y_i32 + node_visual_height as i32 > 0 {
            // Partially visible - create bounds for the visible portion
            let visible_height = (rendered_y_i32 + node_visual_height as i32) as u16;
            Rect::new(rendered_x, 0, node.width, visible_height)
        } else {
            // Completely above viewport
            Rect::empty()
        }
    } else {
        // Normal case - node starts within or below viewport
        Rect::new(
            rendered_x,
            rendered_y_i32 as u16,
            node.width,
            node_visual_height,
        )
    };

    // Calculate rendered_y for actual rendering (clamped to 0 for partially visible elements)
    let rendered_y = rendered_y_i32.max(0) as u16;

    // Check if node is visible within current clip rect
    if !node_bounds.intersects(clip_rect) {
        return; // Skip rendering if completely outside clip area
    }

    // Calculate clip rect for rendering this element (border, background)
    // This ensures the element itself doesn't render outside the parent's clip area
    let element_clip = node_bounds.intersection(clip_rect);

    // Calculate clip rect for children based on overflow setting
    let children_clip = if let Some(style) = &node.style {
        match style.overflow {
            Some(Overflow::Hidden) | Some(Overflow::Scroll) | Some(Overflow::Auto) => {
                // Clip children to the padding edge (CSS behavior)
                // This means children can render in padding area but not in border area
                let border_offset = if style.border.as_ref().is_some_and(|b| b.enabled) {
                    1
                } else {
                    0
                };

                // Calculate padding box bounds (inside border, includes padding)
                //
                // Example with border=1, padding=2:
                // ┌─────────────┐ (0,0,10x6) ← node bounds
                // │╔═══════════╗│ ← border at (0,0)
                // │║ ┌───────┐ ║│ ← padding box at (1,1,8x4)
                // │║ │content│ ║│ ← content at (3,3,4x0)
                // │║ └───────┘ ║│
                // │╚═══════════╝│
                // └─────────────┘
                let padding_box_x = rendered_x + border_offset;
                // Use actual position for padding box to ensure proper clipping
                let padding_box_y = (rendered_y_i32 + border_offset as i32).max(0) as u16;
                let padding_box_width = node.width.saturating_sub(border_offset * 2);
                // Adjust height if padding box starts above viewport
                let padding_box_height = if rendered_y_i32 + (border_offset as i32) < 0 {
                    // If padding box starts above viewport, reduce height
                    let below_viewport = rendered_y_i32 + node.height as i32;
                    if below_viewport > (border_offset as i32) {
                        (below_viewport - border_offset as i32)
                            .min(node.height as i32 - (border_offset as i32) * 2)
                            as u16
                    } else {
                        0
                    }
                } else {
                    node.height.saturating_sub(border_offset * 2)
                };

                let padding_box_bounds = Rect::new(
                    padding_box_x,
                    padding_box_y,
                    padding_box_width,
                    padding_box_height,
                );
                padding_box_bounds.intersection(clip_rect)
            }
            _ => {
                // If overflow is none (or not set), use parent's clip rect
                *clip_rect
            }
        }
    } else {
        *clip_rect
    };

    match &node.node_type {
        RenderNodeType::Element => {
            // Determine the effective background for the node's text children
            // Start with parent's background to ensure proper inheritance chain
            let mut effective_bg = parent_bg;

            // Render the element itself (border and background) using element_clip
            // This ensures the element doesn't render outside its parent's bounds
            //
            // Visual example of element_clip vs children_clip:
            //
            // Parent with overflow:hidden, child extends beyond:
            // ┌─────────────────────┐ ← Parent's node_bounds
            // │╔═══════════════════╗│ ← Parent's border (uses element_clip)
            // │║ padding           ║│ ← Parent's padding area
            // │║ ┌─────────────────┼──┐ ← Child extends beyond parent
            // │║ │ Child content   │  │
            // │║ │ is clipped at───┼──┘ ← children_clip (padding edge)
            // │║ └─────────────────┘│
            // │╚═══════════════════╝│
            // └─────────────────────┘
            //
            // - element_clip: Used to render parent's border/background
            // - children_clip: Used to clip child content (at padding edge when overflow:hidden)

            // Draw border if enabled
            if let Some(style) = &node.style {
                if let Some(border) = &style.border
                    && border.enabled
                    && node.width > 1
                    && node.height > 1
                {
                    // Get border characters based on style
                    let (top_left, top, top_right, left, right, bottom_left, bottom, bottom_right) =
                        match border.style {
                            crate::style::BorderStyle::Single => {
                                ('┌', '─', '┐', '│', '│', '└', '─', '┘')
                            }
                            crate::style::BorderStyle::Double => {
                                ('╔', '═', '╗', '║', '║', '╚', '═', '╝')
                            }
                            crate::style::BorderStyle::Thick => {
                                ('┏', '━', '┓', '┃', '┃', '┗', '━', '┛')
                            }
                            crate::style::BorderStyle::Rounded => {
                                ('╭', '─', '╮', '│', '│', '╰', '─', '╯')
                            }
                            crate::style::BorderStyle::Dashed => {
                                ('┌', '╌', '┐', '╎', '╎', '└', '╌', '┘')
                            }
                        };

                    // Draw border within the clipped area
                    let border_bounds = node_bounds.intersection(&element_clip);
                    use crate::style::BorderEdges;

                    // Top border
                    if border.edges.contains(BorderEdges::TOP)
                        && border_bounds.y == rendered_y
                        && border_bounds.height > 0
                    {
                        for x in border_bounds.x..border_bounds.right() {
                            let ch = if x == rendered_x
                                && x >= border_bounds.x
                                && border.edges.contains(BorderEdges::TOP_LEFT)
                            {
                                top_left // Top-left corner
                            } else if x == rendered_x + node.width - 1
                                && x < border_bounds.right()
                                && border.edges.contains(BorderEdges::TOP_RIGHT)
                            {
                                top_right // Top-right corner
                            } else if x != rendered_x && x != rendered_x + node.width - 1 {
                                top // Horizontal line (skip corners if they're not enabled)
                            } else {
                                ' ' // Empty space if corner not enabled
                            };
                            let mut cell = Cell::new(ch);
                            if ch != ' ' {
                                cell.fg = Some(border.color);
                            }
                            // Always set background for border cells (including empty corners)
                            cell.bg = style.background.or(parent_bg);
                            buffer.set_cell(x, rendered_y, cell);
                        }
                    }

                    // Bottom border
                    let bottom_y = rendered_y + node.height - 1;
                    if border.edges.contains(BorderEdges::BOTTOM)
                        && bottom_y < border_bounds.bottom()
                        && bottom_y >= border_bounds.y
                    {
                        for x in border_bounds.x..border_bounds.right() {
                            let ch = if x == rendered_x
                                && x >= border_bounds.x
                                && border.edges.contains(BorderEdges::BOTTOM_LEFT)
                            {
                                bottom_left // Bottom-left corner
                            } else if x == rendered_x + node.width - 1
                                && x < border_bounds.right()
                                && border.edges.contains(BorderEdges::BOTTOM_RIGHT)
                            {
                                bottom_right // Bottom-right corner
                            } else if x != rendered_x && x != rendered_x + node.width - 1 {
                                bottom // Horizontal line (skip corners if they're not enabled)
                            } else {
                                ' ' // Empty space if corner not enabled
                            };
                            let mut cell = Cell::new(ch);
                            if ch != ' ' {
                                cell.fg = Some(border.color);
                            }
                            // Always set background for border cells (including empty corners)
                            cell.bg = style.background.or(parent_bg);
                            buffer.set_cell(x, bottom_y, cell);
                        }
                    }

                    // Left and right borders
                    for y in (border_bounds.y.max(rendered_y + 1))
                        ..(border_bounds.bottom().min(rendered_y + node.height - 1))
                    {
                        // Left border
                        if border.edges.contains(BorderEdges::LEFT)
                            && rendered_x >= border_bounds.x
                            && rendered_x < border_bounds.right()
                        {
                            let mut cell = Cell::new(left);
                            cell.fg = Some(border.color);
                            // Use element's background if it has one, otherwise inherit from parent
                            cell.bg = style.background.or(parent_bg);
                            buffer.set_cell(rendered_x, y, cell);
                        }

                        // Right border
                        let right_x = rendered_x + node.width - 1;
                        if border.edges.contains(BorderEdges::RIGHT)
                            && right_x >= border_bounds.x
                            && right_x < border_bounds.right()
                        {
                            let mut cell = Cell::new(right);
                            cell.fg = Some(border.color);
                            // Use element's background if it has one, otherwise inherit from parent
                            cell.bg = style.background.or(parent_bg);
                            buffer.set_cell(right_x, y, cell);
                        }
                    }

                    // Draw standalone corners if edges are not present
                    if !border.edges.contains(BorderEdges::TOP)
                        && !border.edges.contains(BorderEdges::LEFT)
                        && border.edges.contains(BorderEdges::TOP_LEFT)
                        && rendered_x >= border_bounds.x
                        && rendered_x < border_bounds.right()
                        && rendered_y >= border_bounds.y
                        && rendered_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(top_left);
                        cell.fg = Some(border.color);
                        // Use element's background if it has one, otherwise inherit from parent
                        cell.bg = style.background.or(parent_bg);
                        buffer.set_cell(rendered_x, rendered_y, cell);
                    }
                    let right_x = rendered_x + node.width - 1;
                    if !border.edges.contains(BorderEdges::TOP)
                        && !border.edges.contains(BorderEdges::RIGHT)
                        && border.edges.contains(BorderEdges::TOP_RIGHT)
                        && right_x >= border_bounds.x
                        && right_x < border_bounds.right()
                        && rendered_y >= border_bounds.y
                        && rendered_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(top_right);
                        cell.fg = Some(border.color);
                        // Use element's background if it has one, otherwise inherit from parent
                        cell.bg = style.background.or(parent_bg);
                        buffer.set_cell(right_x, rendered_y, cell);
                    }
                    let bottom_y = rendered_y + node.height - 1;
                    if !border.edges.contains(BorderEdges::BOTTOM)
                        && !border.edges.contains(BorderEdges::LEFT)
                        && border.edges.contains(BorderEdges::BOTTOM_LEFT)
                        && rendered_x >= border_bounds.x
                        && rendered_x < border_bounds.right()
                        && bottom_y >= border_bounds.y
                        && bottom_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(bottom_left);
                        cell.fg = Some(border.color);
                        // Use element's background if it has one, otherwise inherit from parent
                        cell.bg = style.background.or(parent_bg);
                        buffer.set_cell(rendered_x, bottom_y, cell);
                    }
                    let right_x = rendered_x + node.width - 1;
                    // bottom_y already calculated above
                    if !border.edges.contains(BorderEdges::BOTTOM)
                        && !border.edges.contains(BorderEdges::RIGHT)
                        && border.edges.contains(BorderEdges::BOTTOM_RIGHT)
                        && right_x >= border_bounds.x
                        && right_x < border_bounds.right()
                        && bottom_y >= border_bounds.y
                        && bottom_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(bottom_right);
                        cell.fg = Some(border.color);
                        // Use element's background if it has one, otherwise inherit from parent
                        cell.bg = style.background.or(parent_bg);
                        buffer.set_cell(right_x, bottom_y, cell);
                    }
                }

                // Fill the div area with background color if there's any effective background
                if let Some(bg) = style.background {
                    effective_bg = Some(bg);
                    // Fill within the clipped area, but skip border cells if border is enabled
                    let fill_bounds = node_bounds.intersection(&element_clip);
                    let has_border = style.border.as_ref().is_some_and(|b| b.enabled);

                    for y in fill_bounds.y..fill_bounds.bottom() {
                        for x in fill_bounds.x..fill_bounds.right() {
                            // Skip border cells if border is enabled
                            if has_border && node.width > 1 && node.height > 1 {
                                let is_border_cell = (y == rendered_y
                                    || y == rendered_y + node.height - 1)
                                    || (x == rendered_x || x == rendered_x + node.width - 1);
                                if is_border_cell {
                                    // Set background only if cell is empty (preserve border character)
                                    if let Some(cell) = buffer.get_cell_mut(x, y)
                                        && cell.bg.is_none()
                                    {
                                        cell.bg = Some(bg);
                                    }
                                    continue;
                                }
                            }

                            let mut cell = Cell::new(' ');
                            cell.bg = Some(bg);
                            buffer.set_cell(x, y, cell);
                        }
                    }
                }
            }

            // Calculate content area to check if we should render children
            // This prevents rendering when border and padding consume all available space
            //
            // Example: element with width=4, height=4, border=1, padding=1
            // ┌─────┐
            // │╔═══╗│ ← Border takes 1px on each side (2px total)
            // │║   ║│ ← Padding takes 1px on each side (2px total)
            // │╚═══╝│ ← Content area: 4 - 2 - 2 = 0 (no space!)
            // └─────┘
            //
            // In this case, content_width = 0 and content_height = 0,
            // so we skip rendering children entirely.
            let padding = node
                .style
                .as_ref()
                .and_then(|s| s.padding)
                .unwrap_or(crate::style::Spacing::all(0));
            let border_offset = if node
                .style
                .as_ref()
                .and_then(|s| s.border.as_ref())
                .is_some_and(|b| b.enabled)
            {
                1
            } else {
                0
            };

            let content_width = node
                .width
                .saturating_sub(padding.left + padding.right + (border_offset * 2));
            let content_height = node
                .height
                .saturating_sub(padding.top + padding.bottom + (border_offset * 2));

            // Only render children if there's content area available
            if content_width > 0 && content_height > 0 {
                // Sort children by z-index for proper layering
                let mut sorted_children: Vec<_> = node.children.iter().collect();
                sorted_children.sort_by_key(|child| child.borrow().z_index);

                // Render children in z-index order with the children clip rect and background
                // Calculate total scroll offset to pass to children
                let child_scroll_offset = if node.scrollable {
                    parent_scroll_offset + node.scroll_y as i16
                } else {
                    parent_scroll_offset
                };

                for child in sorted_children {
                    render_node_with_offset(
                        &child.borrow(),
                        buffer,
                        &children_clip,
                        effective_bg,
                        child_scroll_offset,
                    );
                }

                // Render scrollbars if needed (for Scroll and Auto modes)
                // Only show scrollbar if explicitly enabled via style
                if node.scrollable
                    && node
                        .style
                        .as_ref()
                        .and_then(|s| s.show_scrollbar)
                        .unwrap_or(true)
                {
                    render_scrollbars(node, buffer, &element_clip, parent_scroll_offset);
                }
            }
        }

        RenderNodeType::Text(text) => {
            // Only render text that's within the clip rect
            let text_width = display_width(text) as u16;

            // For alignment, we need to use the parent's content width if available
            // Text nodes typically have width equal to their content, so we check parent
            let available_width = if node.width > text_width {
                // If the node has extra width (e.g., from parent layout), use it
                node.width
            } else {
                // Otherwise just use the text width (no alignment possible)
                text_width
            };

            // Calculate alignment offset
            let align_offset = if let Some(text_style) = &node.text_style
                && let Some(align) = text_style.align
                && available_width > text_width
            {
                match align {
                    crate::style::TextAlign::Left => 0,
                    crate::style::TextAlign::Center => {
                        // Center text within the available width
                        available_width.saturating_sub(text_width) / 2
                    }
                    crate::style::TextAlign::Right => {
                        // Right align text within the available width
                        available_width.saturating_sub(text_width)
                    }
                }
            } else {
                0 // Default to left alignment or no space for alignment
            };

            // Apply alignment offset to the rendered position
            let aligned_x = rendered_x + align_offset;
            let text_bounds = crate::bounds::Rect::new(aligned_x, rendered_y, text_width, 1);

            if text_bounds.intersects(clip_rect) {
                // Calculate visible portion of text in display columns
                let visible_start_col = if aligned_x < clip_rect.x {
                    (clip_rect.x - aligned_x) as usize
                } else {
                    0
                };

                let visible_end_col = if aligned_x + text_width > clip_rect.right() {
                    (clip_rect.right() - aligned_x) as usize
                } else {
                    display_width(text)
                };

                if visible_start_col < visible_end_col {
                    // Use substring_by_columns to extract the visible portion safely
                    let visible_text =
                        substring_by_columns(text, visible_start_col, visible_end_col);
                    let render_x = aligned_x.max(clip_rect.x);

                    // Use the full text style if available, otherwise fall back to individual color fields
                    if let Some(text_style) = &node.text_style {
                        // Create a merged text style with background inheritance
                        let mut merged_style = text_style.clone();
                        if merged_style.background.is_none() {
                            merged_style.background = parent_bg;
                        }
                        buffer.write_styled_str(
                            render_x,
                            rendered_y,
                            visible_text,
                            Some(&merged_style),
                        );
                    } else {
                        // Fallback to old method if no full text style
                        let text_bg = node.style.as_ref().and_then(|s| s.background).or(parent_bg);
                        buffer.write_str(
                            render_x,
                            rendered_y,
                            visible_text,
                            node.text_color,
                            text_bg,
                        );
                    }
                }
            }
        }

        RenderNodeType::TextWrapped(lines) => {
            // Skip lines that have scrolled out of view above the clip region
            let skip_lines = if rendered_y_i32 < 0 {
                (-rendered_y_i32) as usize
            } else {
                0
            };
            let start_index = skip_lines.min(lines.len());

            // Render each visible line of wrapped text
            for (line_idx, line) in lines.iter().enumerate().skip(start_index) {
                let visual_index = (line_idx - start_index) as u16;
                let line_y = rendered_y + visual_index;

                if line_y >= clip_rect.bottom() {
                    break;
                }

                if line_y >= clip_rect.y {
                    let line_width = display_width(line) as u16;

                    // Calculate alignment offset for this line
                    let align_offset = if let Some(text_style) = &node.text_style
                        && let Some(align) = text_style.align
                    {
                        match align {
                            crate::style::TextAlign::Left => 0,
                            crate::style::TextAlign::Center => {
                                // Center each line independently within the node's width
                                node.width.saturating_sub(line_width) / 2
                            }
                            crate::style::TextAlign::Right => {
                                // Right align each line independently within the node's width
                                node.width.saturating_sub(line_width)
                            }
                        }
                    } else {
                        0 // Default to left alignment
                    };

                    // Apply alignment offset to the rendered position
                    let aligned_x = rendered_x + align_offset;
                    let text_bounds = crate::bounds::Rect::new(aligned_x, line_y, line_width, 1);

                    if text_bounds.intersects(clip_rect) {
                        // Calculate visible portion of this line in display columns
                        let visible_start_col = if aligned_x < clip_rect.x {
                            (clip_rect.x - aligned_x) as usize
                        } else {
                            0
                        };

                        let visible_end_col = if aligned_x + line_width > clip_rect.right() {
                            (clip_rect.right() - aligned_x) as usize
                        } else {
                            display_width(line)
                        };

                        if visible_start_col < visible_end_col {
                            // Use substring_by_columns to extract the visible portion safely
                            let visible_text =
                                substring_by_columns(line, visible_start_col, visible_end_col);
                            let render_x = aligned_x.max(clip_rect.x);

                            // Use the full text style if available
                            if let Some(text_style) = &node.text_style {
                                // Create a merged text style with background inheritance
                                let mut merged_style = text_style.clone();
                                if merged_style.background.is_none() {
                                    merged_style.background = parent_bg;
                                }
                                buffer.write_styled_str(
                                    render_x,
                                    line_y,
                                    visible_text,
                                    Some(&merged_style),
                                );
                            } else {
                                // Fallback to old method if no full text style
                                let text_bg =
                                    node.style.as_ref().and_then(|s| s.background).or(parent_bg);
                                buffer.write_str(
                                    render_x,
                                    line_y,
                                    visible_text,
                                    node.text_color,
                                    text_bg,
                                );
                            }
                        }
                    }
                }
            }
        }

        RenderNodeType::RichText(spans) => {
            // Calculate total width of all spans for alignment
            let total_width: u16 = spans
                .iter()
                .map(|span| display_width(&span.content) as u16)
                .sum();

            // Calculate alignment offset
            let align_offset = if let Some(text_style) = &node.text_style
                && let Some(align) = text_style.align
            {
                match align {
                    crate::style::TextAlign::Left => 0,
                    crate::style::TextAlign::Center => {
                        // Center the entire rich text line within the node's width
                        node.width.saturating_sub(total_width) / 2
                    }
                    crate::style::TextAlign::Right => {
                        // Right align the entire rich text line within the node's width
                        node.width.saturating_sub(total_width)
                    }
                }
            } else {
                0 // Default to left alignment
            };

            // Apply alignment offset to the starting position
            let aligned_x = rendered_x + align_offset;
            let text_bounds = crate::bounds::Rect::new(aligned_x, rendered_y, total_width, 1);

            if text_bounds.intersects(clip_rect) {
                let mut current_x = aligned_x;

                // Render each span with its own style
                for span in spans {
                    let span_width = display_width(&span.content) as u16;

                    // Check if this span is visible
                    if current_x + span_width > clip_rect.x && current_x < clip_rect.right() {
                        // Calculate visible portion of span
                        let visible_start_col = if current_x < clip_rect.x {
                            (clip_rect.x - current_x) as usize
                        } else {
                            0
                        };

                        let visible_end_col = if current_x + span_width > clip_rect.right() {
                            (clip_rect.right() - current_x) as usize
                        } else {
                            display_width(&span.content)
                        };

                        if visible_start_col < visible_end_col {
                            let visible_text = substring_by_columns(
                                &span.content,
                                visible_start_col,
                                visible_end_col,
                            );
                            let render_x = current_x.max(clip_rect.x);

                            // Apply span's style, falling back to parent background
                            if let Some(span_style) = &span.style {
                                let mut merged_style = span_style.clone();
                                if merged_style.background.is_none() {
                                    merged_style.background = parent_bg;
                                }
                                buffer.write_styled_str(
                                    render_x,
                                    rendered_y,
                                    visible_text,
                                    Some(&merged_style),
                                );
                            } else {
                                // No style on this span - use default with parent background
                                buffer.write_str(
                                    render_x,
                                    rendered_y,
                                    visible_text,
                                    None,
                                    parent_bg,
                                );
                            }
                        }
                    }

                    current_x += span_width;
                }
            }
        }

        RenderNodeType::RichTextWrapped(lines) => {
            // Skip lines that have scrolled out of view above the clip region
            let skip_lines = if rendered_y_i32 < 0 {
                (-rendered_y_i32) as usize
            } else {
                0
            };
            let start_index = skip_lines.min(lines.len());

            // Render each visible line of wrapped styled text
            for (line_idx, line_spans) in lines.iter().enumerate().skip(start_index) {
                let visual_index = (line_idx - start_index) as u16;
                let line_y = rendered_y + visual_index;

                if line_y >= clip_rect.bottom() {
                    break;
                }

                if line_y >= clip_rect.y {
                    // Calculate total line width
                    let line_width: u16 = line_spans
                        .iter()
                        .map(|span| display_width(&span.content) as u16)
                        .sum();

                    // Calculate alignment offset for this line
                    let align_offset = if let Some(text_style) = &node.text_style
                        && let Some(align) = text_style.align
                    {
                        match align {
                            crate::style::TextAlign::Left => 0,
                            crate::style::TextAlign::Center => {
                                // Center each line independently within the node's width
                                node.width.saturating_sub(line_width) / 2
                            }
                            crate::style::TextAlign::Right => {
                                // Right align each line independently within the node's width
                                node.width.saturating_sub(line_width)
                            }
                        }
                    } else {
                        0 // Default to left alignment
                    };

                    // Apply alignment offset to the starting position
                    let aligned_x = rendered_x + align_offset;
                    let text_bounds = crate::bounds::Rect::new(aligned_x, line_y, line_width, 1);

                    if text_bounds.intersects(clip_rect) {
                        let mut current_x = aligned_x;

                        // Render each span in this line with its own style
                        for span in line_spans {
                            let span_width = display_width(&span.content) as u16;

                            // Check if this span is visible
                            if current_x + span_width > clip_rect.x && current_x < clip_rect.right()
                            {
                                // Calculate visible portion of span
                                let visible_start_col = if current_x < clip_rect.x {
                                    (clip_rect.x - current_x) as usize
                                } else {
                                    0
                                };

                                let visible_end_col = if current_x + span_width > clip_rect.right()
                                {
                                    (clip_rect.right() - current_x) as usize
                                } else {
                                    display_width(&span.content)
                                };

                                if visible_start_col < visible_end_col {
                                    let visible_text = substring_by_columns(
                                        &span.content,
                                        visible_start_col,
                                        visible_end_col,
                                    );
                                    let render_x = current_x.max(clip_rect.x);

                                    // Apply span's style, falling back to parent background
                                    if let Some(span_style) = &span.style {
                                        let mut merged_style = span_style.clone();
                                        if merged_style.background.is_none() {
                                            merged_style.background = parent_bg;
                                        }
                                        buffer.write_styled_str(
                                            render_x,
                                            line_y,
                                            visible_text,
                                            Some(&merged_style),
                                        );
                                    } else {
                                        // No style on this span - use default with parent background
                                        buffer.write_str(
                                            render_x,
                                            line_y,
                                            visible_text,
                                            None,
                                            parent_bg,
                                        );
                                    }
                                }
                            }

                            current_x += span_width;
                        }
                    }
                }
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::ScreenBuffer,
        render_tree::RenderNode,
        style::{Color, Style},
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_text_inherits_parent_background() {
        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 0;
        text_node.y = 0;
        text_node.width = 5;
        text_node.height = 1;
        // No style set - should inherit parent's background

        // Add text as child of parent
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));
        parent_rc.borrow_mut().children.push(text_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 3);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 3);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the parent's blue background
        for x in 0..5 {
            let cell = buffer.get_cell(x, 0).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Text at position {x} should have blue background"
            );
        }
    }

    #[test]
    fn test_text_own_background_takes_precedence() {
        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a text node with its own red background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 0;
        text_node.y = 0;
        text_node.width = 5;
        text_node.height = 1;
        text_node.style = Some(Style {
            background: Some(Color::Red),
            ..Default::default()
        });

        // Add text as child of parent
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));
        parent_rc.borrow_mut().children.push(text_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 3);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 3);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have their own red background, not parent's blue
        for x in 0..5 {
            let cell = buffer.get_cell(x, 0).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Text at position {x} should have red background"
            );
        }
    }

    #[test]
    fn test_multi_level_background_inheritance() {
        // Create a grandparent div with blue background
        let mut grandparent = RenderNode::element();
        grandparent.x = 0;
        grandparent.y = 0;
        grandparent.width = 15;
        grandparent.height = 5;
        grandparent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a parent div WITHOUT background
        let mut parent = RenderNode::element();
        parent.x = 1;
        parent.y = 1;
        parent.width = 10;
        parent.height = 3;
        // No background style - should inherit from grandparent

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 1;
        text_node.y = 1;
        text_node.width = 5;
        text_node.height = 1;
        // No style set - should inherit through parent from grandparent

        // Build the tree
        let grandparent_rc = Rc::new(RefCell::new(grandparent));
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));

        parent_rc.borrow_mut().children.push(text_rc);
        grandparent_rc.borrow_mut().children.push(parent_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(15, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 15, 5);
        render_node_to_buffer(&grandparent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the grandparent's blue background
        // Text is at absolute position (2, 2) due to nested positioning
        for x in 2..7 {
            let cell = buffer.get_cell(x, 2).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Text at position {x} should inherit blue background from grandparent"
            );
        }
    }

    #[test]
    fn test_border_background_inheritance() {
        use crate::style::{Border, BorderEdges, BorderStyle};

        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 5;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a child div with border but no background
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::ALL,
            }),
            // No background - border should inherit parent's blue
            ..Default::default()
        });

        // Build the tree
        let parent_rc = Rc::new(RefCell::new(parent));
        let child_rc = Rc::new(RefCell::new(child));
        parent_rc.borrow_mut().children.push(child_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 5);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that border cells have the parent's blue background
        // Top border
        for x in 1..6 {
            let cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Top border at position {x} should have blue background"
            );
        }

        // Left border
        for y in 1..4 {
            let cell = buffer.get_cell(1, y).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Left border at position y={y} should have blue background"
            );
        }
    }

    #[test]
    fn test_border_uses_element_bg_when_available() {
        use crate::style::{Border, BorderEdges, BorderStyle};

        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 5;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a child div with border AND its own red background
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            background: Some(Color::Red), // Has its own background
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::ALL,
            }),
            ..Default::default()
        });

        // Build the tree
        let parent_rc = Rc::new(RefCell::new(parent));
        let child_rc = Rc::new(RefCell::new(child));
        parent_rc.borrow_mut().children.push(child_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 5);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that border cells have the child's red background, not parent's blue
        // Top border
        for x in 1..6 {
            let cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Top border at position {x} should have red background from element, not blue from parent"
            );
        }

        // Left border
        for y in 1..4 {
            let cell = buffer.get_cell(1, y).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Left border at position y={y} should have red background from element, not blue from parent"
            );
        }
    }

    #[test]
    fn test_selective_border_edges_background() {
        use crate::style::{Border, BorderEdges, BorderStyle};

        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 5;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a child with only horizontal borders (no corners)
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            background: Some(Color::Red),
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::TOP | BorderEdges::BOTTOM, // Only top and bottom, no corners
            }),
            ..Default::default()
        });

        // Build the tree
        let parent_rc = Rc::new(RefCell::new(parent));
        let child_rc = Rc::new(RefCell::new(child));
        parent_rc.borrow_mut().children.push(child_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 5);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that ALL cells in the border row have red background
        // Including the corner positions (x=1 and x=5) even though they're empty
        for x in 1..6 {
            let top_cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                top_cell.bg,
                Some(Color::Red),
                "Top border row at x={x} should have red background, even empty corners"
            );

            let bottom_cell = buffer.get_cell(x, 3).unwrap();
            assert_eq!(
                bottom_cell.bg,
                Some(Color::Red),
                "Bottom border row at x={x} should have red background, even empty corners"
            );
        }
    }

    #[test]
    fn test_element_with_own_bg_overrides_inheritance() {
        // Create a grandparent div with blue background
        let mut grandparent = RenderNode::element();
        grandparent.x = 0;
        grandparent.y = 0;
        grandparent.width = 15;
        grandparent.height = 5;
        grandparent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a parent div with red background (overrides blue)
        let mut parent = RenderNode::element();
        parent.x = 1;
        parent.y = 1;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Red),
            ..Default::default()
        });

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 1;
        text_node.y = 1;
        text_node.width = 5;
        text_node.height = 1;
        // Should inherit red from parent, not blue from grandparent

        // Build the tree
        let grandparent_rc = Rc::new(RefCell::new(grandparent));
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));

        parent_rc.borrow_mut().children.push(text_rc);
        grandparent_rc.borrow_mut().children.push(parent_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(15, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 15, 5);
        render_node_to_buffer(&grandparent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the parent's red background (not grandparent's blue)
        for x in 2..7 {
            let cell = buffer.get_cell(x, 2).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Text at position {x} should have red background from parent, not blue from grandparent"
            );
        }
    }

    #[test]
    fn test_text_center_alignment() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(10)
            .height(1)
            .child(Text::new("Hi").align(TextAlign::Center).into())
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 10);

        let mut buffer = ScreenBuffer::new(20, 10);
        let clip_rect = crate::Rect::new(0, 0, 20, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            render_node_to_buffer(&root.borrow(), &mut buffer, &clip_rect, None);
        }

        // "Hi" is 2 chars wide, container is 10 wide
        // Should be centered at position 4 (10 - 2) / 2 = 4

        let cell_h = buffer.get_cell(4, 0).unwrap();
        let cell_i = buffer.get_cell(5, 0).unwrap();
        assert_eq!(cell_h.char, 'H', "Expected 'H' at position 4");
        assert_eq!(cell_i.char, 'i', "Expected 'i' at position 5");
    }

    #[test]
    fn test_text_right_alignment() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(10)
            .height(1)
            .child(Text::new("End").align(TextAlign::Right).into())
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 10);

        let mut buffer = ScreenBuffer::new(20, 10);
        let clip_rect = crate::Rect::new(0, 0, 20, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            render_node_to_buffer(&root.borrow(), &mut buffer, &clip_rect, None);
        }

        // "End" is 3 chars wide, container is 10 wide
        // Should be right-aligned at position 7 (10 - 3 = 7)
        let cell_e = buffer.get_cell(7, 0).unwrap();
        let cell_n = buffer.get_cell(8, 0).unwrap();
        let cell_d = buffer.get_cell(9, 0).unwrap();
        assert_eq!(cell_e.char, 'E');
        assert_eq!(cell_n.char, 'n');
        assert_eq!(cell_d.char, 'd');
    }

    #[test]
    fn test_justify_content_start() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(20)
            .height(3)
            .direction(Direction::Horizontal)
            .justify_content(JustifyContent::Start)
            .children(vec![
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 3);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Check that children are positioned at start (left)
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            assert_eq!(child0.x, 0);
            assert_eq!(child1.x, 3);
            assert_eq!(child2.x, 6);
        }
    }

    #[test]
    fn test_justify_content_center() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(20)
            .height(3)
            .direction(Direction::Horizontal)
            .justify_content(JustifyContent::Center)
            .children(vec![
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 3);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Total width of children = 9, container = 20, so space = 11
            // Center should start at 11/2 = 5.5, rounded down to 5
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            assert_eq!(child0.x, 5);
            assert_eq!(child1.x, 8);
            assert_eq!(child2.x, 11);
        }
    }

    #[test]
    fn test_justify_content_end() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(20)
            .height(3)
            .direction(Direction::Horizontal)
            .justify_content(JustifyContent::End)
            .children(vec![
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 3);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Total width of children = 9, container = 20, so space = 11
            // End should start at 11
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            assert_eq!(child0.x, 11);
            assert_eq!(child1.x, 14);
            assert_eq!(child2.x, 17);
        }
    }

    #[test]
    fn test_justify_content_space_between() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(20)
            .height(3)
            .direction(Direction::Horizontal)
            .justify_content(JustifyContent::SpaceBetween)
            .children(vec![
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
                Div::new().width(3).height(1).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 3);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Total width of children = 9, container = 20, so space = 11
            // Space between 3 items = 11 / (3-1) = 11/2 = 5.5, rounded down to 5
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            // Total children width = 9, container = 20, available space = 11
            // SpaceBetween distributes the 11 pixels as spacing between items
            // With 3 items, there are 2 gaps, so each gap = 11/2 = 5 (truncated)
            assert_eq!(child0.x, 0); // First at start
            assert_eq!(child1.x, 8); // width + spacing
            assert_eq!(child2.x, 16); // Expected based on space between logic
        }
    }

    #[test]
    fn test_align_items_center() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(10)
            .height(10)
            .direction(Direction::Horizontal)
            .align_items(AlignItems::Center)
            .children(vec![
                Div::new().width(3).height(2).into(),
                Div::new().width(3).height(4).into(),
                Div::new().width(3).height(6).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(10, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Check vertical centering
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            // Child 0: height 2, container 10, centered at (10-2)/2 = 4
            assert_eq!(child0.y, 4);
            // Child 1: height 4, container 10, centered at (10-4)/2 = 3
            assert_eq!(child1.y, 3);
            // Child 2: height 6, container 10, centered at (10-6)/2 = 2
            assert_eq!(child2.y, 2);
        }
    }

    #[test]
    fn test_align_items_end() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(10)
            .height(10)
            .direction(Direction::Horizontal)
            .align_items(AlignItems::End)
            .children(vec![
                Div::new().width(3).height(2).into(),
                Div::new().width(3).height(4).into(),
                Div::new().width(3).height(6).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(10, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            // Check vertical end alignment
            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            // Child 0: height 2, container 10, at end: 10-2 = 8
            assert_eq!(child0.y, 8);
            // Child 1: height 4, container 10, at end: 10-4 = 6
            assert_eq!(child1.y, 6);
            // Child 2: height 6, container 10, at end: 10-6 = 4
            assert_eq!(child2.y, 4);
        }
    }

    #[test]
    fn test_align_self_override() {
        use crate::prelude::*;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(10)
            .height(10)
            .direction(Direction::Horizontal)
            .align_items(AlignItems::Start) // Parent alignment
            .children(vec![
                Div::new().width(3).height(2).into(),
                Div::new()
                    .width(3)
                    .height(4)
                    .align_self(AlignSelf::Center)
                    .into(), // Override
                Div::new()
                    .width(3)
                    .height(6)
                    .align_self(AlignSelf::End)
                    .into(), // Override
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(10, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 3);

            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            // Child 0: follows parent alignment (start), so y = 0
            assert_eq!(child0.y, 0);
            // Child 1: overrides with center, height 4, centered at (10-4)/2 = 3
            assert_eq!(child1.y, 3);
            // Child 2: overrides with end, height 6, at end: 10-6 = 4
            assert_eq!(child2.y, 4);
        }
    }

    #[test]
    fn test_wrap_with_justify_content() {
        use crate::prelude::*;
        use crate::style::WrapMode;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(25)
            .height(10)
            .direction(Direction::Horizontal)
            .wrap(WrapMode::Wrap)
            .justify_content(JustifyContent::Center)
            .children(vec![
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(2).into(),
                // These should wrap to second row
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(2).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(25, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();
            assert_eq!(root_ref.children.len(), 5);

            // First row: 3 items, width = 24, available = 1, centered
            let child0 = root_ref.children[0].borrow();
            let _child1 = root_ref.children[1].borrow();
            let _child2 = root_ref.children[2].borrow();

            // First row should be centered (1 pixel available / 2 = 0)
            assert_eq!(child0.x, 0);
            assert_eq!(child0.y, 0);

            // Second row: 2 items, width = 16, available = 9, centered at 4
            let child3 = root_ref.children[3].borrow();
            let child4 = root_ref.children[4].borrow();

            assert_eq!(child3.x, 4); // Centered: 9/2 = 4
            assert_eq!(child3.y, 2); // Second row
            assert_eq!(child4.x, 12); // 4 + 8
            assert_eq!(child4.y, 2);
        }
    }

    #[test]
    fn test_wrap_with_align_items() {
        use crate::prelude::*;
        use crate::style::WrapMode;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(25)
            .height(10)
            .direction(Direction::Horizontal)
            .wrap(WrapMode::Wrap)
            .align_items(AlignItems::Center)
            .gap(1)
            .children(vec![
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(4).into(), // Taller item
                Div::new().width(8).height(2).into(),
                // These wrap to second row
                Div::new().width(8).height(3).into(),
                Div::new().width(8).height(1).into(), // Shorter item
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(25, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();

            // With width=25 and gap=1, only 2 items fit per row (8 + 1 + 8 = 17 < 25, but 17 + 1 + 8 = 26 > 25)
            // Row 1: items 0,1 (max height = 4)
            // Row 2: items 2,3 (max height = 3)
            // Row 3: item 4 (height = 1)

            let child0 = root_ref.children[0].borrow();
            let child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();
            let child3 = root_ref.children[3].borrow();
            let child4 = root_ref.children[4].borrow();

            // Row 1: items with heights 2 and 4, row height = 4
            // Child 0: height 2, centered in row height 4: (4-2)/2 = 1
            assert_eq!(child0.y, 1);
            // Child 1: height 4, centered in row height 4: (4-4)/2 = 0
            assert_eq!(child1.y, 0);

            // Row 2: items with heights 2 and 3, row height = 3
            // Y offset = row1_height(4) + gap(1) = 5
            // Child 2: height 2, centered in row height 3: (3-2)/2 = 0 (rounds down)
            assert_eq!(child2.y, 5);
            // Child 3: height 3, centered in row height 3: (3-3)/2 = 0
            assert_eq!(child3.y, 5);

            // Row 3: item with height 1
            // Y offset = row1_height(4) + gap(1) + row2_height(3) + gap(1) = 9
            // Child 4: height 1, no centering needed (single item in row)
            assert_eq!(child4.y, 9);
        }
    }

    #[test]
    fn test_wrap_with_space_between() {
        use crate::prelude::*;
        use crate::style::WrapMode;
        use crate::vdom::VDom;
        use crate::vnode::VNode;

        let node: VNode = Div::new()
            .width(30)
            .height(10)
            .direction(Direction::Horizontal)
            .wrap(WrapMode::Wrap)
            .justify_content(JustifyContent::SpaceBetween)
            .children(vec![
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(2).into(),
                Div::new().width(8).height(2).into(),
                // Wrap to next row
                Div::new().width(10).height(2).into(),
                Div::new().width(10).height(2).into(),
            ])
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(30, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            let root_ref = root.borrow();

            // First row: 3 items of width 8 each, total = 24, available = 6
            // SpaceBetween: first at 0, last at end, middle distributed
            let child0 = root_ref.children[0].borrow();
            let _child1 = root_ref.children[1].borrow();
            let child2 = root_ref.children[2].borrow();

            assert_eq!(child0.x, 0); // First item at start
            assert_eq!(child2.x, 22); // Last item at end (30 - 8 = 22)

            // Second row: 2 items of width 10 each, total = 20, available = 10
            let child3 = root_ref.children[3].borrow();
            let child4 = root_ref.children[4].borrow();

            assert_eq!(child3.x, 0); // First item at start
            assert_eq!(child4.x, 20); // Last item at end (30 - 10 = 20)
        }
    }
}

/// Renders scrollbar indicators for a scrollable node.
///
/// Shows vertical scrollbar when content exceeds viewport.
fn render_scrollbars(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_scroll_offset: i16,
) {
    // Determine if scrollbar is needed
    let needs_scrollbar = node.content_height > node.height;

    // Only show scrollbar for Auto mode if content overflows
    if let Some(style) = &node.style
        && let Some(Overflow::Auto) = style.overflow
        && !needs_scrollbar
    {
        return;
    }

    // Calculate rendered position with parent scroll offset
    let rendered_y = if parent_scroll_offset > 0 {
        node.y.saturating_sub(parent_scroll_offset as u16)
    } else {
        node.y
    };
    let rendered_x = node.x;

    // Vertical scrollbar
    if needs_scrollbar && node.height > 2 {
        let scrollbar_x = rendered_x + node.width.saturating_sub(1);
        let scrollbar_height = node.height;

        // Calculate thumb position and size
        let content_ratio = node.height as f32 / node.content_height as f32;
        let thumb_height = ((scrollbar_height as f32 * content_ratio).ceil() as u16).max(1);
        let scroll_ratio =
            node.scroll_y as f32 / node.content_height.saturating_sub(node.height) as f32;
        let thumb_y = rendered_y
            + ((scrollbar_height.saturating_sub(thumb_height) as f32 * scroll_ratio) as u16);

        // Draw scrollbar track
        for y in rendered_y..rendered_y + scrollbar_height {
            if clip_rect.contains_point(scrollbar_x, y) {
                let ch = if y >= thumb_y && y < thumb_y + thumb_height {
                    '█' // Thumb
                } else {
                    '│' // Track
                };
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::BrightBlack);
                buffer.set_cell(scrollbar_x, y, cell);
            }
        }
    }
}
