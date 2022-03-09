
pub fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i , &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return  i;
        }
    }
    s.len()
}

pub fn first_world_str(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return &s[..i]; // slice to 0-i
        }
    }
    &s[..]   // slice to 0 - len
}
