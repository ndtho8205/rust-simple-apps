enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted {:?}", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

enum Number {
    Zero,
    One,
    Two,
}

fn main() {
    use crate::WebEvent::*;

    let load = PageLoad;
    let unload = PageUnload;
    let pressed = KeyPress('x');
    let pasted = Paste("Hello, World!".to_owned());
    let clicked = Click { x: 1, y: 2 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(clicked);

    println!("");
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
}
