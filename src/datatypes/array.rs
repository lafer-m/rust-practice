

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