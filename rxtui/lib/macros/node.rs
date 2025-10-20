//! Implementation of the node! macro
//!
//! This file contains the main node! macro and its internal parsing/building helpers.
//! The macro provides a declarative syntax for building TUI components.

/// Main macro for building TUI components with a declarative syntax
///
/// # Basic Syntax
///
/// - **Divs**: `div(props) [children]` - Properties in parentheses, children in brackets
/// - **Text**: `text("content", props)` - Content first, then properties
/// - **Input**: `input(props)` - Text input field with properties
/// - **Spacers**: `spacer(size)` - Simple spacing elements
/// - **Components**: `node(instance)` - Embed other components
///
/// # Examples
///
/// ## Basic Div with Text
/// ```ignore
/// use rxtui::prelude::*;
///
/// node! {
///     div(bg: black, pad: 2) [
///         text("Hello World", color: white, bold),
///         text("Welcome to Radical TUI", color: cyan)
///     ]
/// }
/// ```
///
/// ## Layout with HStack and VStack
/// ```ignore
/// node! {
///     div(bg: "#1a1a1a", pad: 2) [
///         // Vertical layout by default
///         text("Header", color: yellow, bold),
///         spacer(1),
///
///         // Horizontal layout
///         hstack(gap: 2) [
///             div(bg: blue, w: 20, h: 10) [
///                 text("Left", color: white)
///             ],
///             div(bg: green, w: 20, h: 10) [
///                 text("Right", color: white)
///             ]
///         ],
///
///         spacer(1),
///
///         // Explicit vertical layout
///         vstack [
///             text("Line 1"),
///             text("Line 2"),
///             text("Line 3")
///         ]
///     ]
/// }
/// ```
///
/// ## Styling Properties
/// ```ignore
/// node! {
///     div(
///         // Colors
///         bg: black,              // Named color
///         border_color: "#FF5733", // Hex color (or use legacy 'border:')
///
///         // Border configuration
///         border_style: rounded,   // Style only (single, double, thick, rounded, dashed)
///         border_color: white,     // Border color
///         border_edges: top | bottom,  // Which edges (can use | for multiple)
///         border_full: (BorderStyle::Double, yellow, BorderEdges::ALL),  // Full config (legacy)
///
///         // Dimensions
///         w: 50,                  // Fixed width
///         h: 20,                  // Fixed height
///         w_frac: 0.5,            // Width as fraction (50%)
///         h_frac: 0.8,            // Height as fraction (80%)
///         w_auto,                // Automatic width
///         h_content,             // Height based on content
///
///         // Spacing
///         pad: 2,                // Padding on all sides
///         pad_h: 1,              // Horizontal padding only
///         pad_v: 1,              // Vertical padding only
///         padding: (Spacing::horizontal(2)), // Direct Spacing expression
///         gap: 1,                // Gap between children
///
///         // Layout
///         dir: horizontal,       // Direction (or use 'h')
///         wrap: wrap,           // Wrap mode (lowercase)
///         overflow: hidden,     // Overflow behavior (lowercase)
///
///         // Positioning
///         pos: absolute,        // Position type (lowercase)
///         absolute,             // Shorthand for absolute positioning
///         top: 5,              // Offset from top
///         right: 10,           // Offset from right
///         bottom: 5,           // Offset from bottom
///         left: 10,            // Offset from left
///         z: 100,              // Z-index for layering
///
///         // Interaction
///         focusable,           // Can receive focus
///         focus_style: (Style::new().border(yellow))  // Style when focused
///     ) [
///         text("Styled Div")
///     ]
/// }
/// ```
///
/// ## Text Styling
/// ```ignore
/// node! {
///     div [
///         // Basic text styles
///         text("Bold text", bold),
///         text("Italic text", italic),
///         text("Underlined", underline),
///         text("Strikethrough", strikethrough),
///
///         // Colors
///         text("Red text", color: red),
///         text("Bright blue", color: bright_blue),
///         text("Custom hex", color: "#00FF00"),
///         text("With background", color: white, bg: blue),
///
///         // Multiple styles
///         text("Important!", color: yellow, bg: red, bold, underline),
///
///         // Text wrapping
///         text("Long text that wraps", wrap: word),
///
///         // Text alignment
///         text("Centered text", align: center),
///         text("Right aligned", align: right),
///         text("Left aligned", align: left)
///     ]
/// }
/// ```
///
/// ## Text Input Fields
/// ```ignore
/// node! {
///     div [
///         // Basic input with placeholder
///         input(placeholder: "Enter your name...", focusable),
///
///         // Styled input with custom colors
///         input(
///             placeholder: "Type here...",
///             cursor_color: yellow,
///             content_color: green,
///             border: cyan,
///             w: 40,
///             h: 3
///         ),
///
///         // Wrapped input for long text
///         input(
///             placeholder: "Long message...",
///             wrap: word,
///             w: 50,
///             h: 5,
///             border: magenta
///         )
///     ]
/// }
/// ```
///
/// ## Rich Text (Inline Styled Text)
/// ```ignore
/// node! {
///     div [
///         // Basic richtext with multiple styled segments
///         richtext [
///             text("Normal text "),
///             text("Bold text", bold),
///             text(" and "),
///             text("colored text", color: red)
///         ],
///
///         // Shorthand styled segments
///         richtext [
///             text("Status: "),
///             colored("Success", green),
///             text(" - "),
///             bold("Important"),
///             text(" - "),
///             italic("Note")
///         ],
///
///         // Code syntax highlighting example
///         richtext(wrap: word_break) [
///             text("fn ", color: magenta),
///             text("calculate", color: yellow),
///             text("("),
///             text("n", color: cyan),
///             text(": "),
///             text("u32", color: blue),
///             text(") -> "),
///             text("u32", color: blue),
///             text(" { ... }")
///         ],
///
///         // Complex inline styling
///         richtext(bg: black) [
///             text("Error: ", color: red, bold),
///             text("File "),
///             text("config.rs", color: cyan, underline),
///             text(" not found at line "),
///             text("42", color: yellow)
///         ],
///
///         // Apply styles to all spans
///         richtext(color: white, bg: dark_gray) [
///             text("All spans "),
///             colored("inherit", green),  // green overrides white
///             text(" the base style")      // uses white from richtext
///         ],
///
///         // With text wrapping
///         richtext(wrap: word) [
///             text("This is a long line of rich text that will "),
///             bold("wrap properly"),
///             text(" while preserving all the "),
///             colored("inline styles", blue),
///             text(" across line boundaries.")
///         ],
///
///         // With alignment
///         richtext(align: center) [
///             text("This "),
///             bold("rich text"),
///             text(" is centered")
///         ],
///         richtext(align: right) [
///             text("Right aligned "),
///             colored("rich text", cyan)
///         ]
///     ]
/// }
/// ```
///
/// ## Event Handlers
/// ```ignore
/// node! {
///     div(bg: black, @char_global('q'): ctx.handler(Msg::Quit), @key_global(esc): ctx.handler(Msg::Exit)) [
///         // Click handler
///         container(bg: blue, focusable, @click: ctx.handler(Msg::Clicked)) [
///             text("Click me", color: white)
///         ],
///
///         // Keyboard handlers
///         container(focusable, @char('a'): ctx.handler(Msg::KeyA), @key(enter): ctx.handler(Msg::Enter), @key(backspace): ctx.handler(Msg::Back)) [
///             text("Press keys here")
///         ],
///
///         // Focus handlers
///         container(focusable, @focus: ctx.handler(Msg::GotFocus), @blur: ctx.handler(Msg::LostFocus)) [
///             text("Focus me")
///         ]
///     ]
/// }
/// ```
///
/// ## Dynamic Content
/// ```ignore
/// node! {
///     div(bg: black) [
///         // Conditional text
///         text(
///             if state.logged_in { "Welcome!" } else { "Please login" },
///             color: (if state.logged_in { green } else { red })
///         ),
///
///         // Formatted text
///         text(format!("Count: {}", state.count), bold),
///
///         // Conditional styling based on state
///         container(
///             bg: (if state.error { red } else { green }),
///             border: (if state.selected { white } else { black })
///         ) [
///             text(state.message, color: white)
///         ],
///
///         // Dynamic dimensions
///         container(
///             w: (window_width / 2),
///             h: (window_height - 10)
///         ) [
///             text("Responsive size")
///         ]
///     ]
/// }
/// ```
///
/// ## Optional Properties
///
/// Use the `!` suffix after a parenthesized expression to conditionally apply properties.
/// These properties are only applied when the value is `Some`:
///
/// ```ignore
/// node! {
///     div(
///         // Optional properties - only applied if Some
///         bg: (state.optional_background)!,      // Option<Color>
///         border: (state.optional_border_color)!, // Option<Color>
///         pad: (calculate_optional_padding())!,   // Option<u16>
///         w: (state.dynamic_width)!,              // Option<u16>
///
///         // Regular properties - always applied
///         h: 20,
///         focusable
///     ) [
///         text("Dynamic Styling",
///             color: (state.text_color)!,         // Option<Color>
///             bg: (state.text_background)!,       // Option<Color>
///             bold                                  // Always bold
///         ),
///
///         // Conditional border example
///         div(
///             border: (if state.selected { Some(Color::Yellow) } else { None })!,
///             pad: 2
///         ) [
///             text("Select me!", color: (state.highlight_color)!)
///         ]
///     ]
/// }
/// ```
///
/// ## Nested Components
/// ```ignore
/// node! {
///     div(bg: black, dir: vertical) [
///         // Include other components
///         node(Header::new("My App")),
///
///         container(h_frac: 0.8) [
///             node(MainContent::new(state.data))
///         ],
///
///         node(StatusBar::new(state.status))
///     ]
/// }
/// ```
///
/// # Color Values
///
/// Colors can be specified in multiple formats:
///
/// - **Named colors**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
/// - **Bright variants**: `bright_black`, `bright_red`, `bright_green`, `bright_yellow`,
///   `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`
/// - **Hex strings**: `"#RGB"`, `"#RRGGBB"` (e.g., `"#F00"`, `"#FF0000"`)
/// - **Expressions**: Any expression that evaluates to `Color` (e.g., `Color::rgb(255, 0, 0)`)
/// - **Conditional**: `(if condition { color1 } else { color2 })`
///
/// # Direction Values
///
/// - `vertical` or `v` - Stack children vertically
/// - `horizontal` or `h` - Arrange children horizontally
///
/// # Property Shortcuts
///
/// Many properties have convenient short names:
///
/// | Short | Full Property | Description |
/// |-------|--------------|-------------|
/// | `bg` | `background` | Background color |
/// | `dir` | `direction` | Layout direction |
/// | `pad` | `padding` | Inner spacing (all sides) |
/// | `pad_h` | `padding` | Horizontal padding only |
/// | `pad_v` | `padding` | Vertical padding only |
/// | `w` | `width` | Fixed width |
/// | `h` | `height` | Fixed height |
/// | `w_frac` | `width_fraction` | Width as fraction (0.0-1.0) |
/// | `h_frac` | `height_fraction` | Height as fraction (0.0-1.0) |
///
/// # Event Handler Reference
///
/// All event handlers use the `@` prefix:
///
/// | Handler | Description | Example |
/// |---------|-------------|---------|
/// | `@click` | Mouse click | `@click: handler` |
/// | `@char(c)` | Character key press | `@char('a'): handler` |
/// | `@key(k)` | Special key press | `@key(enter): handler` |
/// | `@key(Char(c))` | Character in key enum | `@key(Char('-')): handler` |
/// | `@key(mod + key)` | Key press with modifiers | `@key(ctrl + 'c'): handler` |
/// | `@char_global(c)` | Global character key | `@char_global('q'): handler` |
/// | `@key_global(k)` | Global special key | `@key_global(esc): handler` |
/// | `@key_global(mod + key)` | Global key with modifiers | `@key_global(ctrl + enter): handler` |
/// | `@focus` | Gained focus | `@focus: handler` |
/// | `@blur` | Lost focus | `@blur: handler` |
/// | `@any_char` | Any character typed | `@any_char: \|c\| handler(c)` |
///
/// # Tips
///
/// 1. **Div is the default root** - The macro expects a div as the root element
/// 2. **Text content comes first** - For readability, text content precedes styling properties
/// 3. **Events go in properties** - Event handlers are placed in the property parentheses
/// 4. **Colors without prefix** - No need for `Color::` prefix on named colors
/// 5. **Expressions need parens** - Complex expressions should be wrapped in parentheses
#[macro_export]
macro_rules! node {
    // Parse the root element
    ($($tt:tt)*) => {{
        $crate::tui_parse_element!($($tt)*)
    }};
}

