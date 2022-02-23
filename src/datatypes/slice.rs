
pub fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i , &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return  i;
        }
    }
    s.len()
}