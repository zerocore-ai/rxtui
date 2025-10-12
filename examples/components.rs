use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

#[derive(Debug, Clone)]
struct ResetSignal;

#[derive(Component)]
struct Counter {
    topic_name: String,
    label: String,
    color: Color,
}

#[derive(Debug, Clone)]
enum DashboardMsg {
    ResetAll,
    Exit,
}

#[derive(Component)]
struct Dashboard;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Counter {
    fn new(topic: impl Into<String>, label: impl Into<String>, color: Color) -> Self {
        Self {
            topic_name: topic.into(),
            label: label.into(),
            color,
        }
    }

    /// Using the new #[update] macro with dynamic topic support!
    #[update(msg = CounterMsg, topics = [self.topic_name => ResetSignal])]
    fn update(&self, _ctx: &Context, messages: Messages, mut state: CounterState) -> Action {
        match messages {
            Messages::CounterMsg(msg) => {
                match msg {
                    CounterMsg::Increment => state.count += 1,
                    CounterMsg::Decrement => state.count -= 1,
                }
                Action::update(state)
            }
            Messages::ResetSignal(_) => Action::update(CounterState::default()),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        // Create a darker version of the color for the label background
        let label_bg = match self.color {
            Color::Rgb(147, 112, 219) => Color::hex("#7B68AA"), // Darker purple for #9370DB
            Color::Rgb(255, 165, 0) => Color::hex("#CC8400"),   // Darker amber for #FFA500
            Color::Rgb(32, 178, 170) => Color::hex("#1A8D88"),  // Darker teal for #20B2AA
            _ => Color::hex("#333333"),                         // Fallback dark gray
        };

        // Create a darker version of the color for focus background
        let focus_bg = match self.color {
            Color::Rgb(147, 112, 219) => Color::hex("#6B4C9A"), // Darker purple for #9370DB
            Color::Rgb(255, 165, 0) => Color::hex("#CC6600"),   // Darker amber for #FFA500
            Color::Rgb(32, 178, 170) => Color::hex("#156B66"),  // Darker teal for #20B2AA
            _ => Color::hex("#222222"),                         // Fallback dark gray
        };

        node! {
            div(
                border: (self.color),
                pad: 2,
                w: 25,
                dir: vertical,
                align: center,
                focusable,
                focus_style: (Style::default().background(focus_bg)),
                @key(down): ctx.handler(CounterMsg::Decrement),
                @key(up): ctx.handler(CounterMsg::Increment)
            ) [
                div(bg: (label_bg), pad_h: 1) [
                    text(&self.label, color: black, bold, align: center)
                ],

                spacer(1),

                text(format!("Count: {}", state.count), color: white, bold, align: center),

                spacer(1),

                hstack(gap: 2, justify: center) [
                    div(bg: "#D32F2F", pad_h: 2, @click: ctx.handler(CounterMsg::Decrement)) [
                        text("-", color: white, bold)
                    ],
                    div(bg: "#388E3C", pad_h: 2, @click: ctx.handler(CounterMsg::Increment)) [
                        text("+", color: white, bold)
                    ]
                ]
            ]
        }
    }
}

impl Dashboard {
    #[update]
    fn update(&self, ctx: &Context, msg: DashboardMsg) -> Action {
        match msg {
            DashboardMsg::ResetAll => {
                // Send reset signal to all counter topics
                ctx.send_to_topic("counter_1", ResetSignal);
                ctx.send_to_topic("counter_2", ResetSignal);
                ctx.send_to_topic("counter_3", ResetSignal);
                Action::none()
            }
            DashboardMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(
                pad: 2,
                align: center,
                w_frac: 1.0,
                @char_global('q'): ctx.handler(DashboardMsg::Exit),
                @key_global(esc): ctx.handler(DashboardMsg::Exit)
            ) [
                // Header
                richtext(align: center) [
                    text("Counter Dashboard", color: "#20B2AA", bold),
                    text(" - Topic Communication Demo", color: bright_white)
                ],

                spacer(1),

                // Instructions
                text(
                    "tab to focus • ↑/↓ to change • r to reset all • q to quit",
                    color: bright_black,
                    align: center
                ),

                spacer(2),

                // Counters
                hstack(gap: 2, justify: center) [
                    node(Counter::new("counter_1", "Alpha", Color::hex("#9370DB"))),
                    node(Counter::new("counter_2", "Beta", Color::hex("#FFA500"))),
                    node(Counter::new("counter_3", "Gamma", Color::hex("#20B2AA")))
                ],

                spacer(2),

                // Reset button
                div(align: center) [
                    div(
                        bg: black,
                        border: white,
                        focusable,
                        focus_style: (Style::default().background(Color::hex("#333")).border(Color::White)),
                        @click: ctx.handler(DashboardMsg::ResetAll),
                        pad_h: 1,
                        @char('r'): ctx.handler(DashboardMsg::ResetAll)
                    ) [
                        text("Reset All Counters (R)", color: white, bold)
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
    App::new()?.run(Dashboard)
}
