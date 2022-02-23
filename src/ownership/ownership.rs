
fn test() {
    let s = String::from("hello");
    refer(&s);  //

    let mut s1 = String::from("hhh");
    refermut(&mut s1);  // workings // 不能同时拥有多个mut的refer, 当有不可变的引用的时候，也不能在引用mut的。
}


fn refer(str: &String) {
    str.push_str("test") // 因为不是可变的引用，所以不可以修改，编译错误
}

fn refermut(str: &mut String) {
    str.push_str("hahha") // this is ok!
}


fn dangle_pointer() -> &String {
    let s = String::from("dangle");
    &s
} // s将会被drop，所以这里会编译报错，引用的内存已经不存在

fn dangle_pointer_ok() -> String {  // 直接return s 本身即可
    let s = String::from("dangle");
    s
}