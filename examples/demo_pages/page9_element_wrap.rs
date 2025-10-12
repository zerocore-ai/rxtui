use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page9ElementWrapDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page9ElementWrapDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h_frac: 1.0) [
                // Title
                text("Page 9: Element Wrapping Demo", color: cyan, bold, underline),
                text("Elements wrap to next row when container width is exceeded", color: bright_black),
                spacer(2),

                // Example 1: Simple colored boxes that wrap
                text("Example 1: Colored boxes with wrapping enabled", color: yellow, bold),
                spacer(1),
                div(bg: (Color::Rgb(20, 30, 30)), dir: horizontal, wrap: wrap, overflow: hidden, w: 45, pad: 1, gap: 1) [
                    // First row (3 boxes fit)
                    div(bg: red, w: 12, h: 3) [],
                    div(bg: green, w: 12, h: 3) [],
                    div(bg: blue, w: 12, h: 3) [],
                    // Second row (these should wrap)
                    div(bg: yellow, w: 12, h: 3) [],
                    div(bg: magenta, w: 12, h: 3) []
                ],
                spacer(2),

                // Example 2: Tags that wrap like a tag cloud
                text("Example 2: Tag cloud with variable widths", color: yellow, bold),
                spacer(1),
                div(bg: (Color::Rgb(20, 30, 30)), dir: horizontal, wrap: wrap, overflow: hidden, w: 50, pad: 1, gap: 1) [
                    div(bg: bright_blue, w: 8, h: 1, pad_h: 2) [
                        text("rust", color: white)
                    ],
                    div(bg: bright_green, w: 12, h: 1, pad_h: 2) [
                        text("terminal", color: black)
                    ],
                    div(bg: bright_yellow, w: 6, h: 1, pad_h: 2) [
                        text("ui", color: black)
                    ],
                    div(bg: bright_magenta, w: 12, h: 1, pad_h: 2) [
                        text("wrapping", color: white)
                    ],
                    div(bg: bright_cyan, w: 8, h: 1, pad_h: 2) [
                        text("flex", color: black)
                    ],
                    div(bg: bright_red, w: 10, h: 1, pad_h: 2) [
                        text("layout", color: white)
                    ],
                    div(bg: bright_blue, w: 8, h: 1, pad_h: 2) [
                        text("demo", color: white)
                    ]
                ],
                spacer(2),

                // Example 3: Comparison - With wrap vs Without wrap
                text("Example 3: Wrap vs No-Wrap comparison", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    // With wrap
                    vstack() [
                        text("With Wrap:", color: green),
                        div(bg: (Color::Rgb(20, 30, 20)), dir: horizontal, wrap: wrap, overflow: hidden, w: 22, pad: 1, gap: 1) [
                            div(bg: red, w: 6, h: 2) [],
                            div(bg: green, w: 6, h: 2) [],
                            div(bg: blue, w: 6, h: 2) [],
                            div(bg: yellow, w: 6, h: 2) [],
                            div(bg: magenta, w: 6, h: 2) [],
                            div(bg: cyan, w: 6, h: 2) []
                        ]
                    ],
                    div(w: 5) [], // Horizontal spacing

                    // Without wrap
                    vstack() [
                        text("Without Wrap (overflow):", color: red),
                        div(bg: (Color::Rgb(30, 20, 20)), dir: horizontal, wrap: nowrap, overflow: hidden, w: 22, pad: 1, gap: 1) [
                            div(bg: red, w: 6, h: 2) [],
                            div(bg: green, w: 6, h: 2) [],
                            div(bg: blue, w: 6, h: 2) [],
                            div(bg: yellow, w: 6, h: 2) [],
                            div(bg: magenta, w: 6, h: 2) [],
                            div(bg: cyan, w: 6, h: 2) []
                        ]
                    ]
                ],
                spacer(3),

                // Example 4: Different gap sizes
                text("Example 4: Different gap sizes between wrapped items", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    // Gap = 0
                    vstack() [
                        text("Gap: 0", color: bright_black),
                        div(bg: (Color::Rgb(30, 30, 30)), dir: horizontal, wrap: wrap, overflow: hidden, w: 15, pad: 1, gap: 0) [
                            div(bg: bright_red, w: 4, h: 2) [],
                            div(bg: bright_green, w: 4, h: 2) [],
                            div(bg: bright_blue, w: 4, h: 2) [],
                            div(bg: bright_yellow, w: 4, h: 2) []
                        ]
                    ],
                    div(w: 5) [], // Horizontal spacing

                    // Gap = 1
                    vstack() [
                        text("Gap: 1", color: bright_black),
                        div(bg: (Color::Rgb(30, 30, 30)), dir: horizontal, wrap: wrap, overflow: hidden, w: 15, pad: 1, gap: 1) [
                            div(bg: bright_red, w: 4, h: 2) [],
                            div(bg: bright_green, w: 4, h: 2) [],
                            div(bg: bright_blue, w: 4, h: 2) [],
                            div(bg: bright_yellow, w: 4, h: 2) []
                        ]
                    ],
                    div(w: 5) [], // Horizontal spacing

                    // Gap = 2
                    vstack() [
                        text("Gap: 2", color: bright_black),
                        div(bg: (Color::Rgb(30, 30, 30)), dir: horizontal, wrap: wrap, overflow: hidden, w: 15, pad: 1, gap: 2) [
                            div(bg: bright_red, w: 4, h: 2) [],
                            div(bg: bright_green, w: 4, h: 2) [],
                            div(bg: bright_blue, w: 4, h: 2) [],
                            div(bg: bright_yellow, w: 4, h: 2) []
                        ]
                    ]
                ]
            ]
        }
    }
}
