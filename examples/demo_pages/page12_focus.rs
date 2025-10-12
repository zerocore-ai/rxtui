use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page12FocusDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page12FocusDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 50) [
                // Title
                text("Page 12: Focus Management Demo", color: bright_white, bold),
                spacer(1),

                // Instructions
                vstack() [
                    text("• Tab: Navigate forward between focusable elements", color: bright_black),
                    text("• Shift+Tab: Navigate backward between focusable elements", color: bright_black),
                    text("• Click: Focus an element directly", color: bright_black),
                    text("• Enter: Activate focused button", color: bright_black),
                    text("• Focused elements have white borders", color: bright_black)
                ],
                spacer(2),

                // Buttons row
                hstack(gap: 2) [
                    node(FocusButton::new(
                        "Button 1",
                        Color::Red
                    )),
                    node(FocusButton::new(
                        "Button 2",
                        Color::Green
                    )),
                    node(FocusButton::new(
                        "Button 3",
                        Color::Blue
                    ))
                ],
                spacer(2),

                // Text Input with actual TextInput component
                vstack() [
                    text("Text Input (cyan border when focused):", color: yellow),
                    input(
                        placeholder: "Type something...",
                        cursor_color: cyan,
                        border: cyan,
                        focusable
                    ),
                ],
                spacer(2),

                // Focus event history
                vstack() [
                    text("Focus Events:", color: yellow, bold),
                    spacer(1),
                    text("Event display simplified due to macro limitations", color: cyan)
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Helper Component
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct FocusButtonState {
    count: u32,
}

#[derive(Debug, Clone)]
enum FocusButtonMsg {
    Increment,
    Focused,
    Blurred,
}

#[derive(Component)]
struct FocusButton {
    label: String,
    color: Color,
}

impl FocusButton {
    fn new(label: &str, color: Color) -> Self {
        Self {
            label: label.to_string(),
            color,
        }
    }

    #[update]
    fn update(&self, ctx: &Context, msg: FocusButtonMsg, mut state: FocusButtonState) -> Action {
        match msg {
            FocusButtonMsg::Increment => {
                state.count += 1;
                Action::update(state)
            }
            FocusButtonMsg::Focused | FocusButtonMsg::Blurred => Action::none(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: FocusButtonState) -> Node {
        let label = self.label.clone();
        let color = self.color;

        node! {
            div(
                border_style: single,
                border_color: (color),
                pad: 1,
                w: 15,
                focusable,
                focus_style: ({
                    Style::default()
                        .border(Color::White)
                        .background(Color::Rgb(30, 30, 40))
                        .padding(Spacing::all(1))
                }),
                @click: ctx.handler(FocusButtonMsg::Increment),
                @key(enter): ctx.handler(FocusButtonMsg::Increment),
                @focus: ctx.handler(FocusButtonMsg::Focused),
                @blur: ctx.handler(FocusButtonMsg::Blurred)
            ) [
                text(label, color: (color)),
                text(format!("Count: {}", state.count), color: white)
            ]
        }
    }
}
