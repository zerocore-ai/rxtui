use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DemoState {
    justify: JustifyContent,
    align: AlignItems,
    vertical: bool,
    show_align_self: bool,
}

#[derive(Component)]
pub struct DivAlignmentDemo;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DemoState {
    fn default() -> Self {
        Self {
            justify: JustifyContent::Start,
            align: AlignItems::Start,
            vertical: false,
            show_align_self: false,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl DivAlignmentDemo {
    #[update]
    fn update(&self, _ctx: &Context, event: &str, mut state: DemoState) -> Action {
        match event {
            "justify_start" => state.justify = JustifyContent::Start,
            "justify_center" => state.justify = JustifyContent::Center,
            "justify_end" => state.justify = JustifyContent::End,
            "justify_space_between" => state.justify = JustifyContent::SpaceBetween,
            "justify_space_around" => state.justify = JustifyContent::SpaceAround,
            "justify_space_evenly" => state.justify = JustifyContent::SpaceEvenly,
            "align_start" => state.align = AlignItems::Start,
            "align_center" => state.align = AlignItems::Center,
            "align_end" => state.align = AlignItems::End,
            "toggle_direction" => state.vertical = !state.vertical,
            "toggle_align_self" => state.show_align_self = !state.show_align_self,
            "exit" => return Action::exit(),
            _ => return Action::none(),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: DemoState) -> Node {
        let dir = if state.vertical {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };
        let justify = state.justify;
        let align = state.align;

        // Build children - varying sizes to demonstrate alignment
        let children = if state.show_align_self {
            // Show align_self overrides - mixed sizes and alignment
            vec![
                node! { div(bg: red, w: 10, h: 3) [text("1", color: white)] },
                node! { div(bg: green, w: 12, h: 5, align_self: end) [text("2(end)", color: white)] },
                node! { div(bg: blue, w: 14, h: 7) [text("3", color: white)] },
                node! { div(bg: yellow, w: 10, h: 4, align_self: start) [text("4(start)", color: black)] },
            ]
        } else {
            // Normal mode - varying sizes to show alignment effects
            vec![
                node! { div(bg: red, w: 10, h: 3) [text("1", color: white)] },
                node! { div(bg: green, w: 12, h: 5) [text("2", color: white)] },
                node! { div(bg: blue, w: 14, h: 7) [text("3", color: white)] },
            ]
        };

        node! {
            div(
                pad: 2,
                w_frac: 1.0,
                h_frac: 1.0,
                align: center,
                @char_global('1'): ctx.handler("justify_start"),
                @char_global('2'): ctx.handler("justify_center"),
                @char_global('3'): ctx.handler("justify_end"),
                @char_global('4'): ctx.handler("justify_space_between"),
                @char_global('5'): ctx.handler("justify_space_around"),
                @char_global('6'): ctx.handler("justify_space_evenly"),
                @char_global('q'): ctx.handler("align_start"),
                @char_global('w'): ctx.handler("align_center"),
                @char_global('e'): ctx.handler("align_end"),
                @char_global('a'): ctx.handler("toggle_align_self"),
                @char_global('d'): ctx.handler("toggle_direction"),
                @key_global(esc): ctx.handler("exit")
            ) [
                // Title
                text("Div Alignment Demo - Mix & Match!", color: yellow, bold),
                text("Justify and Align work on perpendicular axes and can be freely combined", color: bright_black),
                spacer(1),

                // Current settings display
                text(format!(
                    "Direction: {} | Justify: {:?} | Align: {:?} | AlignSelf: {}",
                    if state.vertical { "Vertical" } else { "Horizontal" },
                    state.justify,
                    state.align,
                    if state.show_align_self { "ON" } else { "OFF" }
                ), color: cyan),
                spacer(1),

                // Demo container
                div(
                    bg: "#333",
                    border: white,
                    dir: dir,
                    justify: justify,
                    align: align,
                    w: 56,
                    h: 21
                ) [...(children)],
                spacer(1),

                // Instructions
                div(border: bright_black, pad: 1, w: 56) [
                    text("JUSTIFY (Main Axis):", color: yellow, bold),
                    text(if state.vertical {
                        "  Controls vertical distribution"
                    } else {
                        "  Controls horizontal distribution"
                    }, color: bright_black),
                    text("  [1] Start        [2] Center       [3] End", color: white),
                    text("  [4] SpaceBetween [5] SpaceAround  [6] SpaceEvenly", color: white),
                    spacer(1),

                    text("ALIGN (Cross Axis):", color: yellow, bold),
                    text(if state.vertical {
                        "  Controls horizontal alignment"
                    } else {
                        "  Controls vertical alignment"
                    }, color: bright_black),
                    text("  [Q] Start        [W] Center       [E] End", color: white),
                    spacer(1),

                    text("OTHER:", color: yellow, bold),
                    text("  [D] Toggle Direction - Switch horizontal/vertical", color: white),
                    text("  [A] Toggle AlignSelf - Show per-child overrides", color: white),
                    text("  [ESC] Exit", color: white)
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(DivAlignmentDemo)
}
