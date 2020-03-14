#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

/*
struct Person<'a> {
    name: &'a str,
    age: u8,
}
*/

fn main() {
    //struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    //println!("{:?}", UnPrintable(3));
    println!("{:#?}", DebugPrintable(3));

    let name = "Tho".to_owned();
    let age = 20;
    println!("{:?}", Person { name, age });

    println!("{0} {0:?}", "actor's");
}
