fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("orig: {:?}", a);

    inc(0, &mut a);
    println!("after inc: {:?}", a);

    let s: &mut [i32] = &mut a[0..1];
    println!("slice: {:?}", s);

    s[0] = 10;
    println!("slice after assignment: {:?}", s);
    println!("array after assignment: {:?}", a);
}

fn inc(i: usize, x: &mut [i32]) {
    x[i] += 5;
}
