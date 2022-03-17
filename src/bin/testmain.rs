extern crate rust_practice;

// use rust_practice::guessgame::guess;
use rust_practice::datatypes::triple;
use rust_practice::datatypes::arr;
use rust_practice::datatypes::plusx2;
use rust_practice::datatypes::first_world;
use rust_practice::datatypes::first_world_str;
use rust_practice::datatypes::area;
use rust_practice::datatypes::Rectangle;


fn main() {
//    guess();
   let rect = Rectangle{
      width: 5,
      height: 6
   };
   let s = String::from("he llo ");
   println!("{} {} {} {} {:#?} {} {}",area((18,20)), plusx2(12, 13), first_world(&s), first_world_str(&s), rect, rect.area(), rect.width());
   arr();
   triple();
}
