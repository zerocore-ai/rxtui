//! Inline Mode Example
//!
//! Demonstrates rendering directly in the terminal without alternate screen.
//! Features multiple stacked components including a scrollable list.
//! Content persists after the app exits, making it suitable for CLI tools.

use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const LOG_ENTRIES: &[&str] = &[
    "[INFO]  Application started successfully",
    "[DEBUG] Loading configuration from ~/.config/app.toml",
    "[INFO]  Connected to database at localhost:5432",
    "[WARN]  Cache miss for key: user_preferences",
    "[DEBUG] Fetching user data from remote API",
    "[INFO]  User authentication successful",
    "[DEBUG] Session token generated: abc...xyz",
    "[INFO]  Loading plugin: syntax-highlighter v2.1.0",
    "[INFO]  Loading plugin: auto-complete v1.5.3",
    "[WARN]  Plugin deprecated: legacy-formatter",
    "[DEBUG] Initializing render pipeline",
    "[INFO]  Theme loaded: dark-ocean",
    "[DEBUG] Font metrics calculated: 14px mono",
    "[INFO]  Workspace opened: ~/projects/rxtui",
    "[DEBUG] Indexing 1,247 files...",
    "[INFO]  Index complete in 0.8s",
    "[WARN]  Large file skipped: binary.dat (50MB)",
    "[DEBUG] LSP server starting: rust-analyzer",
    "[INFO]  LSP ready after 2.3s",
    "[DEBUG] Registering keyboard shortcuts",
    "[INFO]  Ready for input",
];

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    TogglePause,
    IncrementCounter,
    DecrementCounter,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct AppState {
    counter: i32,
    paused: bool,
    selected_log: usize,
}

#[derive(Component)]
struct InlineDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl InlineDemo {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: AppState) -> Action {
        match msg {
            Msg::TogglePause => {
                state.paused = !state.paused;
                Action::update(state)
            }
            Msg::IncrementCounter => {
                state.counter += 1;
                Action::update(state)
            }
            Msg::DecrementCounter => {
                state.counter -= 1;
                Action::update(state)
            }
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: AppState) -> Node {
        // Build log entries with color coding
        let logs: Vec<Node> = LOG_ENTRIES
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let color = if line.contains("[ERROR]") {
                    Color::Red
                } else if line.contains("[WARN]") {
                    Color::Yellow
                } else if line.contains("[DEBUG]") {
                    Color::BrightBlack
                } else {
                    Color::Cyan
                };

                let prefix = if i == state.selected_log {
                    "▶ "
                } else {
                    "  "
                };
                node! { text(format!("{prefix}{line}"), color: (color)) }
            })
            .collect();

        node! {
            div(
                dir: vertical,
                gap: 1,
                w_frac: 1.0,
                @key_global(esc): ctx.handler(Msg::Exit)
            ) [
                // Stats Row
                hstack(gap: 2) [
                    // Counter widget
                    div(
                        border_style: rounded,
                        border_color: magenta,
                        pad: 1,
                        w: 24,
                        focusable,
                        @key(up): ctx.handler(Msg::IncrementCounter),
                        @key(down): ctx.handler(Msg::DecrementCounter)
                    ) [
                        vstack(gap: 1, align: center) [
                            text("Counter", color: magenta, bold),
                            text(format!("{:+}", state.counter), color: white, bold),
                            text("↑/↓ to change", color: bright_black)
                        ]
                    ],

                    // Status widget
                    div(
                        border_style: rounded,
                        border_color: (if state.paused { Color::Yellow } else { Color::Green }),
                        pad: 1,
                        w: 24,
                        focusable,
                        @char('p'): ctx.handler(Msg::TogglePause)
                    ) [
                        vstack(gap: 1, align: center) [
                            text("Status", color: (if state.paused { Color::Yellow } else { Color::Green }), bold),
                            text(
                                if state.paused { "⏸ PAUSED" } else { "▶ RUNNING" },
                                color: white,
                                bold
                            ),
                            text("p to toggle", color: bright_black)
                        ]
                    ],
                ],

                // Scrollable Log Section (tall content to test viewport occlusion)
                div(
                    border_style: rounded,
                    border_color: bright_black,
                    pad: 1,
                    h: 12,
                    focusable
                ) [
                    vstack(gap: 0, overflow: scroll,) [
                        text("─── Application Log ───", color: bright_black, bold),
                        ...(logs)
                    ]
                ],

                // Footer
                div(pad_h: 1) [
                    richtext [
                        text("tab", color: cyan),
                        text(" focus │ ", color: bright_black),
                        text("esc", color: cyan),
                        text(" exit │ ", color: bright_black),
                        text("Content persists after exit!", color: yellow)
                    ]
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    // Use App::inline() for inline rendering mode
    // Content will persist after exit (preserve_on_exit is true by default)
    App::inline()?.run(InlineDemo)?;

    // This message appears after the UI since content is preserved
    println!("\nDemo completed. Notice the UI above is preserved!");

    Ok(())
}
