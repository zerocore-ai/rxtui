use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ScrollDemoMsg {
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    ResetScroll,
}

#[derive(Debug, Clone)]
struct ScrollDemoState {
    info_text: String,
}

#[derive(Component)]
pub struct Page15ScrollableDemo;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ScrollDemoState {
    fn default() -> Self {
        Self {
            info_text: "Use arrow keys, Page Up/Down, Home/End to scroll. Mouse wheel also works!"
                .to_string(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page15ScrollableDemo {
    #[update]
    fn update(&self, ctx: &Context, msg: ScrollDemoMsg, mut state: ScrollDemoState) -> Action {
        match msg {
            ScrollDemoMsg::ScrollUp => {
                state.info_text = "Scrolled up!".to_string();
            }
            ScrollDemoMsg::ScrollDown => {
                state.info_text = "Scrolled down!".to_string();
            }
            ScrollDemoMsg::ScrollLeft => {
                state.info_text = "Scrolled left!".to_string();
            }
            ScrollDemoMsg::ScrollRight => {
                state.info_text = "Scrolled right!".to_string();
            }
            ScrollDemoMsg::ResetScroll => {
                state.info_text = "Scroll reset!".to_string();
            }
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 60) [
                // Title
                text("Page 15: Scrollable Content - Use arrow keys or mouse wheel to scroll", color: bright_white),
                spacer(1),
                text("Click on a scrollable area to focus it, then use arrow keys to scroll", color: bright_yellow),
                spacer(2),

                // Example 1: Basic scrolling with scrollbar
                text("Example 1 - Basic Vertical Scrolling (with scrollbar):", color: white),
                spacer(1),
                div(dir: horizontal, gap: 2) [
                    // Basic vertical scrolling with scrollbar (default)
                    div(bg: blue, overflow: scroll, border: yellow, w: 50, h: 10, pad: 1, focusable) [
                        text("=== With Scrollbar (default) ===", color: bright_yellow),
                        text("Line 1: Start of content", color: white),
                        text("Line 2: Use ↑↓ arrow keys to scroll", color: white),
                        text("Line 3: Or use mouse wheel", color: white),
                        text("Line 4: Page Up/Down for larger jumps", color: white),
                        text("Line 5: Home key to go to top", color: white),
                        text("Line 6: End key to go to bottom", color: white),
                        text("Line 7: Content continues...", color: white),
                        text("Line 8: More lines here", color: white),
                        text("Line 9: Keep scrolling", color: white),
                        text("Line 10: Even more content", color: white),
                        text("Line 11: Still more to see", color: white),
                        text("Line 12: Almost there", color: white),
                        text("Line 13: Getting close", color: white),
                        text("Line 14: Nearly done", color: white),
                        text("Line 15: One more", color: white),
                        text("Line 16: Second to last", color: white),
                        text("Line 17: Last line!", color: bright_green),
                        text("=== End of Content ===", color: bright_yellow)
                    ],

                    // Basic vertical scrolling without scrollbar
                    div(bg: green, overflow: scroll, border: cyan, w: 50, h: 10, pad: 1, focusable, show_scrollbar: false) [
                        text("=== Without Scrollbar ===", color: bright_cyan),
                        text("Line 1: Start of content", color: white),
                        text("Line 2: Scrolling works", color: white),
                        text("Line 3: But no scrollbar shown", color: white),
                        text("Line 4: Cleaner look", color: white),
                        text("Line 5: More content", color: white),
                        text("Line 6: Keep scrolling", color: white),
                        text("Line 7: Even more content", color: white),
                        text("Line 8: Almost there", color: white),
                        text("Line 9: Nearly done", color: white),
                        text("Line 10: Last line!", color: bright_green),
                        text("=== End of Content ===", color: bright_cyan)
                    ]
                ],
                spacer(2),

                // Example 2: Nested scrolling with mixed scrollbar settings
                text("Example 2 - Nested Scrollable Areas:", color: white),
                spacer(1),
                div(bg: magenta, overflow: scroll, border: cyan, w: 80, h: 20, pad: 1, focusable) [
                        text("=== Nested Scrollable Container ===", color: bright_cyan),
                        text("This outer container scrolls", color: white),
                        spacer(1),

                        // First nested scrollable (with scrollbar)
                        div(bg: green, overflow: scroll, border: white, w: 60, h: 6, pad: 1, focusable) [
                            text("--- Inner Scroll Area 1 ---", color: bright_white),
                            text("Nested Item 1.1", color: yellow),
                            text("Nested Item 1.2", color: yellow),
                            text("Nested Item 1.3", color: yellow),
                            text("Nested Item 1.4", color: yellow),
                            text("Nested Item 1.5", color: yellow),
                            text("Nested Item 1.6", color: yellow),
                            text("Nested Item 1.7", color: yellow),
                            text("Nested Item 1.8", color: yellow),
                            text("Nested Item 1.9", color: yellow),
                            text("Nested Item 1.10", color: yellow),
                            text("--- End of Inner 1 ---", color: bright_white)
                        ],

                        spacer(1),
                        text("More content in outer container", color: white),
                        spacer(1),

                        // Second nested scrollable (without scrollbar)
                        div(bg: red, overflow: scroll, border: white, w: 60, h: 6, pad: 1, focusable, show_scrollbar: false) [
                            text("--- Inner Scroll Area 2 (No Scrollbar) ---", color: bright_white),
                            text("Nested Item 2.1", color: cyan),
                            text("Nested Item 2.2", color: cyan),
                            text("Nested Item 2.3", color: cyan),
                            text("Nested Item 2.4", color: cyan),
                            text("Nested Item 2.5", color: cyan),
                            text("Nested Item 2.6", color: cyan),
                            text("Nested Item 2.7", color: cyan),
                            text("Nested Item 2.8", color: cyan),
                            text("Nested Item 2.9", color: cyan),
                            text("Nested Item 2.10", color: cyan),
                            text("--- End of Inner 2 ---", color: bright_white)
                        ],

                        spacer(1),
                        text("Even more outer content", color: white),
                        text("Keep scrolling the outer container", color: white),
                        text("To see all nested scroll areas", color: white),
                        text("Each area scrolls independently", color: white),
                        text("=== End of Nested Container ===", color: bright_cyan)
                ],
                spacer(2),

                // Example 3: Auto mode with and without scrollbar
                text("Example 3 - Auto Overflow Mode:", color: white),
                spacer(1),
                div(dir: horizontal, gap: 2) [
                    div(bg: bright_black, overflow: auto, border: yellow, w: 40, h: 5, pad: 1, focusable) [
                        text("Auto mode with scrollbar", color: bright_yellow),
                        text("Shows scrollbar only when needed", color: white),
                        text("Line 3", color: white),
                        text("Line 4", color: white),
                        text("Line 5", color: white),
                        text("Line 6 - overflow content", color: white),
                        text("Line 7 - more overflow", color: white)
                    ],
                    div(bg: bright_black, overflow: auto, border: magenta, w: 40, h: 5, pad: 1, focusable, show_scrollbar: false) [
                        text("Auto mode without scrollbar", color: bright_magenta),
                        text("Never shows scrollbar", color: white),
                        text("Line 3", color: white),
                        text("Line 4", color: white),
                        text("Line 5", color: white),
                        text("Line 6 - overflow content", color: white),
                        text("Line 7 - more overflow", color: white)
                    ]
                ]
            ]
        }
    }
}
