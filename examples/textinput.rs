use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    InputChanged(String),
    InputSubmitted,
    PasswordChanged(String),
    PasswordSubmitted,
    SearchChanged(String),
    SearchSubmitted,
    ExitFocus(bool),
    ClearFocus,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct TextInputTestState {
    input_value: String,
    input_submit_count: usize,
    password_value: String,
    password_submit_count: usize,
    search_value: String,
    search_history: Vec<String>,
    exit_focused: bool,
}

#[derive(Component)]
struct TextInputTest;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl TextInputTest {
    #[update]
    fn update(&self, ctx: &Context, msg: Msg, mut state: TextInputTestState) -> Action {
        match msg {
            Msg::InputChanged(value) => {
                state.input_value = value;
            }
            Msg::InputSubmitted => {
                state.input_submit_count += 1;
            }
            Msg::PasswordChanged(value) => {
                state.password_value = value;
            }
            Msg::PasswordSubmitted => {
                state.password_submit_count += 1;
            }
            Msg::SearchChanged(value) => {
                state.search_value = value;
            }
            Msg::SearchSubmitted => {
                if !state.search_value.is_empty() {
                    state.search_history.push(state.search_value.clone());
                    if state.search_history.len() > 5 {
                        state.search_history.remove(0);
                    }
                }
            }
            Msg::ExitFocus(focused) => {
                state.exit_focused = focused;
            }
            Msg::ClearFocus => {
                ctx.blur_focus();
            }
            Msg::Exit => return Action::exit(),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: TextInputTestState) -> Node {
        if ctx.is_first_render() {
            ctx.focus_self();
        }

        let exit_text_color = if state.exit_focused {
            Color::Rgb(200, 32, 32)
        } else {
            Color::BrightWhite
        };

        node! {
            div(
                bg: black,
                pad: 2,
                w_frac: 1.0,
                h: 36,
                dir: vertical,
                @key(esc): ctx.handler(Msg::Exit)
            ) [
                text("Press Enter to submit | Esc to exit", color: bright_black),
                spacer(1),

                input(
                    placeholder: "Type and press Enter...",
                    border: cyan,
                    w: 40,
                    focusable,
                    @change: ctx.handler_with_value(Msg::InputChanged),
                    @submit: ctx.handler(Msg::InputSubmitted),
                    @key(esc): ctx.handler(Msg::ClearFocus)
                ),
                text(
                    format!("Value: '{}' | Submits: {}",
                        state.input_value,
                        state.input_submit_count
                    ),
                    color: bright_black
                ),
                spacer(1),

                input(
                    placeholder: "Enter password and press Enter...",
                    password,
                    border: magenta,
                    w: 40,
                    focusable,
                    @change: ctx.handler_with_value(Msg::PasswordChanged),
                    @submit: ctx.handler(Msg::PasswordSubmitted),
                    @key(esc): ctx.handler(Msg::ClearFocus)
                ),
                text(
                    format!("Password length: {} | Submits: {}",
                        state.password_value.len(),
                        state.password_submit_count
                    ),
                    color: bright_black
                ),
                spacer(1),

                input(
                    placeholder: "Search and press Enter...",
                    border: green,
                    w: 40,
                    focusable,
                    clear_on_submit,
                    @change: ctx.handler_with_value(Msg::SearchChanged),
                    @submit: ctx.handler(Msg::SearchSubmitted),
                    @key(esc): ctx.handler(Msg::ClearFocus)
                ),
                text(
                    format!("Current search: '{}'", state.search_value),
                    color: bright_black
                ),
                text(
                    if state.search_history.is_empty() {
                        "No searches yet".to_string()
                    } else {
                        format!("Recent searches: {}",
                            state.search_history
                                .iter()
                                .rev()
                                .take(3)
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                    color: bright_black
                ),
                spacer(2),

                div(
                    border: (Color::Rgb(90, 0, 0)),
                    border_style: rounded,
                    focusable,
                    focus_style: ({
                        Style::default()
                            .border(Color::Rgb(200, 40, 40))
                            .background(Color::Rgb(60, 0, 0))
                    }),
                    w: 16,
                    @click: ctx.handler(Msg::Exit),
                    @key(enter): ctx.handler(Msg::Exit),
                    @key(esc): ctx.handler(Msg::Exit),
                    @focus: ctx.handler(Msg::ExitFocus(true)),
                    @blur: ctx.handler(Msg::ExitFocus(false))
                ) [
                    text("Exit", color: (exit_text_color), bold)
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;
    app.run(TextInputTest)?;
    Ok(())
}
