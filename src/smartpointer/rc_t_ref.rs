

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use List::{Cons,Nil};

#[test]
fn test_box_smart_pointer() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2,Rc::new(Nil)))));
    let b = Rc::new(Cons(0, Rc::clone(&a)));        // Rc::clone only increments the reference count, no deep-copy happened
    {
        let c = Rc::new(Cons(-1, Rc::clone(&a)));
        println!("ref counts: {}", Rc::strong_count(&a))   // 3 ref counts
    }
    println!("ref counts: {}", Rc::strong_count(&a));      // 2 ref counts, when c go out of the scope , a's ref counts will reduce one.
}