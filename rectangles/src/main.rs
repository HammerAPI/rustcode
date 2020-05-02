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
    let width1: u32 = 30;
    let height1: u32 = 50;
    println!("\nUsing two variables:");
    println!("The area of the rectangle is {} units", area_sides(width1, height1));

    let rect1 = (30, 50);
    println!("\nUsing a tuple:");
    println!("The area of the rectangle is {} units", area_tuple(rect1));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\nUsing a struct");
    println!("The area of the rectangle is {} units", area_struct(&rect));
    println!("The area of the rectangle is {} units", rect.area());
    println!("Unformatted print: {:?}", rect);
    println!("Formatted print: {:#?}", rect);

    let small_rect = Rectangle {
        width: 20,
        height: 40,
    };
    println!("Can rect hold small_rect? {}", rect.can_hold(&small_rect));
    println!("Can small_rect hold rect? {}", small_rect.can_hold(&rect));


    let sq = Rectangle::square(3);
    println!("Square is: {:#?}", sq);
}

fn area_sides(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
