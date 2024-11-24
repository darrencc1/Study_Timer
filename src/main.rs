//Must add dependencies inside the Cargo.toml

use notify_rust::Notification;
use std::thread;
use std::time::Duration;
//can be used to trigger an alarm at a specific time, works with times and dates. 
use chrono::{Local, Timelike};
//This can play a sound/send notification when alarm triggers. 
use rodio::{Decoder, OutputStream};
//notift-rust can be used for desktop applications. 


fn main()
{

}