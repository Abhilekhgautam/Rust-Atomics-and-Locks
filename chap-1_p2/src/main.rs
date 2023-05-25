// we often need to share data, to ensure shared data gets dropped and
// deallocated, we can't completly give its ownership, instead we can
// share ownership.

#[allow(unused)]
fn changed_main_one() {
    // Rust standard helps to create a reference counted variable.
    // When we clone such variable an internal count increases,
    // the value is dropped only when the count is 0.
    use std::rc::Rc;

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    // they all refer to  same memory location
    println!("{:?}", a.as_ptr());
    println!("{:?}", b.as_ptr());
}

// Rc are not thread safe, i.e., we cannot pass Rc within threads
// If multiple threads had an Rc to same allocation and both of them might try
// to update the reference counter, which results in unpredicatble result
#[allow(unused)]
fn changed_main_two() {
    // Arc are similar to Rc except they are thread safe
    // Arc stands for Atomic Reference count, which means that any modification to refernce counter
    // is an indivisible (atomic) operation, making it safe to use with multiple thread.
    use std::sync::Arc;
    use std::thread;

    let a = Arc::new([2, 4, 6]);
    // naming a clone is a difficult thing
    //let b = a.clone();

    // refers to same location
    println!("{:?}", a.as_ptr());
    //println!("{:?}", b.as_ptr());

    thread::spawn({
        let a = a.clone();
        move || dbg!(a)
    })
    .join()
    .unwrap();
}

// Rust's borrowing rule can be slightly limiting, especially when dealing with multiple threads
// but we have a work around for that, interior mutablity

// A Cell<T> wraps a T, but allows mutation through shared reference
// Getting a borrow off a value of a Cell is not a easy task

// Cell<T> is not thread safe
#[allow(unused)]
fn changed_main_three() {
    use std::cell::Cell;

    // a is immutable
    let a = Cell::new(2);
    println!("Before Mutating: {a:?}");
    // but we can mutate it
    a.set(3);
    println!("After Mutating: {a:?}");

    // Cell<T> is not easy to deal with as it doesn't allow to directly borrow the value it holds

    // a_borrowed holds the value held by a previously.
    let mut a_borrowed = a.take(); // a is set to Default::default()
    println!("a is now : {a:?}");
    println!("a previously held: {a_borrowed}");
    a_borrowed = 5;
    a.set(a_borrowed); // set a to a new value
}

fn main() {}
