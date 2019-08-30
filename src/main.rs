use std::env;
use std::process;
use std::u8;

const ERROR_MESSAGE: &str = "[x] Please use a similar format when specifying values:
    rhr #fff\n    rhr #ffffff\n    rhr 0 0 0\n    rhr 223 21 64";

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn from_hex(hex_color: &str) -> Color {
        let hex_color: &str = &hex_color[1..];
        if hex_color.len() != 3 && hex_color.len() != 6 {
            exit();
        }
        let mut final_hex = String::new();
        if hex_color.len() == 3 {
            for (_, hex_letter) in hex_color.chars().enumerate() {
                final_hex.push(hex_letter);
                final_hex.push(hex_letter);
            }
        } else {
            final_hex = String::from(hex_color);
        }
        let red: u8 = parse_from_hex(&final_hex[0..2]);
        let green: u8 = parse_from_hex(&final_hex[2..4]);
        let blue: u8 = parse_from_hex(&final_hex[4..6]);

        Color { red, green, blue }
    }

    fn from_rgb(rgb: (u8, u8, u8)) -> Color {
        Color {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
        }
    }

    fn to_hex(&self) -> String {
        hex::encode(vec![self.red, self.green, self.blue])
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit();
    }
    let first_argument = &args[1];
    let result: String = if first_argument.starts_with("#") {
        let color: Color = Color::from_hex(first_argument);
        format!("{:?}", color.to_rgb())
    } else {
        if args.len() < 4 {
            exit();
        }
        let red: u8 = parse_to_byte(&first_argument);
        let green: u8 = parse_to_byte(&args[2]);
        let blue: u8 = parse_to_byte(&args[3]);
        let color: Color = Color::from_rgb((red, green, blue));
        format!("#{}", color.to_hex())
    };

    println!("{}", result);
}

fn parse_to_byte(value: &str) -> u8 {
    match value.parse() {
        Ok(val) => val,
        Err(_) => exit(),
    }
}

fn parse_from_hex(value: &str) -> u8 {
    match u8::from_str_radix(value, 16) {
        Ok(val) => val,
        Err(_) => exit(),
    }
}

fn exit() -> ! {
    println!("{}", ERROR_MESSAGE);
    process::exit(1);
}
