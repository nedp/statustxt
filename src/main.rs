#![feature(thread_sleep, time, duration)]

extern crate x11;
extern crate statusbar;

use std::ptr;
use std::ffi;

use std::time;
use std::thread;

use x11::xlib;

use statusbar::cpu;
use statusbar::memory;

const STEP_SECONDS: u64 = 1;


fn main() {
    let step = time::Duration::from_secs(STEP_SECONDS);
    let mut prev_cpu_stats;

    // Get the initial cpu stats.
    {
        let (stats, prev) = Stats::initial(step);
        prev_cpu_stats = prev;
        let title = stats.format_title();
        unsafe { set_root_title(&title); }
    }

    // Each step, recalculate all values.
    loop {
        thread::sleep(step);
        let stats = Stats::since(&mut prev_cpu_stats);

        let title = stats.format_title();
        unsafe { set_root_title(&title); }
    }
}

struct Stats {
    cpu_load: cpu::Load,

    available_mb: u32,
    free_swap_gb: f32,
}

impl Stats {

    fn initial(step: time::Duration) -> (Stats, cpu::Stats) {
        let (cpu_load, cpu_stats) = cpu::measure_load(step);

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        (Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,
        }, cpu_stats)
    }

    fn since(prev_cpu_stats: &mut cpu::Stats) -> Stats {
        let (cpu_load, cpu_stats) = cpu::load_since(prev_cpu_stats);
        *prev_cpu_stats = cpu_stats;

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,
        }
    }

    fn format_title(&self) -> String {
        format!("CPU[{}%] RAM[{}MB] Swap[{:.1}GB]",
            self.cpu_load, self.available_mb, self.free_swap_gb)
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
