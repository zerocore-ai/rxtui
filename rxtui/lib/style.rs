//! Styling system for terminal UI models.
//!
//! This module provides types and builders for styling terminal UI elements
//! including colors, spacing, and layout direction.
//!
//! ## Color Support
//!
//! The framework supports:
//! - 16 standard terminal colors (8 normal + 8 bright)
//! - 24-bit RGB colors (on terminals that support it)
//!
//! ## Style Composition
//!
//! ```text
//!   Style
//!   ├── background: Color
//!   ├── direction: Layout
//!   └── padding: Spacing
//! ```

use bitflags::bitflags;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Dimension specification for element sizing.
///
/// Determines how an element's width or height is calculated.
/// Supports fixed sizes, percentage-based sizing, automatic
/// sizing based on available space, and content-based sizing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    /// Fixed size in terminal cells
    Fixed(u16),

    /// Percentage of parent's dimension (stored as 0.0 to 1.0)
    ///
    /// For root elements, this is relative to the viewport.
    /// For child elements, this is relative to the parent's
    /// content box (after padding is applied). Helpers such as
    /// `w_frac`/`h_frac` accept 0.0–1.0 fractions and clamp values
    /// into this range internally.
    Percentage(f32),

    /// Automatically size based on available space
    ///
    /// Auto-sized elements share the remaining space equally
    /// after fixed and percentage sizes are calculated.
    /// For text nodes, auto width uses content length.
    Auto,

    /// Size based on content
    ///
    /// Element grows to fit its children's natural size.
    /// For containers, this means:
    /// - Horizontal layout: width = sum of children widths, height = max child height
    /// - Vertical layout: width = max child width, height = sum of children heights
    ///
    /// For text nodes, uses the natural text dimensions.
    Content,
}

/// Represents spacing values for all four sides of an element.
///
/// Used for padding, margins, and borders. Values are in terminal cells.
///
/// ## Visual Representation
///
/// ```text
///         top
///     ┌─────────┐
///     │         │
/// left│ Content │right
///     │         │
///     └─────────┘
///        bottom
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing {
    /// Spacing above the content
    pub top: u16,

    /// Spacing to the right of the content
    pub right: u16,

    /// Spacing below the content
    pub bottom: u16,

    /// Spacing to the left of the content
    pub left: u16,
}

/// Terminal color definitions.
///
/// Supports both standard 16-color palette and 24-bit RGB colors.
///
/// ## Color Palette
///
/// Standard colors (0-7):
/// ```text
/// Black   Red     Green   Yellow
/// Blue    Magenta Cyan    White
/// ```
///
/// Bright colors (8-15):
/// ```text
/// BrightBlack   BrightRed     BrightGreen   BrightYellow
/// BrightBlue    BrightMagenta BrightCyan    BrightWhite
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Standard black (color 0)
    Black,

    /// Standard red (color 1)
    Red,

    /// Standard green (color 2)
    Green,

    /// Standard yellow (color 3)
    Yellow,

    /// Standard blue (color 4)
    Blue,

    /// Standard magenta (color 5)
    Magenta,

    /// Standard cyan (color 6)
    Cyan,

    /// Standard white (color 7)
    White,

    /// Bright black / dark gray (color 8)
    BrightBlack,

    /// Bright red (color 9)
    BrightRed,

    /// Bright green (color 10)
    BrightGreen,

    /// Bright yellow (color 11)
    BrightYellow,

    /// Bright blue (color 12)
    BrightBlue,

    /// Bright magenta (color 13)
    BrightMagenta,

    /// Bright cyan (color 14)
    BrightCyan,

    /// Bright white (color 15)
    BrightWhite,

    /// 24-bit RGB color (requires terminal support)
    Rgb(u8, u8, u8),
}

