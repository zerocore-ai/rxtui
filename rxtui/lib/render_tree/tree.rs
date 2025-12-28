use crate::bounds::Rect;
use crate::component::ComponentId;
use crate::render_tree::node::{RenderNode, RenderNodeType};
use crate::style::{Dimension, Direction, Overflow};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Container for the render tree with layout capabilities.
///
/// The render tree maintains the root node and provides
/// methods for layout calculation and hit testing.
pub struct RenderTree {
    /// The root node of the render tree
    pub root: Option<Rc<RefCell<RenderNode>>>,

    /// The currently focused node (uses RefCell for interior mutability)
    focused_node: RefCell<Option<Rc<RefCell<RenderNode>>>>,

    /// The currently hovered node (uses RefCell for interior mutability)
    hovered_node: RefCell<Option<Rc<RefCell<RenderNode>>>>,

    /// Tracks whether a focus clear has been requested this frame
    pending_focus_clear: Arc<AtomicBool>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RenderTree {
    /// Creates a new empty render tree.
    pub fn new() -> Self {
        Self {
            root: None,
            focused_node: RefCell::new(None),
            hovered_node: RefCell::new(None),
            pending_focus_clear: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns the shared flag controlling pending focus clears.
    pub fn focus_clear_flag(&self) -> Arc<AtomicBool> {
        self.pending_focus_clear.clone()
    }

    /// Returns a debug string representation of the render tree.
    ///
    /// This recursively prints the tree structure with indentation showing
    /// the hierarchy and node details like position, size, and type.
    pub fn debug_string(&self) -> String {
        match &self.root {
            Some(root) => {
                let mut output = String::new();
                output.push_str("=== Render Tree ===\n");
                Self::debug_node(&root.borrow(), &mut output, 0);
                output.push_str("==================\n");
                output
            }
            None => "=== Render Tree ===\n(empty)\n==================\n".to_string(),
        }
    }

    /// Recursively builds debug string for a node and its children.
    fn debug_node(node: &RenderNode, output: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);

        // Node type and position
        match &node.node_type {
            RenderNodeType::Element => {
                output.push_str(&format!(
                    "{}Div @ ({}, {}) [{}x{}]",
                    indent, node.x, node.y, node.width, node.height
                ));
            }
            RenderNodeType::Text(content) => {
                output.push_str(&format!(
                    "{}Text @ ({}, {}) [{}x{}]: \"{}\"",
                    indent,
                    node.x,
                    node.y,
                    node.width,
                    node.height,
                    content.replace('\n', "\\n")
                ));
            }
            RenderNodeType::TextWrapped(lines) => {
                output.push_str(&format!(
                    "{}TextWrapped @ ({}, {}) [{}x{}]: {} lines",
                    indent,
                    node.x,
                    node.y,
                    node.width,
                    node.height,
                    lines.len()
                ));
                for line in lines {
                    output.push_str(&format!("\n{}  \"{}\"", indent, line.replace('\n', "\\n")));
                }
            }
            RenderNodeType::RichText(spans) => {
                output.push_str(&format!(
                    "{}RichText @ ({}, {}) [{}x{}]: {} spans",
                    indent,
                    node.x,
                    node.y,
                    node.width,
                    node.height,
                    spans.len()
                ));
                for span in spans {
                    output.push_str(&format!(
                        "\n{}  \"{}\"",
                        indent,
                        span.content.replace('\n', "\\n")
                    ));
                }
            }
            RenderNodeType::RichTextWrapped(lines) => {
                output.push_str(&format!(
                    "{}RichTextWrapped @ ({}, {}) [{}x{}]: {} lines",
                    indent,
                    node.x,
                    node.y,
                    node.width,
                    node.height,
                    lines.len()
                ));
                for (i, line) in lines.iter().enumerate() {
                    output.push_str(&format!(
                        "\n{}  Line {}: {} spans",
                        indent,
                        i + 1,
                        line.len()
                    ));
                    for span in line {
                        output.push_str(&format!(
                            "\n{}    \"{}\"",
                            indent,
                            span.content.replace('\n', "\\n")
                        ));
                    }
                }
            }
        }

        // Style info
        if let Some(style) = &node.style {
            if let Some(bg) = &style.background {
                output.push_str(&format!(" bg:{bg:?}"));
            }
            if let Some(dir) = &style.direction {
                output.push_str(&format!(" dir:{dir:?}"));
            }
            if let Some(padding) = &style.padding {
                output.push_str(&format!(
                    " pad:({},{},{},{})",
                    padding.top, padding.right, padding.bottom, padding.left
                ));
            }
            if let Some(overflow) = &style.overflow {
                output.push_str(&format!(" overflow:{overflow:?}"));
            }
        }

        // Text color for text nodes
        if matches!(
            &node.node_type,
            RenderNodeType::Text(_) | RenderNodeType::TextWrapped(_)
        ) && let Some(color) = &node.text_color
        {
            output.push_str(&format!(" color:{color:?}"));
        }

        // Dirty flag
        if node.dirty {
            output.push_str(" [DIRTY]");
        }

        output.push('\n');

        // Recursively print children
        for child in &node.children {
            Self::debug_node(&child.borrow(), output, depth + 1);
        }
    }

    /// Sets the root node of the render tree.
    pub fn set_root(&mut self, root: Rc<RefCell<RenderNode>>) {
        self.root = Some(root);
    }

    /// Performs layout for the entire tree within the given viewport.
    ///
    /// Respects the root node's specified dimensions if set, otherwise
    /// uses the viewport size. Clamps dimensions to viewport bounds.
    pub fn layout(&mut self, viewport_width: u16, viewport_height: u16) {
        self.layout_with_options(viewport_width, viewport_height, false);
    }

    /// Performs layout with additional options for inline rendering mode.
    ///
    /// When `unclamped_height` is true, height is not clamped to the viewport.
    /// This is used for inline mode where content can grow beyond viewport bounds.
    pub fn layout_with_options(
        &mut self,
        viewport_width: u16,
        viewport_height: u16,
        unclamped_height: bool,
    ) {
        if let Some(root) = &self.root {
            let mut root_ref = root.borrow_mut();
            root_ref.set_position(0, 0);

            // Calculate intrinsic size for content-based dimensions
            let (intrinsic_width, intrinsic_height) = root_ref.calculate_intrinsic_size();

            // For the root node, resolve dimensions using viewport as parent
            if let Some(style) = &root_ref.style {
                // Clone the dimension values to avoid borrow checker issues
                let width_dim = style.width;
                let height_dim = style.height;

                // Resolve width (always clamped to viewport)
                match width_dim {
                    Some(Dimension::Fixed(w)) => {
                        root_ref.width = w.min(viewport_width);
                    }
                    Some(Dimension::Percentage(pct)) => {
                        let calculated_width = (viewport_width as f32 * pct) as u16;
                        root_ref.width = calculated_width.max(1).min(viewport_width);
                    }
                    Some(Dimension::Content) => {
                        // Use intrinsic width, capped at viewport
                        root_ref.width = intrinsic_width.min(viewport_width);
                    }
                    Some(Dimension::Auto) => {
                        // For root element, auto means full viewport width
                        root_ref.width = viewport_width;
                    }
                    None => {
                        // No dimension specified - use intrinsic size
                        root_ref.width = intrinsic_width.min(viewport_width);
                    }
                }

                // Resolve height (optionally unclamped for inline mode)
                match height_dim {
                    Some(Dimension::Fixed(h)) => {
                        root_ref.height = if unclamped_height {
                            h
                        } else {
                            h.min(viewport_height)
                        };
                    }
                    Some(Dimension::Percentage(pct)) => {
                        let calculated_height = (viewport_height as f32 * pct) as u16;
                        root_ref.height = if unclamped_height {
                            calculated_height.max(1)
                        } else {
                            calculated_height.max(1).min(viewport_height)
                        };
                    }
                    Some(Dimension::Content) => {
                        // Use intrinsic height, optionally capped at viewport
                        root_ref.height = if unclamped_height {
                            intrinsic_height
                        } else {
                            intrinsic_height.min(viewport_height)
                        };
                    }
                    Some(Dimension::Auto) => {
                        // For root element, auto means full viewport height
                        root_ref.height = viewport_height;
                    }
                    None => {
                        // No dimension specified - use intrinsic size
                        root_ref.height = if unclamped_height {
                            intrinsic_height
                        } else {
                            intrinsic_height.min(viewport_height)
                        };
                    }
                }
            } else {
                // No style - use intrinsic dimensions, optionally capped at viewport
                root_ref.width = intrinsic_width.min(viewport_width);
                root_ref.height = if unclamped_height {
                    intrinsic_height
                } else {
                    intrinsic_height.min(viewport_height)
                };
            }

            // Layout children with root's resolved dimensions
            let direction = root_ref
                .style
                .as_ref()
                .and_then(|s| s.direction)
                .unwrap_or(Direction::Vertical);
            root_ref.layout_children_with_parent(direction);
        }
    }

    /// Finds the topmost node at the given terminal coordinates.
    ///
    /// Used for mouse event handling. Returns the deepest node
    /// in the tree that contains the given point.
    pub fn find_node_at(&self, x: u16, y: u16) -> Option<Rc<RefCell<RenderNode>>> {
        if let Some(root) = &self.root {
            // Start with no clipping and no scroll offset
            Self::find_node_at_recursive(root, x, y, None, 0)
        } else {
            None
        }
    }

    /// Recursively searches for a node containing the given point.
    ///
    /// Performs depth-first search, checking children before parents
    /// to ensure the topmost (visually) node is returned.
    /// Respects overflow clipping - nodes with overflow:hidden will
    /// clip their children's click areas.
    /// Text nodes are transparent to clicks and pass events to their parent.
    fn find_node_at_recursive(
        node: &Rc<RefCell<RenderNode>>,
        x: u16,
        y: u16,
        clip_rect: Option<Rect>,
        parent_scroll_offset: i16,
    ) -> Option<Rc<RefCell<RenderNode>>> {
        let node_ref = node.borrow();

        // Calculate the actual rendered position with parent scroll offset
        let rendered_y = if parent_scroll_offset > 0 {
            node_ref.y.saturating_sub(parent_scroll_offset as u16)
        } else {
            node_ref.y
        };
        let rendered_x = node_ref.x;

        // Get bounds with scroll offset applied
        let node_bounds = Rect::new(rendered_x, rendered_y, node_ref.width, node_ref.height);

        // Check if this node is clickable
        let is_node_clickable = if let Some(ref clip) = clip_rect {
            // If we have a clip rect, the node must be within both its bounds and the clip
            node_bounds.contains_point(x, y) && clip.contains_point(x, y)
        } else {
            // If no clip rect, just check if point is within node bounds
            node_bounds.contains_point(x, y)
        };

        // Calculate clip rect for children based on overflow setting
        let child_clip = if let Some(style) = &node_ref.style {
            match style.overflow {
                Some(Overflow::Hidden) | Some(Overflow::Scroll) | Some(Overflow::Auto) => {
                    // Clip children at the padding edge for scrollable/hidden containers
                    if let Some(ref existing_clip) = clip_rect {
                        Some(node_bounds.intersection(existing_clip))
                    } else {
                        Some(node_bounds)
                    }
                }
                _ => {
                    // If overflow is none, pass through the existing clip rect
                    clip_rect
                }
            }
        } else {
            // No style, pass through the existing clip rect
            clip_rect
        };

        // Calculate scroll offset to pass to children
        let child_scroll_offset = if node_ref.scrollable {
            parent_scroll_offset + node_ref.scroll_y as i16
        } else {
            parent_scroll_offset
        };

        // Always check children first, even if this node isn't clickable
        // This is important for overflow:none where children can extend outside
        for child in &node_ref.children {
            if let Some(found) =
                Self::find_node_at_recursive(child, x, y, child_clip, child_scroll_offset)
            {
                // Check if the found child is a text node
                let found_ref = found.borrow();
                if matches!(
                    found_ref.node_type,
                    RenderNodeType::Text(_) | RenderNodeType::TextWrapped(_)
                ) {
                    // Text nodes are transparent to clicks, don't return them
                    drop(found_ref);
                    continue;
                }
                drop(found_ref);
                return Some(found);
            }
        }

        // Only return this node if it's clickable and no child matched
        // Text nodes should never be returned as click targets
        if is_node_clickable
            && !matches!(
                node_ref.node_type,
                RenderNodeType::Text(_) | RenderNodeType::TextWrapped(_)
            )
        {
            drop(node_ref); // Release borrow before cloning
            return Some(node.clone());
        }

        None
    }

    /// Collects all dirty regions in the render tree.
    ///
    /// Returns a vector of rectangles representing areas that need redrawing.
    /// Adjacent rectangles are merged for efficiency.
    pub fn collect_dirty_regions(&self) -> Vec<Rect> {
        let mut regions = Vec::new();
        if let Some(root) = &self.root {
            Self::collect_dirty_regions_recursive(root, &mut regions);
        }
        // Merge overlapping regions
        self.merge_regions(regions)
    }

    /// Recursively collects dirty regions from a node and its children.
    fn collect_dirty_regions_recursive(node: &Rc<RefCell<RenderNode>>, regions: &mut Vec<Rect>) {
        let node_ref = node.borrow();
        if node_ref.dirty {
            regions.push(node_ref.bounds());
        }
        for child in &node_ref.children {
            Self::collect_dirty_regions_recursive(child, regions);
        }
    }

    /// Merges overlapping or adjacent rectangles to minimize redraw operations.
    fn merge_regions(&self, mut regions: Vec<Rect>) -> Vec<Rect> {
        if regions.is_empty() {
            return regions;
        }

        // Simple merge algorithm - can be optimized later
        let mut merged = Vec::new();
        regions.sort_by_key(|r| (r.y, r.x));

        let mut current = regions[0];
        for region in regions.into_iter().skip(1) {
            if current.intersects(&region) || self.are_adjacent(&current, &region) {
                current = current.union(&region);
            } else {
                merged.push(current);
                current = region;
            }
        }
        merged.push(current);
        merged
    }

    /// Checks if two rectangles are adjacent (touching but not overlapping).
    fn are_adjacent(&self, a: &Rect, b: &Rect) -> bool {
        // Horizontally adjacent
        (a.right() == b.x || b.right() == a.x) &&
        !(a.bottom() <= b.y || b.bottom() <= a.y) ||
        // Vertically adjacent
        (a.bottom() == b.y || b.bottom() == a.y) &&
        !(a.right() <= b.x || b.right() <= a.x)
    }

    /// Marks all nodes in the tree as clean (not dirty).
    pub fn clear_all_dirty(&self) {
        if let Some(root) = &self.root {
            Self::clear_dirty_recursive(root);
        }
    }

    /// Recursively clears dirty flags in the tree.
    fn clear_dirty_recursive(node: &Rc<RefCell<RenderNode>>) {
        let mut node_ref = node.borrow_mut();
        node_ref.clear_dirty();
        let children = node_ref.children.clone();
        drop(node_ref);
        for child in children {
            Self::clear_dirty_recursive(&child);
        }
    }

    //--------------------------------------------------------------------------------------------------
    // Focus Management
    //--------------------------------------------------------------------------------------------------

    /// Collects all focusable nodes in the tree in tab order (depth-first traversal).
    pub fn collect_focusable_nodes(&self) -> Vec<Rc<RefCell<RenderNode>>> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            Self::collect_focusable_recursive(root, &mut nodes);
        }
        nodes
    }

