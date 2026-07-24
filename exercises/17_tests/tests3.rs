struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        // Returning a `Result` would be better here. But we want to learn
        // how to test functions that can panic.
        // if width <= 0 && height <= 0 {
        //     panic!("Rectangle width and height must be positive");
        // } else if width <= 0 {
        //     panic!("Rectangle width must be positive");
        // } else if height <= 0 {
        //     panic!("Rectangle height must be positive");
        // } else {
        //     Rectangle { width, height }
        // }

        // match (width, height) {
        //     (..1, ..1) => panic!("Rectangle width and height must be positive"),
        //     (..1, _) => panic!("Rectangle width must be positive"),
        //     (_, ..1) => panic!("Rectangle height must be positive"),
        //     (_, _) => Rectangle { width, height },
        // }

        match (width, height) {
            (1.., 1..) => Rectangle { width, height },
            (1.., _) => panic!("Rectangle height must be positive"),
            (_, 1..) => panic!("Rectangle width must be positive"),
            (_, _) => panic!("Rectangle width and height must be positive"),
        }

        // match (width > 0, height > 0) {
        //     (true, true) => Rectangle { width, height },
        //     (true, false) => panic!("Rectangle height must be positive"),
        //     (false, true) => panic!("Rectangle width must be positive"),
        //     (false, false) => panic!("Rectangle width and height must be positive"),
        // }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: This test should check if the rectangle has the size that we
        // pass to its constructor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative width.
    #[test]
    #[should_panic(expected = "Rectangle width must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }
    
    #[test]
    #[should_panic(expected = "Rectangle width must be positive")]
    fn zero_width() {
        let _rect = Rectangle::new(0, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle height must be positive")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }

    #[test]
    #[should_panic(expected = "Rectangle height must be positive")]
    fn zero_height() {
        let _rect = Rectangle::new(10, 0);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn zero_both() {
        let _rect = Rectangle::new(0, 0);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_both() {
        let _rect = Rectangle::new(-30, -10);
    }
}
