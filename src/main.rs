extern crate chrono;

mod battery;
mod ram;
mod cpu;

use chrono::Local;
use std::{thread, time, env, process::Command};


const SEPERATOR: char = '';
//const HEIGHT = 13;

//cont HEIGHT_FLAG: &str = "--height=";
//const SEPERATOR_FLAG: &str = "--seperator=";
const SLEEP_FLAG: &str = "-s=";
const BATTERY_FLAG: &str = "-b";
const BATTERY_DRY_FLAG: &str = "-b=dry";

fn main() {
    let mut battery = false;
    let mut battery_graphics = true;

    let mut sleep_length: u64 = 1;

    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg == BATTERY_FLAG {
            battery = true;
        }
        else if arg == BATTERY_DRY_FLAG {
            battery = true;
            battery_graphics = false;
        }
    }

    loop {
        let mut s: String = SEPERATOR.to_string();

        s = format!("{} ram: {}% {}", SEPERATOR, ram::percent_used(), s);

        if battery {
            if battery_graphics {
                s = format!(" {} {} {}% {}", SEPERATOR, battery::battery_draw_string(), battery::battery_percentage(), s);
            } else {
                s = format!(" {} {}% {}", SEPERATOR, battery::battery_percentage(), s);
            }
        }
        s = format!("{} {}", s, get_time());

        set_bar(s);

        sleep_in_seconds(sleep_length);
    }
}

fn set_bar(s: String) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(s)
        .spawn()
        .expect("unable to envoke xsetroot command")
        .wait()
        .expect("unable to wait for xsetroot command");
}

fn get_time() -> String {
    let local_time = Local::now();

    let mut s: String = String::from(local_time.format("%I:%M %p").to_string());
    s = format!("{} {} {}", s, SEPERATOR, String::from(local_time.format("%a, %b %e, %Y").to_string()));

    return s;
}

fn sleep_in_seconds(s: u64) {
    thread::sleep(time::Duration::from_millis(s * 1000));
}
