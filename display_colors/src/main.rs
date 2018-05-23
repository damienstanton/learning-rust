use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:X}",
            self.red,
            self.green,
            self.blue,
            self.red // TODO: figure out how to concat the u8 struct type 🤔
        )
    }
}

fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ].iter()
    {
        // println!("{:?}", *color);
        println!("{}", *color);
    }
}
