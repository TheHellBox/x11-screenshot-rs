//! A library for capturing screenshots on X11.

#![warn(missing_docs)]

extern crate image;
extern crate libc;
extern crate x11;
use self::image::{Pixel, Rgb, RgbImage};
use self::libc::{c_int, c_ulong};
use std::{ptr, slice};
use x11::xlib;
/// A handle to an X11 screen.
pub struct Screen {
    display: *mut xlib::Display,
    screen: *mut xlib::Screen,
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
    /// Tries to open the X11 display, then returns a handle to the default screen.
    ///
    /// Returns `None` if the display could not be opened.
    pub fn open() -> Option<Screen> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return None;
            }
            let screen = xlib::XDefaultScreenOfDisplay(display);
            let root = xlib::XRootWindowOfScreen(screen);
            Some(Screen {
                display,
                screen,
                window: root,
            })
        }
    }
    /// Captures a screenshot of the entire screen.
    pub fn capture(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let screen: &mut xlib::Screen = &mut unsafe { *self.screen };
        self.capture_area(screen.width as u32, screen.height as u32, 0, 0)
    }
    /// Captures a screenshot of the provided area.
    pub fn capture_area(
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

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
