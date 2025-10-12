use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page7AutoSizingDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page7AutoSizingDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 60) [
                // Title
                text("Page 7: Auto Sizing", color: bright_white),
                spacer(2),

                // Example 1: Pure auto sizing (all children auto)
                text("Example 1 - All children auto-sized (equal distribution):", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 6, pad: 1, dir: horizontal) [
                    div(bg: red, w_auto, h_frac: 1.0) [
                        text("Auto 1", color: white)
                    ],
                    div(bg: green, w_auto, h_frac: 1.0) [
                        text("Auto 2", color: black)
                    ],
                    div(bg: blue, w_auto, h_frac: 1.0) [
                        text("Auto 3", color: white)
                    ]
                ],
                spacer(2),

                // Example 2: Mixed fixed and auto
                text("Example 2 - Mixed fixed and auto (auto fills remaining):", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 6, pad: 1, dir: horizontal) [
                    div(bg: cyan, w: 10, h_frac: 1.0, pad: 1) [
                        text("Fixed 10", color: black)
                    ],
                    div(bg: magenta, w_auto, h_frac: 1.0, pad: 1) [
                        text("Auto", color: white)
                    ],
                    div(bg: yellow, w: 15, h_frac: 1.0, pad: 1) [
                        text("Fixed 15", color: black)
                    ]
                ],
                spacer(2),

                // Example 3: Mixed percentage and auto
                text("Example 3 - Mixed percentage and auto:", color: white),
                spacer(1),
                div(bg: bright_black, w: 60, h: 6, pad: 1, dir: horizontal) [
                    div(bg: bright_red, w_frac: 0.3, h_frac: 1.0, pad: 1) [
                        text("30%", color: white)
                    ],
                    div(bg: bright_green, w_auto, h_frac: 1.0, pad: 1) [
                        text("Auto", color: black)
                    ],
                    div(bg: bright_blue, w_frac: 0.2, h_frac: 1.0, pad: 1) [
                        text("20%", color: white)
                    ]
                ],
                spacer(2),

                // Example 4: Complex mix (fixed, percentage, and auto)
                text("Example 4 - Complex mix (fixed, percentage, and multiple auto):", color: white),
                spacer(1),
                div(bg: bright_black, w: 80, h: 6, pad: 1, dir: horizontal) [
                    div(bg: bright_cyan, w: 12, h_frac: 1.0, pad: 1) [
                        text("Fixed 12", color: black)
                    ],
                    div(bg: bright_magenta, w_auto, h_frac: 1.0, pad: 1) [
                        text("Auto 1", color: white)
                    ],
                    div(bg: bright_yellow, w_frac: 0.25, h_frac: 1.0, pad: 1) [
                        text("25%", color: black)
                    ],
                    div(bg: bright_red, w_auto, h_frac: 1.0, pad: 1) [
                        text("Auto 2", color: white)
                    ],
                    div(bg: bright_green, w: 8, h_frac: 1.0, pad: 1) [
                        text("Fixed 8", color: black)
                    ]
                ],
                spacer(2),

                // Example 5: Vertical auto sizing
                text("Example 5 - Vertical auto sizing:", color: white),
                spacer(1),
                div(bg: bright_black, w: 50, h: 20, pad: 1, dir: vertical) [
                    div(bg: red, w_frac: 1.0, h: 3) [
                        text("Fixed height 3", color: white)
                    ],
                    div(bg: green, w_frac: 1.0, h_auto) [
                        text("Auto height", color: black)
                    ],
                    div(bg: blue, w_frac: 1.0, h_frac: 0.3) [
                        text("30% height", color: white)
                    ],
                    div(bg: magenta, w_frac: 1.0, h_auto) [
                        text("Auto height", color: white)
                    ]
                ]
            ]
        }
    }
}
