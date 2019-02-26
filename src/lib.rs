extern crate image;
extern crate libc;
extern crate x11;
use self::image::{Pixel, Rgb, RgbImage};
use self::libc::{c_int, c_ulong};
use std::{ptr, slice};
use x11::xlib;
pub struct Screen {
    display: *mut xlib::Display,
    window: xlib::Window,
}
#[derive(Debug)]
struct Bgr {
    b: u8,
    g: u8,
    r: u8,
    _pad: u8,
}

impl Screen {
    pub fn new() -> Screen {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            Screen {
                display: display,
                window: root,
            }
        }
    }
    pub fn cap_frame(
        &self,
        w: u32,
        h: u32,
        x: i32,
        y: i32,
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let img = unsafe {
            xlib::XGetImage(
                self.display,
                self.window,
                x,
                y,
                w,
                h,
                !1 as c_ulong,
                2 as c_int,
            )
        };

        let mut fullimg = RgbImage::new(w, h);

        if !img.is_null() {
            let image = unsafe { &mut *img };
            let sl: &[Bgr] = unsafe {
                slice::from_raw_parts(
                    (image).data as *const _,
                    (image).width as usize * (image).height as usize,
                )
            };
            let clone = fullimg.clone();
            let mut pixs = clone.enumerate_pixels();
            for mut x in sl {
                let (xc, yc, _p) = pixs.next().unwrap();
                fullimg.put_pixel(xc, yc, *Rgb::from_slice(&[x.r, x.g, x.b]));
            }
        }

        unsafe {
            xlib::XDestroyImage(img as *mut _);
        }
        fullimg
    }
}
