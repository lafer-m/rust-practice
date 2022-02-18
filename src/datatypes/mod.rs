mod array;
mod truple;
pub use truple::triple;
pub use array::arr;

// 引用其他内部模块
use super::guess;
use super::guessgame::hello;