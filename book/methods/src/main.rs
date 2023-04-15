#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
    let rectangles = [&rect1, &rect2, &rect3];

    for elem in rectangles {
        println!(
            "The area of rectangle is {} square pixels.",
            elem.area()
        );
        if elem.width() {
            println!(
                "The rectangle has a nonzero width; it is {}", elem.width
            );
        }
    }
    println!("Can rect1 hold rect2? {}", rectangles[0].can_hold(&rectangles[1]));
    println!("Can rect1 hold rect3? {}", rectangles[0].can_hold(&rectangles[2]));
}

