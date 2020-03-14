use std::fmt;

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f32,
    y: f32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{real: {}, imag: {}}}", self.real, self.imag)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Display: {0}\nDebug: {0:?}", MinMax(0, 1));

    println!("");
    println!("Display: {0}\nDebug: {0:?}", Point2D { x: 0.0, y: 1.0 });

    println!("");
    println!(
        "Display: {0}\nDebug: {0:?}",
        Complex {
            real: 1.2,
            imag: 2.4
        }
    );

    println!("");
    println!("Display: {0}\nDebug: {0:?}", List(vec![1, 2, 3]));
}
