#[allow(dead_code)]
fn destroy_box(boxed: Box<i32>) {
    println!("Destroying box that contains {}", boxed);
}

fn borrow_i32(borrowed: &i32) {
    println!("Borrow int {}", borrowed);
}

fn main() {
    let boxed = Box::new(5);
    let stacked = 6;

    borrow_i32(&boxed);
    borrow_i32(&stacked);

    //destroy_box(boxed);
    println!("{}", &boxed);
}
