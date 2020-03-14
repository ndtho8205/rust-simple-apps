fn main() {
    let a = 10;
    let b = 30;
    println!("{:5} + {:5} = {:5}", a, b, add_int(a, b));

    let c = 5.34578;
    let d = 7.43092;
    println!("{:5.2} + {:5.2} = {:5.2}", c, d, add_float(c, d));
}

fn add_int(a: i32, b: i32) -> i32 {
    a + b
}

fn add_float(a: f32, b: f32) -> f32 {
    a + b
}
