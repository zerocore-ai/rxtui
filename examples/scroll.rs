use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const BODY_LINES: &[&str] = &[
    "The terminal can host surprisingly rich reading experiences when paired with smooth scrolling.",
    "This demo fills a single container with - a long-form article so you can practice focused navigation.",
    "",
    "Scrolling tips:",
    "  • Use the ↑ and ↓ arrow keys for line-by-line movement.",
    "  • Page Up/Page Down jump a full viewport at a time.",
    "  • Home and End take you to the start or end of the article.",
    "  • Mouse wheels and touchpads work automatically when the panel is focused.",
    "",
    "Chapter 1 - Setting the Scene:",
    "  The hum of servers echoes in the background while a developer studies logs.",
    "  Lines of text flow upward, revealing patterns, anomalies, and the occasional surprise.",
    "  With a scrollable viewport the developer can linger on details without losing the bigger picture.",
    "",
    "Chapter 2 - Building the Interface:",
    "  RxTUI components keep layout and interaction terse, yet expressive.",
    "  A single `div` with `overflow: scroll` handles keyboard, mouse, and focus behaviors.",
    "  Styling remains declarative: borders, spacing, and alignment compose like CSS-inspired building blocks.",
    "",
    "Chapter 3 - Ergonomic Patterns:",
    "  Keep navigation hints close to the scroll surface so users know how to explore.",
    "  Combine headings, separators, and whitespace to make dense text approachable.",
    "  Remember that even in a terminal, typography and layout choices shape comprehension.",
    "",
    "Chapter 4 - When Content Grows:",
    "  Logs, documentation, chat transcripts, and game narratives all benefit from scrolling containers.",
    "  By anchoring the viewport height you prevent the UI from exploding beyond the terminal boundaries.",
    "  Nested scroll regions can isolate noise, letting readers focus on what matters right now.",
    "",
    "Chapter 5 - Wrapping Up:",
    "  Scroll surfaces are foundational to building comfortable text-first interfaces.",
    "  Experiment with scrollbar visibility, focus styling, and keyboard shortcuts to match your audience.",
    "  With practice, terminal apps feel less like raw text dumps and more like polished editors.",
    "",
    "You reached the end! Press Esc to return to your shell.",
];

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ScrollViewMsg {
    Exit,
}

#[derive(Debug, Clone, Default)]
struct ScrollViewState;

#[derive(Component)]
pub struct ScrollTextExample;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ScrollTextExample {
    #[update]
    fn update(&self, _ctx: &Context, msg: ScrollViewMsg, _state: ScrollViewState) -> Action {
        match msg {
            ScrollViewMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, _state: ScrollViewState) -> Node {
        let body: Vec<Node> = BODY_LINES
            .iter()
            .map(|line| node! { text(*line, color: white) })
            .collect();

        node! {
            div(
                bg: "#030507",
                dir: vertical,
                pad: 2,
                gap: 1,
                w_frac: 1.0,
                h_frac: 1.0,
                align: center,
                @key_global(esc): ctx.handler(ScrollViewMsg::Exit)
            ) [
                text("Scroll Demo - Reading View", color: bright_white, bold),
                text("Focus the article panel and use your preferred scrolling method to browse the content.", color: bright_black),
                spacer(1),
                div(
                    bg: "#10161b",
                    border: cyan,
                    pad: 2,
                    gap: 1,
                    overflow: scroll,
                    w: 80,
                    h: 20,
                    focusable
                ) [
                    ...(body)
                ],
                spacer(1),
                text("Tip: Tab to focus the article if the scroll keys stop responding.", color: bright_black)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(ScrollTextExample)
}
