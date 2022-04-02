

/**
# example
add one value to another
```
let arg = 5;
let answer = rust_practice::add_one(arg);
```

# panics
description panic situations

# errors
descript error types

# safety
why unsafe to call
*/
pub fn add_one(x: i32) -> i32 {
    x + 1
}