use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page14TextInputDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page14TextInputDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 50) [
                // Title
                text("Page 14: TextInput Components", color: bright_white, bold),
                spacer(1),

                // Instructions
                text("Interactive text input components with various configurations", color: bright_black),
                spacer(2),

                // Input with default cursor
                text("Default style with cursor (white on black):", color: white),

                input(placeholder: "Click here and type...", focusable),
                spacer(1),

                // Input with custom cursor color
                text("Custom cursor color (cyan):", color: white),
                input(
                    placeholder: "Custom cursor...",
                    cursor_color: cyan,
                    border: cyan,
                    focusable
                ),
                spacer(1),

                // Input with word wrapping for long text
                text("Word-wrapped input (type a long sentence):", color: white),
                input(
                    placeholder: "Type a long message...",
                    wrap: word_break,
                    w: 40,
                    h: 5,
                    border: magenta,
                    focusable
                ),
                spacer(1),

                // Input with custom content and cursor styling
                text("Custom content (green) and cursor (yellow):", color: white),
                input(
                    placeholder: "Start typing...",
                    content_color: green,
                    bold,
                    cursor_color: yellow,
                    border: green,
                    w: 40,
                    bg: (Color::Rgb(10, 10, 10)),
                    focusable
                ),
                spacer(1),

                // Password input
                text("Password input (content masked with bullets):", color: white),
                input(
                    placeholder: "Enter password...",
                    password,
                    border: red,
                    w: 30,
                    focusable
                ),
                spacer(1),

                // Password input with custom styling
                text("Styled password input:", color: white),
                input(
                    placeholder: "Enter secure password...",
                    password: true,
                    content_color: yellow,
                    cursor_color: bright_yellow,
                    border: yellow,
                    bg: (Color::Rgb(20, 20, 0)),
                    w: 35,
                    focusable
                ),
                spacer(1),
            ]
        }
    }
}
