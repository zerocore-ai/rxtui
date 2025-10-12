use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
struct GapDemo;

#[derive(Clone, Copy)]
enum GapMsg {
    Exit,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl GapDemo {
    #[update]
    fn update(&self, _ctx: &Context, msg: GapMsg) -> Action {
        match msg {
            GapMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(
                dir: vertical,
                pad: 2,
                gap: 2,
                bg: "#101417",
                @key_global(esc): ctx.handler(GapMsg::Exit),
                @char_global('q'): ctx.handler(GapMsg::Exit)
            ) [
                div(bg: "#1c2530", pad: 1) [
                    text("Parent div uses gap: 2", color: "#9ac1f5"),
                    text("Each child is spaced by two cells.", color: "#9ac1f5")
                ],
                div(dir: horizontal, gap: 1, pad: 1, bg: "#1f3b4d") [
                    text("Nested >", color: "#f5d67a"),
                    text("row >", color: "#f5d67a"),
                    text("gap: 1", color: "#f5d67a")
                ],
                div(dir: vertical, gap: 1, pad: 1, bg: "#233646") [
                    text("Another nested stack", color: "#c5f58f"),
                    text("with vertical gap: 1", color: "#c5f58f")
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(GapDemo)
}
