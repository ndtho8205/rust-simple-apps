fn main() {
    let mut names = ["Pepper", "Bob", "Andy", "Mark"];
    for name in names.iter() {
        println!("Hello, {}!", name);
    }

    println!();

    for name in names.iter_mut() {
        *name = match *name {
            "Bob" => "Who am i?",
            _ => name,
        }
    }
    for name in names.iter() {
        println!("Hello, {}!", name);
    }
}
