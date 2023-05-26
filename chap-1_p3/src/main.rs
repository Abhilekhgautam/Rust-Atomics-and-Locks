// What is thread Safety?

// We saw various types earlier, some of them were thread safe like Arc<T>, while
// Cell<T> was unsafe

// Rust uses two special traits to keep track of types which can be safely used used across
// threads.

// 1. Send : A type is Send if it can be sent to another thread. i.e., if ownership of a value of
//    that type can be transferred to another thread, it is is a Send. Eg: Arc<i32>

// 2. Sync: A type is Sync if it can be shared with another thread. i.e., a type T is Sync if and
//    only if a share reference of that type, &T is Send. Eg: i32 is sync, while Cell<T> is not.

// All primitive types like i32, bool are both Send and Sync. Both of these traits are auto traits,
// which means that they are automatically implemented for types based on their fields. A sturct
// with fields that are all Send and Sync, is itself also Send and Sync

use std::cell::Cell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
// all the fields of Demo are Send and Sync, so Demo is also Send and Sync
#[allow(unused)]
pub struct Demo {
    x: i32,
}

// To opt out of this, add a type that doesn't implement the trait

// Fo this a special std::marker::PhantomData<T> comes in handy

#[allow(unused)]
pub struct DemoTwo {
    x: i32,

    // PhantomData<Cell<()>> is treated as if it were a Cell<()>. Since Cell<()> is not sync DemoTwo is
    // also not Sync.
    _not_sync: PhantomData<Cell<()>>, // PhantomData is Zero-Sized
}
// Raw Pointers are neither Send nor Sync, So X is neither.
#[allow(unused)]
pub struct X {
    x: *mut i32,
}

// We can use impl block to implement trait for X
unsafe impl Send for X {}
unsafe impl Sync for X {}

#[allow(unused)]
fn changed_main_one() {
    use std::rc::Rc;
    #[allow(unused_variables)]
    let x = Rc::new(50);

    // thread::spawn requires type to be Send.
    /*
     thread::spawn(move || {
         // err: Rc is not thread safe.
         println!("{x:?}");
     });
    */
}

// Mutex : Mutual Exclusion
// Mutex allows to share mutable data between threads.

// Mutex has 2 states : locked and unlocked. A thread can lock a mutex and until that thread is not
// unlocked other threads will just block.
#[allow(unused)]
fn changed_main_two() {
    use std::sync::Mutex;

    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                // lock returns a MutexGuard that represents the guarantee that we have locked the
                // mutx.
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // dropping a guard means unlocking the mutex.
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
}

// Thread Parking: It is a way to wait for notification from another thread. A thread can park
// itself, which puts it into sleep, stopping from consuming any CPU Cycle. Another thread can
// unpark it by waking it up from sleep.
#[allow(unused)]
fn changed_main_three() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // consumer
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // producer.
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1))
        }
    });
}

// Conditional Variable: These variables have basic two operations, wait and notify. Threads can
// wait on a condition variable, afeter which they can be woken up when another thread notifies
// that same condition variable.

// We can create a condition variable for specific events or conditions in which we are interested
// in and wait on that condition. Any thread that causes that event or condition to happen notifies
// the condition varibale.
fn main() {
    use std::sync::Condvar;

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // consumer
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    // wait for the not_empty condition
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        // producer
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            // notify that the queue is not empty.
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
