fn main() {
    println!("{}", my_age(20))
}

fn my_age(a: i32) -> String {
    return format!("I'm {} years old", a);
}