/// Parse a single element (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_parse_element {
    // Div with properties and children
    (div($($props:tt)*) [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [$($props)*],
            children: [$($children)*]
        )
    }};

    // Div with no properties
    (div [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [],
            children: [$($children)*]
        )
    }};

    // Text with content and properties
    (text($content:expr, $($props:tt)*)) => {{
        $crate::tui_build_text!($content, $($props)*)
    }};

    // Text with just content
    (text($content:expr)) => {{
        $crate::Text::new($content).into()
    }};

    // RichText with properties and spans
    (richtext($($props:tt)*) [$($spans:tt)*]) => {{
        $crate::tui_build_richtext!(
            props: [$($props)*],
            spans: [$($spans)*]
        )
    }};

    // RichText with just spans
    (richtext [$($spans:tt)*]) => {{
        $crate::tui_build_richtext!(
            props: [],
            spans: [$($spans)*]
        )
    }};

    // Spacer
    (spacer($size:expr)) => {{
        $crate::Div::<$crate::Node>::new().height($size).into()
    }};

    // Component
    (node($comp:expr)) => {{
        $crate::Node::Component(std::sync::Arc::new($comp))
    }};

    // Input with properties
    (input($($props:tt)*)) => {{
        $crate::tui_build_input!($($props)*)
    }};

    // Input without properties
    (input) => {{
        $crate::Node::Component(std::sync::Arc::new($crate::TextInput::new()))
    }};

    // VStack with properties
    (vstack($($props:tt)*) [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [dir: vertical, $($props)*],
            children: [$($children)*]
        )
    }};

    // VStack without properties
    (vstack [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [dir: vertical],
            children: [$($children)*]
        )
    }};

    // HStack with properties
    (hstack($($props:tt)*) [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [dir: horizontal, $($props)*],
            children: [$($children)*]
        )
    }};

    // HStack without properties
    (hstack [$($children:tt)*]) => {{
        $crate::tui_build_div!(
            props: [dir: horizontal],
            children: [$($children)*]
        )
    }};
}

/// Build a div (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_build_div {
    // With properties
    (props: [$($props:tt)+], children: [$($children:tt)*]) => {{
        #[allow(unused_mut)]
        let mut __div = $crate::Div::<$crate::Node>::new();

        // Apply properties
        __div = $crate::tui_apply_props!(__div, $($props)+);

        // Parse children and collect nodes
        let mut __children_vec = Vec::new();
        $crate::tui_parse_children!(__children_vec, __div, $($children)*)

        // The macro returns the final div
    }};

    // Without properties
    (props: [], children: [$($children:tt)*]) => {{
        #[allow(unused_mut)]
        let mut __div = $crate::Div::<$crate::Node>::new();

        // Parse children and collect nodes
        let mut __children_vec = Vec::new();
        $crate::tui_parse_children!(__children_vec, __div, $($children)*)

        // The macro returns the final div
    }};
}

