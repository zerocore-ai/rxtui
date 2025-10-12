use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const ITEMS: &[(&str, &str)] = &[
    ("Inbox", "5 unread conversations waiting"),
    ("Team Standup", "Next session today at 10:00"),
    ("Deploy Preview", "Version 1.8.2 awaiting approval"),
    ("Release Notes", "Draft ready for review"),
];

const ACCENTS: [Color; 4] = [Color::Cyan, Color::Magenta, Color::Yellow, Color::Green];

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
struct HoverShowcase;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl HoverShowcase {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str) -> Action {
        if matches!(msg, "exit") {
            Action::exit()
        } else {
            Action::none()
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(
                dir: vertical,
                pad: 2,
                gap: 2,
                bg: (Color::Rgb(12, 13, 24)),
                w_frac: 1.0,
                h_frac: 1.0,
                @key_global(esc): ctx.handler("exit"),
            ) [
                text("Interactive Hover Cards", color: cyan, bold),
                text("Move your mouse across the cards to see hover styling.", color: bright_black),
                div(dir: vertical, gap: 1) [
                    ...(ITEMS.iter().enumerate().map(|(index, (title, subtitle))| {
                        let accent = ACCENTS[index % ACCENTS.len()];
                        node! {
                            div(
                                dir: vertical,
                                gap: 0,
                                pad: 1,
                                bg: (Color::Rgb(24, 26, 36)),
                                border_style: (BorderStyle::Rounded, Color::Rgb(34, 37, 49)),
                                focusable,
                                focus_style: (
                                    Style {
                                        border: Some(Border::with_style(BorderStyle::Rounded, Color::BrightBlue)),
                                        ..Style::default()
                                    }
                                ),
                                hover_style: (
                                    Style {
                                        background: Some(Color::Rgb(36, 40, 56)),
                                        border: Some(Border::with_style(BorderStyle::Rounded, accent)),
                                        ..Style::default()
                                    }
                                )
                            ) [
                                text(*title, color: white, bold),
                                text(*subtitle, color: bright_black)
                            ]
                        }
                    }).collect::<Vec<_>>())
                ],
                spacer(1),
                div(dir: vertical, gap: 1) [
                    text("Try the hover-enabled search box:", color: bright_black),
                    input(
                        placeholder: "Hover or focus me...",
                        w: 46,
                        bg: (Color::Rgb(20, 22, 32)),
                        border_style: (BorderStyle::Rounded, Color::Rgb(34, 37, 49)),
                        focus_style: (
                            Style {
                                border: Some(Border::with_style(BorderStyle::Rounded, Color::BrightBlue)),
                                padding: Some(Spacing::horizontal(1)),
                                ..Style::default()
                            }
                        ),
                        hover_style: (
                            Style {
                                background: Some(Color::Rgb(36, 40, 56)),
                                border: Some(Border::with_style(BorderStyle::Rounded, Color::Cyan)),
                                padding: Some(Spacing::horizontal(1)),
                                ..Style::default()
                            }
                        )
                    )
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(HoverShowcase)
}
