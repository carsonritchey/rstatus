extern crate chrono;

use chrono::Local;
use std::process::Command;
use std::{thread, time};

const SEPERATOR: char = '';
// time format:  %I:%M %p  %a, %b %d, %Y

fn main() {
    let mut n = 11;
    loop {
        let mut s: String = SEPERATOR.to_string();
        s = format!("{} {}", s, get_time());
        s = format!("^r1,1,11,11^^f{}^ {}", n, s);

        set_bar(s);

        sleep_in_seconds(1);
    }
}

fn set_bar(s: String) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(s)
        .spawn()
        .expect("unable to envoke xsetroot command");
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
