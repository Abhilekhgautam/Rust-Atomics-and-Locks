// ID Allocation

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[allow(unused)]
fn allocate_new_id() -> u32 {
    // NEXT_ID is not always zero, although it is inside a fn block
    // it behaves similar to as if it was declared outside the fn block.
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    // increments the stored number and returns the previous number
    let id = NEXT_ID.fetch_add(1, Relaxed);
    // panics if more than 1000 id's are generated
    // this panics after the NEXT_ID has been incremented
    // if threads keep calling this fn it will ultimately overflow
    // after some number of panics.
    assert!(id < 1000, "too many IDs!");
    id
}

// Compare and Exchange Operations: This operation checks if the atomic value is
// equal to given value, and only if that is the case it replace it with a new value,
// all atomically as a single operation and return the previous value.

/// ```rust
///  impl AtomicI32{
///     pub fn compare_exchange(
///      &self,
///      expected: i32,
///      new: i32,
///      success_order: Ordering,
///      failure_order: Ordering,
///     ) -> Result<i32, i32>
///     {
///       // get the current value
///       let v = self.load();
///
///       if v == expected {
///        // value is as expected, replace it and report success_order
///        self.store(new);
///        Ok(v)
///       } else{
///          // The value was not as expected.
///          // Report Failure
///          Err(v)
///       }
///     
///     }
///  }
/// ```
///

// Incrementing a AtomicU32 without using fetch_add.

#[allow(unused)]
fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        // compare if the current value is same as we loaded, store the new value
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            // Other thread changed the value after we loaded
            Err(v) => current = v,
        }
    }
}

#[allow(unused)]
fn allocate_new_id_updated() -> u32 {
    static NEW_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEW_ID.load(Relaxed);

    loop {
        assert!(id < 1000, "too many ID's generated");
        match NEW_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

// this function panics
fn main() {
    use std::thread;
    use std::time::Duration;
    // endlessly call the function allocate_new_id_updated
    // which panics if called for more than 1000 times
    thread::scope(|s| {
        s.spawn(|| loop {
            let generated_id = allocate_new_id_updated();
            println!("Newly generated_id : {generated_id}");
            thread::sleep(Duration::from_millis(100));
        });

        loop {
            let generated_id = allocate_new_id_updated();
            println!("Newly generated_id : {generated_id}");
            thread::sleep(Duration::from_millis(100));
        }
    });
}
