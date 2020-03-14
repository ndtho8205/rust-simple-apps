fn main() {
    let n = 10;

    println!(
        "{}",
        if n < 0 {
            "smaller than 10"
        } else if n == 10 {
            "equal to 10"
        } else {
            "equal or larger than 10"
        }
    )
}