/// Layout direction for arranging child elements.
///
/// ## Visual Examples
///
/// Vertical:
/// ```text
/// ┌─────────┐
/// │ Child 1 │
/// ├─────────┤
/// │ Child 2 │
/// ├─────────┤
/// │ Child 3 │
/// └─────────┘
/// ```
///
/// Horizontal:
/// ```text
/// ┌─────┬─────┬─────┐
/// │ Ch1 │ Ch2 │ Ch3 │
/// └─────┴─────┴─────┘
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// Stack children vertically (top to bottom)
    Vertical,

    /// Stack children horizontally (left to right)
    Horizontal,
}

/// Overflow behavior for content that exceeds container bounds.
///
/// Controls how content is displayed when it's larger than its container.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Overflow {
    /// Content is not clipped and may be rendered outside the container bounds (default)
    None,

    /// Content is clipped at the container boundaries
    Hidden,

    /// Content is clipped but scrollable with keyboard and mouse
    Scroll,

    /// Automatically show scrollbars when content overflows
    Auto,
}

/// Text alignment modes for controlling horizontal text positioning.
///
/// Determines how text content is aligned within its container.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextAlign {
    /// Align text to the left edge (default)
    #[default]
    Left,

    /// Center text horizontally
    Center,

    /// Align text to the right edge
    Right,
}

/// Text wrapping modes for controlling how text breaks across lines.
///
/// Determines how text content wraps when it exceeds its container width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWrap {
    /// No wrapping - text overflows or is clipped (default)
    None,

    /// Break at any character boundary
    /// Good for fixed-width content or when space is limited
    Character,

    /// Break only at word boundaries (spaces)
    /// Words longer than line width will overflow
    Word,

    /// Break at word boundaries, but break words if necessary
    /// Ensures text never exceeds the specified width
    WordBreak,
}

/// Element wrapping modes for controlling how children wrap.
///
/// Determines how child elements wrap when they exceed container width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WrapMode {
    /// No wrapping - children laid out in single row/column (default)
    NoWrap,

    /// Wrap children to next row/column when space runs out
    Wrap,

    /// Wrap children in reverse direction
    WrapReverse,
}

/// Positioning mode for elements.
///
/// Determines how an element is positioned relative to its parent or the viewport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    /// Element is positioned in normal document flow (default)
    /// Children are laid out according to parent's direction
    Relative,

    /// Element is positioned relative to its nearest positioned ancestor
    /// Removed from normal document flow, doesn't affect sibling layout
    Absolute,

    /// Element is positioned relative to the viewport
    /// Similar to absolute but always relative to the terminal window
    Fixed,
}

/// Controls how content is distributed along the main axis.
///
/// The main axis is determined by the Direction:
/// - Horizontal: main axis is horizontal (left to right)
/// - Vertical: main axis is vertical (top to bottom)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JustifyContent {
    /// Pack items at the start of the main axis (default)
    #[default]
    Start,

    /// Center items along the main axis
    Center,

    /// Pack items at the end of the main axis
    End,

    /// Distribute items evenly, first at start, last at end
    SpaceBetween,

    /// Distribute items evenly with equal space around each item
    SpaceAround,

    /// Distribute items evenly with equal space between and around items
    SpaceEvenly,
}

/// Controls how items are aligned on the cross axis.
///
/// The cross axis is perpendicular to the main axis:
/// - Horizontal layout: cross axis is vertical
/// - Vertical layout: cross axis is horizontal
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlignItems {
    /// Align items at the start of the cross axis (default)
    #[default]
    Start,

    /// Center items along the cross axis
    Center,

    /// Align items at the end of the cross axis
    End,
}

/// Allows an item to override its parent's AlignItems setting.
///
/// Individual items can specify their own alignment on the cross axis,
/// overriding the parent container's AlignItems value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlignSelf {
    /// Use the parent's AlignItems value (default)
    #[default]
    Auto,

    /// Align at the start of the cross axis
    Start,

    /// Center along the cross axis
    Center,

    /// Align at the end of the cross axis
    End,
}

