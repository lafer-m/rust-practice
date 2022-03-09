



struct user {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool
}


// 三种 普通的struct , tripple struct用于实现区分不同类型的tripple , 没有任何数据的struct用于实现trait
// struct同样是有作用范围的。


#[derive(Debug)]  // define the debug trait
pub struct Rectangle {
  pub  width: u32,
  pub  height: u32,
}


pub fn area(angle: &Rectangle) -> u32 {
    println!("{:?}", angle);
    angle.width * angle.height
}


impl Rectangle {
   pub fn area(&self) -> u32 {
        self.width * self.height
    }
    // public method 
   pub fn width(&self) -> u32 {
        self.width + self.lenthpri()
    } 

    // private method
    fn lenthpri(&self) -> u32 {
        self.height
    }
}