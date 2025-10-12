use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
struct Counter;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Counter {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str, mut count: i32) -> Action {
        match msg {
            "inc" => Action::update(count + 1),
            "dec" => Action::update(count - 1),
            _ => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, count: i32) -> Node {
        node! {
            div(
                pad: 2,
                align: center,
                w_frac: 1.0,
                gap: 1,
                @key(up): ctx.handler("inc"),
                @key(down): ctx.handler("dec"),
                @key(esc): ctx.handler("exit")
            ) [
                text(format!("Count: {count}"), color: white, bold),
                text("use ↑/↓ to change, esc to exit", color: bright_black)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(Counter)
}
