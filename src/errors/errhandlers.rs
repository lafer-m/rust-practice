

// not recoverable, just panic!("panic") macro
// panic!("test");
// recoverable error retun Result, T\E are generic type
// enum Result<T, E> {
//     Ok(T),
//     Err(E), 
// }
use std::fs::File;
use std::io::{self,ErrorKind, Read};

pub fn open_file()  {
    let f = File::open("test.txt");
    let _ = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("err {:?}", err),
            },
            other => panic!("err {:?}", other),
        }
    };
}

fn test() {
    // failed open panic, otherwise  return the file.
    let _ = File::open("test.txt").expect("failed to open the test.txt file");
}

fn readusername()  -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? like a suger
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}