bitflags! {
    /// Flags to control which border edges and corners are rendered.
    ///
    /// Can be combined using bitwise OR to create custom border configurations.
    /// For example: `BorderEdges::TOP | BorderEdges::BOTTOM` for horizontal borders only.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BorderEdges: u8 {
        /// Top edge
        const TOP = 0b00000001;
        /// Right edge
        const RIGHT = 0b00000010;
        /// Bottom edge
        const BOTTOM = 0b00000100;
        /// Left edge
        const LEFT = 0b00001000;
        /// Top-left corner
        const TOP_LEFT = 0b00010000;
        /// Top-right corner
        const TOP_RIGHT = 0b00100000;
        /// Bottom-right corner
        const BOTTOM_RIGHT = 0b01000000;
        /// Bottom-left corner
        const BOTTOM_LEFT = 0b10000000;

        /// All edges and corners
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits() |
                    Self::TOP_LEFT.bits() | Self::TOP_RIGHT.bits() | Self::BOTTOM_RIGHT.bits() | Self::BOTTOM_LEFT.bits();
        /// All edges (no corners)
        const EDGES = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
        /// All corners (no edges)
        const CORNERS = Self::TOP_LEFT.bits() | Self::TOP_RIGHT.bits() | Self::BOTTOM_RIGHT.bits() | Self::BOTTOM_LEFT.bits();
        /// Horizontal edges (top and bottom)
        const HORIZONTAL = Self::TOP.bits() | Self::BOTTOM.bits();
        /// Vertical edges (left and right)
        const VERTICAL = Self::LEFT.bits() | Self::RIGHT.bits();
    }
}

/// Border style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// Single line border (┌─┐│└┘)
    #[default]
    Single,

    /// Double line border (╔═╗║╚╝)
    Double,

    /// Thick/heavy line border (┏━┓┃┗┛)
    Thick,

    /// Rounded corners (╭─╮│╰╯)
    Rounded,

    /// Dashed line border (┌╌┐╎└╌┘)
    Dashed,
}

/// Border configuration for UI elements.
///
/// Defines border styling including whether borders are shown
/// and their color. Borders are drawn inset, taking up space
/// from the element's content area.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    /// Whether to show the border
    pub enabled: bool,

    /// Border style
    pub style: BorderStyle,

    /// Border color
    pub color: Color,

    /// Which edges and corners to render
    pub edges: BorderEdges,
}

/// Complete style definition for a UI element.
///
/// Combines colors, layout, and spacing properties.
/// All properties are optional and inherit defaults if not set.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// Background fill color
    pub background: Option<Color>,

    /// Layout direction for children
    pub direction: Option<Direction>,

    /// Inner spacing around content
    pub padding: Option<Spacing>,

    /// Overflow behavior for content exceeding bounds
    pub overflow: Option<Overflow>,

    /// Width dimension specification
    pub width: Option<Dimension>,

    /// Height dimension specification
    pub height: Option<Dimension>,

    /// Border configuration
    pub border: Option<Border>,

    /// Positioning mode (relative, absolute, fixed)
    pub position: Option<Position>,

    /// Z-index for layering (higher values render on top)
    pub z_index: Option<i32>,

    /// Position offset from top edge (for absolute/fixed positioning)
    pub top: Option<i16>,

    /// Position offset from right edge (for absolute/fixed positioning)
    pub right: Option<i16>,

    /// Position offset from bottom edge (for absolute/fixed positioning)
    pub bottom: Option<i16>,

    /// Position offset from left edge (for absolute/fixed positioning)
    pub left: Option<i16>,

    /// Wrapping mode for child elements
    pub wrap: Option<WrapMode>,

    /// Gap between wrapped rows/columns
    pub gap: Option<u16>,

    /// Outer spacing around element
    pub margin: Option<Spacing>,

    /// Minimum width constraint
    pub min_width: Option<u16>,

    /// Minimum height constraint
    pub min_height: Option<u16>,

    /// Maximum width constraint
    pub max_width: Option<u16>,

    /// Maximum height constraint
    pub max_height: Option<u16>,

    /// Border color
    pub border_color: Option<Color>,

    /// Absolute X position (legacy, prefer left/right)
    pub x: Option<u16>,

    /// Absolute Y position (legacy, prefer top/bottom)
    pub y: Option<u16>,

    /// Whether to show scrollbar for scrollable content
    pub show_scrollbar: Option<bool>,

    /// Controls how content is distributed along the main axis
    pub justify_content: Option<JustifyContent>,

    /// Controls how items are aligned on the cross axis
    pub align_items: Option<AlignItems>,

    /// Allows this element to override parent's align_items
    pub align_self: Option<AlignSelf>,
}

