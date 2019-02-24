fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    println!("rect1 is {:#?}", rect1);

    assert_eq!(rect1.area(), 1500);
    assert_eq!(rect1.can_hold(&rect2), true);
    assert_eq!(rect1.can_hold(&rect3), false);

    println!("{:#?}", Rectangle::square(30));
}

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
        Rectangle { width: size, height: size }
    }
}
