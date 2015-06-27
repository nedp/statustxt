use std::io::prelude::*;
use std::fs;

use std::thread;
use std::time;

use std::fmt;

pub fn load_since(last: &Stats) -> (Load, Stats) {

    // Get the current cpu stats.
    let next: Stats = read_cpu_stats();

    // Calculate the recent CPU load
    let dtotal = next.total - last.total;
    let didle = next.idle - last.idle;
    let load = Load::from(((1000 * (dtotal - didle) / dtotal + 5) / 10));

    (load, next)
}

pub fn measure_load(duration: time::Duration) -> (Load, Stats) {

    // Get a reference point then wait a bit.
    let first: Stats = read_cpu_stats();
    thread::sleep(duration);

    load_since(&first)
}

pub struct Stats {
    idle: u64,
    total: u64,
}

pub struct Load(u32);

impl From<u64> for Load {
    fn from(value: u64) -> Load { Load(value as u32) }
}

impl fmt::Display for Load {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(formatter)
    }
}

pub fn read_cpu_stats() -> Stats {
    const STAT_FNAME: &'static str = "/proc/stat";
    const IDLE_FIELD: usize = 3; // From `% man proc` (zero indexed)

    // Read the proc stat file.
    let mut stat_string = String::new();
    let mut stat_file = fs::File::open(STAT_FNAME).expect(
        "Couldn't open the proc stat file");
    stat_file.read_to_string(&mut stat_string).expect(
        "Couldn't read from the proc stat file");
    let stat_line = stat_string.lines().next().expect(
        "Couldn't get first line from the proc stat file");

    // Parse the cpu time values from the first proc stat line.
    let mut fields = stat_line.split_whitespace().skip(1);
    let mut field_values: [u64; 10] = [0; 10];
    for i in 0..10 {
        let field = fields.next().expect(
                &format!("Couldn't retrieve field #{} from the proc stat file", i));
        field_values[i] = field.parse::<u64>().expect(
                &format!("Couldn't parse field #{} from the proc stat file", i));
    }
    let total = field_values.iter().sum::<u64>();
    let idle = field_values[IDLE_FIELD];

    Stats{total: total, idle: idle}
}
