// Atomics -> Operation that is indivisible, it is either fully completed or it never happened.

// Multiple threads concurrently reading and modifying the same variable normally results in
// undefinded behaviour. However atomic operations allow different threads to safely read and
// modify the same variable because only one operation is being carried out at a time.

// A stop flag : Useful to notify a thread to stop working

use std::sync::atomic::Ordering::Relaxed;
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

// probably the optimizer will throw this away.
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
#[allow(unused)]
fn changed_main_two() {
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
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}

// Progress Reporting using thread parking.
#[allow(unused)]
fn changed_main_three() {
    use std::sync::atomic::AtomicUsize;

    let main_thread = thread::current();
    let num_done = AtomicUsize::new(0);
    thread::scope(|s| {
        // process items one at a time
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // wake up the main thread
            }
        });

        // this is the main thread
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Progress: {n}/100");
            // go to sleep
            thread::park_timeout(Duration::from_millis(800));
        }
    });

    println!("Done");
}

// Progress Reporting using mulitple threads.
#[allow(unused)]
fn changed_main_four() {
    use std::sync::atomic::AtomicUsize;

    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });

            // main thread displays progress
            loop {
                let n = num_done.load(Relaxed);
                if n == 100 {
                    break;
                }
                println!("Under Progress: {n}/ 100");
            }
        }
    });
    println!("Done");
}

// Showing Statistics:

fn main() {
    use std::sync::atomic::{AtomicU64, AtomicUsize};
    use std::time::Instant;

    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(1, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }
        // main thread displays the statistcs of the progress.
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);

            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working nothing done yet");
            } else {
                println!(
                    "Progress.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}