/// Style properties specific to text elements.
///
/// Controls the visual appearance of text including color,
/// decorations, and other text-specific properties.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// Foreground color of the text
    pub color: Option<Color>,

    /// Background color behind the text
    pub background: Option<Color>,

    /// Bold text weight
    pub bold: Option<bool>,

    /// Italic text style
    pub italic: Option<bool>,

    /// Underlined text decoration
    pub underline: Option<bool>,

    /// Strikethrough text decoration
    pub strikethrough: Option<bool>,

    /// Text wrapping mode
    pub wrap: Option<TextWrap>,

    /// Text alignment within container
    pub align: Option<TextAlign>,
}

/// Builder for creating styles with a fluent API.
///
/// ## Example
///
/// ```text
/// Style::new()
///     .background(Color::Blue)
///     .padding(Spacing::all(2))
///     .build()
/// ```
pub struct StyleBuilder {
    /// The style being built
    style: Style,
}

/// Builder for creating text styles with a fluent API.
///
/// ## Example
///
/// ```text
/// TextStyle::new()
///     .color(Color::Red)
///     .background(Color::Blue)
///     .build()
/// ```
pub struct TextStyleBuilder {
    /// The text style being built
    style: TextStyle,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Color {
    /// Creates a Color from a hex string.
    ///
    /// Supports multiple formats:
    /// - 1 digit: `"F"` or `"#F"` → grayscale RGB(255, 255, 255)
    /// - 3 digits: `"F53"` or `"#F53"` → RGB(255, 85, 51)
    /// - 6 digits: `"FF5733"` or `"#FF5733"` → RGB(255, 87, 51)
    ///
    /// The `#` prefix is optional. Parsing is case-insensitive.
    ///
    /// ## Examples
    ///
    /// ```text
    /// let white = Color::from_hex("#F")?;      // RGB(255, 255, 255)
    /// let gray = Color::from_hex("8")?;        // RGB(136, 136, 136)
    /// let red = Color::from_hex("#F00")?;      // RGB(255, 0, 0)
    /// let orange = Color::from_hex("#FF5733")?; // RGB(255, 87, 51)
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        // Remove the # prefix if present
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            1 => {
                // Single digit - use for all channels (grayscale)
                let digit = hex.chars().next().unwrap();
                let value = parse_hex_digit(digit)?;
                // Expand single hex digit to two (e.g., F -> FF)
                let expanded = (value << 4) | value;
                Ok(Color::Rgb(expanded, expanded, expanded))
            }
            3 => {
                // 3 digits - expand each to 2 digits
                let mut chars = hex.chars();
                let r = parse_hex_digit(chars.next().unwrap())?;
                let g = parse_hex_digit(chars.next().unwrap())?;
                let b = parse_hex_digit(chars.next().unwrap())?;
                // Expand single hex digit to two (e.g., F -> FF)
                let r_expanded = (r << 4) | r;
                let g_expanded = (g << 4) | g;
                let b_expanded = (b << 4) | b;
                Ok(Color::Rgb(r_expanded, g_expanded, b_expanded))
            }
            6 => {
                // 6 digits - parse as RRGGBB
                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color format")?;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color format")?;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color format")?;
                Ok(Color::Rgb(r, g, b))
            }
            _ => Err("Hex color must be 1, 3, or 6 digits"),
        }
    }

    /// Creates a Color from a hex string, panicking on invalid input.
    ///
    /// This is a convenience method for use with compile-time constants
    /// where you know the hex string is valid.
    ///
    /// ## Examples
    ///
    /// ```text
    /// const BRAND_COLOR: Color = Color::hex("#FF5733");
    /// const WHITE: Color = Color::hex("F");
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the hex string is invalid.
    pub fn hex(hex: &str) -> Self {
        Self::from_hex(hex).expect("Invalid hex color")
    }

    /// Creates an RGB color from individual red, green, and blue components.
    ///
    /// This is a convenience constructor that's equivalent to using the
    /// `Color::Rgb` variant directly.
    ///
    /// ## Examples
    ///
    /// ```text
    /// let orange = Color::rgb(255, 165, 0);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }
}

