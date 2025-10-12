use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page8TextWrapDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page8TextWrapDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 60) [
                // Title
                text("Page 8: Text Wrapping Examples", color: bright_white),
                spacer(2),

                // Example 1: No wrapping (default)
                text("Example 1 - No wrapping (TextWrap::None):", color: white),
                spacer(1),
                div(bg: blue, w: 30, h: 5, pad: 1) [
                    text(
                        "This is a very long text that will overflow the container without any wrapping applied",
                        color: white,
                        wrap: none
                    )
                ],
                spacer(2),

                // Example 2: Character wrapping
                text("Example 2 - Character wrapping (TextWrap::Character):", color: white),
                spacer(1),
                div(bg: green, w: 30, h: 6, pad: 1) [
                    text(
                        "This is a very long text that will be wrapped at character boundaries regardless of word breaks",
                        color: black,
                        wrap: character
                    )
                ],
                spacer(2),

                // Example 3: Word wrapping
                text("Example 3 - Word wrapping (TextWrap::Word):", color: white),
                spacer(1),
                div(bg: cyan, w: 30, h: 6, pad: 1) [
                    text(
                        "This text will wrap at word boundaries. If a verylongwordexceedsthewidth it will overflow.",
                        color: black,
                        wrap: word
                    )
                ],
                spacer(2),

                // Example 4: Word-break wrapping
                text("Example 4 - Word-break wrapping (TextWrap::WordBreak):", color: white),
                spacer(1),
                div(bg: magenta, w: 30, h: 7, pad: 1) [
                    text(
                        "This text wraps at word boundaries, but verylongwordsthatexceedthewidthwillbebrokenacrosslines",
                        color: white,
                        wrap: word_break
                    )
                ],
                spacer(2),

                // Example 5: Different widths with same text
                text("Example 5 - Same text with different container widths:", color: white),
                spacer(1),
                hstack(h: 8) [
                    div(bg: red, w: 20, h: 8, pad: 1) [
                        text("The quick brown fox jumps over the lazy dog", color: white, wrap: word)
                    ],
                    div(w: 2) [],
                    div(bg: yellow, w: 30, h: 8, pad: 1) [
                        text("The quick brown fox jumps over the lazy dog", color: black, wrap: word)
                    ],
                    div(w: 2) [],
                    div(bg: bright_blue, w: 40, h: 8, pad: 1) [
                        text("The quick brown fox jumps over the lazy dog", color: white, wrap: word)
                    ]
                ],
                spacer(2),

                // Example 6: Poetry/formatted text
                text("Example 6 - Formatted text with character wrap:", color: white),
                spacer(1),
                div(bg: bright_green, w: 25, h: 6, pad: 1) [
                    text(
                        "Roses are red, violets are blue, text wrapping works, and so do you!",
                        color: black,
                        wrap: word
                    )
                ]
            ]
        }
    }
}
