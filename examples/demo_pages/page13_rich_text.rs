//! Page 13: RichText Demo
//!
//! Demonstrates the RichText type for creating styled text with multiple spans
//! and various formatting options.

use rxtui::prelude::*;

#[derive(Component)]
pub struct Page13;

impl Page13 {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 1, w_frac: 1.0) [
                // Title
                text("Page 13: RichText Demo", color: bright_white),
                spacer(1),

                // Description
                text("RichText allows inline styling with multiple spans", color: white),
                spacer(1),

                // Scrollable content area
                div(dir: vertical, gap: 1) [
                    // Example 1: Basic colored text
                    text("1. Basic Colored Text:", color: yellow, bold),
                    richtext [
                        text("This is "),
                        text("red", color: red),
                        text(", "),
                        text("green", color: green),
                        text(", and "),
                        text("blue", color: blue),
                        text(" text!"),
                    ],

                    // Example 2: Text formatting
                    text("2. Text Formatting:", color: yellow, bold),
                    richtext [
                        text("Normal text, "),
                        text("bold text", bold),
                        text(", "),
                        text("italic text", italic),
                        text(", and "),
                        text("underlined text", underline),
                    ],

                    // Example 3: Combined styles
                    text("3. Combined Styles:", color: yellow, bold),
                    richtext [
                        text("Here's "),
                        text("bold red text", color: red, bold),
                        text(" and "),
                        text("italic blue on yellow", color: blue, bg: yellow, italic),
                    ],

                    // Example 4: Status indicators
                    text("4. Status Indicators:", color: yellow, bold),
                    hstack(gap: 2) [
                        richtext [
                            text(" SUCCESS ", color: black, bg: green, bold),
                        ],
                        richtext [
                            text(" WARNING ", color: black, bg: yellow, bold),
                        ],
                        richtext [
                            text(" ERROR ", color: white, bg: red, bold),
                        ]
                    ],

                    // Example 5: Text wrapping modes
                    text("5. Text Wrapping Modes:", color: yellow, bold),

                    div(pad: 1) [
                        text("Character Wrapping (w: 30):", color: cyan),
                        div(w: 30) [
                            richtext(wrap: character) [
                                text("This is "),
                                text("character", color: magenta),
                                text(" wrapping. "),
                                text("Words can be ", bold),
                                text("broken anywhere", italic),
                                text(" to fit width."),
                            ]
                        ]
                    ],

                    div(pad: 1) [
                        text("Word Wrapping (w: 30):", color: cyan),
                        div(w: 30) [
                            richtext(wrap: word) [
                                text("This is "),
                                text("word", color: cyan),
                                text(" wrapping. "),
                                text("Words stay intact", bold),
                                text(" and "),
                                text("wrap at spaces", italic),
                                text(" when possible."),
                            ]
                        ]
                    ],

                    div(pad: 1) [
                        text("WordBreak Wrapping (w: 30):", color: cyan),
                        div(w: 30) [
                            richtext(wrap: word_break) [
                                text("This uses "),
                                text("WordBreak", color: green),
                                text(" mode. "),
                                text("Supercalifragilisticexpialidocious", bold),
                                text(" will break if needed but "),
                                text("normal words", italic),
                                text(" wrap at spaces."),
                            ]
                        ]
                    ],

                    // Example 6: Code syntax highlighting
                    text("6. Code Syntax Highlighting:", color: yellow, bold),
                    div(w: 50, bg: black, pad: 1) [
                        richtext(wrap: word) [
                            text("fn ", color: magenta),
                            text("calculate_fibonacci", color: yellow),
                            text("("),
                            text("n", color: cyan),
                            text(": "),
                            text("u32", color: blue),
                            text(") -> "),
                            text("u32", color: blue),
                            text(" {"),
                        ],
                        richtext [
                            text("    "),
                            text("match", color: magenta),
                            text(" n {"),
                        ],
                        richtext [
                            text("        "),
                            text("0", color: cyan),
                            text(" | "),
                            text("1", color: cyan),
                            text(" => n,"),
                        ],
                        richtext(wrap: word_break) [
                            text("        _ => "),
                            text("calculate_fibonacci", color: yellow),
                            text("(n - "),
                            text("1", color: cyan),
                            text(") + "),
                            text("calculate_fibonacci", color: yellow),
                            text("(n - "),
                            text("2", color: cyan),
                            text(")"),
                        ],
                        richtext [
                            text("    }"),
                        ],
                        richtext [
                            text("}"),
                        ]
                    ],

                    // Example 7: Progress bar
                    text("7. Progress Bar:", color: yellow, bold),
                    hstack(gap: 1) [
                        richtext [
                            text("["),
                            text("████████", color: green),
                            text("████", color: yellow),
                            text("░░░░░░░░", color: white),
                            text("]"),
                        ],
                        text("60%", color: cyan)
                    ],

                    // Example 8: Top-level styling
                    text("8. Top-Level Styling:", color: yellow, bold),
                    div(pad: 1) [
                        richtext(color: white, bg: magenta) [
                            text("All spans inherit "),
                            text("white on magenta", bold),
                            text(" from top-level props."),
                        ],
                        spacer(1),
                        richtext(bold_all) [
                            text("All this text is bold because of "),
                            text("bold_all", color: cyan),
                            text(" property!"),
                        ]
                    ],

                    // Example 9: Inline vs Block comparison
                    text("9. Inline vs Block Text:", color: yellow, bold),
                    div(pad: 1) [
                        text("Regular Text (block):", color: green),
                        text("This is regular text.", color: white),
                        text("Each text() creates a new line.", color: white),
                        spacer(1),
                        text("RichText (inline spans):", color: green),
                        richtext [
                            text("This "),
                            text("stays ", color: red),
                            text("on ", color: green),
                            text("one ", color: blue),
                            text("line."),
                        ]
                    ],

                    // Example 10: Dynamic content
                    text("10. Dynamic Content Example:", color: yellow, bold),
                    div(pad: 1) [
                        richtext [
                            text("Server Status: "),
                            text("● ONLINE", color: green, bold),
                            text(" | CPU: "),
                            text("45%", color: yellow),
                            text(" | Memory: "),
                            text("2.3GB", color: cyan),
                            text(" | Uptime: "),
                            text("12d 3h", color: white),
                        ]
                    ],
                ]
            ]
        }
    }
}
