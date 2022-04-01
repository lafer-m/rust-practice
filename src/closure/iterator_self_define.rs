
struct Counter {
    count: u32
}

impl Counter {
    fn new() ->Counter {
        Counter {
            count:0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}




#[test]
fn test_iterator() {
    let cnt = Counter::new();
    for i in cnt {
        println!("{}", i)
        // assert_eq!(i, i+1)
    }
}