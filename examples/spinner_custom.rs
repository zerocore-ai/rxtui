use rxtui::components::{Spinner, SpinnerSpeed};
use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    Exit,
}

#[derive(Component)]
struct CustomSpinnerDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl CustomSpinnerDemo {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg) -> Action {
        match msg {
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        // Create custom spinners with different patterns
        let rocket_spinner = Spinner::new()
            .custom_pattern(vec!["🚀", "✨", "💫", "⭐", "🌟"])
            .speed(SpinnerSpeed::Normal)
            .color(Color::Yellow);

        let loading_spinner = Spinner::new()
            .custom_pattern(vec![
                "L   ", "LO  ", "LOA ", "LOAD", "OADI", "ADIN", "DING", "ING ", "NG  ", "G   ",
                "    ",
            ])
            .speed(SpinnerSpeed::Custom(100));

        let progress_spinner = Spinner::new()
            .custom_pattern(vec![
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
            ])
            .speed(SpinnerSpeed::Custom(150))
            .color(Color::Green);

        let custom_dots = Spinner::new()
            .custom_pattern(vec![
                "●    ",
                "●●   ",
                "●●●  ",
                "●●●● ",
                "●●●●●",
                " ●●●●",
                "  ●●●",
                "   ●●",
                "    ●",
            ])
            .speed(SpinnerSpeed::Fast)
            .color(Color::Cyan);

        node! {
            div(
                w_frac: 1.0,
                h_frac: 1.0,
                align: center,
                justify: center,
                @key(esc): ctx.handler(Msg::Exit),
                @char('q'): ctx.handler(Msg::Exit)
            ) [
                div(
                    pad: 3,
                    w: 60,
                    border_style: rounded,
                    border_color: cyan,
                    align: center
                ) [
                    text("Custom Spinner Patterns", color: bright_cyan, bold),
                    spacer(2),

                    // Show different custom spinners
                    div(pad_h: 2) [
                        div(justify: space_between) [
                            text("Rocket Animation: ", color: bright_black),
                            node(rocket_spinner),
                        ],
                        spacer(1),

                        div(justify: space_between) [
                            text("Loading Text: ", color: bright_black),
                            node(loading_spinner),
                        ],
                        spacer(1),

                        div(justify: space_between) [
                            text("Progress Bar: ", color: bright_black),
                            node(progress_spinner),
                        ],
                        spacer(1),

                        div(justify: space_between) [
                            text("Custom Dots: ", color: bright_black),
                            node(custom_dots),
                        ],
                    ],

                    spacer(2),
                    text("Press 'q' or ESC to exit", color: bright_black)
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.fast_polling().run(CustomSpinnerDemo)
}
