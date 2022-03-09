
mod guessgame;
use guessgame::guess;
mod datatypes;
use datatypes::triple;
use datatypes::arr;
use datatypes::plusx2;
use datatypes::first_world;
use datatypes::first_world_str;
use datatypes::area;
use datatypes::Rectangle;

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
