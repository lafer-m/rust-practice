use super::guess;
use super::hello;

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
}