/// Parse children and handle events (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_parse_children {
    // Base case - done parsing (no tokens left)
    ($children:ident, $container:expr) => {{
        if !$children.is_empty() {
            $container.children($children).into()
        } else {
            $container.into()
        }
    }};

    // Base case - done parsing (trailing comma)
    ($children:ident, $container:expr,) => {{
        if !$children.is_empty() {
            $container.children($children).into()
        } else {
            $container.into()
        }
    }};

    // Child: div with props (and more children)
    ($children:ident, $container:expr, div($($props:tt)*) [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(div($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: div with props (last child)
    ($children:ident, $container:expr, div($($props:tt)*) [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(div($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: div without props (and more children)
    ($children:ident, $container:expr, div [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(div [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: div without props (last child)
    ($children:ident, $container:expr, div [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(div [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: text with props (and more children)
    ($children:ident, $container:expr, text($content:expr, $($props:tt)*), $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(text($content, $($props)*));
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: text with props (last child)
    ($children:ident, $container:expr, text($content:expr, $($props:tt)*)) => {{
        let child = $crate::tui_parse_element!(text($content, $($props)*));
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: text without props (and more children)
    ($children:ident, $container:expr, text($content:expr), $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(text($content));
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: text without props (last child)
    ($children:ident, $container:expr, text($content:expr)) => {{
        let child = $crate::tui_parse_element!(text($content));
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: spacer (and more children)
    ($children:ident, $container:expr, spacer($size:expr), $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(spacer($size));
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: spacer (last child)
    ($children:ident, $container:expr, spacer($size:expr)) => {{
        let child = $crate::tui_parse_element!(spacer($size));
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: component (and more children)
    ($children:ident, $container:expr, node($comp:expr), $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(node($comp));
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: component (last child)
    ($children:ident, $container:expr, node($comp:expr)) => {{
        let child = $crate::tui_parse_element!(node($comp));
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: expression in parentheses (and more children)
    ($children:ident, $container:expr, ($expr:expr), $($rest:tt)*) => {{
        let child = $expr;
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: expression in parentheses (last child)
    ($children:ident, $container:expr, ($expr:expr)) => {{
        let child = $expr;
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: spread expression with ... (and more children)
    ($children:ident, $container:expr, ...($expr:expr), $($rest:tt)*) => {{
        let items: Vec<$crate::Node> = $expr;
        $children.extend(items);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: spread expression with ... (last child)
    ($children:ident, $container:expr, ...($expr:expr)) => {{
        let items: Vec<$crate::Node> = $expr;
        $children.extend(items);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: input with props (and more children)
    ($children:ident, $container:expr, input($($props:tt)*), $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(input($($props)*));
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: input with props (last child)
    ($children:ident, $container:expr, input($($props:tt)*)) => {{
        let child = $crate::tui_parse_element!(input($($props)*));
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: input without props (and more children)
    ($children:ident, $container:expr, input, $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(input);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: input without props (last child)
    ($children:ident, $container:expr, input) => {{
        let child = $crate::tui_parse_element!(input);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: vstack with props (and more children)
    ($children:ident, $container:expr, vstack($($props:tt)*) [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(vstack($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: vstack with props (last child)
    ($children:ident, $container:expr, vstack($($props:tt)*) [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(vstack($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: vstack without props (and more children)
    ($children:ident, $container:expr, vstack [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(vstack [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: vstack without props (last child)
    ($children:ident, $container:expr, vstack [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(vstack [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: hstack with props (and more children)
    ($children:ident, $container:expr, hstack($($props:tt)*) [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(hstack($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: hstack with props (last child)
    ($children:ident, $container:expr, hstack($($props:tt)*) [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(hstack($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: hstack without props (and more children)
    ($children:ident, $container:expr, hstack [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(hstack [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: hstack without props (last child)
    ($children:ident, $container:expr, hstack [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(hstack [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: richtext with props (and more children)
    ($children:ident, $container:expr, richtext($($props:tt)*) [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(richtext($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: richtext with props (last child)
    ($children:ident, $container:expr, richtext($($props:tt)*) [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(richtext($($props)*) [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};

    // Child: richtext without props (and more children)
    ($children:ident, $container:expr, richtext [$($inner:tt)*], $($rest:tt)*) => {{
        let child = $crate::tui_parse_element!(richtext [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container, $($rest)*)
    }};

    // Child: richtext without props (last child)
    ($children:ident, $container:expr, richtext [$($inner:tt)*]) => {{
        let child = $crate::tui_parse_element!(richtext [$($inner)*]);
        $children.push(child);
        $crate::tui_parse_children!($children, $container)
    }};
}

/// Apply properties to div (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_apply_props {
    // Base case - return the container
    ($container:expr,) => { $container };
    ($container:expr) => { $container };

    // Background
    ($container:expr, bg: $color:tt, $($rest:tt)*) => {{
        let c = $container.background($crate::color_value!($color));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, bg: $color:tt) => {{
        $container.background($crate::color_value!($color))
    }};

    // Background with expression
    ($container:expr, bg: ($color:expr), $($rest:tt)*) => {{
        let c = $container.background($color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};

    // Background - optional with ! suffix on expression
    ($container:expr, bg: ($color:expr)!, $($rest:tt)*) => {{
        let c = if let Some(color_val) = $color {
            $container.background(color_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, bg: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $container.background(color_val)
        } else {
            $container
        }
    }};

    // Direction
    ($container:expr, dir: $dir:tt, $($rest:tt)*) => {{
        let c = $container.direction($crate::direction_value!($dir));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, dir: $dir:tt) => {{
        $container.direction($crate::direction_value!($dir))
    }};

    // Padding (single value - all sides)
    ($container:expr, pad: $pad:expr, $($rest:tt)*) => {{
        let c = $container.padding($crate::Spacing::all($pad));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, pad: $pad:expr) => {{
        $container.padding($crate::Spacing::all($pad))
    }};

    // Padding - optional with ! suffix on expression
    ($container:expr, pad: ($pad:expr)!, $($rest:tt)*) => {{
        let c = if let Some(pad_val) = $pad {
            $container.padding($crate::Spacing::all(pad_val))
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, pad: ($pad:expr)!) => {{
        if let Some(pad_val) = $pad {
            $container.padding($crate::Spacing::all(pad_val))
        } else {
            $container
        }
    }};

    // Horizontal padding only
    ($container:expr, pad_h: $pad:expr, $($rest:tt)*) => {{
        let c = $container.padding($crate::Spacing::horizontal($pad));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, pad_h: $pad:expr) => {{
        $container.padding($crate::Spacing::horizontal($pad))
    }};

    // Vertical padding only
    ($container:expr, pad_v: $pad:expr, $($rest:tt)*) => {{
        let c = $container.padding($crate::Spacing::vertical($pad));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, pad_v: $pad:expr) => {{
        $container.padding($crate::Spacing::vertical($pad))
    }};

    // Direct padding expression
    ($container:expr, padding: ($padding:expr), $($rest:tt)*) => {{
        let c = $container.padding($padding);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, padding: ($padding:expr)) => {{
        $container.padding($padding)
    }};

    // Width
    ($container:expr, w: $width:expr, $($rest:tt)*) => {{
        let c = $container.width($width);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, w: $width:expr) => {{
        $container.width($width)
    }};

    // Width - optional with ! suffix on expression
    ($container:expr, w: ($width:expr)!, $($rest:tt)*) => {{
        let c = if let Some(width_val) = $width {
            $container.width(width_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, w: ($width:expr)!) => {{
        if let Some(width_val) = $width {
            $container.width(width_val)
        } else {
            $container
        }
    }};

    // Width fraction
    ($container:expr, w_frac: $frac:expr, $($rest:tt)*) => {{
        let c = $container.width_fraction($frac);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, w_frac: $frac:expr) => {{
        $container.width_fraction($frac)
    }};

    // Width auto
    ($container:expr, w_auto, $($rest:tt)*) => {{
        let c = $container.width_auto();
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, w_auto) => {{
        $container.width_auto()
    }};

    // Width content
    ($container:expr, w_content, $($rest:tt)*) => {{
        let c = $container.width_content();
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, w_content) => {{
        $container.width_content()
    }};

    // Height
    ($container:expr, h: $height:expr, $($rest:tt)*) => {{
        let c = $container.height($height);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, h: $height:expr) => {{
        $container.height($height)
    }};

    // Height - optional with ! suffix on expression
    ($container:expr, h: ($height:expr)!, $($rest:tt)*) => {{
        let c = if let Some(height_val) = $height {
            $container.height(height_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, h: ($height:expr)!) => {{
        if let Some(height_val) = $height {
            $container.height(height_val)
        } else {
            $container
        }
    }};

    // Height fraction
    ($container:expr, h_frac: $frac:expr, $($rest:tt)*) => {{
        let c = $container.height_fraction($frac);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, h_frac: $frac:expr) => {{
        $container.height_fraction($frac)
    }};

    // Height auto
    ($container:expr, h_auto, $($rest:tt)*) => {{
        let c = $container.height_auto();
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, h_auto) => {{
        $container.height_auto()
    }};

    // Height content
    ($container:expr, h_content, $($rest:tt)*) => {{
        let c = $container.height_content();
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, h_content) => {{
        $container.height_content()
    }};

    // Gap
    ($container:expr, gap: $gap:expr, $($rest:tt)*) => {{
        let c = $container.gap($gap);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, gap: $gap:expr) => {{
        $container.gap($gap)
    }};

    // Border color (renamed from border for clarity)
    ($container:expr, border_color: $color:tt, $($rest:tt)*) => {{
        let c = $container.border_color($crate::color_value!($color));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_color: $color:tt) => {{
        $container.border_color($crate::color_value!($color))
    }};

    // Border color with expression
    ($container:expr, border_color: ($color:expr), $($rest:tt)*) => {{
        let c = $container.border_color($color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};

    // Border color - optional with ! suffix on expression
    ($container:expr, border_color: ($color:expr)!, $($rest:tt)*) => {{
        let c = if let Some(color_val) = $color {
            $container.border_color(color_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_color: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $container.border_color(color_val)
        } else {
            $container
        }
    }};

    // Legacy border support (maps to border_color)
    ($container:expr, border: none, $($rest:tt)*) => {{
        let c = $container.border_with($crate::style::Border::none());
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border: none) => {{
        $container.border_with($crate::style::Border::none())
    }};
    ($container:expr, border: $color:tt, $($rest:tt)*) => {{
        let c = $container.border_color($crate::color_value!($color));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border: $color:tt) => {{
        $container.border_color($crate::color_value!($color))
    }};
    ($container:expr, border: ($color:expr), $($rest:tt)*) => {{
        let c = $container.border_color($color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border: ($color:expr)!, $($rest:tt)*) => {{
        let c = if let Some(color_val) = $color {
            $container.border_color(color_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $container.border_color(color_val)
        } else {
            $container
        }
    }};

    // Border style (now only takes BorderStyle, not color)
    ($container:expr, border_style: single, $($rest:tt)*) => {{
        let c = $container.border_style($crate::BorderStyle::Single);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: double, $($rest:tt)*) => {{
        let c = $container.border_style($crate::BorderStyle::Double);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: thick, $($rest:tt)*) => {{
        let c = $container.border_style($crate::BorderStyle::Thick);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: rounded, $($rest:tt)*) => {{
        let c = $container.border_style($crate::BorderStyle::Rounded);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: dashed, $($rest:tt)*) => {{
        let c = $container.border_style($crate::BorderStyle::Dashed);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: ($style:expr), $($rest:tt)*) => {{
        let c = $container.border_style($style);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: ($style:expr)!, $($rest:tt)*) => {{
        let c = if let Some(style_val) = $style {
            $container.border_style(style_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: single) => {{
        $container.border_style($crate::BorderStyle::Single)
    }};
    ($container:expr, border_style: double) => {{
        $container.border_style($crate::BorderStyle::Double)
    }};
    ($container:expr, border_style: thick) => {{
        $container.border_style($crate::BorderStyle::Thick)
    }};
    ($container:expr, border_style: rounded) => {{
        $container.border_style($crate::BorderStyle::Rounded)
    }};
    ($container:expr, border_style: dashed) => {{
        $container.border_style($crate::BorderStyle::Dashed)
    }};
    ($container:expr, border_style: ($style:expr)) => {{
        $container.border_style($style)
    }};
    ($container:expr, border_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $container.border_style(style_val)
        } else {
            $container
        }
    }};

    // Legacy border_style with color (still supported for compatibility)
    ($container:expr, border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let c = $container.border_style_with_color($style, $color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_style: ($style:expr, $color:expr)) => {{
        $container.border_style_with_color($style, $color)
    }};

    // Border edges - simple syntax support
    ($container:expr, border_edges: top, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::TOP);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: bottom, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::BOTTOM);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: left, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::LEFT);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: right, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::RIGHT);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: horizontal, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::HORIZONTAL);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: vertical, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::VERTICAL);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: all, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::ALL);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: corners, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::CORNERS);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: edges, $($rest:tt)*) => {{
        let c = $container.border_edges($crate::BorderEdges::EDGES);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    // Support for | operator in edges
    ($container:expr, border_edges: $edge1:ident | $($edge2:ident)|+, $($rest:tt)*) => {{
        let edges = $crate::tui_edge_value!($edge1) $(| $crate::tui_edge_value!($edge2))+;
        let c = $container.border_edges(edges);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    // Expression-based edges
    ($container:expr, border_edges: ($edges:expr), $($rest:tt)*) => {{
        let c = $container.border_edges($edges);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_edges: ($edges:expr)!, $($rest:tt)*) => {{
        let c = if let Some(edges_val) = $edges {
            $container.border_edges(edges_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    // Terminal rules (no rest)
    ($container:expr, border_edges: top) => {{
        $container.border_edges($crate::BorderEdges::TOP)
    }};
    ($container:expr, border_edges: bottom) => {{
        $container.border_edges($crate::BorderEdges::BOTTOM)
    }};
    ($container:expr, border_edges: left) => {{
        $container.border_edges($crate::BorderEdges::LEFT)
    }};
    ($container:expr, border_edges: right) => {{
        $container.border_edges($crate::BorderEdges::RIGHT)
    }};
    ($container:expr, border_edges: horizontal) => {{
        $container.border_edges($crate::BorderEdges::HORIZONTAL)
    }};
    ($container:expr, border_edges: vertical) => {{
        $container.border_edges($crate::BorderEdges::VERTICAL)
    }};
    ($container:expr, border_edges: all) => {{
        $container.border_edges($crate::BorderEdges::ALL)
    }};
    ($container:expr, border_edges: corners) => {{
        $container.border_edges($crate::BorderEdges::CORNERS)
    }};
    ($container:expr, border_edges: edges) => {{
        $container.border_edges($crate::BorderEdges::EDGES)
    }};
    ($container:expr, border_edges: $edge1:ident | $($edge2:ident)|+) => {{
        let edges = $crate::tui_edge_value!($edge1) $(| $crate::tui_edge_value!($edge2))+;
        $container.border_edges(edges)
    }};
    ($container:expr, border_edges: ($edges:expr)) => {{
        $container.border_edges($edges)
    }};
    ($container:expr, border_edges: ($edges:expr)!) => {{
        if let Some(edges_val) = $edges {
            $container.border_edges(edges_val)
        } else {
            $container
        }
    }};

    // Full border configuration
    ($container:expr, border_full: ($style:expr, $color:expr, $edges:expr), $($rest:tt)*) => {{
        let c = $container.border_full($style, $color, $edges);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, border_full: ($style:expr, $color:expr, $edges:expr)) => {{
        $container.border_full($style, $color, $edges)
    }};

    // Focusable with value
    ($container:expr, focusable: $val:expr, $($rest:tt)*) => {{
        let c = $container.focusable($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focusable: $val:expr) => {{
        $container.focusable($val)
    }};

    // Focusable shorthand
    ($container:expr, focusable, $($rest:tt)*) => {{
        let c = $container.focusable(true);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focusable) => {{
        $container.focusable(true)
    }};

    // Show scrollbar with value
    ($container:expr, show_scrollbar: $val:expr, $($rest:tt)*) => {{
        let c = $container.show_scrollbar($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, show_scrollbar: $val:expr) => {{
        $container.show_scrollbar($val)
    }};

    // Focus style
    ($container:expr, focus_style: ($style:expr), $($rest:tt)*) => {{
        let c = $container.focus_style($style);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_style: ($style:expr)) => {{
        $container.focus_style($style)
    }};

    // Focus style - optional with ! suffix on expression
    ($container:expr, focus_style: ($style:expr)!, $($rest:tt)*) => {{
        let c = if let Some(style_val) = $style {
            $container.focus_style(style_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $container.focus_style(style_val)
        } else {
            $container
        }
    }};

    // Focus border helpers
    ($container:expr, focus_border: none, $($rest:tt)*) => {{
        let c = $container.focus_border_with($crate::style::Border::none());
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_border: none) => {{
        $container.focus_border_with($crate::style::Border::none())
    }};
    ($container:expr, focus_border: $color:tt, $($rest:tt)*) => {{
        let c = $container.focus_border($crate::color_value!($color));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_border: $color:tt) => {{
        $container.focus_border($crate::color_value!($color))
    }};
    ($container:expr, focus_border: ($color:expr), $($rest:tt)*) => {{
        let c = $container.focus_border($color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_border: ($color:expr)) => {{
        $container.focus_border($color)
    }};
    ($container:expr, focus_border: ($color:expr)!, $($rest:tt)*) => {{
        let c = if let Some(color_val) = $color {
            $container.focus_border(color_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_border: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $container.focus_border(color_val)
        } else {
            $container
        }
    }};

    // Focus border style helpers
    ($container:expr, focus_border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let c = $container.focus_border_style($style, $color);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, focus_border_style: ($style:expr, $color:expr)) => {{
        $container.focus_border_style($style, $color)
    }};

    // Hover style
    ($container:expr, hover_style: ($style:expr), $($rest:tt)*) => {{
        let c = $container.hover_style($style);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, hover_style: ($style:expr)) => {{
        $container.hover_style($style)
    }};

    // Hover style - optional with ! suffix on expression
    ($container:expr, hover_style: ($style:expr)!, $($rest:tt)*) => {{
        let c = if let Some(style_val) = $style {
            $container.hover_style(style_val)
        } else {
            $container
        };
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, hover_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $container.hover_style(style_val)
        } else {
            $container
        }
    }};

    // Z-index
    ($container:expr, z: $index:expr, $($rest:tt)*) => {{
        let c = $container.z_index($index);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, z: $index:expr) => {{
        $container.z_index($index)
    }};

    // Position
    ($container:expr, pos: $pos:tt, $($rest:tt)*) => {{
        let c = $container.position($crate::position_value!($pos));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, pos: $pos:tt) => {{
        $container.position($crate::position_value!($pos))
    }};

    // Justify Content
    ($container:expr, justify: $justify:tt, $($rest:tt)*) => {{
        let c = $container.justify_content($crate::justify_content_value!($justify));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, justify: $justify:tt) => {{
        $container.justify_content($crate::justify_content_value!($justify))
    }};
    ($container:expr, justify_content: $justify:tt, $($rest:tt)*) => {{
        let c = $container.justify_content($crate::justify_content_value!($justify));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, justify_content: $justify:tt) => {{
        $container.justify_content($crate::justify_content_value!($justify))
    }};

    // Align Items
    ($container:expr, align: $align:tt, $($rest:tt)*) => {{
        let c = $container.align_items($crate::align_items_value!($align));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, align: $align:tt) => {{
        $container.align_items($crate::align_items_value!($align))
    }};
    ($container:expr, align_items: $align:tt, $($rest:tt)*) => {{
        let c = $container.align_items($crate::align_items_value!($align));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, align_items: $align:tt) => {{
        $container.align_items($crate::align_items_value!($align))
    }};

    // Align Self
    ($container:expr, align_self: $align:tt, $($rest:tt)*) => {{
        let c = $container.align_self($crate::align_self_value!($align));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, align_self: $align:tt) => {{
        $container.align_self($crate::align_self_value!($align))
    }};

    // Absolute positioning shorthand
    ($container:expr, absolute, $($rest:tt)*) => {{
        let c = $container.position($crate::Position::Absolute);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, absolute) => {{
        $container.position($crate::Position::Absolute)
    }};

    // Positioning offsets
    ($container:expr, top: $val:expr, $($rest:tt)*) => {{
        let c = $container.top($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, top: $val:expr) => {{
        $container.top($val)
    }};

    ($container:expr, right: $val:expr, $($rest:tt)*) => {{
        let c = $container.right($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, right: $val:expr) => {{
        $container.right($val)
    }};

    ($container:expr, bottom: $val:expr, $($rest:tt)*) => {{
        let c = $container.bottom($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, bottom: $val:expr) => {{
        $container.bottom($val)
    }};

    ($container:expr, left: $val:expr, $($rest:tt)*) => {{
        let c = $container.left($val);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, left: $val:expr) => {{
        $container.left($val)
    }};

    // Wrap mode
    ($container:expr, wrap: $mode:tt, $($rest:tt)*) => {{
        let c = $container.wrap($crate::wrap_value!($mode));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, wrap: $mode:tt) => {{
        $container.wrap($crate::wrap_value!($mode))
    }};

    // Overflow
    ($container:expr, overflow: $mode:tt, $($rest:tt)*) => {{
        let c = $container.overflow($crate::overflow_value!($mode));
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, overflow: $mode:tt) => {{
        $container.overflow($crate::overflow_value!($mode))
    }};

    // Event handlers

    // @click handler
    ($container:expr, @click: $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_click($handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @click: $handler:expr) => {{
        $container.on_click($handler)
    }};

    // @char handler
    ($container:expr, @char($ch:literal): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_char($ch, $handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @char($ch:literal): $handler:expr) => {{
        $container.on_char($ch, $handler)
    }};

    // @char_global handler
    ($container:expr, @char_global($ch:literal): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_char_global($ch, $handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @char_global($ch:literal): $handler:expr) => {{
        $container.on_char_global($ch, $handler)
    }};

    // @key with Char(...) handler
    ($container:expr, @key(Char($ch:literal)): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_key($crate::Key::Char($ch), $handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @key(Char($ch:literal)): $handler:expr) => {{
        $container.on_key($crate::Key::Char($ch), $handler)
    }};

    // @key with modifiers handler
    ($container:expr, @key($modifier:ident + $($mods:tt)+): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_key_with_modifiers(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        );
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @key($modifier:ident + $($mods:tt)+): $handler:expr) => {{
        $container.on_key_with_modifiers(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        )
    }};

    // @key handler
    ($container:expr, @key($key:tt): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_key($crate::key_value!($key), $handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @key($key:tt): $handler:expr) => {{
        $container.on_key($crate::key_value!($key), $handler)
    }};

    // @key_global handler
    ($container:expr, @key_global($key:tt): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_key_global($crate::key_value!($key), $handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @key_global($key:tt): $handler:expr) => {{
        $container.on_key_global($crate::key_value!($key), $handler)
    }};

    // @key_global with modifiers handler
    ($container:expr, @key_global($modifier:ident + $($mods:tt)+): $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_key_with_modifiers_global(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        );
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @key_global($modifier:ident + $($mods:tt)+): $handler:expr) => {{
        $container.on_key_with_modifiers_global(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        )
    }};

    // @focus handler
    ($container:expr, @focus: $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_focus($handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @focus: $handler:expr) => {{
        $container.on_focus($handler)
    }};

    // @blur handler
    ($container:expr, @blur: $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_blur($handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @blur: $handler:expr) => {{
        $container.on_blur($handler)
    }};

    // @any_char handler
    ($container:expr, @any_char: $handler:expr, $($rest:tt)*) => {{
        let c = $container.on_any_char($handler);
        $crate::tui_apply_props!(c, $($rest)*)
    }};
    ($container:expr, @any_char: $handler:expr) => {{
        $container.on_any_char($handler)
    }};
}

/// Build text with properties (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_build_text {
    ($content:expr,) => {{
        $crate::Text::new($content).into()
    }};

    ($content:expr, $($props:tt)*) => {{
        #[allow(unused_mut)]
        let __text = $crate::Text::new($content);
        // Always add trailing comma for consistent parsing
        let __text = $crate::tui_apply_text_props!(__text, $($props)* ,);
        __text.into()
    }};
}

/// Apply text properties (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_apply_text_props {
    // Base case - return the text
    ($text:expr,) => { $text };
    ($text:expr) => { $text };

    // Color
    ($text:expr, color: $color:tt, $($rest:tt)*) => {{
        let t = $text.color($crate::color_value!($color));
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, color: $color:tt) => {{
        $text.color($crate::color_value!($color))
    }};

    // Color with expression
    ($text:expr, color: ($color:expr), $($rest:tt)*) => {{
        let t = $text.color($color);
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};

    // Color - optional with ! suffix on expression
    ($text:expr, color: ($color:expr)!, $($rest:tt)*) => {{
        let t = if let Some(color_val) = $color {
            $text.color(color_val)
        } else {
            $text
        };
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, color: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $text.color(color_val)
        } else {
            $text
        }
    }};

    // Background
    ($text:expr, bg: $color:tt, $($rest:tt)*) => {{
        let t = $text.background($crate::color_value!($color));
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, bg: $color:tt) => {{
        $text.background($crate::color_value!($color))
    }};

    // Background - optional with ! suffix on expression
    ($text:expr, bg: ($color:expr)!, $($rest:tt)*) => {{
        let t = if let Some(bg_val) = $color {
            $text.background(bg_val)
        } else {
            $text
        };
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, bg: ($color:expr)!) => {{
        if let Some(bg_val) = $color {
            $text.background(bg_val)
        } else {
            $text
        }
    }};

    // Bold
    ($text:expr, bold, $($rest:tt)*) => {{
        let t = $text.bold();
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, bold) => {{
        $text.bold()
    }};

    // Italic
    ($text:expr, italic, $($rest:tt)*) => {{
        let t = $text.italic();
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, italic) => {{
        $text.italic()
    }};

    // Underline
    ($text:expr, underline, $($rest:tt)*) => {{
        let t = $text.underline();
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, underline) => {{
        $text.underline()
    }};

    // Strikethrough
    ($text:expr, strikethrough, $($rest:tt)*) => {{
        let t = $text.strikethrough();
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, strikethrough) => {{
        $text.strikethrough()
    }};

    // Wrap mode
    ($text:expr, wrap: $mode:tt, $($rest:tt)*) => {{
        let t = $text.wrap($crate::text_wrap_value!($mode));
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, wrap: $mode:tt) => {{
        $text.wrap($crate::text_wrap_value!($mode))
    }};

    // Alignment
    ($text:expr, align: $align:tt, $($rest:tt)*) => {{
        let t = $text.align($crate::text_align_value!($align));
        $crate::tui_apply_text_props!(t, $($rest)*)
    }};
    ($text:expr, align: $align:tt) => {{
        $text.align($crate::text_align_value!($align))
    }};
}

/// Build RichText elements (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_build_richtext {
    // With properties and spans
    (props: [$($props:tt)*], spans: [$($spans:tt)*]) => {{
        #[allow(unused_mut)]
        let mut __richtext = $crate::RichText::new();
        __richtext = $crate::tui_add_richtext_spans!(__richtext, $($spans)*);
        // Apply top-level properties if any
        let __richtext = $crate::tui_apply_richtext_props!(__richtext, $($props)*);
        __richtext.into()
    }};

    // No properties, just spans
    (props: [], spans: [$($spans:tt)*]) => {{
        #[allow(unused_mut)]
        let mut __richtext = $crate::RichText::new();
        __richtext = $crate::tui_add_richtext_spans!(__richtext, $($spans)*);
        __richtext.into()
    }};
}

/// Add spans to RichText (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_add_richtext_spans {
    // Base case - trailing comma
    ($rt:expr,) => { $rt };
    // Base case - no comma
    ($rt:expr) => { $rt };

    // Text span with properties
    ($rt:expr, text($content:expr, $($props:tt)*), $($rest:tt)*) => {{
        // Create a TextStyle from the properties
        let mut style = $crate::TextStyle::default();
        style = $crate::tui_apply_span_style!(style, $($props)*,);
        let rt = $rt.styled($content, style);
        $crate::tui_add_richtext_spans!(rt, $($rest)*)
    }};

    // Plain text span
    ($rt:expr, text($content:expr), $($rest:tt)*) => {{
        let rt = $rt.text($content);
        $crate::tui_add_richtext_spans!(rt, $($rest)*)
    }};

    // Last span cases (no trailing comma)
    ($rt:expr, text($content:expr, $($props:tt)*)) => {{
        // Create a TextStyle from the properties
        let mut style = $crate::TextStyle::default();
        style = $crate::tui_apply_span_style!(style, $($props)*,);
        $rt.styled($content, style)
    }};

    ($rt:expr, text($content:expr)) => {{
        $rt.text($content)
    }};
}

/// Apply style properties to TextStyle for RichText spans (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_apply_span_style {
    // Base case
    ($style:expr,) => { $style };
    ($style:expr) => { $style };

    // Color
    ($style:expr, color: $color:tt, $($rest:tt)*) => {{
        let mut s = $style;
        s.color = Some($crate::color_value!($color));
        $crate::tui_apply_span_style!(s, $($rest)*)
    }};
    ($style:expr, color: $color:tt) => {{
        let mut s = $style;
        s.color = Some($crate::color_value!($color));
        s
    }};

    // Background
    ($style:expr, bg: $color:tt, $($rest:tt)*) => {{
        let mut s = $style;
        s.background = Some($crate::color_value!($color));
        $crate::tui_apply_span_style!(s, $($rest)*)
    }};
    ($style:expr, bg: $color:tt) => {{
        let mut s = $style;
        s.background = Some($crate::color_value!($color));
        s
    }};

    // Bold
    ($style:expr, bold, $($rest:tt)*) => {{
        let mut s = $style;
        s.bold = Some(true);
        $crate::tui_apply_span_style!(s, $($rest)*)
    }};
    ($style:expr, bold) => {{
        let mut s = $style;
        s.bold = Some(true);
        s
    }};

    // Italic
    ($style:expr, italic, $($rest:tt)*) => {{
        let mut s = $style;
        s.italic = Some(true);
        $crate::tui_apply_span_style!(s, $($rest)*)
    }};
    ($style:expr, italic) => {{
        let mut s = $style;
        s.italic = Some(true);
        s
    }};

    // Underline
    ($style:expr, underline, $($rest:tt)*) => {{
        let mut s = $style;
        s.underline = Some(true);
        $crate::tui_apply_span_style!(s, $($rest)*)
    }};
    ($style:expr, underline) => {{
        let mut s = $style;
        s.underline = Some(true);
        s
    }};
}

/// Apply top-level properties to RichText (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_apply_richtext_props {
    // Base case
    ($rt:expr,) => { $rt };
    ($rt:expr) => { $rt };

    // Wrap mode
    ($rt:expr, wrap: $wrap:tt, $($rest:tt)*) => {{
        let rt = $rt.wrap($crate::text_wrap_value!($wrap));
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Color all spans
    ($rt:expr, color: $color:tt, $($rest:tt)*) => {{
        let rt = $rt.color($crate::color_value!($color));
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Background for all spans
    ($rt:expr, bg: $color:tt, $($rest:tt)*) => {{
        let rt = $rt.background($crate::color_value!($color));
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Bold all spans
    ($rt:expr, bold_all, $($rest:tt)*) => {{
        let rt = $rt.bold_all();
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Italic all spans
    ($rt:expr, italic_all, $($rest:tt)*) => {{
        let rt = $rt.italic_all();
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Alignment
    ($rt:expr, align: $align:tt, $($rest:tt)*) => {{
        let rt = $rt.align($crate::text_align_value!($align));
        $crate::tui_apply_richtext_props!(rt, $($rest)*)
    }};

    // Single property cases (no trailing comma)
    ($rt:expr, wrap: $wrap:tt) => {{
        $rt.wrap($crate::text_wrap_value!($wrap))
    }};

    ($rt:expr, color: $color:tt) => {{
        $rt.color($crate::color_value!($color))
    }};

    ($rt:expr, bg: $color:tt) => {{
        $rt.background($crate::color_value!($color))
    }};

    ($rt:expr, bold_all) => {{
        $rt.bold_all()
    }};

    ($rt:expr, align: $align:tt) => {{
        $rt.align($crate::text_align_value!($align))
    }};

    ($rt:expr, italic_all) => {{
        $rt.italic_all()
    }};
}

/// Internal helper macro for building text elements
#[macro_export]
#[doc(hidden)]
macro_rules! tui_text {
    ($content:expr, $($props:tt)*) => {{
        $crate::tui_build_text!($content, $($props)*)
    }};
}

/// Build input with properties (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_build_input {
    () => {{
        $crate::Node::Component(std::sync::Arc::new($crate::TextInput::new()))
    }};

    ($($props:tt)*) => {{
        #[allow(unused_mut)]
        let __input = $crate::TextInput::new();
        // Always add trailing comma for consistent parsing
        let __input = $crate::tui_apply_input_props!(__input, $($props)* ,);
        $crate::Node::Component(std::sync::Arc::new(__input))
    }};
}

/// Apply input properties (internal)
#[doc(hidden)]
#[macro_export]
macro_rules! tui_apply_input_props {
    // Base case - return the input
    ($input:expr,) => { $input };
    ($input:expr) => { $input };

    // Placeholder
    ($input:expr, placeholder: $text:expr, $($rest:tt)*) => {{
        let i = $input.placeholder($text);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, placeholder: $text:expr) => {{
        $input.placeholder($text)
    }};

    // Focusable
    ($input:expr, focusable: $value:expr, $($rest:tt)*) => {{
        let i = $input.focusable($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focusable: $value:expr) => {{
        $input.focusable($value)
    }};

    // Focusable shorthand
    ($input:expr, focusable, $($rest:tt)*) => {{
        let i = $input.focusable(true);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focusable) => {{
        $input.focusable(true)
    }};

    // Width
    ($input:expr, w: $value:expr, $($rest:tt)*) => {{
        let i = $input.width($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, w: $value:expr) => {{
        $input.width($value)
    }};

    // Width - optional with ! suffix on expression
    ($input:expr, w: ($value:expr)!, $($rest:tt)*) => {{
        let i = if let Some(width_val) = $value {
            $input.width(width_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, w: ($value:expr)!) => {{
        if let Some(width_val) = $value {
            $input.width(width_val)
        } else {
            $input
        }
    }};

    // Width fraction
    ($input:expr, w_frac: $frac:expr, $($rest:tt)*) => {{
        let i = $input.width_fraction($frac);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, w_frac: $frac:expr) => {{
        $input.width_fraction($frac)
    }};

    // Width auto
    ($input:expr, w_auto, $($rest:tt)*) => {{
        let i = $input.width_auto();
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, w_auto) => {{
        $input.width_auto()
    }};

    // Width content
    ($input:expr, w_content, $($rest:tt)*) => {{
        let i = $input.width_content();
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, w_content) => {{
        $input.width_content()
    }};

    ($input:expr, width: $value:expr, $($rest:tt)*) => {{
        let i = $input.width($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, width: $value:expr) => {{
        $input.width($value)
    }};

    // Height
    ($input:expr, h: $value:expr, $($rest:tt)*) => {{
        let i = $input.height($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, h: $value:expr) => {{
        $input.height($value)
    }};

    // Height - optional with ! suffix on expression
    ($input:expr, h: ($value:expr)!, $($rest:tt)*) => {{
        let i = if let Some(height_val) = $value {
            $input.height(height_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, h: ($value:expr)!) => {{
        if let Some(height_val) = $value {
            $input.height(height_val)
        } else {
            $input
        }
    }};

    // Height fraction
    ($input:expr, h_frac: $frac:expr, $($rest:tt)*) => {{
        let i = $input.height_fraction($frac);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, h_frac: $frac:expr) => {{
        $input.height_fraction($frac)
    }};

    // Height auto
    ($input:expr, h_auto, $($rest:tt)*) => {{
        let i = $input.height_auto();
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, h_auto) => {{
        $input.height_auto()
    }};

    // Height content
    ($input:expr, h_content, $($rest:tt)*) => {{
        let i = $input.height_content();
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, h_content) => {{
        $input.height_content()
    }};

    ($input:expr, height: $value:expr, $($rest:tt)*) => {{
        let i = $input.height($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, height: $value:expr) => {{
        $input.height($value)
    }};

    // Focus style
    ($input:expr, focus_style: ($style:expr), $($rest:tt)*) => {{
        let i = $input.focus_style($style);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_style: ($style:expr)) => {{
        $input.focus_style($style)
    }};

    // Focus style optional expression
    ($input:expr, focus_style: ($style:expr)!, $($rest:tt)*) => {{
        let i = if let Some(style_val) = $style {
            $input.focus_style(style_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $input.focus_style(style_val)
        } else {
            $input
        }
    }};

    // Focus border color
    ($input:expr, focus_border: none, $($rest:tt)*) => {{
        let i = $input.focus_border_with($crate::style::Border::none());
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: none) => {{
        $input.focus_border_with($crate::style::Border::none())
    }};
    ($input:expr, focus_border: $color:tt, $($rest:tt)*) => {{
        let i = $input.focus_border($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: $color:tt) => {{
        $input.focus_border($crate::color_value!($color))
    }};
    ($input:expr, focus_border: ($color:expr), $($rest:tt)*) => {{
        let i = $input.focus_border($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: ($color:expr)) => {{
        $input.focus_border($color)
    }};

    // Focus border style with color
    ($input:expr, focus_border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let i = $input.focus_border_style($style, $color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border_style: ($style:expr, $color:expr)) => {{
        $input.focus_border_style($style, $color)
    }};

    // Focus background
    ($input:expr, focus_background: $color:tt, $($rest:tt)*) => {{
        let i = $input.focus_background($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_background: $color:tt) => {{
        $input.focus_background($crate::color_value!($color))
    }};
    ($input:expr, focus_background: ($color:expr), $($rest:tt)*) => {{
        let i = $input.focus_background($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_background: ($color:expr)) => {{
        $input.focus_background($color)
    }};

    // Focus padding using scalar value
    ($input:expr, focus_padding: $value:expr, $($rest:tt)*) => {{
        let i = $input.focus_padding($crate::Spacing::all($value));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_padding: $value:expr) => {{
        $input.focus_padding($crate::Spacing::all($value))
    }};
    ($input:expr, focus_padding: ($spacing:expr), $($rest:tt)*) => {{
        let i = $input.focus_padding($spacing);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_padding: ($spacing:expr)) => {{
        $input.focus_padding($spacing)
    }};

    // Hover style
    ($input:expr, hover_style: ($style:expr), $($rest:tt)*) => {{
        let i = $input.hover_style($style);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_style: ($style:expr)) => {{
        $input.hover_style($style)
    }};

    // Hover style optional expression
    ($input:expr, hover_style: ($style:expr)!, $($rest:tt)*) => {{
        let i = if let Some(style_val) = $style {
            $input.hover_style(style_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $input.hover_style(style_val)
        } else {
            $input
        }
    }};

    // Hover border color
    ($input:expr, hover_border: $color:tt, $($rest:tt)*) => {{
        let i = $input.hover_border($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_border: $color:tt) => {{
        $input.hover_border($crate::color_value!($color))
    }};
    ($input:expr, hover_border: ($color:expr), $($rest:tt)*) => {{
        let i = $input.hover_border($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_border: ($color:expr)) => {{
        $input.hover_border($color)
    }};

    // Hover background
    ($input:expr, hover_background: $color:tt, $($rest:tt)*) => {{
        let i = $input.hover_background($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_background: $color:tt) => {{
        $input.hover_background($crate::color_value!($color))
    }};
    ($input:expr, hover_background: ($color:expr), $($rest:tt)*) => {{
        let i = $input.hover_background($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_background: ($color:expr)) => {{
        $input.hover_background($color)
    }};

    // Hover padding using scalar value
    ($input:expr, hover_padding: $value:expr, $($rest:tt)*) => {{
        let i = $input.hover_padding($crate::Spacing::all($value));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_padding: $value:expr) => {{
        $input.hover_padding($crate::Spacing::all($value))
    }};

    // Hover padding with explicit spacing expression
    ($input:expr, hover_padding: ($spacing:expr), $($rest:tt)*) => {{
        let i = $input.hover_padding($spacing);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_padding: ($spacing:expr)) => {{
        $input.hover_padding($spacing)
    }};

    // Hover padding shorthand alias
    ($input:expr, hover_pad: $value:expr, $($rest:tt)*) => {{
        let i = $input.hover_padding($crate::Spacing::all($value));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_pad: $value:expr) => {{
        $input.hover_padding($crate::Spacing::all($value))
    }};
    ($input:expr, hover_pad: ($spacing:expr), $($rest:tt)*) => {{
        let i = $input.hover_padding($spacing);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, hover_pad: ($spacing:expr)) => {{
        $input.hover_padding($spacing)
    }};

    // Border color (now using the more explicit name)
    ($input:expr, border_color: $color:tt, $($rest:tt)*) => {{
        let i = $input.border($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_color: $color:tt) => {{
        $input.border($crate::color_value!($color))
    }};

    // Border color with expression
    ($input:expr, border_color: ($color:expr), $($rest:tt)*) => {{
        let i = $input.border($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_color: ($color:expr)) => {{
        $input.border($color)
    }};

    // Border style with color
    ($input:expr, border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let i = $input.border_style($style, $color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_style: ($style:expr, $color:expr)) => {{
        $input.border_style($style, $color)
    }};

    // Legacy border support (maps to border_color)
    ($input:expr, border: none, $($rest:tt)*) => {{
        let i = $input.border_with($crate::style::Border::none());
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border: none) => {{
        $input.border_with($crate::style::Border::none())
    }};
    ($input:expr, border: $color:tt, $($rest:tt)*) => {{
        let i = $input.border($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border: $color:tt) => {{
        $input.border($crate::color_value!($color))
    }};

    // Border with expression (legacy)
    ($input:expr, border: ($color:expr), $($rest:tt)*) => {{
        let i = $input.border($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border: ($color:expr)) => {{
        $input.border($color)
    }};

    // Border color - optional with ! suffix on expression
    ($input:expr, border_color: ($color:expr)!, $($rest:tt)*) => {{
        let i = if let Some(color_val) = $color {
            $input.border(color_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_color: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $input.border(color_val)
        } else {
            $input
        }
    }};

    // Legacy border support (maps to border_color) - optional with ! suffix
    ($input:expr, border: ($color:expr)!, $($rest:tt)*) => {{
        let i = if let Some(color_val) = $color {
            $input.border(color_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $input.border(color_val)
        } else {
            $input
        }
    }};

    // Border style with explicit style and color
    ($input:expr, border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let i = $input.border_style($style, $color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_style: ($style:expr, $color:expr)) => {{
        $input.border_style($style, $color)
    }};

    // Border edges - simple syntax support
    ($input:expr, border_edges: top, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::TOP);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: bottom, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::BOTTOM);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: left, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::LEFT);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: right, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::RIGHT);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: horizontal, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::HORIZONTAL);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: vertical, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::VERTICAL);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: all, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::ALL);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: corners, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::CORNERS);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: edges, $($rest:tt)*) => {{
        let i = $input.border_edges($crate::BorderEdges::EDGES);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};

    // Support for | operator in edges
    ($input:expr, border_edges: $edge1:ident | $($edge2:ident)|+, $($rest:tt)*) => {{
        let edges = $crate::tui_edge_value!($edge1) $(| $crate::tui_edge_value!($edge2))+;
        let i = $input.border_edges(edges);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};

    // Expression-based edges
    ($input:expr, border_edges: ($edges:expr), $($rest:tt)*) => {{
        let i = $input.border_edges($edges);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_edges: ($edges:expr)!, $($rest:tt)*) => {{
        let i = if let Some(edges_val) = $edges {
            $input.border_edges(edges_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};

    // Terminal rules (no rest)
    ($input:expr, border_edges: top) => {{
        $input.border_edges($crate::BorderEdges::TOP)
    }};
    ($input:expr, border_edges: bottom) => {{
        $input.border_edges($crate::BorderEdges::BOTTOM)
    }};
    ($input:expr, border_edges: left) => {{
        $input.border_edges($crate::BorderEdges::LEFT)
    }};
    ($input:expr, border_edges: right) => {{
        $input.border_edges($crate::BorderEdges::RIGHT)
    }};
    ($input:expr, border_edges: horizontal) => {{
        $input.border_edges($crate::BorderEdges::HORIZONTAL)
    }};
    ($input:expr, border_edges: vertical) => {{
        $input.border_edges($crate::BorderEdges::VERTICAL)
    }};
    ($input:expr, border_edges: all) => {{
        $input.border_edges($crate::BorderEdges::ALL)
    }};
    ($input:expr, border_edges: corners) => {{
        $input.border_edges($crate::BorderEdges::CORNERS)
    }};
    ($input:expr, border_edges: edges) => {{
        $input.border_edges($crate::BorderEdges::EDGES)
    }};
    ($input:expr, border_edges: $edge1:ident | $($edge2:ident)|+) => {{
        let edges = $crate::tui_edge_value!($edge1) $(| $crate::tui_edge_value!($edge2))+;
        $input.border_edges(edges)
    }};
    ($input:expr, border_edges: ($edges:expr)) => {{
        $input.border_edges($edges)
    }};
    ($input:expr, border_edges: ($edges:expr)!) => {{
        if let Some(edges_val) = $edges {
            $input.border_edges(edges_val)
        } else {
            $input
        }
    }};

    // Full border configuration
    ($input:expr, border_full: ($style:expr, $color:expr, $edges:expr), $($rest:tt)*) => {{
        let i = $input.border_full($style, $color, $edges);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, border_full: ($style:expr, $color:expr, $edges:expr)) => {{
        $input.border_full($style, $color, $edges)
    }};

    // Focus style
    ($input:expr, focus_style: ($style:expr), $($rest:tt)*) => {{
        let i = $input.focus_style($style);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_style: ($style:expr)) => {{
        $input.focus_style($style)
    }};
    ($input:expr, focus_style: ($style:expr)!, $($rest:tt)*) => {{
        let i = if let Some(style_val) = $style {
            $input.focus_style(style_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_style: ($style:expr)!) => {{
        if let Some(style_val) = $style {
            $input.focus_style(style_val)
        } else {
            $input
        }
    }};

    // Focus border color helpers
    ($input:expr, focus_border: none, $($rest:tt)*) => {{
        let i = $input.focus_border_with($crate::style::Border::none());
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: none) => {{
        $input.focus_border_with($crate::style::Border::none())
    }};
    ($input:expr, focus_border: $color:tt, $($rest:tt)*) => {{
        let i = $input.focus_border($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: $color:tt) => {{
        $input.focus_border($crate::color_value!($color))
    }};
    ($input:expr, focus_border: ($color:expr), $($rest:tt)*) => {{
        let i = $input.focus_border($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: ($color:expr)) => {{
        $input.focus_border($color)
    }};
    ($input:expr, focus_border: ($color:expr)!, $($rest:tt)*) => {{
        let i = if let Some(color_val) = $color {
            $input.focus_border(color_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $input.focus_border(color_val)
        } else {
            $input
        }
    }};

    // Focus border style with explicit style and color
    ($input:expr, focus_border_style: ($style:expr, $color:expr), $($rest:tt)*) => {{
        let i = $input.focus_border_style($style, $color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_border_style: ($style:expr, $color:expr)) => {{
        $input.focus_border_style($style, $color)
    }};

    // Focus background color helpers
    ($input:expr, focus_background: $color:tt, $($rest:tt)*) => {{
        let i = $input.focus_background($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_background: $color:tt) => {{
        $input.focus_background($crate::color_value!($color))
    }};
    ($input:expr, focus_background: ($color:expr), $($rest:tt)*) => {{
        let i = $input.focus_background($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_background: ($color:expr)) => {{
        $input.focus_background($color)
    }};
    ($input:expr, focus_background: ($color:expr)!, $($rest:tt)*) => {{
        let i = if let Some(color_val) = $color {
            $input.focus_background(color_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_background: ($color:expr)!) => {{
        if let Some(color_val) = $color {
            $input.focus_background(color_val)
        } else {
            $input
        }
    }};

    // Focus padding helpers
    ($input:expr, focus_padding: ($padding:expr), $($rest:tt)*) => {{
        let i = $input.focus_padding($padding);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_padding: ($padding:expr)) => {{
        $input.focus_padding($padding)
    }};
    ($input:expr, focus_padding: ($padding:expr)!, $($rest:tt)*) => {{
        let i = if let Some(padding_val) = $padding {
            $input.focus_padding(padding_val)
        } else {
            $input
        };
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, focus_padding: ($padding:expr)!) => {{
        if let Some(padding_val) = $padding {
            $input.focus_padding(padding_val)
        } else {
            $input
        }
    }};

    // Z-index
    ($input:expr, z: $index:expr, $($rest:tt)*) => {{
        let i = $input.z_index($index);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, z: $index:expr) => {{
        $input.z_index($index)
    }};

    // Position
    ($input:expr, pos: $pos:tt, $($rest:tt)*) => {{
        let i = $input.position($crate::position_value!($pos));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, pos: $pos:tt) => {{
        $input.position($crate::position_value!($pos))
    }};

    // Absolute positioning shorthand
    ($input:expr, absolute, $($rest:tt)*) => {{
        let i = $input.absolute();
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, absolute) => {{
        $input.absolute()
    }};

    // Positioning offsets
    ($input:expr, top: $val:expr, $($rest:tt)*) => {{
        let i = $input.top($val);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, top: $val:expr) => {{
        $input.top($val)
    }};

    ($input:expr, right: $val:expr, $($rest:tt)*) => {{
        let i = $input.right($val);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, right: $val:expr) => {{
        $input.right($val)
    }};

    ($input:expr, bottom: $val:expr, $($rest:tt)*) => {{
        let i = $input.bottom($val);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, bottom: $val:expr) => {{
        $input.bottom($val)
    }};

    ($input:expr, left: $val:expr, $($rest:tt)*) => {{
        let i = $input.left($val);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, left: $val:expr) => {{
        $input.left($val)
    }};

    // Background color
    ($input:expr, bg: $color:tt, $($rest:tt)*) => {{
        let i = $input.background($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, bg: $color:tt) => {{
        $input.background($crate::color_value!($color))
    }};

    // Background with expression
    ($input:expr, bg: ($color:expr), $($rest:tt)*) => {{
        let i = $input.background($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, bg: ($color:expr)) => {{
        $input.background($color)
    }};

    // Cursor color
    ($input:expr, cursor_color: $color:tt, $($rest:tt)*) => {{
        let i = $input.cursor_color($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, cursor_color: $color:tt) => {{
        $input.cursor_color($crate::color_value!($color))
    }};

    // Cursor color with expression
    ($input:expr, cursor_color: ($color:expr), $($rest:tt)*) => {{
        let i = $input.cursor_color($color);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, cursor_color: ($color:expr)) => {{
        $input.cursor_color($color)
    }};

    // Content color
    ($input:expr, content_color: $color:tt, $($rest:tt)*) => {{
        let i = $input.content_color($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, content_color: $color:tt) => {{
        $input.content_color($crate::color_value!($color))
    }};

    // Content color shorthand
    ($input:expr, color: $color:tt, $($rest:tt)*) => {{
        let i = $input.content_color($crate::color_value!($color));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, color: $color:tt) => {{
        $input.content_color($crate::color_value!($color))
    }};

    // Content bold
    ($input:expr, content_bold: $value:expr, $($rest:tt)*) => {{
        let i = $input.content_bold($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, content_bold: $value:expr) => {{
        $input.content_bold($value)
    }};

    // Content bold shorthand
    ($input:expr, bold, $($rest:tt)*) => {{
        let i = $input.content_bold(true);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, bold) => {{
        $input.content_bold(true)
    }};

    // Text wrapping
    ($input:expr, wrap: $mode:ident, $($rest:tt)*) => {{
        let i = $input.wrap($crate::text_wrap_value!($mode));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, wrap: $mode:ident) => {{
        $input.wrap($crate::text_wrap_value!($mode))
    }};

    // Text wrapping with expression
    ($input:expr, wrap: ($mode:expr), $($rest:tt)*) => {{
        let i = $input.wrap($mode);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, wrap: ($mode:expr)) => {{
        $input.wrap($mode)
    }};

    // Padding
    ($input:expr, pad: $value:expr, $($rest:tt)*) => {{
        let i = $input.padding($crate::Spacing::all($value));
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, pad: $value:expr) => {{
        $input.padding($crate::Spacing::all($value))
    }};

    // Password mode with value
    ($input:expr, password: $value:expr, $($rest:tt)*) => {{
        let i = $input.password($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, password: $value:expr) => {{
        $input.password($value)
    }};

    // Password mode shorthand (enables password mode)
    ($input:expr, password, $($rest:tt)*) => {{
        let i = $input.password(true);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, password) => {{
        $input.password(true)
    }};

    // Clear on submit with explicit value
    ($input:expr, clear_on_submit: $value:expr, $($rest:tt)*) => {{
        let i = $input.clear_on_submit($value);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, clear_on_submit: $value:expr) => {{
        $input.clear_on_submit($value)
    }};

    // Clear on submit shorthand (enables clear on submit)
    ($input:expr, clear_on_submit, $($rest:tt)*) => {{
        let i = $input.clear_on_submit(true);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, clear_on_submit) => {{
        $input.clear_on_submit(true)
    }};

    // @change handler
    ($input:expr, @change: $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_change($handler);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @change: $handler:expr) => {{
        $input.on_change($handler)
    }};

    // @submit handler
    ($input:expr, @submit: $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_submit($handler);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @submit: $handler:expr) => {{
        $input.on_submit($handler)
    }};

    // @key handler
    ($input:expr, @key($key:tt): $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_key($crate::key_value!($key), $handler);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @key($key:tt): $handler:expr) => {{
        $input.on_key($crate::key_value!($key), $handler)
    }};

    // @key with modifiers handler
    ($input:expr, @key($modifier:ident + $($mods:tt)+): $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_key_with_modifiers(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        );
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @key($modifier:ident + $($mods:tt)+): $handler:expr) => {{
        $input.on_key_with_modifiers(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        )
    }};

    // @key_global handler
    ($input:expr, @key_global($key:tt): $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_key_global($crate::key_value!($key), $handler);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @key_global($key:tt): $handler:expr) => {{
        $input.on_key_global($crate::key_value!($key), $handler)
    }};

    // @key_global with modifiers handler
    ($input:expr, @key_global($modifier:ident + $($mods:tt)+): $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_key_with_modifiers_global(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        );
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @key_global($modifier:ident + $($mods:tt)+): $handler:expr) => {{
        $input.on_key_with_modifiers_global(
            $crate::key_with_modifiers_value!($modifier + $($mods)+),
            $handler,
        )
    }};

    // @blur handler
    ($input:expr, @blur: $handler:expr, $($rest:tt)*) => {{
        let i = $input.on_blur($handler);
        $crate::tui_apply_input_props!(i, $($rest)*)
    }};
    ($input:expr, @blur: $handler:expr) => {{
        $input.on_blur($handler)
    }};
}
