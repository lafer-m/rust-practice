


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn match_other(x: u8) -> u8 {
    match x {
        7 => 1,
        6 => 1,
        // other => func(other),
        _ => 0, // this will match all other values
    }
}