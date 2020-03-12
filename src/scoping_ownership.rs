fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c)
}

fn main() {
    let x = 5u32;
    let y = x; // copy
    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains: {}", a);
    let b = a; // move `a` into `b`
               //println!("After moving: a = {}", a);
    println!("After moving: b = {}", b);

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box; // moved
                                         //println!("immutable_box contains {}", immutable_box);
    println!("mutable_box contains {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box contains {}", mutable_box);
}
