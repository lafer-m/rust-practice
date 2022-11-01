
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

use std::{slice, ops::Add};



// raw pointer
// *const T and *mut T

#[test]
fn test_unsafe() {

    unsafe {
        unsafefunc();
    }

}


fn split_slice(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // get the raw *mut T pointer
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
}


unsafe fn unsafefunc() {}