/// Parses a single hex digit into a u8 value.
fn parse_hex_digit(c: char) -> Result<u8, &'static str> {
    let upper = c.to_ascii_uppercase();
    match upper {
        '0'..='9' => Ok(upper as u8 - b'0'),
        'A'..='F' => Ok(upper as u8 - b'A' + 10),
        _ => Err("Invalid hex digit"),
    }
}

impl Style {
    /// Creates the default focus style for focusable elements.
    ///
    /// Provides a yellow border to indicate focus state.
    /// This style is automatically applied to focusable elements
    /// and can be overridden with custom focus styles.
    pub fn default_focus() -> Style {
        Style {
            border: Some(Border {
                enabled: true,
                style: BorderStyle::Single,
                color: Color::Yellow,
                edges: BorderEdges::ALL,
            }),
            ..Default::default()
        }
    }

    /// Merges two styles, with the overlay style taking precedence.
    ///
    /// This is used to apply focus styles on top of base styles.
    /// Any property set in the overlay will override the base.
    pub fn merge(base: Option<Style>, overlay: Option<Style>) -> Option<Style> {
        match (base, overlay) {
            (None, None) => None,
            (Some(base), None) => Some(base),
            (None, Some(overlay)) => Some(overlay),
            (Some(mut base), Some(overlay)) => {
                // Overlay each property if it's set
                if overlay.background.is_some() {
                    base.background = overlay.background;
                }
                if overlay.direction.is_some() {
                    base.direction = overlay.direction;
                }
                if overlay.padding.is_some() {
                    base.padding = overlay.padding;
                }
                if overlay.overflow.is_some() {
                    base.overflow = overlay.overflow;
                }
                if overlay.width.is_some() {
                    base.width = overlay.width;
                }
                if overlay.height.is_some() {
                    base.height = overlay.height;
                }
                if overlay.border.is_some() {
                    base.border = overlay.border;
                }
                if overlay.position.is_some() {
                    base.position = overlay.position;
                }
                if overlay.z_index.is_some() {
                    base.z_index = overlay.z_index;
                }
                if overlay.top.is_some() {
                    base.top = overlay.top;
                }
                if overlay.right.is_some() {
                    base.right = overlay.right;
                }
                if overlay.bottom.is_some() {
                    base.bottom = overlay.bottom;
                }
                if overlay.left.is_some() {
                    base.left = overlay.left;
                }
                if overlay.wrap.is_some() {
                    base.wrap = overlay.wrap;
                }
                if overlay.gap.is_some() {
                    base.gap = overlay.gap;
                }
                if overlay.show_scrollbar.is_some() {
                    base.show_scrollbar = overlay.show_scrollbar;
                }
                if overlay.justify_content.is_some() {
                    base.justify_content = overlay.justify_content;
                }
                if overlay.align_items.is_some() {
                    base.align_items = overlay.align_items;
                }
                if overlay.align_self.is_some() {
                    base.align_self = overlay.align_self;
                }
                Some(base)
            }
        }
    }

    /// Sets the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the layout direction for child elements.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Sets the inner padding around content.
    pub fn padding(mut self, padding: Spacing) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Sets the overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Sets the width dimension.
    pub fn width(mut self, width: Dimension) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height dimension.
    pub fn height(mut self, height: Dimension) -> Self {
        self.height = Some(height);
        self
    }

