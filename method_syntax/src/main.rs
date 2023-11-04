#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }

    fn get_width(&self) -> u32 { self.width }

    fn can_hold(&self, rect: &Rectangle) -> bool { 
        (self.width >= rect.width) && (self.height >= rect.height)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square1 = Rectangle::square(50);

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    println!("The width of the rectangle is {}.", rect1.get_width());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("You got a square, it's size is {}.", square1.area());
}
