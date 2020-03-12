fn main() {
    let max = |a: i32, b: i32| -> i32 {
        if a > b {
            a
        } else {
            b
        }
    };

    println!("max(1, 2) = {}", max(1, 2));
}
