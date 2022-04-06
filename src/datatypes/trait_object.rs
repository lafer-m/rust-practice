// For example, given a trait Trait, the following are all trait objects:

// dyn Trait
// dyn Trait + Send
// dyn Trait + Send + Sync
// dyn Trait + 'static
// dyn Trait + Send + 'static
// dyn Trait +
// dyn 'static + Trait.
// dyn (Trait)


pub trait Draw {
    fn draw(&self); 
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter(){
            c.draw();
        } 
    }
}


pub struct Button {
    a: u32,
    b: u32,
}

impl Draw for Button {  // impl Draw trait
    fn draw(&self) {
        println!("button")
    }
}

pub struct Select {   // impl Draw trait
    a: u8,
    b: i8,
}

impl Draw for Select {
    fn draw(&self) {
        println!("select")
    }
}


#[test]
fn test_trait_object() {
    let b = Button{a:1,b:2};
    let s = Select{a:1,b:2};
    let mut v: Vec<Box<dyn Draw>> = Vec::new();
    v.push(Box::new(b));
    v.push(Box::new(s));
    let l = Screen {
        components: v,
    };
    l.run();
}