use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum AbsoluteDemoMsg {
    ToggleModal,
    SetSelectedLayer(i32),
}

#[derive(Debug, Clone, Default)]
struct AbsoluteDemoState {
    show_modal: bool,
    selected_layer: i32,
}

#[derive(Component)]
pub struct Page5AbsoluteDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page5AbsoluteDemo {
    #[update]
    fn update(&self, ctx: &Context, msg: AbsoluteDemoMsg, mut state: AbsoluteDemoState) -> Action {
        match msg {
            AbsoluteDemoMsg::ToggleModal => {
                state.show_modal = !state.show_modal;
            }
            AbsoluteDemoMsg::SetSelectedLayer(layer) => {
                state.selected_layer = layer;
            }
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: AbsoluteDemoState) -> Node {
        let selected_text = if state.selected_layer == 0 {
            "None".to_string()
        } else {
            format!("Layer {}", state.selected_layer)
        };

        let main_content = node! {
            div(bg: black, dir: vertical, pad: 1, w_frac: 1.0, h: 60, @char('m'): ctx.handler(AbsoluteDemoMsg::ToggleModal)) [
                // Title and instructions
                text("Page 5: Absolute Positioning & Z-Index Demo", color: bright_white),
                text(
                    format!("Press 'm' for modal | Click layers to select | Selected: {}", selected_text),
                    color: bright_yellow
                ),
                spacer(1),

                // Container for absolute positioning demo
                div(pos: relative, w_frac: 0.95, h: 35, bg: bright_black, border: white) [
                    // Layer 1 (z-index: dynamic based on selection)
                    div(
                        absolute,
                        top: 2,
                        left: 3,
                        w: 30,
                        h: 10,
                        z: (if state.selected_layer == 1 { 100 } else { 1 }),
                        bg: (if state.selected_layer == 1 { Color::BrightRed } else { Color::Red }),
                        border: white,
                        pad: 1,
                        @click: ctx.handler(AbsoluteDemoMsg::SetSelectedLayer(1))
                    ) [
                        text(
                            format!("Layer 1 (z-index: {})", if state.selected_layer == 1 { 100 } else { 1 }),
                            color: white
                        ),
                        text(
                            if state.selected_layer == 1 { "SELECTED - Top" } else { "Click to bring to top" },
                            color: bright_white
                        )
                    ],

                    // Layer 2 (z-index: dynamic based on selection)
                    div(
                        absolute,
                        top: 5,
                        left: 12,
                        w: 30,
                        h: 10,
                        z: (if state.selected_layer == 2 { 100 } else { 2 }),
                        bg: (if state.selected_layer == 2 { Color::BrightGreen } else { Color::Green }),
                        border_style: double,
                        border_color: white,
                        pad: 1,
                        @click: ctx.handler(AbsoluteDemoMsg::SetSelectedLayer(2))
                    ) [
                        text(
                            format!("Layer 2 (z-index: {})", if state.selected_layer == 2 { 100 } else { 2 }),
                            color: black
                        ),
                        text(
                            if state.selected_layer == 2 { "SELECTED - Top" } else { "Click to bring to top" },
                            color: bright_black
                        )
                    ],

                    // Layer 3 (z-index: dynamic based on selection)
                    div(
                        absolute,
                        top: 8,
                        left: 21,
                        w: 30,
                        h: 10,
                        z: (if state.selected_layer == 3 { 100 } else { 3 }),
                        bg: (if state.selected_layer == 3 { Color::BrightBlue } else { Color::Blue }),
                        border_style: thick,
                        border_color: white,
                        pad: 1,
                        @click: ctx.handler(AbsoluteDemoMsg::SetSelectedLayer(3))
                    ) [
                        text(
                            format!("Layer 3 (z-index: {})", if state.selected_layer == 3 { 100 } else { 3 }),
                            color: white
                        ),
                        text(
                            if state.selected_layer == 3 { "SELECTED - Top" } else { "Click to bring to top" },
                            color: bright_white
                        )
                    ],

                    // Fixed notification badge (always on top)
                    div(
                        absolute,
                        top: 1,
                        right: 2,
                        w: 20,
                        h: 6,
                        z: 200,
                        bg: bright_magenta,
                        border_style: rounded,
                        border_color: white,
                        pad: 1
                    ) [
                        text("Notification", color: black),
                        text("z-index: 200", color: bright_white)
                    ],

                    // Bottom positioned element
                    div(
                        absolute,
                        bottom: 1,
                        left: 3,
                        w: 25,
                        h: 3,
                        z: 4,
                        bg: bright_cyan,
                        border: black,
                        pad_h: 1
                    ) [
                        text("Bottom positioned", color: black)
                    ]
                ],

                // Info text
                spacer(1),
                text("Click on overlapping layers to bring them to the front", color: white),
                text("Press 'm' to show modal dialog overlay", color: bright_white)
            ]
        };

        // Add modal if visible
        if state.show_modal {
            let modal = node! {
                div(
                    pos: fixed,
                    top: 8,
                    left: 20,
                    w: 40,
                    h: 10,
                    z: 1001,
                    bg: bright_white,
                    border_style: rounded,
                    border_color: black,
                    pad: 2
                ) [
                    div(bg: bright_cyan, pad: 1) [
                        text("Modal Dialog", color: black)
                    ],
                    div(pad_v: 1) [
                        text("This modal uses fixed positioning", color: black),
                        text("with z-index 1001 to overlay everything.", color: black)
                    ],
                    div(pad: 1) [
                        text("Press 'm' again to close", color: bright_black)
                    ]
                ]
            };

            // Create container with both main content and modal
            node! {
                div(w_frac: 1.0, h_frac: 1.0) [
                    (main_content),
                    (modal)
                ]
            }
        } else {
            main_content
        }
    }
}
