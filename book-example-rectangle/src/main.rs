#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct AdvancedRectangle {
    width: u32,
    height: u32,
}

impl AdvancedRectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, other: &AdvancedRectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square_builder(size: u32) -> AdvancedRectangle {
        AdvancedRectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area_params(width, height)
    );

    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect)
    );

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rectangle)
    );
    println!("Rect is {:#?}", rectangle);

    let advanced_rectangle = AdvancedRectangle {
        width: 30,
        height: 50,
    };
    let other_rectangle = AdvancedRectangle {
        width: 25,
        height: 40,
    };
    let big_rectangle = AdvancedRectangle {
        width: 100,
        height: 150,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        advanced_rectangle.area()
    );
    println!("Rect is {:#?}", advanced_rectangle);
    println!(
        "Can {:#?} fit inside {:#?}? {}",
        other_rectangle,
        advanced_rectangle,
        advanced_rectangle.can_fit(&other_rectangle)
    );
    println!(
        "Can {:#?} fit inside {:#?}? {}",
        big_rectangle,
        advanced_rectangle,
        advanced_rectangle.can_fit(&big_rectangle)
    );

    let square = AdvancedRectangle::square_builder(25);
    println!("The new square is {:#?}", square);
}

fn area_params(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
