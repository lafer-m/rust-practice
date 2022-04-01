

#[test]
fn test_iterator() {
    let i = vec![1,23,7];

    let mut iter = i.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    let total: i32 = iter.sum();   // this will be zero, because iter has been consume down.
    println!("{}", total) 

}


