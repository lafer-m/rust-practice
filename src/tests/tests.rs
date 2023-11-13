

use regex::Regex;


#[test]
fn test_hello() {
    println!("hello, world");
    assert_eq!(4, 2+2)
}

#[test]
fn reg_test() {
    let name_pattern = Regex::new(r"^([a-zA-Z0-9\u4E00-\u9FFF-]+)$").unwrap();
    let name = "ahhdhawkdwak-中文a";
    let name1 = "000d=adwadw";
    assert!(name_pattern.is_match(name));
    assert!(!name_pattern.is_match(name1));
    let chinese_pattern = Regex::new(r"^[a-zA-Z0-9\u4E00-\u9FFF-]+$").unwrap();

    let strings = vec![
        "Hello-你好",   // Valid (hyphen is in the middle)
        "Hello你好-",   // Valid (hyphen is in the middle)
        "-Hello你好",   // Invalid (hyphen at the beginning)
        "Hello你好-",   // Invalid (hyphen at the end)
        "Hello你-好",   // Valid (hyphen is in the middle)
        "-Hello你好-",  // Invalid (hyphen at the beginning and end)
    ];

    for s in strings {
        if chinese_pattern.is_match(s) && !s.starts_with('-') && !s.ends_with('-') {
            println!("Valid: {}", s);
        } else {
            println!("Invalid: {}", s);
        }
    }
}
