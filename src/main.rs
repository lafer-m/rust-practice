
mod guessgame;
use guessgame::guess;
mod datatypes;
use datatypes::triple;
use datatypes::arr;
use datatypes::plusx2;
use datatypes::first_world;

fn main() {
//    guess();
   let s = String::from("he llo ");
   println!("{} {}", plusx2(12, 13), first_world(&s));
   arr();
   triple();
}
