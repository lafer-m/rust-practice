

// not recoverable, just panic!("panic") macro
// panic!("test");
// recoverable error retun Result, T\E are generic type
// enum Result<T, E> {
//     Ok(T),
//     Err(E), 
// }
use std::fs::File;

pub fn open_file()  {
    let f = File::open("test.txt");
    let _ = match f {
        Ok(file) => file,
        Err(_) => panic!("open the test.txt err"),
    };
}




