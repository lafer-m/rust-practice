
use std::thread;

#[test]
fn test_thread() {
    let v = vec![1,2,3,4,5];
    let handler = thread::spawn(move || {
        let mut it = v.iter().map(|x| {
            println!("spawn x: {}", x);
        });
        it.next();
        it.next();
        it.next();
        it.next();
        it.next();
    });
    println!("this is main thread!");
    handler.join().unwrap();
}