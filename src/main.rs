//Must add dependencies inside the Cargo.toml


use rodio::{Decoder, OutputStream, Source, Sink};
// use std::thread;
use std::{thread, time::Duration, fs::File, io::BufReader};
//can be used to trigger an alarm at a specific time, works with times and dates. 
// use chrono::{Local, Timelike};
//This can play a sound/send notification when alarm triggers. 
// use rodio::{Decoder, OutputStream}
//notift-rust can be used for desktop applications. 
//std::io allows input from user. 
// use notify_rust::Notification;
use std::io;

fn main() 
{
    let study_time = study_time();
    let short_break_length;
    let long_break_length;
    (short_break_length, long_break_length) = break_length();
    let no_sessions = no_study_sessions();
    println!(
        "You will be studying for {:.2} minutes with a short break of {:.2} minutes. Every {} study sessions, you will take a long break for {:.2} minutes!",
        study_time, short_break_length, no_sessions, long_break_length
    );
    alarm();
        // while break_time != 0.0
        // {
        //     thread::sleep(Duration::from_secs(1));
        //     break_time -= 1.0;
        //     if break_time % 60.0 == 0.0
        //     {
        //         println!("You have {} minutes left on your break", break_time / 60.0);
        //     }
        // }
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

fn alarm_sound(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default()
        .expect("Failed to initialize output stream");
    
    if !std::path::Path::new(file_path).exists() {
        eprintln!("Audio file not found: {}", file_path);
        return;
    }
    
    let file = BufReader::new(File::open(file_path).expect("Failed to open the audio file"));
    let source = Decoder::new(file).expect("Failed to decode the audio file");
    
    use rodio::Sink;
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");
    sink.append(source);
    sink.sleep_until_end();
}

fn alarm()
{
    let study_time = study_time();
    let short_break; 
    let long_break;
    let mut no_sessions = no_study_sessions() as i32;
    (short_break, long_break) = break_length();
    let mut seconds:i32 = 5;
    println!("Your timer will start in {} seconds", seconds);
    while seconds > 0
    {
        println!("{}", seconds);
        thread::sleep(Duration::from_secs(1));//this is what actual waits for 1 second.
        seconds -= 1;
    }
    //timer
    while no_sessions != 0
    {   
        let mut clock_time = (study_time * 60.0) as i32;
        let mut break_time = (short_break * 60.0) as i32;
        println!("{} minutes remaining", study_time);
        while clock_time != 0
        {
            thread::sleep(Duration::from_secs(1));
            clock_time -= 1;
            
            if clock_time % 60 == 0
            { 
                println!("You have {} minutes left", clock_time / 60);
            }
            if clock_time == 0
            {
                no_sessions -= 1;
                alarm_sound("./assets/alarm_sound.mp3");

            }
        }
        println!("Congratulations it is BREAK TIME!");
        while break_time != 0
        {
            thread::sleep(Duration::from_secs(1));
            break_time -= 1;
            if break_time % 60 == 0
            {
                println!("You have {} minutes left on your break!", break_time);
            }
            if break_time == 0
            {
                alarm_sound("./alarm_sound.mp3");
            }
        }
    }
}
    