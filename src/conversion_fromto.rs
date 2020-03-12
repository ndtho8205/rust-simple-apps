use std::convert::{From, TryFrom};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct EvenNumber {
    value: i32,
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item % 2 == 0 {
            Ok(EvenNumber { value: item })
        } else {
            Err(())
        }
    }
}

fn main() {
    let num = Number::from(30);
    println!("Number: {:?}", num);

    let new_num: Number = 40.into();
    println!("New number: {:?}", new_num);

    let even_num = EvenNumber::try_from(50);
    println!(
        "Err? {} - Even number: {:?}",
        even_num.is_err(),
        even_num.unwrap()
    );

    let even_num = EvenNumber::try_from(55);
    println!("Err? {} - Even number: {:?}", even_num.is_err(), even_num);
}
