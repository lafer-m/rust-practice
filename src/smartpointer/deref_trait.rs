use std::ops::Deref;

struct Mybox<T>(T);

impl <T> Mybox<T> {
    fn new(v: T) -> Mybox<T> {
        Mybox(v)
    }
}

impl<T> Deref for Mybox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_deref() {
    let l = Mybox::new(12);
    println!("{}", *l);  // *l == *(l.deref()) == *(&l.0)
    let l1 = Mybox::new(String::from("world"));
    hello(&l1) // can convert Mybox to &str slice , because Mybox impl the Deref trait, &l1== &(*l1)[..]
}


fn hello(s: &str) {
    println!("hello {}!", s);
}