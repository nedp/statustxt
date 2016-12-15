use std::io::prelude::*;
use std::fs;

/// Returns true if the machine is connected to an AC power supply,
/// else false.
pub fn read_ac_presence() -> bool {
    const FNAME: &'static str = "/sys/class/power_supply/AC/online";

    // Read the file.
    let mut string = String::new();
    let mut file = fs::File::open(FNAME).expect("Couldn't open the ac presence file");
    file.read_to_string(&mut string)
        .expect("Couldn't read from the ac presence file");

    match string[..].trim().parse() {
        Ok(1) => true,
        Ok(0) => false,
        _ => panic!("Failed to parse the ac presence file."),
    }
}

/// Returns the battery level as a percentage.
///
/// Needs to be optional because you can take the battery out of a laptop.
pub fn read_battery_level() -> Option<u32> {
    const FNAME: &'static str = "/sys/class/power_supply/BAT0/capacity";

    // Read the file.
    fs::File::open(FNAME)
        .ok()
        .map(|mut file| {
            let mut string = String::new();
            file.read_to_string(&mut string)
                .expect("Couldn't read from the battery capacity file.");
            string.trim()
                .parse()
                .expect("Failed to parse the battery level file.")
        })
}
