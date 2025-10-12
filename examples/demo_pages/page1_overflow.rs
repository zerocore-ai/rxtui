use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
enum OverflowDemoMsg {
    SetParent1Color(usize),
    SetChild1Color(usize),
    SetParent2Color(usize),
    SetChild2Color(usize),
    SetParent3Color(usize),
    SetChild3Color(usize),
    SetLevel1Color(usize),
    SetLevel2Color(usize),
    SetLevel3Color(usize),
}

#[derive(Debug, Clone)]
struct OverflowDemoState {
    parent1_color_idx: usize,
    child1_color_idx: usize,
    parent2_color_idx: usize,
    child2_color_idx: usize,
    parent3_color_idx: usize,
    child3_color_idx: usize,
    level1_color_idx: usize,
    level2_color_idx: usize,
    level3_color_idx: usize,
}

#[derive(Component)]
pub struct Page1OverflowDemo;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for OverflowDemoState {
    fn default() -> Self {
        Self {
            parent1_color_idx: 0,
            child1_color_idx: 1,
            parent2_color_idx: 3,
            child2_color_idx: 5,
            parent3_color_idx: 7,
            child3_color_idx: 9,
            level1_color_idx: 2,
            level2_color_idx: 4,
            level3_color_idx: 6,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page1OverflowDemo {
    fn get_colors() -> [Color; 12] {
        [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Cyan,
            Color::Magenta,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightBlue,
            Color::BrightYellow,
            Color::BrightCyan,
            Color::BrightMagenta,
        ]
    }

    #[update]
    fn update(&self, ctx: &Context, msg: OverflowDemoMsg, mut state: OverflowDemoState) -> Action {
        let colors_len = Self::get_colors().len();

        match msg {
            OverflowDemoMsg::SetParent1Color(idx) => {
                state.parent1_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetChild1Color(idx) => {
                state.child1_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetParent2Color(idx) => {
                state.parent2_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetChild2Color(idx) => {
                state.child2_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetParent3Color(idx) => {
                state.parent3_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetChild3Color(idx) => {
                state.child3_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetLevel1Color(idx) => {
                state.level1_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetLevel2Color(idx) => {
                state.level2_color_idx = idx % colors_len;
            }
            OverflowDemoMsg::SetLevel3Color(idx) => {
                state.level3_color_idx = idx % colors_len;
            }
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: OverflowDemoState) -> Node {
        let colors = Self::get_colors();
        let colors_len = colors.len();

        let parent1_color = colors[state.parent1_color_idx];
        let child1_color = colors[state.child1_color_idx];
        let parent2_color = colors[state.parent2_color_idx];
        let child2_color = colors[state.child2_color_idx];
        let parent3_color = colors[state.parent3_color_idx];
        let child3_color = colors[state.child3_color_idx];
        let level1_color = colors[state.level1_color_idx];
        let level2_color = colors[state.level2_color_idx];
        let level3_color = colors[state.level3_color_idx];

        node! {
            div(bg: black, dir: vertical, pad: 2, w_frac: 1.0, h: 60) [
                // Title
                text("Page 1: Overflow Behavior", color: bright_white),
                spacer(2),

                // Example 1: Child smaller than parent
                text("Example 1 - Child smaller than parent (click to change colors):", color: white),
                spacer(1),
                div(bg: (parent1_color), w: 20, h: 6, pad: 1, @click: ctx.handler(OverflowDemoMsg::SetParent1Color(
                        (state.parent1_color_idx + 1) % colors_len
                    ))) [
                    div(bg: (child1_color), w: 10, h: 3, @click: ctx.handler(OverflowDemoMsg::SetChild1Color(
                            (state.child1_color_idx + 1) % colors_len
                        ))) [
                        text("Small", color: black)
                    ]
                ],
                spacer(3),

                // Example 2: Child larger than parent (overflow none)
                text("Example 2 - Child larger than parent (overflow: none):", color: white),
                spacer(1),
                div(bg: (parent2_color), w: 15, h: 5, pad: 1, @click: ctx.handler(OverflowDemoMsg::SetParent2Color(
                        (state.parent2_color_idx + 1) % colors_len
                    ))) [
                    div(bg: (child2_color), border: white, w: 20, h: 8, @click: ctx.handler(OverflowDemoMsg::SetChild2Color(
                            (state.child2_color_idx + 1) % colors_len
                        ))) [
                        text("Overflow", color: white)
                    ]
                ],
                spacer(6),

                // Example 3: Child larger than parent with overflow hidden
                text("Example 3 - Child larger than parent (overflow: hidden):", color: white),
                spacer(1),
                div(bg: (parent3_color), overflow: hidden, pad: 1, w: 15, h: 5, @click: ctx.handler(OverflowDemoMsg::SetParent3Color(
                        (state.parent3_color_idx + 1) % colors_len
                    ))) [
                    div(bg: (child3_color), border: white, w: 20, h: 8, @click: ctx.handler(OverflowDemoMsg::SetChild3Color(
                            (state.child3_color_idx + 1) % colors_len
                        ))) [
                        text("Hidden", color: black)
                    ]
                ],
                spacer(3),

                // Example 4: 3-level nesting with different overflow settings
                text("Example 4 - 3-level nesting (hidden -> none -> hidden):", color: white),
                spacer(1),
                div(bg: (level1_color), overflow: hidden, pad: 1, w: 25, h: 8, @click: ctx.handler(OverflowDemoMsg::SetLevel1Color(
                        (state.level1_color_idx + 1) % colors_len
                    ))) [
                    div(bg: (level2_color), pad: 1, w: 30, h: 10, @click: ctx.handler(OverflowDemoMsg::SetLevel2Color(
                            (state.level2_color_idx + 1) % colors_len
                        ))) [
                        div(bg: (level3_color), overflow: hidden, pad: 1, w: 35, h: 12, @click: ctx.handler(OverflowDemoMsg::SetLevel3Color(
                                (state.level3_color_idx + 1) % colors_len
                            ))) [
                            text("3-Level", color: white)
                        ]
                    ]
                ],
                spacer(3),

                // Example 5: Text overflow with overflow: none
                text("Example 5 - Text overflow (none - text can overflow):", color: white),
                spacer(1),
                div(bg: bright_blue, w: 15, h: 3, pad: 1) [
                    text("This is a very long text that will overflow the container bounds", color: white)
                ],
                spacer(3),

                // Example 6: Text overflow with overflow: hidden
                text("Example 6 - Text overflow (hidden - text is clipped):", color: white),
                spacer(1),
                div(bg: bright_green, overflow: hidden, w: 15, h: 3, pad: 1) [
                    text("This is a very long text that will be clipped at the container boundary", color: black)
                ]
            ]
        }
    }
}
