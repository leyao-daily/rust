#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 300, height: 500 };
    let rect2 = Rectangle { width: 100, height: 400 };
    let rect3 = Rectangle { width: 600, height: 450 };
    let rect4 = Rectangle::square(350);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle 4 is {} square pixels.",
        rect4.area()
    );


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

