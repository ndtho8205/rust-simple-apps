fn age() -> u32 {
    15
}

fn main() {
    match age() {
        0 => println!("I'm not born yet"),
        a @ 1..=12 => println!("I'm a child of age {:?}", a),
        a @ 13..=19 => println!("I'm a teen of age {:?}", a),
        a => println!("I'm an old person of age {:?}", a),
    }
}
