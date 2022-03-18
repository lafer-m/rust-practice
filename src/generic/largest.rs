
use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larg = list[0];
    for &item in list {
        if item > larg {
            larg = item;
        }
    }
    larg
}