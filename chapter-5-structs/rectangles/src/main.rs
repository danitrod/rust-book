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
        width: 50,
        height: 22,
    };

    let rect3 = Rectangle {
        width: 23,
        height: 10,
    };

    let square = Rectangle::square(5);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("Prettified rect is {:#?}", rect1);
    println!("Debug rect is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels. [Using method]",
        rect1.area()
    );
    println!(
        "rect1 can hold rect2: {}\nrect1 can hold rect3: {}\nrect1 can hold square: {}",
        rect1.can_hold(&rect2),
        rect1.can_hold(&rect3),
        rect1.can_hold(&square)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
