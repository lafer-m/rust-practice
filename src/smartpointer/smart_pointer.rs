
// use box store a value on the heap, not on the stack.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons,Nil};

#[test]
fn test_box_smart_pointer() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let l :u32 = 1 << 32 -1;
    // let l1 :u32 = 1<<32 -1;
    // println!("{}", l+l1);
    println!("{:?}", list)
}