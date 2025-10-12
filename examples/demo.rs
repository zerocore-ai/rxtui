mod demo_pages;

use demo_pages::*;
use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum DemoMessage {
    SetPage(i32),
    NextPage,
    PrevPage,
    Exit,
}

#[derive(Debug, Clone)]
enum NavMsg {
    SetPage(i32),
}

#[derive(Debug, Clone)]
struct DemoState {
    current_page: i32,
}

#[derive(Component)]
struct Demo;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DemoState {
    fn default() -> Self {
        Self { current_page: 1 }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Demo {
    #[update(msg = DemoMessage, topics = ["navigation" => NavMsg])]
    fn update(&self, ctx: &Context, messages: Messages, mut state: DemoState) -> Action {
        let msg = match messages {
            Messages::DemoMessage(msg) => msg,
            Messages::NavMsg(NavMsg::SetPage(page)) => DemoMessage::SetPage(page),
        };

        match msg {
            DemoMessage::SetPage(page) => {
                state.current_page = page;
            }
            DemoMessage::NextPage => {
                state.current_page = (state.current_page % 16) + 1;
            }
            DemoMessage::PrevPage => {
                state.current_page = if state.current_page == 1 {
                    16
                } else {
                    state.current_page - 1
                };
            }
            DemoMessage::Exit => {
                return Action::exit();
            }
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: DemoState) -> Node {
        let page_content = match state.current_page {
            1 => node! { node(page1_overflow::Page1OverflowDemo) },
            2 => node! { node(page2_direction::Page2DirectionDemo) },
            3 => node! { node(page3_percentages::Page3PercentagesDemo) },
            4 => node! { node(page4_borders::Page4BordersDemo) },
            5 => node! { node(page5_absolute::Page5AbsoluteDemo) },
            6 => node! { node(page6_text_styles::Page6TextStylesDemo) },
            7 => node! { node(page7_auto_sizing::Page7AutoSizingDemo) },
            8 => node! { node(page8_text_wrap::Page8TextWrapDemo) },
            9 => node! { node(page9_element_wrap::Page9ElementWrapDemo) },
            10 => node! { node(page10_unicode::Page10UnicodeDemo) },
            11 => node! { node(page11_content_sizing::Page11ContentSizingDemo) },
            12 => node! { node(page12_focus::Page12FocusDemo) },
            13 => node! { node(page13_rich_text::Page13) },
            14 => node! { node(page14_text_input::Page14TextInputDemo) },
            15 => node! { node(page15_scrollable::Page15ScrollableDemo) },
            16 => node! { node(page16_text_alignment::Page16TextAlignmentDemo) },
            _ => node! { node(page1_overflow::Page1OverflowDemo) },
        };

        // Now we can use expressions in the node! macro
        node! {
            div(
                bg: black, dir: vertical, pad: 1, w_frac: 1.0, h_frac: 1.0,
                @char_global('q'): ctx.handler(DemoMessage::Exit),
                @key_global(esc): ctx.handler(DemoMessage::Exit),
                @char('1'): ctx.handler(DemoMessage::SetPage(1)),
                @char('2'): ctx.handler(DemoMessage::SetPage(2)),
                @char('3'): ctx.handler(DemoMessage::SetPage(3)),
                @char('4'): ctx.handler(DemoMessage::SetPage(4)),
                @char('5'): ctx.handler(DemoMessage::SetPage(5)),
                @char('6'): ctx.handler(DemoMessage::SetPage(6)),
                @char('7'): ctx.handler(DemoMessage::SetPage(7)),
                @char('8'): ctx.handler(DemoMessage::SetPage(8)),
                @char('9'): ctx.handler(DemoMessage::SetPage(9)),
                @char('0'): ctx.handler(DemoMessage::SetPage(10)),
                @char('-'): ctx.handler(DemoMessage::SetPage(11)),
                @char('='): ctx.handler(DemoMessage::SetPage(12)),
                @char('['): ctx.handler(DemoMessage::SetPage(13)),
                @char(']'): ctx.handler(DemoMessage::SetPage(14)),
                @char('\\'): ctx.handler(DemoMessage::SetPage(15)),
                @char(';'): ctx.handler(DemoMessage::SetPage(16)),
                @key(right): ctx.handler(DemoMessage::NextPage),
                @key(left): ctx.handler(DemoMessage::PrevPage)
            ) [
                // Header
                div(bg: bright_black, dir: horizontal, pad: 1, w_frac: 1.0, h: 3) [
                    text("Radical TUI Demo", color: bright_cyan),
                    div(w: 10) [],
                    text("Use ← → or 1-9 to navigate, 'q' to quit", color: bright_yellow)
                ],

                // Tab bar
                node(TabBar::new(state.current_page)),

                // Page content using expression
                (page_content)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Types: Tab Bar
//--------------------------------------------------------------------------------------------------

#[derive(Component)]
struct TabBar {
    current_page: i32,
}

//--------------------------------------------------------------------------------------------------
// Methods: Tab Bar
//--------------------------------------------------------------------------------------------------

impl TabBar {
    fn new(current_page: i32) -> Self {
        Self { current_page }
    }

    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: blue, dir: horizontal, h: 3, w_frac: 1.0) [
                node(Tab::new(1, "[1] Overflow", self.current_page)),
                node(Tab::new(2, "[2] Direction", self.current_page)),
                node(Tab::new(3, "[3] Percentages", self.current_page)),
                node(Tab::new(4, "[4] Borders", self.current_page)),
                node(Tab::new(5, "[5] Absolute", self.current_page)),
                node(Tab::new(6, "[6] Text Styles", self.current_page)),
                node(Tab::new(7, "[7] Auto Sizing", self.current_page)),
                node(Tab::new(8, "[8] Text Wrap", self.current_page)),
                node(Tab::new(9, "[9] Element Wrap", self.current_page)),
                node(Tab::new(10, "[0] Unicode", self.current_page)),
                node(Tab::new(11, "[-] Content Size", self.current_page)),
                node(Tab::new(12, "[=] Focus", self.current_page)),
                node(Tab::new(13, "[[] RichText", self.current_page)),
                node(Tab::new(14, "[]] TextInput", self.current_page)),
                node(Tab::new(15, "[\\] Scrollable", self.current_page)),
                node(Tab::new(16, "[;] Alignment", self.current_page))
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Types: Tab
//--------------------------------------------------------------------------------------------------

#[derive(Component, Default)]
struct Tab {
    page_num: i32,
    label: String,
    current_page: i32,
}

//--------------------------------------------------------------------------------------------------
// Methods: Tab
//--------------------------------------------------------------------------------------------------

impl Tab {
    fn new(page_num: i32, label: &str, current_page: i32) -> Self {
        Self {
            page_num,
            label: label.to_string(),
            current_page,
        }
    }

    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        let is_current = self.current_page == self.page_num;
        let bg_color = if is_current { Color::Cyan } else { Color::Blue };
        let text_color = if is_current {
            Color::Black
        } else {
            Color::White
        };
        let label = self.label.clone();
        let page_num = self.page_num;

        node! {
            div(bg: (bg_color), pad: 1, h: 3, w_auto, @click: ctx.topic_handler("navigation", NavMsg::SetPage(page_num))) [
                text(label, color: (text_color))
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(Demo)
}
