// Example: Locking

//Mutex are most common use case for release and acquire ordering

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    // compare_exchange takes 2 memory ordering param
    // 1. for when comparision succeeded
    // 2. for when camparision failed.
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        //Safety: We hold the lock, so nothing else is accessing DATA
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn changed_main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
///```rust
///use std::sync::atomic::AtomicPtr;
///
///fn get_data() -> &'static Data {
/// static PTR: AtomicPtr = AtomicPtr::new(std::ptr::null_mut());
///
/// let mut p = PTR.load(Acquire);
///
/// if p.is_null(){
///  p = Box::into_raw(Box::new(generate_data()));
///  if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p , Release, Acquire){
///    // Safety: p is not shared with any other thread
///    drop(unsafe{ Box::from_raw(p)});
///    p = e;
///  }
/// }
/// // Safety: p is not null and points to a properly initialize value
/// unsafe { &*p }
///
///}
// We need to use `Acquire` for both the load memory ordering and `compare_exchange`
// failure memoring ordering to be able to synchronize with the operation that stores
// the pointer. This store happens when the compare_exchange succeeds, so we must use
// Release as its success ordering.

// Sequentially Consistent Ordering
// It includes all the guarantees of acquire ordering and release ordering
use std::sync::atomic::Ordering::SeqCst;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
