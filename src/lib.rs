
pub mod errors;
pub mod guessgame;
pub mod datatypes;
pub mod generic;
pub mod closure;
#[cfg(test)] // only run test when run cargo test, not running test when build.
mod tests;


pub mod document;
mod smartpointer;
mod concurrency;
mod oop;
mod pattern;
mod unsafely;
mod macros;
pub use document::doc::add_one;
