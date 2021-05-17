
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height:size}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 20, height: 60};
    let squar = Rectangle::square(50);
    println!("rect1 is {:?}.",rect1);
    println!("And its area is {} square pixels",rect1.area());
    println!("is {} that rect1 can hold rect2",
             rect1.can_hold(&rect2));
    println!("another rectangle is the {:?}",squar);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
