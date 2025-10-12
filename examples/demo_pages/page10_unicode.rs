use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct Page10UnicodeDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page10UnicodeDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h_frac: 1.0) [
                // Title
                text("Page 10: Unicode Text Rendering", color: cyan, bold, underline),
                text("Support for CJK characters, emojis, and full-width text", color: bright_black),
                spacer(2),

                // Example 1: Character width comparison
                text("Example 1: Character Width Comparison", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    // ASCII
                    vstack() [
                        text("ASCII (1 column each):", color: green),
                        div(bg: (Color::Rgb(20, 30, 20)), border: bright_black, w: 25, h: 3, pad: 1) [
                            text("ABCD 1234 !@#$", color: white)
                        ]
                    ],

                    // CJK
                    vstack() [
                        text("CJK (2 columns each):", color: cyan),
                        div(bg: (Color::Rgb(20, 20, 30)), border: bright_black, w: 25, h: 3, pad: 1) [
                            text("中文 日本語 한글", color: white)
                        ]
                    ],

                    // Full-width
                    vstack() [
                        text("Full-width (2 cols):", color: magenta),
                        div(bg: (Color::Rgb(30, 20, 30)), border: bright_black, w: 25, h: 3, pad: 1) [
                            text("ＡＢＣＤ １２３４", color: white)
                        ]
                    ]
                ],
                spacer(2),

                // Example 2: Mixed content with wrapping
                text("Example 2: Mixed Content with Text Wrapping", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    vstack() [
                        text("Mixed ASCII + CJK:", color: bright_black),
                        div(bg: (Color::Rgb(30, 30, 30)), border: bright_black, overflow: hidden, w: 28, h: 5, pad: 1) [
                            text(
                                "Hello 世界! This is 混合文本 with both English and 中文 characters mixed together.",
                                color: white,
                                wrap: word
                            )
                        ]
                    ],

                    vstack() [
                        text("Long CJK text:", color: bright_black),
                        div(bg: (Color::Rgb(30, 30, 30)), border: bright_black, overflow: hidden, w: 28, h: 5, pad: 1) [
                            text(
                                "这是一段很长的中文文本用来测试文字换行功能是否正常工作。",
                                color: white,
                                wrap: character
                            )
                        ]
                    ]
                ],
                spacer(2),

                // Example 3: Emoji rendering
                text("Example 3: Emoji Support", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    vstack() [
                        text("Basic emojis:", color: bright_black),
                        div(bg: (Color::Rgb(25, 25, 35)), border: bright_black, w: 20, h: 4, pad: 1) [
                            text("😀 😃 😄 😁 😅 😂 🤣 😊 😇 🙂", color: white, wrap: character)
                        ]
                    ],

                    vstack() [
                        text("Symbols:", color: bright_black),
                        div(bg: (Color::Rgb(25, 25, 35)), border: bright_black, w: 20, h: 4, pad: 1) [
                            text("❤️ 💚 💙 💜 ⭐ ✨ 🌟 ⚡ 🔥 💧", color: white, wrap: character)
                        ]
                    ],

                    vstack() [
                        text("Flags:", color: bright_black),
                        div(bg: (Color::Rgb(25, 25, 35)), border: bright_black, w: 20, h: 4, pad: 1) [
                            text("🇺🇸 🇬🇧 🇯🇵 🇰🇷 🇨🇳 🇩🇪 🇫🇷 🇮🇹 🇪🇸 🇧🇷", color: white, wrap: character)
                        ]
                    ]
                ],
                spacer(2),

                // Example 4: Wrapping mode comparison with Unicode
                text("Example 4: Text Wrapping Modes with Unicode", color: yellow, bold),
                spacer(1),
                hstack(gap: 2) [
                    // Character wrap
                    vstack() [
                        text("Character wrap:", color: green),
                        div(bg: (Color::Rgb(20, 30, 20)), border: bright_black, overflow: hidden, w: 18, h: 5, pad: 1) [
                            text(
                                "Hello世界Testing文字wrapping功能verification",
                                color: white,
                                wrap: character
                            )
                        ]
                    ],

                    // Word wrap
                    vstack() [
                        text("Word wrap:", color: blue),
                        div(bg: (Color::Rgb(20, 20, 30)), border: bright_black, overflow: hidden, w: 18, h: 5, pad: 1) [
                            text(
                                "Hello世界 Testing文字 wrapping功能 verification",
                                color: white,
                                wrap: word
                            )
                        ]
                    ],

                    // Word-break wrap
                    vstack() [
                        text("Word-break:", color: magenta),
                        div(bg: (Color::Rgb(30, 20, 30)), border: bright_black, overflow: hidden, w: 18, h: 5, pad: 1) [
                            text(
                                "Hello世界VeryLongWord文字wrapping功能verification",
                                color: white,
                                wrap: word_break
                            )
                        ]
                    ]
                ],
                spacer(2),

                // Info note
                div(bg: (Color::Rgb(20, 20, 30)), border: bright_black, pad: 1) [
                    text("Note:", color: yellow, bold),
                    text("• CJK characters and emojis typically occupy 2 terminal columns", color: bright_black),
                    text("• Terminal font and emulator affect emoji rendering", color: bright_black),
                    text("• Text wrapping respects display width, not byte count", color: bright_black)
                ]
            ]
        }
    }
}
