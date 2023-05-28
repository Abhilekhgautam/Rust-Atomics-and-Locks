// Memory Ordering

// Processors and compilers perform all sorts of trick to make our program run as fast as
// possible. Processor might determine that two particular consecutive instruction in the program
// will not affect each other and can execute them out of order.
// While a instruction is blocked fetching some data from the memory, several other instruction can
// be executed before that instruction finishes. However that shouldn't change the way the program
// is supposed to behave

// Compilers also reorder the instruction when it thinks that result in faster execution without
// changing the behaviour of program.

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
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
static X: AtomicI32 = AtomicI32::new(0);
fn main() {
    use std::thread;

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
