use std::io::prelude::*;
use std::fs;

const MEMINFO_FNAME: &'static str = "/proc/meminfo";

pub fn available_and_free_swap_kb() -> (u32, u32) {
    const VALUE_FIELD: usize = 1;
    const AVAILABLE_LINE: usize = 2; // Zero indexed.
    const FREE_SWAP_LINE: usize = 15; // Zero indexed.

    // Get the contents of the memory info file.
    let meminfo = {
        let mut meminfo = String::new();
        let mut meminfo_file = fs::File::open(MEMINFO_FNAME)
            .expect("Couldn't open the memory info file");
        meminfo_file.read_to_string(&mut meminfo)
            .expect("Couldn't read from the memory info file");
        meminfo
    };

    // Get the lines for available and swap memory from the memory info.
    let (available_line, swap_line) = {
        let mut lines = meminfo.lines()
            .skip(AVAILABLE_LINE);
        let available_line = lines.next()
            .expect("Couldn't read the available line of memory info");

        // Currently *after* the available line, so subtract 1 from difference:
        let mut lines = lines.skip(FREE_SWAP_LINE - AVAILABLE_LINE - 1);
        let swap_line = lines.next()
            .expect("Couldn't read the free swap line of memory info");
        (available_line, swap_line)
    };

    // Retrieve the fields from the memory info.
    let available_field = available_line.split_whitespace()
        .skip(VALUE_FIELD)
        .next()
        .expect("Couldn't retrieve the available memory info field");
    let swap_field = swap_line.split_whitespace()
        .skip(VALUE_FIELD)
        .next()
        .expect("Couldn't retrieve the free swap memory info field");

    // Parse and return the memory values.
    let available_value = available_field.parse::<u32>()
        .expect("Couldn't parse the available memory info field");
    let swap_value = swap_field.parse::<u32>()
        .expect("Couldn't parse the free_swap memory info field");
    (available_value, swap_value)
}
