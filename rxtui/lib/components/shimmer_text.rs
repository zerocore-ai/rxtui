use crate::Context;
use crate::component::{Action, Component, Message, MessageExt};
use crate::effect::Effect;
use crate::node::{Node, RichText, TextSpan};
use crate::style::{Color, TextStyle};
use std::time::Duration;

//--------------------------------------------------------------------------------------------------
// Types: Internal
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ShimmerMsg {
    Tick,
}

#[derive(Debug, Clone, Default)]
struct ShimmerState {
    phase: usize,
}

//--------------------------------------------------------------------------------------------------
// Types: Public API
//--------------------------------------------------------------------------------------------------

/// Animation speed settings for [`ShimmerText`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShimmerSpeed {
    frame_delay_ms: u64,
    phase_step: usize,
}

/// A shimmering text component that creates a sweeping highlight effect.
///
/// The component renders text with a moving glow, similar to loading placeholders.
/// It manages its own animation loop using the effects system.
///
/// # Example
///
/// ```ignore
/// use rxtui::components::{ShimmerSpeed, ShimmerText};
///
/// let shimmer = ShimmerText::new("Loading...")
///     .speed(ShimmerSpeed::fast())
///     .gradient(Color::Rgb(60, 80, 130), Color::Rgb(210, 225, 255));
/// ```
#[derive(Clone)]
pub struct ShimmerText {
    content: String,
    speed: ShimmerSpeed,
    highlight_band: usize,
    base_color: (u8, u8, u8),
    highlight_color: (u8, u8, u8),
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const DEFAULT_HIGHLIGHT_BAND: usize = 6;
const DEFAULT_BASE_COLOR: (u8, u8, u8) = (70, 90, 130);
const DEFAULT_HIGHLIGHT_COLOR: (u8, u8, u8) = (210, 225, 255);

//--------------------------------------------------------------------------------------------------
// Trait Implementations: ShimmerSpeed
//--------------------------------------------------------------------------------------------------

impl Default for ShimmerSpeed {
    fn default() -> Self {
        Self::medium()
    }
}

impl ShimmerSpeed {
    /// Creates a new speed configuration.
    ///
    /// * `frame_delay_ms` - Delay between frames in milliseconds.
    /// * `phase_step` - Number of characters the highlight advances per frame.
    pub const fn new(frame_delay_ms: u64, phase_step: usize) -> Self {
        Self {
            frame_delay_ms,
            phase_step: if phase_step == 0 { 1 } else { phase_step },
        }
    }

    /// Slow shimmer animation (≈12 FPS).
    pub const fn slow() -> Self {
        Self::new(80, 1)
    }

    /// Medium shimmer animation (≈20 FPS).
    pub const fn medium() -> Self {
        Self::new(50, 1)
    }

    /// Fast shimmer animation (≈30 FPS) with a wider step to keep motion smooth.
    pub const fn fast() -> Self {
        Self::new(33, 2)
    }

    fn frame_delay(&self) -> Duration {
        Duration::from_millis(self.frame_delay_ms.max(1))
    }

    fn phase_step(&self) -> usize {
        self.phase_step.max(1)
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: ShimmerText Builders
//--------------------------------------------------------------------------------------------------

impl ShimmerText {
    /// Creates a new shimmer text component with the provided content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            speed: ShimmerSpeed::default(),
            highlight_band: DEFAULT_HIGHLIGHT_BAND,
            base_color: DEFAULT_BASE_COLOR,
            highlight_color: DEFAULT_HIGHLIGHT_COLOR,
        }
    }

    /// Sets the animation speed.
    pub fn speed(mut self, speed: ShimmerSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the highlight band width (minimum of 1).
    /// Larger bands create a softer, wider shimmer.
    pub fn highlight_band(mut self, band: usize) -> Self {
        self.highlight_band = band.max(1);
        self
    }

    /// Sets the base color of the text.
    pub fn base_color(mut self, color: Color) -> Self {
        self.base_color = color_to_rgb(color);
        self
    }

    /// Sets the highlight color of the shimmer.
    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color_to_rgb(color);
        self
    }

