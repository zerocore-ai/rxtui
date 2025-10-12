use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    SetProgress(f32),
    Exit,
}

#[derive(Debug, Clone)]
struct ProgressState {
    progress: f32, // 0.0 to 1.0
}

#[derive(Component)]
struct ProgressBar;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ProgressState {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl ProgressBar {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: ProgressState) -> Action {
        match msg {
            Msg::SetProgress(value) => {
                state.progress = value;
                Action::update(state)
            }
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: ProgressState) -> Node {
        let percentage = (state.progress * 100.0) as u32;
        let bar_width = 50;
        let filled = ((state.progress * bar_width as f32) as usize).min(bar_width);
        let empty = bar_width - filled;

        node! {
            div(
                pad: 2,
                align: center,
                gap: 1,
                w_frac: 1.0,
                @key_global(esc): ctx.handler(Msg::Exit),
                @char_global('q'): ctx.handler(Msg::Exit)
            ) [
                hstack(gap: 1) [
                    // Progress bar with smooth gradient using peachy colors
                    hstack [
                        ...((0..filled).map(|i| {
                            // Calculate position for multi-stop gradient
                            let progress = i as f32 / bar_width as f32;

                            // Peachy gradient with multiple stops:
                            // 0.0: Coral     RGB(255, 127, 80)
                            // 0.33: Peach    RGB(255, 192, 128)
                            // 0.66: Salmon   RGB(255, 160, 122)
                            // 1.0: Pink      RGB(255, 182, 193)

                            let (r, g, b) = if progress < 0.33 {
                                // Coral to Peach
                                let t = progress / 0.33;
                                (
                                    255,
                                    (127.0 + 65.0 * t) as u8,  // 127 -> 192
                                    (80.0 + 48.0 * t) as u8,    // 80 -> 128
                                )
                            } else if progress < 0.66 {
                                // Peach to Salmon
                                let t = (progress - 0.33) / 0.33;
                                (
                                    255,
                                    (192.0 - 32.0 * t) as u8,   // 192 -> 160
                                    (128.0 - 6.0 * t) as u8,     // 128 -> 122
                                )
                            } else {
                                // Salmon to Pink
                                let t = (progress - 0.66) / 0.34;
                                (
                                    255,
                                    (160.0 + 22.0 * t) as u8,   // 160 -> 182
                                    (122.0 + 71.0 * t) as u8,   // 122 -> 193
                                )
                            };

                            node! {
                                text("█", color: (Color::Rgb(r, g, b)))
                            }
                        }).collect::<Vec<Node>>()),
                        text("·".repeat(empty), color: (Color::Rgb(80, 80, 80))),
                    ],

                    text(format!("{:>3}%", percentage), color: white, bold)
                ],

                // Instructions
                text("press esc or q to exit", color: bright_black)
            ]
        }
    }

    #[effect]
    async fn animate_progress(&self, ctx: &Context) {
        // Continuously animate the progress bar
        loop {
            for i in 0..=100 {
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                ctx.send(Msg::SetProgress(i as f32 / 100.0));
            }
            // Reset and loop
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(ProgressBar)
}
