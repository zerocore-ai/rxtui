//! Virtual DOM implementation for efficient UI updates.
//!
//! The Virtual DOM (VDom) is the core of the reactive rendering system.
//! It maintains the current UI state, performs diffing, and applies patches
//! to update the render tree efficiently.
//!
//! ## Virtual DOM Architecture
//!
//! ```text
//!   Model Render                  VDom Processing
//!   ┌─────────────┐            ┌──────────────┐
//!   │  New Node   │───render──▶│     VDom     │
//!   │    Tree     │            └──────┬───────┘
//!   └─────────────┘                   │
//!                                     ▼
//!                               ┌──────────────┐
//!                               │  Diff with   │
//!                               │Current State │
//!                               └──────┬───────┘
//!                                      │
//!                                      ▼
//!                               ┌──────────────┐
//!                               │   Generate   │
//!                               │   Patches    │
//!                               └──────┬───────┘
//!                                      │
//!                                      ▼
//!                               ┌──────────────┐
//!                               │Apply Patches │
//!                               │to RenderTree │
//!                               └──────────────┘
//! ```
//!
//! ## Update Flow
//!
//! 1. Model renders new Node tree
//! 2. VDom diffs new tree against current tree
//! 3. Diff generates minimal set of patches
//! 4. Patches are applied to update render tree
//! 5. Render tree is drawn to terminal

use crate::diff::{Patch, diff};
use crate::render_tree::{RenderNode, RenderNodeType, RenderTree};
use crate::utils::display_width;
use crate::vnode::VNode;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, atomic::AtomicBool};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Virtual DOM manager that coordinates rendering and updates.
///
/// Maintains the current virtual node tree and render tree,
/// performing efficient updates through diffing and patching.
pub struct VDom {
    /// The render tree containing positioned nodes ready for drawing
    render_tree: RenderTree,

