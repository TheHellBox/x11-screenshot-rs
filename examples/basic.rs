extern crate x11-screenshot;
fn main() {
    let screen = x11_screenshot::Screen::new();
    let frame = screen.cap_frame(1920,1080,0,0); //ScrW, ScrH, PosX, PosY
    frame.save("./a.png").unwrap(); // Save image
    //Image docs http://www.piston.rs/image/image/index.html
}
