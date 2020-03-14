use std::fmt;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Rectangle {
        return Rectangle {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
        };
    }

    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        (2.0 * (x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, delta_x: f64, delta_y: f64) {
        self.p1.x += delta_x;
        self.p1.y += delta_y;

        self.p2.x += delta_x;
        self.p2.y += delta_y;
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect[{}, {}]", self.p1, self.p2)
    }
}

fn main() {
    let rect1 = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("{}", rect1);
    println!(
        "\tArea: {}\n\tPerimeter: {}",
        rect1.area(),
        rect1.perimeter()
    );

    println!();

    let mut rect2 = Rectangle::new(10.0, 10.0, 30.0, 30.0);
    println!("{}", rect2);
    println!(
        "\tArea: {}\n\tPerimeter: {}",
        rect2.area(),
        rect2.perimeter()
    );
    rect2.translate(-10.0, -10.0);
    println!("After translate (-10.0, -10.0): {}", rect2);
}
