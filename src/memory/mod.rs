use std::io::prelude::*;
use std::fs;

const MEMINFO_FNAME: &'static str = "/proc/meminfo";

pub fn available_and_free_swap_kb() -> (u32, u32) {
    const VALUE_FIELD: usize = 1;
    const AVAILABLE_LINE: usize = 2; // Zero indexed.
    const FREE_SWAP_LINE: usize = 15; // Zero indexed.

    let mut mem_string = String::new();
    {
        let mut mem_file = fs::File::open(MEMINFO_FNAME).expect(
            "Couldn't open the memory info file");
        mem_file.read_to_string(&mut mem_string).expect(
            "Couldn't read from the memory info file");
    }
    let mem_string = mem_string;

    let available_line;
    let swap_line;
    {
        let mut lines = mem_string.lines().skip(AVAILABLE_LINE);
        available_line = lines.next().expect(
            "Couldn't read the available line of memory info");

        // Currently *after* the available line, so subtract 1 from difference:
        let mut lines = lines.skip(FREE_SWAP_LINE - AVAILABLE_LINE - 1);
        swap_line = lines.next().expect(
            "Couldn't read the free swap line of memory info");
    }

    let available_field =
        available_line.split_whitespace().skip(VALUE_FIELD).next().expect(
        "Couldn't retrieve the available memory info field");
    let swap_field =
        swap_line.split_whitespace().skip(VALUE_FIELD).next().expect(
        "Couldn't retrieve the free swap memory info field");

    let available_value = available_field.parse::<u32>().expect(
        "Couldn't parse the available memory info field");
    let swap_value = swap_field.parse::<u32>().expect(
            "Couldn't parse the free_swap memory info field");

    (available_value, swap_value)
}
