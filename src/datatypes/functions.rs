


pub fn plusx2(a: i32, b: i32) -> i32 {
    let mut result =  plus(a, b);
    result = if result > 10 { 10 } else {result};
    result*2
}

fn plus(a: i32, b: i32) ->i32  {
    // this is a statement
    println!("{}{}", a, b );
    // this is a return value 
    a+b
}