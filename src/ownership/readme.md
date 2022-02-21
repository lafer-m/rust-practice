ownership理解

这种叫做move s1 to s2, 只会在栈上复制，堆上不复制，也就是都指向相同的堆内存地址，rust同时会使s1变为invalid，从而避免了释放两遍相同的堆内存， 所以以下代码打印s1是非法的；
当scope结束的时候，rust会自动调用drop，释放s2的堆内存；
```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```


当要尝试在堆上复制的时候，需要调用通用的.clone函数。
```
let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

如下这种代码又是可行的，是因为像intger这种数据，是直接存在栈上的，所以，栈上复制就行了，x也还是有效的。
```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

 如果一个type实现了Copy trait(特性)，将一个变量赋予另外一个后，这个变量就还是有效的可用的;  
 如果一个type或其中的一部分实现Dopy trait， 那么它将不被允许实现Copy trait;  

实现了Copy trait的类型： 
 ```
All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

 ```

ownership 的概念
 ```
 fn main() {
    let s = String::from("hello");  // s 进入生效域

    takes_ownership(s);             // s move进 函数中
                                    // 这个时候s就是无效的了

    let x = 5;                      // x 进入生效域

    makes_copy(x);                  // x move进函数中
                                    // 但是i32是Copy trait, 所以在函数之后x还是有效的

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
 ```