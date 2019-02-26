extern crate x11_screenshot;
fn main() {
    let screen = x11_screenshot::Screen::new();
    let frame = screen.capture();
    frame.save("./a.png").unwrap(); // Save image
                                    // Image docs http://www.piston.rs/image/image/index.html
}
