const DIR: &str = "/sys/class/power_supply";
const CAPACITY_INDEX: usize = 12;

use std::fs;
use std::io::{BufRead, BufReader};

fn battery_count() -> u16 {
    let paths = fs::read_dir(DIR).unwrap();
    let mut t: u16 = 0;
    for path in paths {
        if path.as_ref().unwrap().path().to_string_lossy().contains("BAT") {
            t += 1;
        }
    }

    return t;
}

pub fn battery_percentage() -> u16 {
    let mut percent = 0;
    for i in 0..battery_count() {
        let dir = format!("{}/BAT{}/uevent", DIR, i);
        let r = BufReader::new(fs::File::open(dir).expect("unable to read battery data"));
        let line = r.lines().nth(CAPACITY_INDEX).expect("no line at capacity index").unwrap();
        let line = line.split("=").last().unwrap();
        percent += line.parse::<u16>().unwrap();
    }

    return percent;
}
