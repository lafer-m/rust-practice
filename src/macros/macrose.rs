
// get the value in Option or return None
macro_rules! opt {
    ($x:expr) => {
        {
            match ($x) {
                Some(v) => v,
                None => return None,
            }
        }
    };
}

#[test]
fn test_opt() {
    assert_eq!(17, opt_t().unwrap());
}


fn opt_t() -> Option<u16> {
    let val = opt!(Some(16));
    return Some(val+1);
}