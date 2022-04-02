use std::mem::drop;
#[derive(Debug)]
struct Custom {
    data: String
}


impl Drop for Custom {
    fn drop(&mut self) {
        println!("drop the custom data {}", self.data)
    }
}


#[test]
fn test_drop_trait() {
   {
    let c = Custom {data:String::from("test")};
    println!("create the custom data {:?}",c)
   }                                            // will drop the custom data
   let c1 = Custom {data:String::from("test1")};
   println!("create the custom data {:?}",c1)
}


#[test]
fn test_manul_drop() {
    let c = Custom {data:String::from("test")};
    println!("create the custom data {:?}",c);
    drop(c);
    println!("custom already dropped")
}