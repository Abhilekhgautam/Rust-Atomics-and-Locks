// Memory Ordering

// Processors and compilers perform all sorts of trick to make our program run as fast as
// possible. Processor might determine that two particular consecutive instruction in the program
// will not affect each other and can execute them out of order.
// While a instruction is blocked fetching some data from the memory, several other instruction can
// be executed before that instruction finishes. However that shouldn't change the way the program
// is supposed to behave

// Compilers also reorder the instruction when it thinks that result in faster execution without
// changing the behaviour of program.

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
/// ```rust
/// fn f(a: &mut i32, b: &mut i32){
///    *a += 1;
///    *b += 1;
///    *a += 1;
/// }
/// ```
/// Here, the compiler most certainly assume that order of the operation doesn't matter so might
/// reorder the operations as:
///
/// ```rust
///   fn f(a: &mut i32, b: &mut i32){
///    *a += 2;
///    *b += 1;
///   }
/// ```
///
/// Later when the program is being executed the processor might for some reason end up executing
/// the second addition before the first one maybe because b was available in the cache while a had
/// to be fetched from the main memory.
///
/// Regardless of any optimization the behaviour of the program remains the same, the order in
/// which they are executed is entirely invisble to the rest of the program.

// The logic for verifying that the specific reordering doesn't change the behaviour of program
// doesn't consider other threads into account, so working with threads (atomics) we need to tell
// our processor and compiler what they can do and they can't with our operations.

// But how to tell them????
// Every rust atomic operation takes an argument of
// `std::sync::atomic::Ordering` enum
// Available Ordering are:
// 1. Relaxed : Relaxed Ordering
// 2. Ordering::{Release, Acquire, AcqRel} : Release and Acquire Ordering
// 3. Ordering::SeqCst : Sequentially consistent ordering.

// Happens-Before Relationship:
// The order of operation is defined in terms of happens-before relationship, this guarantees the
// happening of an event before another event.
//
// The basic happen-befor rule is that everything that happens withing the same thread happens in
// order. If a thread is executing f(); g(); then f() happens befor g().

/// Assuming a and b are concurrently executed by different threads:
/// ```rust
///   static X: AtomicI32 = AtomicI32::new(0);
///   static Y: AtomicI32 = AtomicI32::new(0);
///
///   fn a(){
///     X.store(10, Relaxed);  // op-1
///     Y.store(20, Relaxed);  // op-2
///   }
///
///   fn b(){
///     let y = Y.load(Relaxed); // op-3
///     let x = X.load(Relaxed); // op-4
///     printl!("{x} {y}");
///   }
/// ```
/// The basic happens-before rule applies within the thread i.e., op-1 is always executed before
/// op-2 and op-3 is always executed before op-4
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
use std::time::Duration;

use std::thread;
static X: AtomicI32 = AtomicI32::new(0);
#[allow(unused)]
fn changed_main() {
    X.store(1, Relaxed);
    // spawning a thread creates a happens-before relationship between what happened before the
    // spawn() call and the new thread.
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    // joining a thread creates a happens-before relationship between the joined thread and what
    // happens after the join() call
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    // this will never fail all thanks to happens-before relationship.
    assert!(x == 1 || x == 2);
}

// Relaxed Ordering: They don't provide any happens-before relationship, instead they just
// guarantee modification order, with which every threads abide by.
// Atomic Operations using relaxed memory ordering do not provide any happens-before relationship,
// what this means is all modification of the same atomic variable happen in an order that is the
// same from the perspective of all thread.

static Y: AtomicI32 = AtomicI32::new(0);

// Here only the fn a is modiying our atomic variable.
// The order of modification is 0->5->15
// Threads cannot see values that are inconsistent with this order.

// Every thread will agree with the order 0->5->15.
fn a() {
    Y.fetch_add(5, Relaxed);
    thread::sleep(Duration::from_millis(200));
    Y.fetch_add(10, Relaxed);
}

fn b() {
    let a = Y.load(Relaxed);
    let b = Y.load(Relaxed);
    thread::sleep(Duration::from_millis(200));
    let c = Y.load(Relaxed);
    let d = Y.load(Relaxed);

    println!("{a} {b} {c} {d}");
}
#[allow(unused)]
fn changed_main_two() {
    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    })
}

// now if we try to split the above function a into two functions:
fn a1() {
    Y.fetch_add(5, Relaxed);
}

fn a2() {
    Y.fetch_add(10, Relaxed);
}

// now there are 2 modification order:
// 0->5->15 or
// 0->10->15

// Even though there are 2 possible order, all the thread will stay consistent with a single order.
// But the order can be any of 2, no one knows exactly which one.
#[allow(unused)]
fn change_main_three() {
    thread::scope(|s| {
        s.spawn(a1);
        s.spawn(a2);
    })
}

// Release and Acquire Ordering
static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

// Release Memory Ordering applies to store.
// Acquire Memory Ordering applies to load.

// The Acquire-Release pair is used to form a happens-before relationship
// Everything before the release-store (1) will be visible when true is loaded
#[allow(unused)]
fn changed_main_three() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        thread::sleep(Duration::from_secs(2));
        READY.store(true, Release); // 1
    });

    // when READY.load(Acquire) returns true, this means that the other thread now
    // know everything what happened before the store, in this case it now knows that
    // 123 has been stored to  DATA.
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting");
    }
    println!("{}", DATA.load(Relaxed));
}

// acquire release even let us do that thing for non atomic variable.
static mut MY_DATA: u64 = 0;

fn main() {
    thread::spawn(|| {
        //Safety: Nothing else is accessing MY_DATA.
        unsafe {
            MY_DATA = 123;
        }
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(1000));
        println!("Waiting...");
    }

    println!("{}", unsafe { MY_DATA });
}
