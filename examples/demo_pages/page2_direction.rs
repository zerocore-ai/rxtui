use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page2DirectionDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page2DirectionDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 45) [
                // Title
                text("Page 2: Direction Examples", color: bright_white),
                spacer(2),

                // Example 1: Horizontal stacking
                text("Example 1 - Multiple children stacked horizontally:", color: white),
                spacer(1),
                div(bg: blue, dir: horizontal, w: 50, h: 6, pad: 1) [
                    div(bg: red, w: 10, h: 4) [
                        text("A", color: white)
                    ],
                    div(bg: green, w: 10, h: 4) [
                        text("B", color: black)
                    ],
                    div(bg: yellow, w: 10, h: 4) [
                        text("C", color: black)
                    ],
                    div(bg: magenta, w: 10, h: 4) [
                        text("D", color: white)
                    ]
                ],
                spacer(3),

                // Example 2: Vertical stacking
                text("Example 2 - Multiple children stacked vertically:", color: white),
                spacer(1),
                div(bg: cyan, dir: vertical, w: 20, h: 12, pad: 1) [
                    div(bg: red, w: 16, h: 2) [
                        text("Item 1", color: white)
                    ],
                    div(bg: green, w: 16, h: 2) [
                        text("Item 2", color: black)
                    ],
                    div(bg: yellow, w: 16, h: 2) [
                        text("Item 3", color: black)
                    ],
                    div(bg: magenta, w: 16, h: 2) [
                        text("Item 4", color: white)
                    ]
                ],
                spacer(3),

                // Example 3: Nested elements with alternating directions
                text("Example 3 - Nested elements with alternating directions:", color: white),
                spacer(1),
                div(bg: bright_black, dir: horizontal, w: 60, h: 16, pad: 1) [
                    // Left column
                    div(bg: bright_blue, dir: vertical, w: 28, h: 14, pad: 1) [
                        div(bg: bright_red, dir: horizontal, w: 24, h: 5, pad: 1) [
                            text("H1", color: white),
                            div(w: 2) [],
                            text("H2", color: white),
                            div(w: 2) [],
                            text("H3", color: white)
                        ],
                        spacer(1),
                        div(bg: bright_green, w: 24, h: 5, pad: 1) [
                            text("Vertical Child", color: black)
                        ]
                    ],
                    div(w: 2) [], // Spacer

                    // Right column
                    div(bg: bright_magenta, dir: vertical, w: 28, h: 14, pad: 1) [
                        text("V Layout", color: white),
                        spacer(1),
                        div(bg: bright_yellow, dir: horizontal, w: 24, h: 3, pad: 1) [
                            div(bg: black, w: 7, h: 1) [],
                            div(bg: bright_cyan, w: 7, h: 1) [],
                            div(bg: black, w: 7, h: 1) []
                        ],
                        spacer(1),
                        div(bg: bright_cyan, dir: vertical, w: 24, h: 4, pad: 1) [
                            text("Item A", color: black),
                            text("Item B", color: black)
                        ]
                    ]
                ]
            ]
        }
    }
}
