#![feature(thread_sleep, time, duration)]

extern crate x11;
extern crate statusbar;

use std::ptr;
use std::ffi;

use std::time;
use std::thread;

use x11::xlib;

use statusbar::cpu;

const STEP_SECONDS: u64 = 1;

fn main() {
    let step = time::Duration::from_secs(STEP_SECONDS);

    // Get the initial cpu stats.
    let (cpu_load, mut prev_cpu_stat) = cpu::measure_load(step);
    let title = format!("CPU: {:>2}%", cpu_load);
    unsafe { set_root_title(&title); }

    // Each step, recalculate all values.
    loop {
        thread::sleep(step);

        let (cpu_load, cpu_stat) = cpu::load_since(&prev_cpu_stat);
        prev_cpu_stat = cpu_stat;

        let title = format!("CPU: {:>2}%", cpu_load);
        unsafe { set_root_title(&title); }
    }
}

unsafe fn set_root_title(title: &str) {
    let title_cstring = ffi::CString::new(title).unwrap();

    // Find the root window.
    let display = xlib::XOpenDisplay(ptr::null());
    if display.is_null() {
        panic!("Couldn't find the x display");
    }
    let screen_num = xlib::XDefaultScreen(display);
    let window = xlib::XRootWindow(display, screen_num);

    // Update the window name and clean up.
    xlib::XStoreName(display, window, title_cstring.as_ptr() as *mut _);
    xlib::XSync(display, 0);
    xlib::XCloseDisplay(display);
}
