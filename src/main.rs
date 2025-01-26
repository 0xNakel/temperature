use std::io::{self, BufRead, Write};
use std::num::ParseFloatError;

fn main() {
    println!("Temperature Converter v 0.1.0");

    print!("Select starting scale  [F]ahrenheit [C]elsius [K]elvin or [Q]uit: ");
    io::stdout().flush();

    let mut input = String::new();
    std::io::BufReader::new(std::io::stdin())
        .read_line(&mut input)
        .unwrap();

    fn f_to_c() -> Result<f32, ParseFloatError> {
        print!("Enter temperature in Fahrenheit: ");
        io::stdout().flush();

        let mut f_deg = String::new();
        io::stdin().read_line(&mut f_deg).unwrap();

        let f_deg: f32 = f_deg.trim().parse()?;

        let c_deg = (f_deg - 32.0) / (9.0 / 5.0);

        Ok(c_deg)
    }

    fn c_to_f() -> Result<f32, ParseFloatError> {
        print!("Enter temperature in Celsius: ");
        io::stdout().flush();

        let mut c_deg = String::new();
        io::stdin().read_line(&mut c_deg).unwrap();

        let c_deg: f32 = c_deg.trim().parse()?;

        let f_deg = c_deg * (9.0 / 5.0) + 32.0;

        Ok(f_deg)
    }
}
