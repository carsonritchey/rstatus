extern crate chrono;

use chrono::Local;
use std::{thread, time, env, process::Command};

mod battery;
use battery::battery_percentage;

const SEPERATOR: char = '';
//const HEIGHT = 13;


fn main() {
    let mut battery = false;
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "-b" {
            battery = true;
        }
    }

    loop {
        let mut s: String = SEPERATOR.to_string();
        if battery {
            s = format!("{} {}% {}", SEPERATOR, battery_percentage(), s);
        }
        s = format!("{} {}", s, get_time());
        //s = format!("^r1,1,11,11^^f{}^ {}", 11, s);

        set_bar(s);

        sleep_in_seconds(1);
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
