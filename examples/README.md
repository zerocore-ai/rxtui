# RxTUI Examples

This directory contains example applications demonstrating various features and patterns of the RxTUI framework.

## Examples

### [counter.rs](./counter.rs)
```bash
cargo run --example counter
```

<img width="100%" alt="counter demo" src="https://github.com/user-attachments/assets/c841f1e6-8bf9-4b5a-bed5-97bc31cc3537" />

A minimal counter demonstrating:
- Basic component structure with `#[update]` and `#[view]` macros
- State management and message handling
- Keyboard event handlers (`↑`/`↓` keys)
- The absolute minimum code needed for an RxTUI app

<br />

### [form.rs](./form.rs)
```bash
cargo run --example form
```

<img width="100%" alt="form demo" src="https://github.com/user-attachments/assets/5c675ab4-144d-4ef1-8545-7921a537bb23" />

Demonstrates form building capabilities:
- Text input fields with focus management
- Form validation and state management
- Submit/cancel actions
- Keyboard navigation between fields
- Error display and user feedback

<br />

### [stopwatch.rs](./stopwatch.rs)
```bash
cargo run --example stopwatch
```

<img width="100%" alt="stopwatch demo" src="https://github.com/user-attachments/assets/98b5702c-cc98-4845-9dbe-e03ac43104f6" />

Time-based UI updates:
- Effects system for side effects
- Timer implementation with start/stop/reset
- Formatting time display
- Combining user actions with background updates

<br />

### [align.rs](./align.rs)
```bash
cargo run --example align
```

<img width="100%" alt="align demo" src="https://github.com/user-attachments/assets/bff6886f-7d38-4e90-a512-04d79a3e6246" />

CSS Flexbox-style alignment demonstration:
- **JustifyContent**: Controls main axis distribution (Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly)
- **AlignItems**: Controls cross axis alignment (Start, Center, End)
- **AlignSelf**: Per-child alignment override
- Interactive controls to test different combinations
- Support for both horizontal and vertical directions
- Shows how justify and align work on perpendicular axes

<br />

### [hover.rs](./hover.rs)
```bash
cargo run --example hover
```

Demonstrates pointer-driven styling:
- Cards highlight with `hover_style` overlays and animated borders
- Keyboard tabbing still works via `focus_style` fallbacks
- Shows how to compose base, focus, and hover layers without extra event wiring
- Includes a `TextInput` with hover/focus styling via `hover_*` helpers
- Great starting point for interactive menus and dashboards

<br />

### [progressbar.rs](./progressbar.rs)
```bash
cargo run --example progressbar
```

<img width="100%" alt="progressbar demo" src="https://github.com/user-attachments/assets/092d84db-66ea-431c-be72-cf48a043e7f6" />

Animated progress bar with visual flair:
- Smooth multi-stop gradient with peachy colors (Coral → Peach → Salmon → Pink)
- Automatic animation using effects system
- Percentage display with real-time updates
- Demonstrates dynamic content generation with iterators
- Shows how to create visually appealing terminal graphics

<br />

### [shimmer_text.rs](./shimmer_text.rs)
```bash
cargo run --example shimmer_text
```

Animated text highlight inspired by shimmer loading placeholders:
- Continuous, reactive highlight sweeping across text
- Demonstrates color blending and per-character styling
- Ships a reusable `ShimmerText` component that takes the message and speed
- Uses the async effects system for smooth animation
- Includes exit shortcuts wired through the context handlers

<br />

### [scroll.rs](./scroll.rs)
```bash
cargo run --example scroll
```

Interactive overview of scroll behaviors:
- Dual vertical panels comparing visible and hidden scrollbars
- Nested scrollable containers with independent focus and overflow control
- Horizontal gallery demonstrating sideways panning without a scrollbar track
- Contextual hint banner that updates as different surfaces receive focus

<br />

### [scroll2.rs](./scroll2.rs)
```bash
cargo run --example scroll2
```

Single-panel reading view:
- Large article body contained within one scrollable surface
- Keyboard, mouse, and touchpad scrolling supported out of the box
- Fixed viewport keeps the layout stable on smaller terminals
- Helpful instructions for focusing and navigating the text

<br />

### [components.rs](./components.rs)
```bash
cargo run --example components
```

<img width="100%" alt="components demo" src="https://github.com/user-attachments/assets/9ad3e411-0ffe-487b-a0c1-93a3271284fc" />

Shows how to build complex UIs from reusable components:
- Multiple independent counter components with different colors
- Inter-component communication via topics
- Dynamic topic names in `#[update]` macro
- Nested component structure (Dashboard → Counter components)
- Both stateful (Counter) and stateless (Dashboard) components

### [spinner.rs](./spinner.rs)
```bash
cargo run --example spinner
```

<img width="100%" alt="spinner demo" src="https://github.com/user-attachments/assets/f791a987-b460-4053-ae7e-36d86534726f" />

Simple loading animation demonstration:
- Animated spinner using Unicode braille characters
- Automatic animation using `#[effect]` attribute
- Clean purple color scheme with rounded borders
- Shows how to create smooth animations with async effects
- Minimal code for animated UI elements

<br />

## Feature Showcase

### [demo.rs](./demo.rs)
```bash
cargo run --example demo
```
Multi-page demo application showcasing:
- Tab-based navigation system
- 15 different pages each demonstrating specific features
- Component communication via topics
- Complex layouts and styling
- Everything RxTUI can do in one app

The demo includes specialized pages for:
1. **Overflow** - Text overflow and truncation handling
2. **Direction** - Vertical/horizontal layouts and flow
3. **Percentages** - Percentage-based sizing
4. **Borders** - Border styles and selective edges
5. **Absolute** - Absolute positioning and modals
6. **Text Styles** - Colors, bold, underline, etc.
7. **Auto Sizing** - Content-based sizing
8. **Text Wrap** - Word wrapping and text flow
9. **Element Wrap** - Flexbox-like element wrapping
10. **Unicode** - Unicode and emoji support
11. **Content Size** - Dynamic content sizing
12. **Focus** - Focus management and keyboard navigation
13. **Rich Text** - Mixed styles within text
14. **Text Input** - Interactive text input fields
15. **Scrollable** - Scrollable regions and overflow
