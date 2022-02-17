// 定义内部模块
mod truple;
// 可以被引用
pub use truple::triple;

// 引用其他内部模块
use super::guess;
use super::guessgame::hello;