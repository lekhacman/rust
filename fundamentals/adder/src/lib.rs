#[cfg(test)]
mod tests {
    use crate::Rectangle;
    use crate::Guess;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7};
        let smaller = Rectangle { length: 5, width: 1};

        assert!(
            larger.can_hold(&smaller),
            "How come smaller can hold a larger?"
        )
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn new_guess() {
        Guess::new(200);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value : i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {value}
    }
}
