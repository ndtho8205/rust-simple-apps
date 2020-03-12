fn main() {
    let mut value = 3;

    println!("Before match: {:?}", value);

    match value {
        ref mut r => {
            println!("Before: {:?}", r);
            *r += 10;
            println!("After: {:?}", r);
        }
    }

    println!("After match: {:?}", value);
}
