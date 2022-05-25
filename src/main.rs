extern crate regex;
extern crate daemonize;
extern crate notify_rust;

use std::env;
use std::process;
use std::thread;
use std::time;
use regex::Regex;
use daemonize::Daemonize;
use notify_rust::Notification;
use notify_rust::Hint;

fn main() {
    let (delay, message) = parse_args();
    if delay == "" && message == "" {
        print_usage();
        process::exit(1);
    }
    let delay_seconds = match parse_delay(&delay) {
        Ok(seconds) => seconds,
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };
    match Daemonize::new().start() {
        Ok(_) => {
            thread::sleep(time::Duration::new(delay_seconds, 0));
            notify(message);
        },
        Err(error) => println!("Error: {}", error)
    }
}

fn print_usage() {
    println!("Usage: {} <DELAY> [MESSAGE]", env::args().nth(0).unwrap());
}

fn notify(message: String) {
    Notification::new()
        .summary("Reminder")
        .body(message.as_str())
        .timeout(10000)
        .hint(Hint::Resident(true))
        .show()
        .unwrap();
}

fn parse_args() -> (String, String) {
    let mut delay: String = String::new();
    let mut message: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            delay = arg;
        } else if index > 1 {
            message = message + " " + &arg;
            message = message.trim().to_string();
        }
    }
    (delay, message)
}

fn parse_delay(delay: &str) -> Result<u64, String> {
    let re = Regex::new(r"^(\d+)([HhMmSs])$").unwrap();
    match re.captures(delay) {
        Some(captures) => {
            let amount: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let unit = captures.get(2).unwrap().as_str();
            Ok(calc_seconds(amount, unit))
        }
        None => { Err(format!("Invalid amount of time: {}", delay)) }
    }
}

fn calc_seconds(amount: u64, unit: &str) -> u64 {
    let multiplier = match unit.to_uppercase().as_str() {
        "H" => { 60 * 60 },
        "M" => { 60 },
        _ => { 1 }
    };
    amount * multiplier
}
