// this give the struct the Debug trait
#[derive(Debug)]
struct Rectangle {
    height: f32,
    width: f32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0.0
    }

    fn height(&self) -> bool {
        self.height > 0.0
    }

    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 12.5,
        width: 4.5,
    };

    let rec2 = Rectangle {
        height: 15.2,
        width: 10.4,
    };

    if rec1.height() && rec1.width() {
        println!("Area is: {}", rec1.area());
        println!("Can rect1 hold rect2: {}", rec1.can_fit(&rec2));
    }

    println!("{rec1:#?}");

    // this takes owenership of the expression, hence passing a reference.
    dbg!(&rec1);

    // let area = area(&rec1);
    // println!("Area is: {}", area);
}

// fn area(rectangle: &Rectangle) -> f32 {
//     rectangle.height * rectangle.width
// }
