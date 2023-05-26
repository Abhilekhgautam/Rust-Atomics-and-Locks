// Atomics -> Operation that is indivisible, it is either fully completed or it never happened.

// Multiple threads concurrently reading and modifying the same variable normally results in
// undefinded behaviour. However atomic operations allow different threads to safely read and
// modify the same variable because only one operation is being carried out at a time.

// A stop flag : Useful to notify a thread to stop working

use std::sync::atomic::Ordering::Relaxed;
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

fn some_work() {
    for _ in 0..100 {
        // do nothing
    }
}
#[allow(unused)]
fn changed_main_one() {
    use std::sync::atomic::AtomicBool;
    static STOP: AtomicBool = AtomicBool::new(false);

    // create a thread to run in the background
    let background_thread = thread::spawn(|| {
        // load returns the value what it stores
        // initially false is passed to the constructor so it retruns false
        // later when main thread calls the store method and pass it true
        // it returns false.
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

// simulating some processing
fn process_item(_i: usize) {
    // sleep for a sec
    thread::sleep(Duration::from_millis(500));
}

// Progress Reporting: We process 100 items one by one on a background_thread, while the main
// thread gives the user regular updates on the progress.
fn main() {
    use std::sync::atomic::AtomicUsize;
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // background_thread working on 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                // modify the value it currently stores.
                num_done.store(i + 1, Relaxed);
            }
        });

        // main thread shows status of the work in progress
        loop {
            // load returns what it currently stores
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Under Progress: ... {}/100", n);
        }
    });
    println!("Done");
}