    /// Sets both base and highlight colors at once.
    pub fn gradient(mut self, base: Color, highlight: Color) -> Self {
        self.base_color = color_to_rgb(base);
        self.highlight_color = color_to_rgb(highlight);
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: ShimmerText Component Logic
//--------------------------------------------------------------------------------------------------

impl ShimmerText {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        if let Some(msg) = msg.downcast::<ShimmerMsg>() {
            match msg {
                ShimmerMsg::Tick => {
                    let total = self.char_count();
                    if total == 0 {
                        return Action::none();
                    }

                    let mut state = ctx.get_state::<ShimmerState>();
                    state.phase = (state.phase + self.speed.phase_step()) % total;
                    return Action::update(state);
                }
            }
        }

        Action::none()
    }

    fn view(&self, ctx: &Context) -> Node {
        let chars: Vec<char> = self.content.chars().collect();
        if chars.is_empty() {
            return RichText::new().into();
        }

        let total = chars.len();
        let state = ctx.get_state::<ShimmerState>();

        let spans = chars
            .into_iter()
            .enumerate()
            .map(|(index, ch)| {
                let intensity = self.intensity_for_index(index, state.phase, total);
                let color = self.blend_color(intensity);

                TextSpan {
                    content: ch.to_string(),
                    style: Some(TextStyle {
                        color: Some(color),
                        ..Default::default()
                    }),
                    is_cursor: false,
                }
            })
            .collect();

        RichText { spans, style: None }.into()
    }

    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        let delay = self.speed.frame_delay();
        let ctx = ctx.clone();

        let effect = Box::pin(async move {
            loop {
                tokio::time::sleep(delay).await;
                ctx.send(ShimmerMsg::Tick);
            }
        });

        vec![effect]
    }

    fn char_count(&self) -> usize {
        self.content.chars().count()
    }

    fn intensity_for_index(&self, index: usize, phase: usize, total: usize) -> f32 {
        if total == 0 {
            return 0.0;
        }

        (0..self.highlight_band)
            .map(|offset| {
                let pos = (phase + offset) % total;
                self.circular_distance(index, pos, total)
            })
            .min()
            .map(|distance| {
                let normalized = 1.0 - (distance as f32 / self.highlight_band as f32);
                normalized.clamp(0.0, 1.0).powf(1.4)
            })
            .unwrap_or(0.0)
    }

    fn circular_distance(&self, a: usize, b: usize, total: usize) -> usize {
        let diff = a.abs_diff(b);
        diff.min(total - diff)
    }

    fn blend_color(&self, intensity: f32) -> Color {
        let eased = 0.2 + intensity * 0.8;

        let r = blend_channel(self.base_color.0, self.highlight_color.0, eased);
        let g = blend_channel(self.base_color.1, self.highlight_color.1, eased);
        let b = blend_channel(self.base_color.2, self.highlight_color.2, eased);

        Color::Rgb(r, g, b)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: Component
//--------------------------------------------------------------------------------------------------

impl Component for ShimmerText {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        ShimmerText::update(self, ctx, msg, topic)
    }

    fn view(&self, ctx: &Context) -> Node {
        ShimmerText::view(self, ctx)
    }

    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        ShimmerText::effects(self, ctx)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Helpers
//--------------------------------------------------------------------------------------------------

fn blend_channel(start: u8, end: u8, factor: f32) -> u8 {
    let start = start as f32;
    let end = end as f32;
    (start + (end - start) * factor).round().clamp(0.0, 255.0) as u8
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Black => (0, 0, 0),
        Color::Red => (205, 49, 49),
        Color::Green => (13, 188, 121),
        Color::Yellow => (229, 229, 16),
        Color::Blue => (36, 114, 200),
        Color::Magenta => (188, 63, 188),
        Color::Cyan => (17, 168, 205),
        Color::White => (229, 229, 229),
        Color::BrightBlack => (102, 102, 102),
        Color::BrightRed => (241, 76, 76),
        Color::BrightGreen => (35, 209, 139),
        Color::BrightYellow => (245, 245, 67),
        Color::BrightBlue => (59, 142, 234),
        Color::BrightMagenta => (214, 112, 214),
        Color::BrightCyan => (41, 184, 219),
        Color::BrightWhite => (255, 255, 255),
        Color::Rgb(r, g, b) => (r, g, b),
    }
}
