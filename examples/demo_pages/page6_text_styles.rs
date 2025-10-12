use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page6TextStylesDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page6TextStylesDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 50) [
                // Title
                text("Page 6: Text Styling Demo", color: bright_white, bold),
                spacer(2),

                // Basic text styles
                text("Basic Text Styles:", color: bright_yellow),
                spacer(1),
                div(dir: vertical, w_frac: 0.9, h: 25) [
                    // Bold text
                    text("This is BOLD text", color: white, bold),
                    spacer(1),

                    // Italic text
                    text("This is ITALIC text", color: bright_cyan, italic),
                    spacer(1),

                    // Underlined text
                    text("This is UNDERLINED text", color: bright_green, underline),
                    spacer(1),

                    // Strikethrough text
                    text("This is STRIKETHROUGH text", color: bright_red, strikethrough),
                    spacer(2),

                    // Combined styles
                    text("Combined Styles:", color: bright_yellow),
                    spacer(1),

                    // Bold + Italic
                    text("Bold + Italic", color: bright_magenta, bold, italic),
                    spacer(1),

                    // Bold + Underline
                    text("Bold + Underline", color: bright_blue, bold, underline),
                    spacer(1),

                    // All styles combined
                    text(
                        "Bold + Italic + Underline + Strikethrough",
                        color: bright_white,
                        bg: bright_black,
                        bold,
                        italic,
                        underline,
                        strikethrough
                    ),
                    spacer(2),

                    // Convenience methods
                    text("Using convenience methods:", color: bright_yellow),
                    spacer(1),
                    text("This uses .strong() (alias for .bold())", color: green, bold),
                    spacer(1),
                    text("This uses .emphasis() (alias for .italic())", color: cyan, italic)
                ]
            ]
        }
    }
}
