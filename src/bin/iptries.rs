use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::thread;

struct B {
    value: i32,
}

struct A {
    b_ref: Arc<RefCell<B>>,
    // Other fields in struct A
}

struct C {
    b_ref: Arc<RefCell<B>>,
    // Other fields in struct C
}

fn main() {
    // Create an instance of B wrapped in RefCell and Arc
    let b_instance = Arc::new(RefCell::new(B { value: 42 }));

    // Clone the Arc for A and C
    let a_instance = A { b_ref: Arc::clone(&b_instance) };
    let c_instance = C { b_ref: Arc::clone(&b_instance) };

    // Change B's value through A and C
    a_instance.b_ref.borrow_mut().value = 50;
    c_instance.b_ref.borrow_mut().value = 60;

    // Spawn a new thread and modify B's value through C
    let b_clone = Arc::clone(&b_instance);
    thread::spawn(move || {
        b_clone.borrow_mut().value = 70;
    })
    .join()
    .unwrap();

    // Access B's value through A and C
    println!("A's B value: {}", a_instance.b_ref.borrow().value);
    println!("C's B value: {}", c_instance.b_ref.borrow().value);
}
