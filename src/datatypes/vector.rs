


// vector can only store the values of the same type
fn vectors() {
    let mut v = Vec::new(); // new with no type annotation
    v.push(1);
    v.push(2);
    {
        let mut v1 = vec![1,2,3]; // new with init, use macro
        v1.push(4);
        println!("{}", v1);
        // &v1[100];   this will panic
        let fourth: &i32 = &v1[3];
        match v1.get(3) {
            None => println!("no fourth element"),
            Some(fourth) => println!("the fourth element is {}", fourth),
        }
        for i in &mut v1 { // must be mut refer
            *i += 50;   // every element add 50. * is the derefer for change the value.
        }



    } // v1 will be droped ,and it's elements.
    
    println!("{}", v);
}


// use enum to store diff types
fn multi_type_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("string")),
    ];

}