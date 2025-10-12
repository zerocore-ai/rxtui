use rxtui::components::{ShimmerSpeed, ShimmerText};
use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const SHIMMER_TITLE: &str = "Reactive Shimmer Text";
const SHIMMER_PRIMARY: &str = "Glow through the terminal with reactive shimmer";
const SHIMMER_SECONDARY: &str = "Gentle shimmer with a calmer sweep";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ExampleMsg {
    Exit,
}

#[derive(Component)]
struct ShimmerExample;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl ShimmerExample {
    #[update]
    fn update(&self, _ctx: &Context, msg: ExampleMsg) -> Action {
        match msg {
            ExampleMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        let fast_shimmer = ShimmerText::new(SHIMMER_PRIMARY)
            .gradient(Color::Rgb(70, 90, 130), Color::Rgb(210, 225, 255))
            .highlight_band(6)
            .speed(ShimmerSpeed::fast());

        let slow_shimmer = ShimmerText::new(SHIMMER_SECONDARY)
            .gradient(Color::BrightBlack, Color::BrightWhite)
            .highlight_band(10)
            .speed(ShimmerSpeed::slow());

        node! {
            div(
                w_frac: 1.0,
                h_frac: 1.0,
                align: center,
                justify: center,
                bg: (Color::Rgb(10, 14, 26)),
                @key_global(esc): ctx.handler(ExampleMsg::Exit),
                @char_global('q'): ctx.handler(ExampleMsg::Exit)
            ) [
                div(
                    pad_h: 6,
                    pad_v: 3,
                    gap: 2,
                    w: 70,
                    border_style: rounded,
                    border_color: (Color::Rgb(90, 120, 190)),
                    bg: (Color::Rgb(18, 24, 40)),
                    align: center
                ) [
                    text(SHIMMER_TITLE, color: cyan, bold),
                    node(fast_shimmer),
                    node(slow_shimmer)
                ],

                text("press esc or q to exit", color: bright_black)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(ShimmerExample)
}
