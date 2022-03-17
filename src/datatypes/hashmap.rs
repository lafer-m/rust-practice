
// hash map
// HashMap<K,V>

use std::collection::HashMap;


fn hash_map() {
    let st = String::from("adawd");
    let mut test = HashMap::new();
    test.insert(st, 10);// st will be invaliad at this point. st was moved into the map.

    let t1 = vec!("a".to_string(),"v".to_string());
    let v1 = vec!(1,2);
    // create hash map from two list
    let mut test1: HashMap<_,_> = t1.into_iter().zip(v1.into_iter()).collect();
    test1.insert(String::from("a"), 10);// overwrite the old key's value
    test1.entry(String::from("a")).or_insert(20); // if key a is not exist, insert it


    let key = String::from("a");
    let value = test1.get(&key);
    println!("key: {} value: {}", key,value );
    match value {
        Some(v) => println!("{}", v),
        None => ()
    }

    for (k,v) in test1 {
        println!("{}: {}", k, v);
    }
}


fn map_count() {
    let test = "hello world good world";
    let mut m = HashMap::new();
    for w in test.split_whitespace() {
        let count = m.entry(w).or_insert(0);
        *count += 1;
    }
    println!("{:?}" , m );
}