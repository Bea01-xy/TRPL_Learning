#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width : size,
            height : size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30, 
        height: 20, 
    };
    let rect2 = Rectangle{
        width: 40, 
        height: 30, 
    };
    let rect3 = Rectangle::square(3);
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));
    //println!("rect1 = {rect1:?}");
    //println!("{}", rect1.area());
}
