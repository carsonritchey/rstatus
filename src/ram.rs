const DIR: &str = "/proc/meminfo";
const TOTAL_INDEX: usize = 0;
const AVAILABLE_INDEX: usize = 1;


use std::fs;
use std::io::{BufRead, BufReader};

pub fn percent_used() -> u16 {
    let r = BufReader::new(fs::File::open(DIR).expect("unable to open /proc/meminfo"));
    let mut lines = r.lines();
    let total: String = lines.nth(TOTAL_INDEX).expect("no line at total index").unwrap().chars().filter(|c| c.is_digit(10)).collect();
    let total: u64 = total.parse::<>().unwrap();

    let available: String = lines.nth(AVAILABLE_INDEX).expect("no line at available index").unwrap().chars().filter(|c| c.is_digit(10)).collect();
    let available: u64 = available.parse::<>().unwrap();

    return 100 - (available as f32 / total as f32 * 100 as f32 + 0.5) as u16;
}