    /// Enables border with the specified color.
    pub fn border(mut self, color: Color) -> Self {
        self.border = Some(Border {
            enabled: true,
            style: BorderStyle::Single,
            color,
            edges: BorderEdges::ALL,
        });
        self
    }

    /// Sets the positioning mode.
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Sets the z-index for layering.
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.z_index = Some(z_index);
        self
    }

    /// Sets the top position offset.
    pub fn top(mut self, top: i16) -> Self {
        self.top = Some(top);
        self
    }

    /// Sets the right position offset.
    pub fn right(mut self, right: i16) -> Self {
        self.right = Some(right);
        self
    }

    /// Sets the bottom position offset.
    pub fn bottom(mut self, bottom: i16) -> Self {
        self.bottom = Some(bottom);
        self
    }

    /// Sets the left position offset.
    pub fn left(mut self, left: i16) -> Self {
        self.left = Some(left);
        self
    }

    /// Sets the wrapping mode for child elements.
    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// Sets the gap between wrapped rows/columns.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Sets whether to show scrollbar for scrollable content.
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = Some(show);
        self
    }
}

impl Border {
    /// Creates a new border with the specified color, default style (Single), and all edges.
    pub fn new(color: Color) -> Self {
        Self {
            enabled: true,
            style: BorderStyle::Single,
            color,
            edges: BorderEdges::ALL,
        }
    }

    /// Creates a disabled border, useful for explicitly clearing borders in styles.
    pub fn none() -> Self {
        Self {
            enabled: false,
            style: BorderStyle::Single,
            color: Color::White,
            edges: BorderEdges::ALL,
        }
    }

    /// Creates a new border with the specified style and color, rendering all edges.
    pub fn with_style(style: BorderStyle, color: Color) -> Self {
        Self {
            enabled: true,
            style,
            color,
            edges: BorderEdges::ALL,
        }
    }

    /// Creates a new border with the specified style, color, and edges.
    pub fn with_edges(style: BorderStyle, color: Color, edges: BorderEdges) -> Self {
        Self {
            enabled: true,
            style,
            color,
            edges,
        }
    }
}

impl Style {
    /// Creates a new style builder with all properties unset.
    #[deprecated(note = "Use Style::default() with builder methods instead")]
    pub fn builder() -> StyleBuilder {
        StyleBuilder {
            style: Style::default(),
        }
    }
}

impl TextStyle {
    /// Merges two text styles, with the overlay style taking precedence.
    ///
    /// This is used to apply custom text styles on top of default styles.
    /// Any property set in the overlay will override the base.
    pub fn merge(base: Option<TextStyle>, overlay: Option<TextStyle>) -> Option<TextStyle> {
        match (base, overlay) {
            (None, None) => None,
            (Some(base), None) => Some(base),
            (None, Some(overlay)) => Some(overlay),
            (Some(mut base), Some(overlay)) => {
                // Overlay each property if it's set
                if overlay.color.is_some() {
                    base.color = overlay.color;
                }
                if overlay.background.is_some() {
                    base.background = overlay.background;
                }
                if overlay.bold.is_some() {
                    base.bold = overlay.bold;
                }
                if overlay.italic.is_some() {
                    base.italic = overlay.italic;
                }
                if overlay.underline.is_some() {
                    base.underline = overlay.underline;
                }
                if overlay.strikethrough.is_some() {
                    base.strikethrough = overlay.strikethrough;
                }
                if overlay.wrap.is_some() {
                    base.wrap = overlay.wrap;
                }
                if overlay.align.is_some() {
                    base.align = overlay.align;
                }
                Some(base)
            }
        }
    }

    /// Creates a new text style builder with all properties unset.
    #[deprecated(note = "Use TextStyle::default() with builder methods instead")]
    pub fn builder() -> TextStyleBuilder {
        TextStyleBuilder {
            style: TextStyle {
                color: None,
                background: None,
                bold: None,
                italic: None,
                underline: None,
                strikethrough: None,
                wrap: None,
                align: None,
            },
        }
    }

