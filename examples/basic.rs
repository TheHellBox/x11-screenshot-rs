extern crate x11_screenshot;
fn main() {
    let screen = x11_screenshot::Screen::new();
    let frame = screen.capture();
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    frame.save("example_screenshot.png").unwrap();
}
