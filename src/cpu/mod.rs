
use std::io::prelude::*;
use std::fs;

use std::thread;
use std::time;

const STAT_FNAME: &'static str = "/proc/stat";

// From '% man proc`, converted to zero index.
const IDLE_FIELD: usize = 3;

pub fn measure_load() -> u64 {

    // First get the inital CPU times
    let first: CpuTimes = read_cpu_times();

    // Get the CPU times a bit later
    thread::sleep(time::Duration::from_secs(1));
    let next: CpuTimes = read_cpu_times();

    // Calculate the recent CPU load
    let dtotal = next.total - first.total;
    let didle = next.idle - first.idle;

    (1000 * (dtotal - didle) / dtotal + 5) / 10
}

struct CpuTimes {
    idle: u64,
    total: u64,
}

fn read_cpu_times() -> CpuTimes {

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

    CpuTimes{total: total, idle: idle}
}
