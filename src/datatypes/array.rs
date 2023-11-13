

pub fn arr() {
    let test: [i32; 5] = [1,2,3,4,5];
    for i in test {
        if i != 2 {
            println!("{}",i);
        }
    }
    let mut index = 0;
    while index < 5 {
        println!("{}", test[index]);
        index += 1;
    }
}


/// ?Sized not sized type, such as i32 u32
fn type_limit<T: ?Sized>(a: &T) {}

#[test]
fn test_l() {
    let a = 0;
    type_limit(&a);
}