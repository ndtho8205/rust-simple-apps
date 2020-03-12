use std::convert::TryFrom;

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl TryFrom<i32> for Circle {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("radius cannot be smaller than zero")
        } else {
            Ok(Circle {
                radius: value as f32,
            })
        }
    }
}

fn main() {
    println!("{:?}", Circle { radius: 10.0 });

    println!("{:?}", Circle::try_from(20));

    println!("{:?}", Circle::try_from(-1));
}
