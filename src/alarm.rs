use rodio::{Decoder, OutputStream};
use std::{fs::File, io::BufReader, thread, time::Duration};

pub fn study_alarm(study_time: f32)
{
    countdown(5);
    timer(study_time)
}
fn countdown(seconds: i32) 
{
    let mut seconds: i32 = seconds;
    println!("Your stuudy timer will start in {} seconds", seconds);
    while seconds > 0 {
        println!("{}", seconds);
        thread::sleep(Duration::from_secs(1)); //this is what actual waits for 1 second.
        seconds -= 1;
    }
}
fn timer(study_time: f32)
{
    let mut clock_time = (study_time * 60.0) as i32;
    println!("{} minutes remaining", study_time);
    while clock_time != 0 {
        thread::sleep(Duration::from_secs(1));
        clock_time -= 1;

        if clock_time % 60 == 0 {
            println!("You have {} minutes left", clock_time / 60);
        }
        if clock_time == 0 {
            alarm_sound("./assets/alarm_sound.mp3");
        }
    }
}

pub fn break_alarm(break_length: f32) {
    let mut break_time = (break_length * 60.0) as i32;
    println!("Congratulations it is BREAK TIME!");
    while break_time != 0 {
        thread::sleep(Duration::from_secs(1));
        break_time -= 1;
        if break_time % 60 == 0 {
            println!("You have {} minutes left on your break!", break_length);
        }
        if break_time == 0 {
            alarm_sound("./assets/alarm_sound.mp3");
        }
    }
}

pub fn long_break_alarm(break_length: f32) {
    let mut long_break_time = (break_length * 60.0) as i32;
    if long_break_time != 0 {
        thread::sleep(Duration::from_secs(1));
        long_break_time -= 1;
        if long_break_time % 60 == 0 {
            println!("YOu have {} minutes left on your long break!", break_length)
        }
        if long_break_time == 0 {
            alarm_sound("./assets/alarm_sound.mp3")
        }
    }
}

fn alarm_sound(file_path: &str) {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to initialize output stream");

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