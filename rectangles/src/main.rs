#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn max(self, other: Self) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let mut rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };
    let square1: Rectangle = Rectangle::square(15);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect is {rect1:#?}");
    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    dbg!(&square1);

    let area1: u32 = rect3.area();
    let area2: u32 = Rectangle::area(&rect3);
    assert_eq!(area1, area2);

    rect3.set_width(2);
    Rectangle::set_width(&mut rect3, 2);

    let rect4: Rectangle = Rectangle {
        width: 0,
        height: 0,
    };

    println!("{}", rect4.area());

    let other_rect: Rectangle = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect: Rectangle = rect4.max(other_rect);

    dbg!(&max_rect);
}