    /// Finds the render node that corresponds to the given component root.
    pub fn find_component_root(
        &self,
        component_id: &ComponentId,
    ) -> Option<Rc<RefCell<RenderNode>>> {
        self.root
            .as_ref()
            .and_then(|root| Self::find_component_root_recursive(root, component_id))
    }

    /// Finds the first focusable render node within the given subtree.
    pub fn find_first_focusable_in(
        &self,
        node: &Rc<RefCell<RenderNode>>,
    ) -> Option<Rc<RefCell<RenderNode>>> {
        Self::find_first_focusable_recursive(node)
    }

    /// Finds the first focusable render node in the entire tree.
    pub fn find_first_focusable_global(&self) -> Option<Rc<RefCell<RenderNode>>> {
        self.root
            .as_ref()
            .and_then(Self::find_first_focusable_recursive)
    }

    /// Recursively collects focusable nodes.
    fn collect_focusable_recursive(
        node: &Rc<RefCell<RenderNode>>,
        nodes: &mut Vec<Rc<RefCell<RenderNode>>>,
    ) {
        let node_ref = node.borrow();

        // Add this node if it's focusable
        if node_ref.focusable {
            nodes.push(node.clone());
        }

        // Recurse through children
        let children = node_ref.children.clone();
        drop(node_ref); // Release borrow before recursing
        for child in &children {
            Self::collect_focusable_recursive(child, nodes);
        }
    }

