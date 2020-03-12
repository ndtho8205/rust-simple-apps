use std::fmt;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    print_color(Color {
        red: 128,
        green: 255,
        blue: 90,
    });

    print_color(Color {
        red: 0,
        green: 2,
        blue: 254,
    });

    print_color(Color {
        red: 0,
        green: 0,
        blue: 0,
    });
}

fn print_color(color: Color) {
    println!("Debug: {:?}", color);
    println!("Display: {}", color);
    println!("UpperHex: {:#X}", color);
    println!()
}
