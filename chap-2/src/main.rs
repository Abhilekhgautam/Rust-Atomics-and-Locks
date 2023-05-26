// Atomics -> Operation that is indivisible, it is either fully completed or it never happened.

// Multiple threads concurrently reading and modifying the same variable normally results in
// undefinded behaviour. However atomic operations allow different threads to safely read and
// modify the same variable because only one operation is being carried out at a time.

// A stop flag : Useful to notify a thread to stop working

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

fn some_work() {
    // do nothing
}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // create a thread to run in the background
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // listen to user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Available Commands: help, stop"),
            "stop" => break,
            _ => println!("Unknown Command"),
        }
    }

    // inform the background_thread to stop
    STOP.store(true, Relaxed);
    // wait for the background_thread
    background_thread.join().unwrap();
}