    /// Sets the text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Makes the text bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Makes the text italic.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Makes the text underlined.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    /// Makes the text strikethrough.
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// Sets the text wrapping mode.
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// Sets the text alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }
}

impl TextStyleBuilder {
    /// Sets the text color.
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }

    /// Sets the background color behind the text.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Makes the text bold.
    pub fn bold(mut self) -> Self {
        self.style.bold = Some(true);
        self
    }

    /// Makes the text italic.
    pub fn italic(mut self) -> Self {
        self.style.italic = Some(true);
        self
    }

    /// Makes the text underlined.
    pub fn underline(mut self) -> Self {
        self.style.underline = Some(true);
        self
    }

    /// Makes the text strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough = Some(true);
        self
    }

    /// Convenience method for making text bold (alias for bold()).
    pub fn strong(self) -> Self {
        self.bold()
    }

    /// Convenience method for making text italic (alias for italic()).
    pub fn emphasis(self) -> Self {
        self.italic()
    }

    /// Sets the text wrapping mode.
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.style.wrap = Some(wrap);
        self
    }

    /// Sets the text alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.style.align = Some(align);
        self
    }

    /// Builds the final TextStyle instance.
    pub fn build(self) -> TextStyle {
        self.style
    }
}

impl Spacing {
    /// Creates spacing with the same value on all sides.
    ///
    /// ```text
    /// Spacing::all(2) creates:
    /// ┌─────────────┐
    /// │   2 cells   │
    /// │   ┌─────┐   │
    /// │   │     │   │
    /// │   └─────┘   │
    /// │   2 cells   │
    /// └─────────────┘
    /// ```
    pub fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates spacing with values only on top and bottom.
    ///
    /// ```text
    /// Spacing::vertical(2) creates:
    /// ┌──────────┐
    /// │  2 cells │
    /// ├──────────┤
    /// │ Content  │
    /// ├──────────┤
    /// │  2 cells │
    /// └──────────┘
    /// ```
    pub fn vertical(value: u16) -> Self {
        Self {
            top: value,
            right: 0,
            bottom: value,
            left: 0,
        }
    }

    /// Creates spacing with values only on left and right.
    ///
    /// ```text
    /// Spacing::horizontal(2) creates:
    /// ┌────┬───────┬────┐
    /// │  2 │Content│ 2  │
    /// └────┴───────┴────┘
    /// ```
    pub fn horizontal(value: u16) -> Self {
        Self {
            top: 0,
            right: value,
            bottom: 0,
            left: value,
        }
    }
}

impl StyleBuilder {
    /// Sets the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Sets the layout direction for child elements.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.style.direction = Some(direction);
        self
    }

    /// Sets the inner padding around content.
    pub fn padding(mut self, padding: Spacing) -> Self {
        self.style.padding = Some(padding);
        self
    }

    /// Sets the overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.style.overflow = Some(overflow);
        self
    }

    /// Sets the width dimension.
    pub fn width(mut self, width: Dimension) -> Self {
        self.style.width = Some(width);
        self
    }

    /// Sets the height dimension.
    pub fn height(mut self, height: Dimension) -> Self {
        self.style.height = Some(height);
        self
    }

    /// Enables border with the specified color.
    pub fn border(mut self, color: Color) -> Self {
        self.style.border = Some(Border {
            enabled: true,
            style: BorderStyle::Single,
            color,
            edges: BorderEdges::ALL,
        });
        self
    }

    /// Sets the positioning mode.
    pub fn position(mut self, position: Position) -> Self {
        self.style.position = Some(position);
        self
    }

    /// Sets the z-index for layering.
    pub fn z_index(mut self, z_index: i32) -> Self {
        self.style.z_index = Some(z_index);
        self
    }

    /// Sets the top position offset.
    pub fn top(mut self, top: i16) -> Self {
        self.style.top = Some(top);
        self
    }

    /// Sets the right position offset.
    pub fn right(mut self, right: i16) -> Self {
        self.style.right = Some(right);
        self
    }

    /// Sets the bottom position offset.
    pub fn bottom(mut self, bottom: i16) -> Self {
        self.style.bottom = Some(bottom);
        self
    }

    /// Sets the left position offset.
    pub fn left(mut self, left: i16) -> Self {
        self.style.left = Some(left);
        self
    }

    /// Sets the wrapping mode for child elements.
    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.style.wrap = Some(wrap);
        self
    }

    /// Sets the gap between wrapped rows/columns.
    pub fn gap(mut self, gap: u16) -> Self {
        self.style.gap = Some(gap);
        self
    }

    /// Builds the final Style instance.
    pub fn build(self) -> Style {
        self.style
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

/// Default style with all properties unset.
impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            direction: None,
            padding: None,
            overflow: None,
            width: None,
            height: None,
            border: None,
            position: None,
            z_index: None,
            top: None,
            right: None,
            bottom: None,
            left: None,
            wrap: None,
            gap: None,
            margin: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            border_color: None,
            x: None,
            y: None,
            show_scrollbar: None,
            justify_content: None,
            align_items: None,
            align_self: None,
        }
    }
}

