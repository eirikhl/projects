// Necessary to print the struct Rectangle with println
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // The syntax {:#?} tells the compiler to print it in debug mode I assume
    println!("rect1 is {:#?}", rect1);

    println!("Area of rect1: {}", rect1.area());

    let rect2 = Rectangle {width: 25, height: 45};
    let rect3 = Rectangle {width: 35, height: 55};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(35);
    println!("{}", rect4.can_hold(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        (rect2.width <= self.width) && (rect2.height <= self.height)
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
