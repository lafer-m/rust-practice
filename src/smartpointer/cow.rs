/// A clone-on-write smart pointer. cow
/// 
/// 


#[cfg(test)]
mod test {
    use std::borrow::Cow;
    #[test]
    fn test_cow() {
        let s = "hello, world!";
        let mut bor = Cow::Borrowed(s);
        assert!(bor.is_borrowed()); // borrowed 
        let owned = bor.to_owned();
        assert!(owned.is_borrowed());
        let x = bor.to_mut(); // copy on write
        // x.push('x');
        assert_eq!(x.to_owned(), String::from(s));
    }
}

