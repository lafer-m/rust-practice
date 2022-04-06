mod array;
mod truple;
mod functions;
mod slice;
mod structa;
mod trait_object;
pub use truple::triple;
pub use array::arr;
pub use functions::plusx2;
pub use slice::first_world;
pub use slice::first_world_str;
pub use truple::area;
pub use structa::Rectangle;

// 引用其他内部模块
use super::guessgame::hello;
use super::guessgame::guess;

