# LCARS-like interface library

A library for building LCARS-like interfaces using Rust.

Very much a work in progress, help welcome.


## Example

```rust
use lcars::prelude::{Application, Color, Font, LcarsText, LcarsButton};

fn main() {
    let mut lcars = Application::new()
        .title("LCARS")
        .fullscreen()
        .background(Color::Black)
        .apply_default_layout() // adds a bunch of elements to the screen
        .build();

    let mut hworld_text = LcarsText::new()
        .box_color(Color::Orange)
        .position(100, 100)
        .text("Hello, World!")
        .size(200, 100).build();

    let mut hworld_button = LcarsButton::new()
        .box_color(Color::Orange)
        .position(100, 300)
        .text("Press me!")
        .on_click(|_| {
            println!("Hello, World!");
        })
        .size(200, 100).build();

    lcars.add_element(hworld_text);
    lcars.add_element(hworld_button);

    lcars.run();
}
```