/// Default text style with all properties unset.
impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: None,
            background: None,
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
            wrap: None,
            align: None,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_parsing() {
        // Test 1-digit hex (grayscale)
        assert_eq!(Color::from_hex("0").unwrap(), Color::Rgb(0, 0, 0));
        assert_eq!(Color::from_hex("#F").unwrap(), Color::Rgb(255, 255, 255));
        assert_eq!(Color::from_hex("8").unwrap(), Color::Rgb(136, 136, 136));

        // Test 3-digit hex
        assert_eq!(Color::from_hex("#F00").unwrap(), Color::Rgb(255, 0, 0));
        assert_eq!(Color::from_hex("0F0").unwrap(), Color::Rgb(0, 255, 0));
        assert_eq!(Color::from_hex("#00F").unwrap(), Color::Rgb(0, 0, 255));
        assert_eq!(Color::from_hex("F53").unwrap(), Color::Rgb(255, 85, 51));

        // Test 6-digit hex
        assert_eq!(Color::from_hex("#FF5733").unwrap(), Color::Rgb(255, 87, 51));
        assert_eq!(Color::from_hex("2ECC71").unwrap(), Color::Rgb(46, 204, 113));
        assert_eq!(
            Color::from_hex("#3498DB").unwrap(),
            Color::Rgb(52, 152, 219)
        );

        // Test case insensitivity
        assert_eq!(Color::from_hex("#abc").unwrap(), Color::Rgb(170, 187, 204));
        assert_eq!(Color::from_hex("#ABC").unwrap(), Color::Rgb(170, 187, 204));
        assert_eq!(
            Color::from_hex("aaBBcc").unwrap(),
            Color::Rgb(170, 187, 204)
        );

        // Test invalid formats
        assert!(Color::from_hex("").is_err());
        assert!(Color::from_hex("12").is_err());
        assert!(Color::from_hex("1234").is_err());
        assert!(Color::from_hex("12345").is_err());
        assert!(Color::from_hex("1234567").is_err());
        assert!(Color::from_hex("GGG").is_err());
        assert!(Color::from_hex("#GGGGGG").is_err());
    }

    #[test]
    fn test_hex_panic_method() {
        // This should work
        let color = Color::hex("#FF5733");
        assert_eq!(color, Color::Rgb(255, 87, 51));
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn test_hex_panic_on_invalid() {
        Color::hex("invalid");
    }

    #[test]
    fn test_rgb_constructor() {
        assert_eq!(Color::rgb(255, 165, 0), Color::Rgb(255, 165, 0));
        assert_eq!(Color::rgb(0, 0, 0), Color::Rgb(0, 0, 0));
        assert_eq!(Color::rgb(255, 255, 255), Color::Rgb(255, 255, 255));
    }
}
