use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page11ContentSizingDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page11ContentSizingDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 60) [
                // Title
                text("Page 11: Content-Based Sizing", color: bright_white, bold),
                spacer(1),
                text("Elements without dimensions grow to fit their content!", color: bright_cyan),
                spacer(2),

                // Example 1: Simple text in container
                text("Example 1 - Container grows to text:", color: white),
                spacer(1),
                div(bg: bright_black, pad: 1) [
                    // No width/height - sizes to content!
                    div(bg: red, pad: 1) [
                        text("This container fits the text perfectly!", color: white)
                    ]
                ],
                spacer(2),

                // Example 2: Horizontal layout grows to children
                text("Example 2 - Horizontal container (width = sum of children):", color: white),
                spacer(1),
                div(bg: bright_black, dir: horizontal, pad: 1) [
                    // No dimensions - automatically sizes to children
                    div(bg: blue, pad: 1) [
                        text("Box 1", color: white)
                    ],
                    div(bg: green, pad: 1) [
                        text("Box 2 is wider", color: black)
                    ],
                    div(bg: magenta, pad: 1) [
                        text("Box 3", color: white)
                    ]
                ],
                spacer(2),

                // Example 3: Vertical layout grows to children
                text("Example 3 - Vertical container (height = sum, width = max):", color: white),
                spacer(1),
                div(bg: bright_black, dir: vertical, pad: 1) [
                    // No dimensions specified
                    div(bg: cyan, pad_h: 1) [
                        text("Short line", color: black)
                    ],
                    div(bg: yellow, pad_h: 1) [
                        text("This is the longest line in the stack", color: black)
                    ],
                    div(bg: bright_red, pad_h: 1) [
                        text("Medium line here", color: white)
                    ]
                ],
                spacer(2),

                // Example 4: Mixed sizing modes
                text("Example 4 - Mixed sizing (content + fixed + percentage):", color: white),
                spacer(1),
                div(bg: bright_black, dir: horizontal, pad: 1, w: 80, h: 5) [
                    // Parent has fixed width
                    // Content-based width
                    div(bg: bright_green, h_frac: 1.0, pad: 1) [
                        text("Content", color: black)
                    ],
                    // Fixed width
                    div(bg: bright_blue, w: 15, h_frac: 1.0, pad: 1) [
                        text("Fixed 15", color: white)
                    ],
                    // Auto width (remaining space)
                    div(bg: bright_magenta, w_auto, h_frac: 1.0, pad: 1) [
                        text("Auto (fills remaining)", color: white)
                    ]
                ],
                spacer(2),

                // Example 5: Deeply nested content sizing
                text("Example 5 - Nested containers all size to content:", color: white),
                spacer(1),
                div(bg: bright_black, pad: 1) [
                    div(bg: bright_cyan, pad: 1) [
                        div(bg: bright_yellow, pad: 1) [
                            div(bg: black, pad: 1) [
                                text("Deeply nested content", color: bright_white, bold)
                            ]
                        ]
                    ]
                ],
                spacer(2),

                // Example 6: Explicit content dimension
                text("Example 6 - Explicit .width_content() and .height_content():", color: white),
                spacer(1),
                div(bg: bright_black, pad: 1) [
                    div(bg: bright_red, w_content, h: 5, pad: 1, dir: vertical) [
                        // Explicitly use content width
                        // But fixed height
                        text("Width from content", color: white),
                        text("Height is fixed at 5", color: bright_white)
                    ]
                ],
                spacer(2),

                // Example 7: Fixed width with text wrapping
                text("Example 7 - Fixed width with text wrapping (height grows):", color: white),
                spacer(1),
                div(bg: bright_black, pad: 1) [
                    div(bg: bright_cyan, w: 30, pad: 1) [
                        // Fixed width of 30
                        // No height specified - grows to fit wrapped text
                        text(
                            "This is a long piece of text that will wrap when it exceeds the fixed width of 30 characters. The container height will automatically grow to accommodate all the wrapped lines!",
                            color: black,
                            wrap: word
                        )
                    ]
                ],
                spacer(2),

                // Example 8: Multiple wrapped text blocks in horizontal layout
                text("Example 8 - Multiple wrapped texts side by side:", color: white),
                spacer(1),
                div(bg: bright_black, dir: horizontal, pad: 1) [
                    // Parent has no dimensions - grows to fit children
                    div(bg: bright_green, w: 25, pad: 1) [
                        // Fixed width, height will grow to fit wrapped content
                        text(
                            "This text wraps within a 25 character width container.",
                            color: black,
                            wrap: word
                        )
                    ],
                    div(bg: bright_yellow, w: 20, pad: 1) [
                        // Fixed width, height grows to content
                        text(
                            "Shorter width means more wrapping needed here.",
                            color: black,
                            wrap: word
                        )
                    ],
                    div(bg: bright_magenta, w: 15, pad: 1) [
                        // Fixed width
                        text(
                            "Very narrow column with lots of text wrapping.",
                            color: white,
                            wrap: character
                        )
                    ]
                ],
                spacer(2),

                // Example 9: Element wrapping with fixed width
                text("Example 9 - Element wrapping (fixed width, content height):", color: white),
                spacer(1),
                div(bg: bright_black, pad: 1) [
                    div(bg: bright_blue, w: 40, dir: horizontal, wrap: wrap, gap: 1, pad: 1) [
                        // Fixed width, no height - grows to fit wrapped elements
                        div(bg: red, pad_h: 2) [
                            text("Item 1", color: white)
                        ],
                        div(bg: green, pad_h: 2) [
                            text("Item 2", color: black)
                        ],
                        div(bg: yellow, pad_h: 2) [
                            text("Item 3", color: black)
                        ],
                        div(bg: magenta, pad_h: 2) [
                            text("Item 4", color: white)
                        ],
                        div(bg: cyan, pad_h: 2) [
                            text("Item 5", color: black)
                        ]
                    ]
                ]
            ]
        }
    }
}
