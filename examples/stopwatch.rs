use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
struct Stopwatch;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl Stopwatch {
    #[update]
    fn update(&self, _ctx: &Context, tick: bool, state: u64) -> Action {
        if !tick {
            return Action::exit();
        }

        Action::update(state + 10) // Increment by 10ms
    }

    #[view]
    fn view(&self, ctx: &Context, state: u64) -> Node {
        let seconds = state / 1000;
        let centiseconds = (state % 1000) / 10;

        node! {
            div(
                pad: 2,
                align: center,
                w_frac: 1.0,
                gap: 1,
                @key(esc): ctx.handler(false),
                @char_global('q'): ctx.handler(false)
            ) [
                richtext[
                    text("Elapsed: ", color: white),
                    text(
                        format!(" {}.{:02}s ", seconds, centiseconds),
                        color: "#ffffff",
                        bg: "#9d29c3",
                        bold
                    ),
                ],

                text("press esc or q to exit", color: bright_black)
            ]
        }
    }

    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            ctx.send(true);
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.fast_polling().run(Stopwatch)
}
