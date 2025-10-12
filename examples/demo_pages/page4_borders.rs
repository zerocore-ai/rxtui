use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page4BordersDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page4BordersDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 1, w_frac: 1.0, h: 60) [
                // Title
                text("Page 4: Borders Demo", color: bright_white),
                spacer(1),

                // Border style examples
                div(dir: vertical, w_frac: 0.9, h: 45) [
                    // Row 1: Single and Double
                    hstack(w_frac: 1.0, h: 8) [
                        div(w_frac: 0.48, h: 8, border: cyan, pad: 1) [
                            text("Single Border (Default)", color: cyan)
                        ],
                        div(w_frac: 0.04) [], // Spacer
                        div(
                            w_frac: 0.48,
                            h: 8,
                            border_style: double,
                            border_color: green,
                            pad: 1
                        ) [
                            text("Double Border", color: green)
                        ]
                    ],
                    spacer(1),

                    // Row 2: Thick and Rounded
                    hstack(w_frac: 1.0, h: 8) [
                        div(
                            w_frac: 0.48,
                            h: 8,
                            border_style: thick,
                            border_color: red,
                            pad: 1
                        ) [
                            text("Thick Border", color: red)
                        ],
                        div(w_frac: 0.04) [], // Spacer
                        div(
                            w_frac: 0.48,
                            h: 8,
                            border_style: rounded,
                            border_color: magenta,
                            pad: 1
                        ) [
                            text("Rounded Border", color: magenta)
                        ]
                    ],
                    spacer(1),

                    // Row 3: Dashed and Mixed Example
                    hstack(w_frac: 1.0, h: 8) [
                        div(
                            w_frac: 0.48,
                            h: 8,
                            border_style: dashed,
                            border_color: yellow,
                            pad: 1
                        ) [
                            text("Dashed Border", color: yellow)
                        ],
                        div(w_frac: 0.04) [], // Spacer
                        div(
                            w_frac: 0.48,
                            h: 8,
                            border_style: double,
                            border_color: bright_blue,
                            bg: bright_black,
                            pad: 1
                        ) [
                            text("With Background", color: bright_white)
                        ]
                    ],
                    spacer(1),

                    // Selective border edges
                    text("Selective Border Edges:", color: white),
                    spacer(1),
                    hstack(w_frac: 1.0, h: 6) [
                        div(
                            w_frac: 0.23,
                            h: 6,
                            border_style: single,
                            border_color: cyan,
                            border_edges: horizontal,
                            pad: 1
                        ) [
                            text("Horizontal", color: cyan)
                        ],
                        div(w_frac: 0.02) [], // Spacer
                        div(
                            w_frac: 0.23,
                            h: 6,
                            border_style: single,
                            border_color: green,
                            border_edges: vertical,
                            pad: 1
                        ) [
                            text("Vertical", color: green)
                        ],
                        div(w_frac: 0.02) [], // Spacer
                        div(
                            w_frac: 0.23,
                            h: 6,
                            border_style: rounded,
                            border_color: magenta,
                            border_edges: corners,
                            pad: 1
                        ) [
                            text("Corners", color: magenta)
                        ],
                        div(w_frac: 0.02) [], // Spacer
                        div(
                            w_frac: 0.23,
                            h: 6,
                            border_style: single,
                            border_color: yellow,
                            border_edges: top | right | top_right,
                            pad: 1
                        ) [
                            text("Custom", color: yellow)
                        ]
                    ],
                    spacer(1),

                    // Complex nested example with mixed border styles
                    text("Complex Nested Example with Mixed Styles:", color: white),
                    spacer(1),
                    div(
                        w_frac: 0.95,
                        h: 12,
                        border_style: double,
                        border_color: bright_blue,
                        pad: 1,
                        dir: horizontal
                    ) [
                        div(
                            w_frac: 0.3,
                            h_frac: 1.0,
                            border_style: rounded,
                            border_color: bright_green,
                            bg: bright_black,
                            border_edges: top | bottom,
                            pad: 1
                        ) [
                            text("Top/Bottom", color: bright_green)
                        ],
                        div(w: 2) [],
                        div(
                            w_frac: 0.3,
                            h_frac: 1.0,
                            border_style: thick,
                            border_color: bright_red,
                            pad: 1
                        ) [
                            text("Full Border", color: bright_red)
                        ],
                        div(w: 2) [],
                        div(
                            w_frac: 0.3,
                            h_frac: 1.0,
                            border_style: dashed,
                            border_color: bright_yellow,
                            bg: bright_black,
                            border_edges: edges,
                            pad: 1
                        ) [
                            text("No Corners", color: bright_yellow)
                        ]
                    ],
                    spacer(1),

                    // Example of no content space
                    text("Border & Padding with No Content Space:", color: white),
                    spacer(1),
                    hstack(w_frac: 1.0, h: 8) [
                        // Example with height=4, border=1x2, padding=1x2, leaving 0 height for content
                        div(w_frac: 0.3, h: 4, border: red, bg: bright_black, pad: 1) [
                            text("No space for text!", color: white)
                        ],
                        div(w_frac: 0.05) [], // Spacer
                        // Example with width too small for border+padding
                        div(
                            w: 6,
                            h: 6,
                            border_style: double,
                            border_color: yellow,
                            bg: bright_black,
                            overflow: hidden,
                            pad: 1
                        ) [
                            text("Squished!", color: yellow)
                        ],
                        div(w_frac: 0.05) [], // Spacer
                        // Extreme case: exactly border+padding size
                        div(
                            w: 4,
                            h: 4,
                            border_style: thick,
                            border_color: cyan,
                            overflow: hidden,
                            bg: bright_black,
                            pad: 1
                        ) [
                            text("Gone!", color: cyan)
                        ]
                    ]
                ]
            ]
        }
    }
}