    /// The current vnode tree representing the UI state
    current_vnode: Option<VNode>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl VDom {
    /// Creates a new empty virtual DOM.
    pub fn new() -> Self {
        Self {
            render_tree: RenderTree::new(),
            current_vnode: None,
        }
    }

    /// Renders a new node tree, updating the UI efficiently.
    ///
    /// This method:
    /// 1. Diffs the new tree against the current render tree
    /// 2. Generates patches for changes
    /// 3. Applies patches to update the render tree
    /// 4. Stores the new node as current state
    ///
    /// ## First Render vs Updates
    ///
    /// ```text
    /// First Render:           Subsequent Renders:
    /// ┌─────────┐           ┌─────────┐
    /// │  Node   │           │  Node   │
    /// └────┬────┘           └────┬────┘
    ///      │                     │
    ///      ▼                     ▼
    /// Create Full           Diff & Patch
    /// RenderTree            RenderTree
    /// ```
    pub fn render(&mut self, vnode: VNode) {
        match &self.render_tree.root {
            Some(root) => {
                let patches = diff(root, &vnode);
                self.apply_patches(patches);
            }
            None => {
                let render_node = self.create_render_node(&vnode);
                self.render_tree.set_root(render_node);
            }
        }
        self.current_vnode = Some(vnode);
    }

    /// Performs layout calculation on the render tree.
    ///
    /// Calculates positions and sizes for all nodes based on
    /// the viewport dimensions and layout rules.
    pub fn layout(&mut self, width: u16, height: u16) {
        self.render_tree.layout(width, height);
    }

    /// Performs layout calculation with additional options for inline mode.
    ///
    /// When `unclamped_height` is true, height is not clamped to viewport.
    /// This is used for inline mode where content can grow beyond viewport bounds.
    pub fn layout_with_options(&mut self, width: u16, height: u16, unclamped_height: bool) {
        self.render_tree
            .layout_with_options(width, height, unclamped_height);
    }

    /// Gets a reference to the current render tree.
    ///
    /// Used by the App to access the tree for drawing and event handling.
    pub fn get_render_tree(&self) -> &RenderTree {
        &self.render_tree
    }

    /// Returns the shared focus-clear flag for coordination with contexts.
    pub fn focus_clear_flag(&self) -> Arc<AtomicBool> {
        self.render_tree.focus_clear_flag()
    }

    /// Creates a render node from a node.
    ///
    /// Recursively converts the node tree into render nodes
    /// with styling and event handlers attached.
    fn create_render_node(&self, vnode: &VNode) -> Rc<RefCell<RenderNode>> {
        match vnode {
            VNode::Div(container) => self.create_div_node(container),
            VNode::Text(text) => self.create_text_node(text),
            VNode::RichText(rich) => self.create_rich_text_node(rich),
        }
    }

    /// Creates a render node for a div.
    ///
    /// Transfers all properties from the div including:
    /// - Style (colors, padding, direction)
    /// - Dimensions (width, height)
    /// - Event handlers (click, keyboard)
    /// - Child nodes (recursively created)
    fn create_div_node(&self, div: &crate::node::Div<VNode>) -> Rc<RefCell<RenderNode>> {
        // Create a standard element render node
        let mut render_node = RenderNode::element();

        // Copy div properties to render node
        render_node.styles = div.styles.clone();
        render_node.events = div.events.clone();
        render_node.focusable = div.focusable;
        render_node.focused = div.focused;
        render_node.hovered = div.hovered;
        render_node.component_path = div.component_path.clone();
        render_node.refresh_state_style();

        let node_rc = Rc::new(RefCell::new(render_node));

        // Process div children
        for child_vnode in &div.children {
            let child_render = match child_vnode {
                VNode::Text(text) => {
                    let mut text_node = RenderNode::text(&text.content);
                    text_node.width = display_width(&text.content) as u16;
                    text_node.height = 1;
                    // Apply text-specific style
                    if let Some(ts) = &text.style {
                        text_node.text_color = ts.color;
                        text_node.text_style = Some(ts.clone());
                        text_node.style = ts.background.map(|bg| crate::style::Style {
                            background: Some(bg),
                            ..Default::default()
                        });
                    }
                    Rc::new(RefCell::new(text_node))
                }
                VNode::RichText(rich) => {
                    let mut rich_node =
                        RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
                    rich_node.width = rich
                        .spans
                        .iter()
                        .map(|span| display_width(&span.content) as u16)
                        .sum();
                    rich_node.height = 1;

                    // Apply top-level text style if present (for wrapping, etc)
                    if let Some(ts) = &rich.style {
                        rich_node.text_style = Some(ts.clone());
                        // Extract common color if all spans have the same
                        if !rich.spans.is_empty() {
                            let first_color = rich.spans[0].style.as_ref().and_then(|s| s.color);
                            if rich.spans.iter().all(|span| {
                                span.style.as_ref().and_then(|s| s.color) == first_color
                            }) {
                                rich_node.text_color = first_color;
                            }
                        }
                    }

                    Rc::new(RefCell::new(rich_node))
                }
                VNode::Div(_) => self.create_render_node(child_vnode),
            };
            RenderNode::add_child_with_parent(&node_rc, child_render);
        }

        node_rc
    }

    /// Creates a render node for text content.
    ///
    /// Text nodes are leaf nodes that contain string content.
    fn create_text_node(&self, text: &crate::node::Text) -> Rc<RefCell<RenderNode>> {
        let mut render_node = RenderNode::text(&text.content);
        // Set proper dimensions for text nodes
        render_node.width = display_width(&text.content) as u16;
        render_node.height = 1;
        // Apply text-specific style
        if let Some(ts) = &text.style {
            render_node.text_color = ts.color;
            render_node.text_style = Some(ts.clone());
            render_node.style = ts.background.map(|bg| crate::style::Style {
                background: Some(bg),
                ..Default::default()
            });
        }
        Rc::new(RefCell::new(render_node))
    }

    /// Creates a render node for styled text content.
    ///
    /// RichText nodes contain multiple text spans with individual styling.
    fn create_rich_text_node(&self, rich: &crate::node::RichText) -> Rc<RefCell<RenderNode>> {
        let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
        // Calculate dimensions - sum of all span widths
        render_node.width = rich
            .spans
            .iter()
            .map(|span| display_width(&span.content) as u16)
            .sum();
        render_node.height = 1;

        // Apply top-level text style if present (for wrapping, etc)
        if let Some(ts) = &rich.style {
            render_node.text_style = Some(ts.clone());
            // Extract common color if all spans have the same
            if !rich.spans.is_empty() {
                let first_color = rich.spans[0].style.as_ref().and_then(|s| s.color);
                if rich
                    .spans
                    .iter()
                    .all(|span| span.style.as_ref().and_then(|s| s.color) == first_color)
                {
                    render_node.text_color = first_color;
                }
            }
        }

        Rc::new(RefCell::new(render_node))
    }

    /// Applies a list of patches to update the render tree.
    ///
    /// Patches are applied in order to transform the current
    /// render tree to match the new node tree.
    fn apply_patches(&mut self, patches: Vec<Patch>) {
        for patch in patches {
            self.apply_patch(patch);
        }
    }

    /// Applies a single patch operation to the render tree.
    ///
    /// ## Patch Types
    ///
    /// - **Replace**: Swap entire node with new one
    /// - **UpdateText**: Change text content
    /// - **UpdateProps**: Update styles/dimensions
    /// - **AddChild**: Insert new child node
    /// - **RemoveChild**: Delete child node
    fn apply_patch(&mut self, patch: Patch) {
        match patch {
            Patch::Replace { old, new } => {
                let new_render = self.create_render_node(&new);
                // Mark new node as dirty
                new_render.borrow_mut().mark_dirty();

                if let Some(parent) = &old.borrow().parent {
                    if let Some(parent_strong) = parent.upgrade() {
                        let mut parent_ref = parent_strong.borrow_mut();
                        if let Some(index) =
                            parent_ref.children.iter().position(|c| Rc::ptr_eq(c, &old))
                        {
                            parent_ref.children[index] = new_render.clone();
                            new_render.borrow_mut().parent = Some(Rc::downgrade(&parent_strong));
                        }
                        // Mark parent as dirty too
                        parent_ref.mark_dirty();
                    }
                } else {
                    self.render_tree.set_root(new_render);
                }
            }
            Patch::UpdateText {
                node,
                new_text,
                new_style,
            } => {
                let mut node_ref = node.borrow_mut();
                // Update width - height will be calculated during layout based on text wrapping
                node_ref.width = display_width(&new_text) as u16;
                node_ref.node_type = RenderNodeType::Text(new_text);

                // Update text style
                node_ref.text_style = new_style.clone();
                if let Some(ts) = &new_style {
                    node_ref.text_color = ts.color;
                    // Update background style if present
                    node_ref.style = ts.background.map(|bg| crate::style::Style {
                        background: Some(bg),
                        ..Default::default()
                    });
                } else {
                    node_ref.text_color = None;
                    // Clear background style if no text style
                    if let Some(existing_style) = &mut node_ref.style {
                        existing_style.background = None;
                    }
                }

                node_ref.mark_dirty();
            }
            Patch::UpdateRichText {
                node,
                new_spans,
                new_style,
            } => {
                let mut node_ref = node.borrow_mut();
                // Update width - height will be calculated during layout based on text wrapping
                node_ref.width = new_spans
                    .iter()
                    .map(|span| display_width(&span.content) as u16)
                    .sum();
                node_ref.node_type = RenderNodeType::RichText(new_spans);
                // Update the text style (which includes alignment)
                node_ref.text_style = new_style;
                node_ref.mark_dirty();
            }
            Patch::UpdateProps { node, div } => {
                let mut node_ref = node.borrow_mut();

                // Preserve the existing focus state from the old node
                let is_focused = node_ref.focused;
                let is_hovered = node_ref.hovered;

                // Update container properties but preserve focus state
                node_ref.styles = div.styles.clone();
                node_ref.events = div.events.clone();
                node_ref.focusable = div.focusable;
                node_ref.focused = is_focused;
                node_ref.hovered = is_hovered;
                node_ref.component_path = div.component_path.clone();
                node_ref.refresh_state_style();
                node_ref.mark_dirty();
            }
            Patch::AddChild {
                parent,
                child,
                index,
            } => {
                let child_render = self.create_render_node(&child);
                {
                    let mut parent_ref = parent.borrow_mut();
                    if index >= parent_ref.children.len() {
                        parent_ref.children.push(child_render.clone());
                    } else {
                        parent_ref.children.insert(index, child_render.clone());
                    }
                    // Mark parent as dirty since its children changed
                    parent_ref.mark_dirty();
                }
                // Set parent reference after inserting
                child_render.borrow_mut().parent = Some(Rc::downgrade(&parent));
            }
            Patch::RemoveChild { parent, index } => {
                let mut parent_ref = parent.borrow_mut();
                if index < parent_ref.children.len() {
                    parent_ref.children.remove(index);
                    // Mark parent as dirty since its children changed
                    parent_ref.mark_dirty();
                }
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for VDom {
    fn default() -> Self {
        Self::new()
    }
}
