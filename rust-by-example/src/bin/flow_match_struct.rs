fn main() {
    struct Person<'a> {
        fullname: (&'a str, &'a str),
        age: u32,
        country: &'a str,
    };

    let bob = Person {
        fullname: ("Bob", "Anderson"),
        age: 32,
        country: "Japan",
    };

    match bob {
        Person { age, .. } if age < 40 => println!("Oh, you are younger than me!"),
        Person {
            country: "Japan", ..
        } => println!("I come from Japan too!"),
        _ => println!("Hello, {}", bob.fullname.0),
    }
}
