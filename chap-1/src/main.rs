// Every rust program starts with exactly one thread, the main thread.
// A main thread can be used to spawn more threads

// New threads are spawned using std::thread::spawn function

use std::thread;

// A static variable is owned by the entire program
static X: [i32; 3] = [2, 4, 6];

/*

 Problem:

  when the main thread finishes executing the main function,
  the program will exit even if other threads are still running

*/

// spawning a thread.
#[allow(unused)]
fn changed_main_one() {
    // spawn thread that will execute the function `f`.
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}
/*
To ensure threads are finished before we return from the main
we can wait for them by joining them.

*/

/*
Unlike the below function where we are passing the name of the
function to std::thread::spawn.

It is more common to pass it a closure.


*/

// joining threads
#[allow(unused)]
fn changed_main_two() {
    // thread::spawn returns a JoinHandle
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    // the join method waits until the thread has finished executing
    // and returns a std::thread::Result
    t1.join().unwrap();
    t2.join().unwrap()
}

// passing a closure instead of a function
#[allow(unused)]
fn changed_main_three() {
    let numbers = vec![1, 2, 3];

    // we used the move closure to transfer the ownership of numbers vec
    // to the spawned thread, else it would have been passed by reference
    // which results in a compiler error because the new thread may outlive
    // the variable causing a dangling reference
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}

// We can get a value back out a thread by returning it from the
// closure. The returned value can be obtained form the Result
// returned by the join method.

// getting a value out of a thread.
#[allow(unused)]
fn changed_main_four() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        // risky:panics if len is 0, but not in this case
        sum / len
    });

    let average = t.join().unwrap();

    println!("The average is : {average}");
}
// Scoped thread
#[allow(unused)]
fn changed_main_five() {
    let numbers = vec![2, 4, 6, 8];

    // if we are sure that our spawned threads don't outlive certain scope that spawned thread
    // could then safely borrow things that don't live forever
    thread::scope(|s| {
        s.spawn(|| {
            println!("{numbers:?}");
        });

        s.spawn(|| {
            for number in numbers.iter() {
                println!("{number}");
            }
        });
    });
}

// mutating borrowed values inside a thread scope
#[allow(unused)]
fn changed_main_six() {
    let mut numbers = vec![2, 4, 6, 8];

    thread::scope(|s| {
        s.spawn(|| {
            println!("{numbers:?}");
            numbers.push(10);
            println!("After update");
            println!("{numbers:?}");
        });

        s.spawn(|| {
            // do anything except using numbers

            // we cannot have a reference to numbers, because Rust ownership rules don't allow 2
            // mutable references at a time.
            // println!("{numbers:?}");

            println!("I wont reference to numbers");
        });
    });

    // A static item always exists (it is never dropped). Every thread can borrow it.

    // No thread can own a static variable it is only borrowed
    thread::spawn(|| {
        println!("From first thread: {X:?}");
    });

    thread::spawn(|| {
        println!("From the second thread: {X:?}");
    });
}

fn main() {}
fn f() {
    println!("Hello from another thread");

    // get the thread id
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
