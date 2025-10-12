use rxtui::components::{Spinner, SpinnerType};
use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    NextSpinner,
    PrevSpinner,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct SpinnerGalleryState {
    current_spinner_index: usize,
}

#[derive(Component)]
struct SpinnerGallery;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl SpinnerGallery {
    #[update]
    fn update(&self, ctx: &Context, msg: Msg, mut state: SpinnerGalleryState) -> Action {
        match msg {
            Msg::NextSpinner => {
                state.current_spinner_index =
                    (state.current_spinner_index + 1) % Self::spinner_list().len();
                Action::update(state)
            }
            Msg::PrevSpinner => {
                let len = Self::spinner_list().len();
                state.current_spinner_index = (state.current_spinner_index + len - 1) % len;
                Action::update(state)
            }
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: SpinnerGalleryState) -> Node {
        let spinners = Self::spinner_list();
        let current_spinner = &spinners[state.current_spinner_index];
        let spinner_name = Self::spinner_name(current_spinner);

        // Create spinner with current type
        let spinner = Spinner::new()
            .spinner_type(current_spinner.clone())
            .color(Color::Cyan);

        node! {
            div(
                w_frac: 1.0,
                h_frac: 1.0,
                align: center,
                justify: center,
                @key(right): ctx.handler(Msg::NextSpinner),
                @key(left): ctx.handler(Msg::PrevSpinner),
                @key(esc): ctx.handler(Msg::Exit),
                @char('q'): ctx.handler(Msg::Exit)
            ) [
                div(
                    pad: 3,
                    w: 50,
                    border_style: rounded,
                    border_color: cyan,
                    align: center
                ) [
                    // Spinner type name
                    text(spinner_name, color: bright_cyan, bold),

                    // The spinner itself
                    div(pad_v: 1) [
                        node(spinner)
                    ],

                    // Navigation info
                    text(
                        format!("{} / {}", state.current_spinner_index + 1, spinners.len()),
                        color: bright_black
                    ),

                    spacer(1),

                    // Controls
                    div(justify: center) [
                        text("←   →", color: yellow, align: center, bold),
                        text("Navigate", color: bright_black),
                    ]
                ]
            ]
        }
    }
}

impl SpinnerGallery {
    fn spinner_list() -> Vec<SpinnerType> {
        vec![
            SpinnerType::Dots,
            SpinnerType::Dots2,
            SpinnerType::Dots3,
            SpinnerType::Line,
            SpinnerType::Line2,
            SpinnerType::Pipe,
            SpinnerType::SimpleDots,
            SpinnerType::SimpleDotsScrolling,
            SpinnerType::Star,
            SpinnerType::Star2,
            SpinnerType::Flip,
            SpinnerType::Hamburger,
            SpinnerType::GrowVertical,
            SpinnerType::GrowHorizontal,
            SpinnerType::Balloon,
            SpinnerType::Balloon2,
            SpinnerType::Noise,
            SpinnerType::Bounce,
            SpinnerType::BoxBounce,
            SpinnerType::BoxBounce2,
            SpinnerType::Triangle,
            SpinnerType::Binary,
            SpinnerType::Arc,
            SpinnerType::Circle,
            SpinnerType::SquareCorners,
            SpinnerType::CircleQuarters,
            SpinnerType::CircleHalves,
            SpinnerType::Squish,
            SpinnerType::Toggle,
            SpinnerType::Toggle2,
            SpinnerType::Toggle3,
            SpinnerType::Arrow,
            SpinnerType::Arrow2,
            SpinnerType::Arrow3,
            SpinnerType::BouncingBar,
            SpinnerType::BouncingBall,
            SpinnerType::Clock,
            SpinnerType::Earth,
            SpinnerType::Moon,
            SpinnerType::Hearts,
            SpinnerType::Smiley,
            SpinnerType::Monkey,
            SpinnerType::Weather,
            SpinnerType::Christmas,
            SpinnerType::Point,
            SpinnerType::Layer,
            SpinnerType::BetaWave,
            SpinnerType::Aesthetic,
        ]
    }

    fn spinner_name(spinner_type: &SpinnerType) -> &'static str {
        match spinner_type {
            SpinnerType::Dots => "Dots",
            SpinnerType::Dots2 => "Dots 2",
            SpinnerType::Dots3 => "Dots 3",
            SpinnerType::Line => "Line",
            SpinnerType::Line2 => "Line 2",
            SpinnerType::Pipe => "Pipe",
            SpinnerType::SimpleDots => "Simple Dots",
            SpinnerType::SimpleDotsScrolling => "Scrolling Dots",
            SpinnerType::Star => "Star",
            SpinnerType::Star2 => "Star 2",
            SpinnerType::Flip => "Flip",
            SpinnerType::Hamburger => "Hamburger",
            SpinnerType::GrowVertical => "Grow Vertical",
            SpinnerType::GrowHorizontal => "Grow Horizontal",
            SpinnerType::Balloon => "Balloon",
            SpinnerType::Balloon2 => "Balloon 2",
            SpinnerType::Noise => "Noise",
            SpinnerType::Bounce => "Bounce",
            SpinnerType::BoxBounce => "Box Bounce",
            SpinnerType::BoxBounce2 => "Box Bounce 2",
            SpinnerType::Triangle => "Triangle",
            SpinnerType::Binary => "Binary",
            SpinnerType::Arc => "Arc",
            SpinnerType::Circle => "Circle",
            SpinnerType::SquareCorners => "Square Corners",
            SpinnerType::CircleQuarters => "Circle Quarters",
            SpinnerType::CircleHalves => "Circle Halves",
            SpinnerType::Squish => "Squish",
            SpinnerType::Toggle => "Toggle",
            SpinnerType::Toggle2 => "Toggle 2",
            SpinnerType::Toggle3 => "Toggle 3",
            SpinnerType::Arrow => "Arrow",
            SpinnerType::Arrow2 => "Arrow 2 (Emoji)",
            SpinnerType::Arrow3 => "Arrow 3",
            SpinnerType::BouncingBar => "Bouncing Bar",
            SpinnerType::BouncingBall => "Bouncing Ball",
            SpinnerType::Clock => "Clock",
            SpinnerType::Earth => "Earth",
            SpinnerType::Moon => "Moon",
            SpinnerType::Hearts => "Hearts",
            SpinnerType::Smiley => "Smiley",
            SpinnerType::Monkey => "Monkey",
            SpinnerType::Weather => "Weather",
            SpinnerType::Christmas => "Christmas",
            SpinnerType::Point => "Point",
            SpinnerType::Layer => "Layer",
            SpinnerType::BetaWave => "Beta Wave",
            SpinnerType::Aesthetic => "Aesthetic",
            SpinnerType::Custom(_) => "Custom",
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(SpinnerGallery)
}
