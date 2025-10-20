//! Tests for the node! macro DSL

use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Basic Div Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_empty_div() {
    let node = node! {
        div []
    };

    match node {
        Node::Div(_) => {}
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_div_with_basic_props() {
    let node = node! {
        div(bg: black, pad: 2, w: 50, h: 20) []
    };

    match node {
        Node::Div(_) => {}
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_div_with_hex_color() {
    let node = node! {
        div(bg: "#FF5733", border: "#00FF00") []
    };

    match node {
        Node::Div(_) => {}
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_div_with_percentage_dimensions() {
    let node = node! {
        div(w_frac: 0.5, h_frac: 0.8) []
    };

    match node {
        Node::Div(_) => {}
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_container_with_auto_dimensions() {
    let node = node! {
        div(w_auto, h_content) []
    };

    match node {
        Node::Div(_) => {}
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_div_focus_border_none() {
    let node = node! {
        div(focusable, focus_border: none) []
    };

    match node {
        Node::Div(container) => {
            let focus_style = container.styles.focus.expect("focus style missing");
            let border = focus_style.border.expect("focus border missing");
            assert!(!border.enabled, "focus border should be disabled");
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_div_focus_border_color() {
    let node = node! {
        div(focus_border: red) []
    };

    match node {
        Node::Div(container) => {
            let focus_style = container.styles.focus.expect("focus style missing");
            let border = focus_style.border.expect("focus border missing");
            assert!(border.enabled, "focus border should be enabled");
            assert_eq!(border.color, Color::Red);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Text Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_simple_text() {
    let node = node! {
        div [
            text("Hello World")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_text_with_color() {
    let node = node! {
        div [
            text("Colored text", color: red)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_text_with_multiple_styles() {
    let node = node! {
        div [
            text("Styled text", color: yellow, bg: blue, bold, underline)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_text_with_bright_colors() {
    let node = node! {
        div [
            text("Bright colors", color: bright_yellow, bg: bright_blue)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_text_with_wrap() {
    let node = node! {
        div [
            text("Long text that should wrap", wrap: word)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Layout Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_vbox_layout() {
    let node = node! {
        vstack [
            text("Line 1"),
            text("Line 2"),
            text("Line 3")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 3);
            assert_eq!(
                container.styles.base.as_ref().and_then(|s| s.direction),
                Some(Direction::Vertical)
            );
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_hbox_layout() {
    let node = node! {
        hstack(gap: 2) [
            text("Left"),
            text("Center"),
            text("Right")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 3);
            let style = container.styles.base.as_ref().unwrap();
            assert_eq!(style.direction, Some(Direction::Horizontal));
            assert_eq!(style.gap, Some(2));
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_nested_layouts() {
    let node = node! {
        vstack [
            text("Header"),
            hstack [
                text("Left"),
                text("Right")
            ],
            text("Footer")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 3);
            assert_eq!(
                container.styles.base.as_ref().and_then(|s| s.direction),
                Some(Direction::Vertical)
            );
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Spacer Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_spacer() {
    let node = node! {
        div [
            text("Above"),
            spacer(2),
            text("Below")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 3);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Dynamic Content Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_dynamic_text() {
    let count = 42;
    let node = node! {
        div [
            text(format!("Count: {}", count), bold)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_conditional_color() {
    let is_error = true;
    let node = node! {
        div(bg: (if is_error { Color::Red } else { Color::Green })) [
            text("Status")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(
                container.styles.base.as_ref().and_then(|s| s.background),
                Some(Color::Red)
            );
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_conditional_text() {
    let logged_in = false;
    let node = node! {
        div [
            text(
                if logged_in { "Welcome!" } else { "Please login" },
                color: (if logged_in { Color::Green } else { Color::Red })
            )
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Positioning Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_absolute_positioning() {
    let node = node! {
        div(pos: absolute, top: 5, left: 10, z: 100) []
    };

    match node {
        Node::Div(container) => {
            let style = container.styles.base.as_ref().unwrap();
            assert_eq!(style.position, Some(Position::Absolute));
            assert_eq!(style.top, Some(5));
            assert_eq!(style.left, Some(10));
            assert_eq!(style.z_index, Some(100));
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_absolute_shorthand() {
    let node = node! {
        div(absolute, top: 0, right: 0) []
    };

    match node {
        Node::Div(container) => {
            let style = container.styles.base.as_ref().unwrap();
            assert_eq!(style.position, Some(Position::Absolute));
            assert_eq!(style.top, Some(0));
            assert_eq!(style.right, Some(0));
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Complex Nesting Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_deeply_nested_structure() {
    let node = node! {
        div(bg: black, pad: 2) [
            text("Title", color: yellow, bold),
            spacer(1),

            vstack(gap: 1) [
                hstack [
                    div(bg: blue, w: 20) [
                        text("Left", color: white)
                    ],
                    div(bg: green, w: 20) [
                        text("Right", color: white)
                    ]
                ],

                div(bg: red) [
                    text("Bottom", color: white)
                ]
            ]
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(
                container.styles.base.as_ref().and_then(|s| s.background),
                Some(Color::Black)
            );
            assert_eq!(container.children.len(), 3);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Property Shortcut Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_direction_shortcuts() {
    let node1 = node! {
        div(dir: v) []
    };

    let node2 = node! {
        div(dir: h) []
    };

    match (node1, node2) {
        (Node::Div(c1), Node::Div(c2)) => {
            assert_eq!(
                c1.styles.base.as_ref().and_then(|s| s.direction),
                Some(Direction::Vertical)
            );
            assert_eq!(
                c2.styles.base.as_ref().and_then(|s| s.direction),
                Some(Direction::Horizontal)
            );
        }
        _ => panic!("Expected div nodes"),
    }
}

#[test]
fn test_all_color_names() {
    let node = node! {
        vstack [
            text("Black", color: black),
            text("Red", color: red),
            text("Green", color: green),
            text("Yellow", color: yellow),
            text("Blue", color: blue),
            text("Magenta", color: magenta),
            text("Cyan", color: cyan),
            text("White", color: white)
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 8);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_wrap_and_overflow_modes() {
    let node = node! {
        div(wrap: wrap, overflow: hidden) [
            text("Content", wrap: word)
        ]
    };

    match node {
        Node::Div(container) => {
            let style = container.styles.base.as_ref().unwrap();
            assert_eq!(style.wrap, Some(WrapMode::Wrap));
            assert_eq!(style.overflow, Some(Overflow::Hidden));
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Focus and Interaction Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_focusable_div() {
    let node = node! {
        div(focusable) []
    };

    match node {
        Node::Div(container) => {
            assert!(container.focusable);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_focusable_with_value() {
    let should_focus = false;
    let node = node! {
        div(focusable: should_focus) []
    };

    match node {
        Node::Div(container) => {
            assert!(!container.focusable);
        }
        _ => panic!("Expected div node"),
    }
}

//--------------------------------------------------------------------------------------------------
// Edge Cases
//--------------------------------------------------------------------------------------------------

#[test]
fn test_empty_text() {
    let node = node! {
        div [
            text("")
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 1);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_multiple_children_with_trailing_comma() {
    let node = node! {
        div [
            text("First"),
            text("Second"),
            text("Third"),
        ]
    };

    match node {
        Node::Div(container) => {
            assert_eq!(container.children.len(), 3);
        }
        _ => panic!("Expected div node"),
    }
}

#[test]
fn test_expression_in_dimensions() {
    let window_width = 100;
    let window_height = 50;

    let node = node! {
        div(
            w: (window_width / 2),
            h: (window_height - 10)
        ) []
    };

    match node {
        Node::Div(container) => {
            let style = container.styles.base.as_ref().unwrap();
            assert_eq!(style.width, Some(Dimension::Fixed(50)));
            assert_eq!(style.height, Some(Dimension::Fixed(40)));
        }
        _ => panic!("Expected div node"),
    }
}
