use std::io::{self, BufRead};
use std::num::ParseFloatError;
use std::process::exit;

fn main() {
    println!("Temperature Converter v 0.1.0");

    fn get_input() -> String {
        println!("Select starting scale:  [F]ahrenheit [C]elsius [K]elvin or [Q]uit ");

        let mut input = String::new();
        std::io::BufReader::new(std::io::stdin())
            .read_line(&mut input)
            .expect("Input not found!");
        input
    }

    let selection = get_input();

    match selection.trim() {
        "F" | "f" => match f_to_c() {
            Ok(c_deg) => println!("Your temperature in Celsius is: {c_deg}"),
            Err(err) => println!("Error found: {err}"),
        },
        "C" | "c" => match c_to_f() {
            Ok(f_deg) => println!("Your temperature in Fahrenheit is: {f_deg}"),
            Err(err) => println!("Error found: {err}"),
        },
        "Q" | "q" => exit(0),
        _ => println!("Option not recognized"),
    }

    fn f_to_c() -> Result<f32, ParseFloatError> {
        println!("Enter temperature in Fahrenheit: ");

        let mut f_deg = String::new();
        io::stdin().read_line(&mut f_deg).unwrap();

        let f_deg: f32 = f_deg.trim().parse()?;

        let c_deg = (f_deg - 32.0) / (9.0 / 5.0);

        Ok(c_deg)
    }

    fn c_to_f() -> Result<f32, ParseFloatError> {
        println!("Enter temperature in Celsius: ");

        let mut c_deg = String::new();
        io::stdin().read_line(&mut c_deg).unwrap();

        let c_deg: f32 = c_deg.trim().parse()?;

        let f_deg = c_deg * (9.0 / 5.0) + 32.0;

        Ok(f_deg)
    }
}
