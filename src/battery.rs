const DIR: &str = "/sys/class/power_supply";
const CAPACITY_INDEX: usize = 12;
const STATUS_INDEX: usize = 2;

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

fn is_charging() -> bool {
    for i in 0..battery_count() {
        let dir = format!("{}/BAT{}/uevent", DIR, i);
        let r = BufReader::new(fs::File::open(dir).expect("unable to read battery data"));
        let line = r.lines().nth(STATUS_INDEX).expect("no line at capacity index").unwrap();
        let line = line.split("=").last().unwrap();
        if line == "Charging" {
            return true;
        }
    }
     
    return false;
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

const BATTERY_FULL: &str = "^r4,1,3,2^^r2,3,7,10^^f9^";
const CHARGING: &str = "hey im charging";
pub fn battery_draw_string() -> String {
    let mut s: String = String::new();
    if is_charging() {
        s = CHARGING.to_string();
    }

    s = format!("{}{}", s, BATTERY_FULL.to_string());    
    return s;
}
