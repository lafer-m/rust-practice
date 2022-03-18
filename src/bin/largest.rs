extern crate rust_practice;

use rust_practice::generic;



fn main() {
    let a = vec![1,2,3,5,6,7];
    let b = vec!["abc", "cssd"];
    // let mut c: Vec<String> = Vec::new();  // if use largest(&c), will not compile, becase the Copy trait limit, String not impl Copy trait
    println!("a: {}, b: {}", generic::largest(&a), generic::largest(&b));
    ()
}