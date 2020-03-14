// rustc scoping_RAII.rs && valgrind ./scoping_RAII

struct DropDebug;

impl Drop for DropDebug {
    fn drop(&mut self) {
        println!("DropDebug is being dropped");
    }
}

fn create_box() {
    let _box1 = Box::new(3);
}

fn main() {
    let _box2 = Box::new(5);
    {
        let _box3 = Box::new(4);
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let _x = DropDebug;
    println!("Made a DropDebug!");
}
