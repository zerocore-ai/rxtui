use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const DEFAULT_HINT: &str = "Focus a panel with Tab or a mouse click, then use arrow keys, Page Up/Down, Home/End, or the mouse wheel to explore scrolling.";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ScrollMsg {
    SetHint(&'static str),
    ResetHint,
    Exit,
}

#[derive(Debug, Clone)]
struct ScrollState {
    hint: &'static str,
}

#[derive(Component)]
pub struct ScrollExample;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ScrollState {
    fn default() -> Self {
        Self { hint: DEFAULT_HINT }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ScrollExample {
    #[update]
    fn update(&self, _ctx: &Context, msg: ScrollMsg, mut state: ScrollState) -> Action {
        match msg {
            ScrollMsg::SetHint(hint) => state.hint = hint,
            ScrollMsg::ResetHint => state.hint = DEFAULT_HINT,
            ScrollMsg::Exit => return Action::exit(),
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: ScrollState) -> Node {
        let vertical_lines: Vec<Node> = (1..=18)
            .map(|i| {
                node! {
                    text(
                        format!("Article line {i:02}: Scroll to reveal the rest of the content stream."),
                        color: white
                    )
                }
            })
            .collect();

        let vertical_hidden_lines: Vec<Node> = (1..=12)
            .map(|i| {
                node! {
                    text(
                        format!("Quiet log entry {i:02}: hidden scrollbar keeps the layout clean."),
                        color: bright_white
                    )
                }
            })
            .collect();

        let nested_outer_tail: Vec<Node> = (1..=6)
            .map(|i| {
                node! {
                    text(
                        format!("Outer note {i:02}: continue scrolling the shell container."),
                        color: bright_white
                    )
                }
            })
            .collect();

        let nested_checklist: Vec<Node> = (1..=10)
            .map(|i| {
                node! {
                    text(
                        format!("Checklist item {i:02}: nested scrolling zone with its own focus."),
                        color: cyan
                    )
                }
            })
            .collect();

        let nested_timeline: Vec<Node> = (1..=10)
            .map(|i| {
                node! {
                    text(
                        format!("Timeline marker {i:02}: smooth scrolling without a track."),
                        color: yellow
                    )
                }
            })
            .collect();

        node! {
            div(
                bg: "#070b15",
                dir: vertical,
                pad: 2,
                gap: 2,
                w_frac: 1.0,
                h_frac: 1.0,
                overflow: scroll,
                focusable,
                @focus: ctx.handler(ScrollMsg::ResetHint),
                @blur: ctx.handler(ScrollMsg::ResetHint),
                @key_global(esc): ctx.handler(ScrollMsg::Exit)
            ) [
                text("Scroll Surfaces Demo", color: bright_white, bold),
                text("Explore vertical, nested, and horizontal scrolling behaviors in RxTUI.", color: bright_black),
                text(format!("Hint: {}", state.hint), color: cyan),
                spacer(1),

                text("Vertical panels", color: bright_yellow, bold),
                div(dir: horizontal, gap: 2) [
                    div(
                        bg: "#102437",
                        border: cyan,
                        pad: 1,
                        overflow: scroll,
                        w: 46,
                        h: 14,
                        focusable,
                        @focus: ctx.handler(ScrollMsg::SetHint(
                            "Visible scrollbar: Up/Down, Page Up/Page Down, or the mouse wheel adjust the feed."
                        )),
                        @blur: ctx.handler(ScrollMsg::ResetHint)
                    ) [
                        text("Long article feed (scrollbar visible)", color: bright_cyan, bold),
                        spacer(1),
                        ...(vertical_lines)
                    ],
                    div(
                        bg: "#13291f",
                        border: green,
                        pad: 1,
                        overflow: scroll,
                        w: 46,
                        h: 14,
                        focusable,
                        show_scrollbar: false,
                        @focus: ctx.handler(ScrollMsg::SetHint(
                            "Hidden scrollbar: same gestures apply, but the track stays invisible for a minimal look."
                        )),
                        @blur: ctx.handler(ScrollMsg::ResetHint)
                    ) [
                        text("Minimal log (scrollbar hidden)", color: bright_green, bold),
                        spacer(1),
                        ...(vertical_hidden_lines)
                    ]
                ],
                spacer(1),

                text("Nested containers", color: bright_yellow, bold),
                div(
                    bg: "#1d1027",
                    border: magenta,
                    pad: 1,
                    gap: 1,
                    overflow: scroll,
                    w: 94,
                    h: 16,
                    focusable,
                    @focus: ctx.handler(ScrollMsg::SetHint(
                        "Outer container: use Tab to enter nested regions, Shift+Tab to return, and scroll to traverse all sections."
                    )),
                    @blur: ctx.handler(ScrollMsg::ResetHint)
                ) [
                    text("Travel planner (outer shell)", color: bright_magenta, bold),
                    text("Scroll here or Tab again to focus an inner panel.", color: bright_white),
                    spacer(1),
                    div(
                        bg: "#11332e",
                        border: cyan,
                        pad: 1,
                        overflow: scroll,
                        w: 70,
                        h: 6,
                        focusable,
                        @focus: ctx.handler(ScrollMsg::SetHint(
                            "Nested checklist: once focused, scrolling stays within this panel until you shift focus."
                        )),
                        @blur: ctx.handler(ScrollMsg::ResetHint)
                    ) [
                        text("Packing checklist", color: bright_cyan, bold),
                        spacer(1),
                        ...(nested_checklist)
                    ],
                    div(
                        bg: "#321414",
                        border: yellow,
                        pad: 1,
                        overflow: scroll,
                        w: 70,
                        h: 6,
                        focusable,
                        show_scrollbar: false,
                        @focus: ctx.handler(ScrollMsg::SetHint(
                            "Nested timeline: no scrollbar shown, but Home/End still jump between anchors."
                        )),
                        @blur: ctx.handler(ScrollMsg::ResetHint)
                    ) [
                        text("Day-by-day timeline", color: bright_yellow, bold),
                        spacer(1),
                        ...(nested_timeline)
                    ],
                    spacer(1),
                    ...(nested_outer_tail)
                ],
                spacer(1),
                text("Press Esc at any time to exit.", color: bright_black)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(ScrollExample)
}
