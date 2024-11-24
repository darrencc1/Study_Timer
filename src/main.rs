//Must add dependencies inside the Cargo.toml

// use notify_rust::Notification;
// use std::thread;
// use std::time::Duration;
//can be used to trigger an alarm at a specific time, works with times and dates. 
// use chrono::{Local, Timelike};
//This can play a sound/send notification when alarm triggers. 
// use rodio::{Decoder, OutputStream}
//notift-rust can be used for desktop applications. 
//std::io allows input from user. 
use std::io;

fn main() {
    // Collect user inputs using the helper functions
    let study_time = study_time();
    let short_break_length;
    let long_break_length;

    (short_break_length, long_break_length) = break_length();
    let no_sessions = no_study_sessions();

    // Display the summary message
    println!(
        "You will be studying for {:.2} minutes with a short break of {:.2} minutes. Every {} study sessions, you will take a long break for {:.2} minutes!",
        study_time, short_break_length, no_sessions, long_break_length
    );
}

fn study_time() -> f32 {
    loop {
        let mut study_time = String::new();
        println!("How long do you want to study for in min.sec?");
        io::stdin()
            .read_line(&mut study_time)
            .expect("Could not read input.");

        let study_time = study_time.trim(); // Trim whitespace and newline
        match study_time.parse::<f32>() {
            Ok(time) => {
                return time; // Return the parsed value
            }
            Err(_) => println!("Invalid number! Please enter a float in the form min.sec!"),
        }
    }
}

fn break_length() -> (f32, f32) {
    loop {
        let mut short_break_length = String::new();
        println!("How long would you like your short breaks to be in min.sec?");
        io::stdin()
            .read_line(&mut short_break_length)
            .expect("Could not read input.");
        let short_break_length = short_break_length.trim();

        let mut long_break_length = String::new();
        println!("How long do you want the long break to be in min.sec?");
        io::stdin()
            .read_line(&mut long_break_length)
            .expect("Could not read input.");
        let long_break_length = long_break_length.trim();

        match (
            short_break_length.parse::<f32>(),
            long_break_length.parse::<f32>(),
        ) {
            (Ok(short), Ok(long)) => {
                return (short, long); 
            }
            _ => println!("Invalid input! Please enter valid floats in the form min.sec."),
        }
    }
}

fn no_study_sessions() -> i32 {
    loop {
        let mut no_sessions = String::new();
        println!("How many study sessions would you like to do?");
        io::stdin()
            .read_line(&mut no_sessions)
            .expect("Could not read input.");
        let no_sessions = no_sessions.trim();

        match no_sessions.parse::<i32>() {
            Ok(no) => {
                return no; // Return the parsed value
            }
            Err(_) => println!("Invalid response! Please input a valid number."),
        }
    }
}

