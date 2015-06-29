use std::io::prelude::*;
use std::fs;

pub fn read_ac_presence() -> bool {
    const FNAME: &'static str = "/sys/class/power_supply/AC/online";

    // Read the file.
    let mut string = String::new();
    let mut file = fs::File::open(FNAME).expect(
        "Couldn't open the ac presence file");
    file.read_to_string(&mut string).expect(
        "Couldn't read from the ac presence file");

    match string[..].trim().parse() {
        Ok(1) => true,
        Ok(0) => false,
        _ => panic!("Failed to parse the ac presence file."),
    }
}

pub fn read_battery_level() -> u32 {
    const FNAME: &'static str = "/sys/class/power_supply/BAT0/capacity";

    // Read the file.
    let mut string = String::new();
    let mut file = fs::File::open(FNAME).expect(
        "Couldn't open the battery capacity file");
    file.read_to_string(&mut string).expect(
        "Couldn't read from the battery capacity file");

    string.trim().parse().expect(
        "Failed to parse the battery level file.")
}
