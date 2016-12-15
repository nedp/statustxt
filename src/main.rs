extern crate x11;
extern crate statustxt;
extern crate time;

use std::ptr;
use std::ffi;

use std::time as stdtime;
use std::thread;

use x11::xlib;

use statustxt::cpu;
use statustxt::memory;

#[cfg(feature = "power")]
use statustxt::power;

const STEP_SECONDS: u64 = 1;

fn main() {

    let step = stdtime::Duration::from_secs(STEP_SECONDS);
    let mut prev_cpu_stats;

    // Get the initial cpu stats.
    {
        let (stats, cpu_stats) = Stats::initial(step);
        prev_cpu_stats = cpu_stats;
        let title = stats.format_title();
        unsafe { set_root_title(&title); }
    }

    // Each step, recalculate all values.
    loop {
        thread::sleep(step);
        let (stats, cpu_stats) = Stats::since(&prev_cpu_stats);
        prev_cpu_stats = cpu_stats;

        let title = stats.format_title();
        unsafe { set_root_title(&title); }
    }
}

/// A structure to store statistics on:
///
/// * CPU load
/// * Available memory
/// * Free swap memory
/// * "power" feature = power supply presence + battery level
#[cfg(feature = "power")]
struct Stats {
    cpu_load: cpu::Load,

    available_mb: u32,
    free_swap_gb: f32,

    ac_is_present: bool,
    battery_level: Option<u32>,

    time: time::Tm,
}

#[cfg(feature = "power")]
impl Stats {

    /// Determines the stats for the initial step.
    ///
    /// Does not require the previous cpu stats, but takes
    /// time equal to the step duration.
    ///
    /// Returns a 2-tuple of the initial stats and the initial cpu stats.
    fn initial(step: stdtime::Duration) -> (Stats, cpu::Stats) {
        let (cpu_load, cpu_stats) = cpu::measure_load(step);

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        let stats = Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,

            ac_is_present: power::read_ac_presence(),
            battery_level: power::read_battery_level(),

            time: time::now(),
        };

        (stats, cpu_stats)
    }

    /// Determines the stats for subsequent (not the first) steps.
    ///
    /// Requires the previous cpu stats, but completes (practically)
    /// immediately.
    ///
    /// Returns a 2-tuple of the statustxt stats and the cpu status for
    /// this step.
    fn since(prev_cpu_stats: &cpu::Stats) -> (Stats, cpu::Stats) {
        let (cpu_load, cpu_stats) = cpu::load_since(prev_cpu_stats);

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        let stats = Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,

            ac_is_present: power::read_ac_presence(),
            battery_level: power::read_battery_level(),

            time: time::now(),
        };

        (stats, cpu_stats)
    }

    fn format_title(&self) -> String {
        use std::str::FromStr;
        let ac_string = match self.ac_is_present {
            true => "AC",
            false => "B",
        };
        let time_string = self.time.strftime("%a %d %b [%T]").expect(
            "Failed to format the date and time.");
        let battery_string =
            self.battery_level.map_or(FromStr::from_str("N/A").unwrap(),
                                      |level| format!("{}%", level));
        format!("C[{}%] R[{}MB] S[{:.1}GB] {}[{}] {}",
            self.cpu_load, self.available_mb, self.free_swap_gb,
            ac_string, battery_string, time_string)
    }

}

/// A structure to store statistics on:
///
/// * CPU load
/// * Available memory
/// * Free swap memory
#[cfg(not(feature = "power"))]
struct Stats {
    cpu_load: cpu::Load,

    available_mb: u32,
    free_swap_gb: f32,

    time: time::Tm,
}

#[cfg(not(feature = "power"))]
impl Stats {

    /// Determines the stats for the initial step.
    ///
    /// Does not require the previous cpu stats, but takes
    /// time equal to the step duration.
    ///
    /// Returns a 2-tuple of the initial stats and the initial cpu stats.
    fn initial(step: stdtime::Duration) -> (Stats, cpu::Stats) {
        let (cpu_load, cpu_stats) = cpu::measure_load(step);

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        let stats = Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,

            time: time::now(),
        };

        (stats, cpu_stats)
    }

    /// Determines the stats for subsequent (not the first) steps.
    ///
    /// Requires the previous cpu stats, but completes (practically)
    /// immediately.
    ///
    /// Returns a 2-tuple of the statustxt stats and the cpu status for
    /// this step.
    fn since(prev_cpu_stats: &cpu::Stats) -> (Stats, cpu::Stats) {
        let (cpu_load, cpu_stats) = cpu::load_since(prev_cpu_stats);

        let (available_kb, free_swap_kb) = memory::available_and_free_swap_kb();

        let stats = Stats {
            cpu_load: cpu_load,

            available_mb: available_kb / 1000,
            free_swap_gb: free_swap_kb as f32 / 1_000_000_f32,

            time: time::now(),
        };

        (stats, cpu_stats)
    }

    fn format_title(&self) -> String {
        let time_string = self.time.strftime("%a %d %b [%T]").expect(
            "Failed to format the date and time.");

        format!("C[{}%] R[{}MB] S[{:.1}GB] {}",
            self.cpu_load, self.available_mb, self.free_swap_gb, time_string)
    }

}

unsafe fn set_root_title(title: &str) {

    // Need a C string for the xlib interface.
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
