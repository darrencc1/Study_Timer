use std::io::{self, Write};
/*Standard Library(No dependencies needed)
io: used for input/output for user commands and printing to consoe. 
Write: a trait that allows you to write data, (flushing outpit steams)
ex: io::stdout().flush() 
--Reads user input immediatly from the commandline and ensures the output is written immediatly. 
 */
use std::sync::{Arc, Mutex};
/*Standard Library
This allows synchronization for SAFELY sharing data between threads
Arc: (Atomic Reference Count) allows multiple threads to SHARE ownership of a value. Ensures memory safety when accessing shared resources across threads
Mutex:(Mutual Exclusion) only **1** thread can access the value it protects at a time, prevents race conditions
ex. pause_flag 
 */
use std::sync::atomic::{AtomicBool, Ordering};
/*Standard Library
Provides atomic types and operations for low level SYNCRONIZATION without locking
AtomicBool: Thread safe bool type supporting atomic operations 
ex. load, store
Ordering: This SPECIFIES memorey ordering guarantees for aomic ops. 
ex. Ordering::SeqCst: Sequential consistency, esuring all threads see changes in SAME order. 
ex2. Ordering::Relaxed:Minimal ordering guarantees, is faster but less predictable. 
ex3 use: This can efficiently help manage flags across threads **Without Locks**.
*/
use std::thread;
/*Standard Library.
Create/Spawn and Manage threads in Rust, 
**Threads can run CONCURRENTLY with the main thread, enabling PARALLEL execution**
ex. thread::spawn (creates new thread.)
ex2 use: In my Study_Timer program I can use this to hadler user input in a seperate thread, to the timer logiv can continue uninterrupted. 
 */
use std::time::Duration;
pub fn interrupt()
{
    loop
    {
        let mut interrupt = String::new();
        println!("Enter command ('p' to pause or resume timer, 'i' to stop/interrupt timer): ");
        std::io::stdin().read_line(&mut command).unwrap();
        match interrupt.trim().to_lowercase().as_str()
        {
            "p" =>
            {
                let mut paused = pause_flag.lock.unwrap();
                *paused = !*paused;
                if *paused
                {
                    println!("The Timer is Paused");
                }
                else
                {
                    println!("Resumed Timer");
                }
            }
            "i" =>
            {
                interrupt_flag.store(true, Ordering::SeqCst);
                println!("Stopping Timer");
                break;
            }
            _ => println!("Invalid command. Use 'p' to pause/play or 'i' to stop timer."),

        }
    }  
}