    /// Recursively finds the component root render node.
    fn find_component_root_recursive(
        node: &Rc<RefCell<RenderNode>>,
        component_id: &ComponentId,
    ) -> Option<Rc<RefCell<RenderNode>>> {
        let (matches_component, children) = {
            let node_ref = node.borrow();
            (
                node_ref
                    .component_path
                    .as_ref()
                    .map(|path| path == component_id)
                    .unwrap_or(false),
                node_ref.children.clone(),
            )
        };

        if matches_component {
            return Some(node.clone());
        }

        for child in &children {
            if let Some(found) = Self::find_component_root_recursive(child, component_id) {
                return Some(found);
            }
        }

        None
    }

    /// Recursively finds the first focusable node in a subtree.
    fn find_first_focusable_recursive(
        node: &Rc<RefCell<RenderNode>>,
    ) -> Option<Rc<RefCell<RenderNode>>> {
        let (is_focusable, children) = {
            let node_ref = node.borrow();
            (node_ref.focusable, node_ref.children.clone())
        };

        if is_focusable {
            return Some(node.clone());
        }

        for child in &children {
            if let Some(found) = Self::find_first_focusable_recursive(child) {
                return Some(found);
            }
        }

        None
    }

    /// Gets the currently focused node.
    pub fn get_focused_node(&self) -> Option<Rc<RefCell<RenderNode>>> {
        self.focused_node.borrow().clone()
    }

