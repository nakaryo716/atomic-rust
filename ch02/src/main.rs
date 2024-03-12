use std::io::stdin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time;
use std::{sync::atomic::AtomicBool, thread};

fn main() {
    // stop_flag();
    // process_report();
    atomic_add();
    
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

#[allow(dead_code)]
fn  process_report() {
    static NUM: AtomicUsize = AtomicUsize::new(0);

    thread::spawn(move || {
        for i in 0..100 {
            if i % 10 == 0 {
                sleep(time::Duration::from_secs(1));
            }
            NUM.store(i + 1, Ordering::Relaxed);
        }
    });

    loop {
        let n = NUM.load(Ordering::Relaxed);

        if n == 100{
            break;
        } else {
            println!("{}/100", n);
        }

        sleep(time::Duration::from_secs(1));
    }
}

// atomicaly add
// Ordering Relaxed is not mind order things that do opereting eatch other
fn atomic_add() {
    static NUM: AtomicUsize = AtomicUsize::new(0);

    thread::spawn(move || {
        for _ in 0..10 {
            NUM.fetch_add(1, Ordering::Relaxed);
            sleep(time::Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        for _ in 0..10 {
            NUM.fetch_add(1, Ordering::Relaxed);
            sleep(time::Duration::from_secs(2));
        }
    });

    loop {
        let n = NUM.load(Ordering::Relaxed);

        if n == 20 {
            println!("done");
            break;
        } else {
            println!("{}/20", n);
        }

        sleep(time::Duration::from_secs(2));
    }
}