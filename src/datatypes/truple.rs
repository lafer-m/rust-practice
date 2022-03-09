use super::guess;
use super::hello;


// calc area
pub fn area(aa: (u32,u32)) -> u32 {
    aa.0 * aa.1
}

pub fn triple() {
    println!("just a mod testing");
    hello();
    guess();
    real_triple();
}

fn real_triple() {
    let t = ("abc", 1, 2.2);
    let (a , b , c) = t;
    println!("{},{},{}",a, b, c);
    println!("{}{}{}", t.0, t.1, t.2);
}