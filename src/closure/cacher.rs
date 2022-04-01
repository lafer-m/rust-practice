


//store a closure in struct
struct Cacher<T>
where T: Fn(u32) -> u32   // the trait bound to the T is a closure func that have a u32 param and return a u32 value.
                          // it can store any closure that match Fn(u32) -> u32
{
    calcul: T,
    value: Option<u32>,
}


#[test]
fn test_closuer() {
    let x = String::from("abc");
    {
        let eq_x = |z| {z==x};  //  will capture the x value in the same scope, borrows x immutably.
        let y = String::from("abc");
        println!("{}", eq_x(y));
    }

    println!("{}", x);
}


// FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, 
    // the closure must take ownership of these variables and move them into the closure when it is defined. 
    // The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
// FnMut can change the environment because it mutably borrows values.
// Fn borrows values from the environment immutably.

#[test]
fn test_ownership_closuer() {
    let x = String::from("abc");
    {
        let eq_x = move |z| z==x;   // move x's value ownership to the closure
        println!("{}" , eq_x("cde"));
    }
    // println!("{}", x); // x will be take owner to the closure
}


