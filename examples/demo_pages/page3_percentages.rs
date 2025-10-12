use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page3PercentagesDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page3PercentagesDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 50) [
                // Title
                text("Page 3: Percentage Dimensions", color: bright_white),
                spacer(2),

                // Example 1: Vertical stacking with percentages
                text("Example 1 - Children with percentage heights (25%, 50%, 25%):", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 12, pad: 1, dir: vertical) [
                    div(bg: red, w_frac: 1.0, h_frac: 0.25) [
                        text("25% height", color: white)
                    ],
                    div(bg: green, w_frac: 1.0, h_frac: 0.5) [
                        text("50% height", color: black)
                    ],
                    div(bg: blue, w_frac: 1.0, h_frac: 0.25) [
                        text("25% height", color: white)
                    ]
                ],
                spacer(3),

                // Example 2: Horizontal stacking with percentages
                text("Example 2 - Children with percentage widths (20%, 30%, 50%):", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 8, pad: 1, dir: horizontal) [
                    div(bg: cyan, w_frac: 0.2, h_frac: 1.0, pad: 1) [
                        text("20%", color: black)
                    ],
                    div(bg: magenta, w_frac: 0.3, h_frac: 1.0, pad: 1) [
                        text("30%", color: white)
                    ],
                    div(bg: yellow, w_frac: 0.5, h_frac: 1.0, pad: 1) [
                        text("50%", color: black)
                    ]
                ],
                spacer(3),

                // Example 3: Mixed width and height percentages
                text("Example 3 - Grid layout with mixed width/height percentages:", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 12, pad: 1, dir: vertical) [
                    // Top row - 40% height
                    div(dir: horizontal, w_frac: 1.0, h_frac: 0.4) [
                        div(bg: bright_red, w_frac: 0.3, h_frac: 1.0, pad: 1) [
                            text("30%x40%", color: white)
                        ],
                        div(bg: bright_green, w_frac: 0.7, h_frac: 1.0, pad: 1) [
                            text("70%x40%", color: black)
                        ]
                    ],
                    // Bottom row - 60% height
                    div(dir: horizontal, w_frac: 1.0, h_frac: 0.6) [
                        div(bg: bright_blue, w_frac: 0.5, h_frac: 1.0, pad: 1) [
                            text("50%x60%", color: white)
                        ],
                        div(bg: bright_yellow, w_frac: 0.25, h_frac: 1.0, pad: 1) [
                            text("25%x60%", color: black)
                        ],
                        div(bg: bright_magenta, w_frac: 0.25, h_frac: 1.0, pad: 1) [
                            text("25%x60%", color: white)
                        ]
                    ]
                ],
                spacer(3),

                // Example 4: Mix of percentage and fixed children
                text("Example 4 - Mix of percentage and fixed-size children:", color: white),
                spacer(1),
                text("Parent has 48 content width (50 - 2 padding). Children: 10 fixed + 50% (24) + 14 fixed = 48", color: bright_black),
                spacer(1),
                div(bg: bright_black, w: 50, h: 10, pad: 1, dir: horizontal) [
                    div(bg: bright_cyan, w: 10, h_frac: 1.0, pad: 1) [
                        text("Fix 10", color: black)
                    ],
                    div(bg: bright_red, w_frac: 0.5, h_frac: 1.0, pad: 1) [
                        text("50% of parent (24)", color: white)
                    ],
                    div(bg: bright_green, w: 14, h_frac: 1.0, pad: 1) [
                        text("Fix 14", color: black)
                    ]
                ]
            ]
        }
    }
}
