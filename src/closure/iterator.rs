

#[test]
fn test_iterator() {
    let i = vec![1,23,7];

    let mut iter = i.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    let total: i32 = iter.sum();   // this will be zero, because iter has been consume down.
    println!("{}", total);

    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x|x+1).collect();
    assert_eq!(v2, vec![2,3,4])
}





#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}


fn find_shoe_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[test]
fn test_shoesize() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("a"),
        },
        Shoe {
            size: 11,
            style: String::from("b"),
        },
        Shoe {
            size: 12,
            style: String::from("c"),
        },
        Shoe {
            size: 10,
            style: String::from("d"),
        },
    ];

    let sizeShoes = find_shoe_size(shoes, 10);
    println!("{:?}",sizeShoes);
    assert_eq!(sizeShoes, vec![
        Shoe {
            size: 10,
            style: String::from("a"),
        },
        Shoe {
            size: 10,
            style: String::from("d"),
        },
    ])
}