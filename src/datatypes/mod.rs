mod array;
mod truple;
mod functions;
mod slice;
pub use truple::triple;
pub use array::arr;
pub use functions::plusx2;
pub use slice::first_world;

// 引用其他内部模块
use super::guess;
use super::guessgame::hello;