    /// Sets the focused node and updates the focused flags.
    pub fn set_focused_node(&self, node: Option<Rc<RefCell<RenderNode>>>) {
        let current = self.focused_node.borrow().clone();

        let is_same_node = match (&current, &node) {
            (Some(old), Some(new)) => Rc::ptr_eq(old, new),
            _ => false,
        };

        if is_same_node {
            return;
        }

        if let Some(old_focused) = current {
            let mut old_ref = old_focused.borrow_mut();
            if let Some(on_blur) = &old_ref.events.on_blur {
                on_blur();
            }
            old_ref.focused = false;
            old_ref.refresh_state_style();
        }

        if let Some(new_focused) = &node {
            let mut new_ref = new_focused.borrow_mut();
            new_ref.focused = true;

            if let Some(on_focus) = &new_ref.events.on_focus {
                on_focus();
            }

            new_ref.refresh_state_style();
        }

        // Reset pending clear whenever focus moves or is explicitly cleared
        self.pending_focus_clear.store(false, Ordering::SeqCst);

        *self.focused_node.borrow_mut() = node;
    }

    /// Sets the hovered node and updates hover flags/styles.
    pub fn set_hovered_node(&self, node: Option<Rc<RefCell<RenderNode>>>) {
        let current = self.hovered_node.borrow().clone();

        let is_same_node = match (&current, &node) {
            (Some(old), Some(new)) => Rc::ptr_eq(old, new),
            _ => false,
        };

        if is_same_node {
            return;
        }

        if let Some(old_hovered) = current {
            let mut old_ref = old_hovered.borrow_mut();
            old_ref.hovered = false;
            old_ref.refresh_state_style();
        }

        if let Some(new_hovered) = &node {
            let mut new_ref = new_hovered.borrow_mut();
            new_ref.hovered = true;
            new_ref.refresh_state_style();
        }

        *self.hovered_node.borrow_mut() = node;
    }

