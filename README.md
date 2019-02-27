# x11-screenshot-rs

Library for making screenshots.

**Docs**: https://docs.rs/x11-screenshot/

## Example
```rust
extern crate x11_screenshot;
fn main() {
    let screen = x11_screenshot::Screen::open().expect("Failed to open screen");
    let frame = screen.capture().expect("Failed to take screenshot");
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    frame.save("example_screenshot.png").unwrap();
}
```