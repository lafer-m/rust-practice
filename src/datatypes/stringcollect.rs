


// strings is implemented as a collection of bytes
// String::new collection or &str (a string slice)




fn string_col() {
    let mut s = "hello".to_string();
    s.push_str(", world");
    let s1 = String::from("h");
    let s2 = String::from("ello");
    let s3 = String::from(", world");

    // let s4 = s1 + "jjadw" + &s2; // after this, s1 will be moved and invalid.

    // let s5 = s1 + &s2 +&s3; // can't compile, because s1 is invalid

    let s6 = format!("{}{} {}", s1,s2,s3);

    // can't compile , String can't index;
    let h = s6[0];
    // String is a wrapper of a Vec<u8> in memory;
    //can iterate
    s6.bytes();
    s6.chars();

}