    /// Moves focus to the next focusable element.
    pub fn focus_next(&self) {
        let focusable = self.collect_focusable_nodes();
        if focusable.is_empty() {
            return;
        }

        let current_focused = self.get_focused_node();

        // Find current index or start at -1 if nothing focused
        let current_idx = if let Some(current) = &current_focused {
            focusable.iter().position(|n| Rc::ptr_eq(n, current))
        } else {
            None
        };

        // Calculate next index
        let next_idx = match current_idx {
            Some(idx) => (idx + 1) % focusable.len(),
            None => 0, // Focus first element if nothing focused
        };

        self.set_focused_node(Some(focusable[next_idx].clone()));
    }

    /// Moves focus to the previous focusable element.
    pub fn focus_prev(&self) {
        let focusable = self.collect_focusable_nodes();
        if focusable.is_empty() {
            return;
        }

        let current_focused = self.get_focused_node();

        // Find current index or start at 0 if nothing focused
        let current_idx = if let Some(current) = &current_focused {
            focusable.iter().position(|n| Rc::ptr_eq(n, current))
        } else {
            None
        };

        // Calculate previous index
        let prev_idx = match current_idx {
            Some(idx) => {
                if idx == 0 {
                    focusable.len() - 1
                } else {
                    idx - 1
                }
            }
            None => focusable.len() - 1, // Focus last element if nothing focused
        };

        self.set_focused_node(Some(focusable[prev_idx].clone()));
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RenderTree {
    fn default() -> Self {
        Self::new()
    }
}
