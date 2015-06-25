//! cpumon
//!
//! Reports the average (across all cores) CPU load
//! over the course of a second.
//!
//! License: GPL3.0 (TODO stick the actual license in here)

extern crate statusbar;
use statusbar::cpu;

fn main() {

    // Calculate the recent CPU Load
    let cpu_load = cpu::measure_load();
    println!("{}%", cpu_load);

}

