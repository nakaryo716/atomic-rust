use std::io::stdin;
use std::sync::atomic::Ordering;
use std::{sync::atomic::AtomicBool, thread};

fn main() {
    stop_flag();
}

// this function is trying atomicbool
// when I input "stop", the back ground task will stop
// This code have problem
// when the background task "do something" take a lot time, the stop command will not excute immediately
pub fn stop_flag() {
    static FLAG: AtomicBool = AtomicBool::new(false);

    let back_ground_task = thread::spawn(move || {
        while !FLAG.load(Ordering::Relaxed) {
            // do something
        }
    });

    for i in stdin().lines() {
        match i.unwrap().as_str() {
            "sudo" => println!("your pass:"),
            "help" => println!("Help...."),
            "stop" => break,
            _ => println!("Command not found"),
        }
    }
    FLAG.store(true, Ordering::Relaxed);

    back_ground_task.join().unwrap();
}
