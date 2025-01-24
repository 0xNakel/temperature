use std::io::{self, BufRead, Write};

enum Options {
    F,
    C,
    K,
    Q,
}
#[derive(Debug)]
struct InputError;

fn main() {
    println!("Temperature Converter v 0.1.0");

    impl Options {
        fn prompt_choice(c: &str) -> Result<Options, InputError> {
            match c {
                "F" | "f" => Ok(Options::F),
                "C" | "c" => Ok(Options::C),
                "K" | "k" => Ok(Options::K),
                "Q" | "q" => Ok(Options::Q),
                _ => Err(InputError),
            }
        }
    }

    fn enum_match(scale: Options) {
        match scale {
            Options::F => unimplemented!(),
            Options::C => unimplemented!(),
            Options::K => unimplemented!(),
            Options::Q => std::process::exit(0),
        }
    }

    print!("Select starting unit [F]ahrenheit [C]elsius [K]elvin or [Q]uit: ");
    io::stdout().flush();
    let mut input = String::new();
    std::io::BufReader::new(std::io::stdin())
        .read_line(&mut input)
        .unwrap();
    let choice = Options::prompt_choice(input.trim()).unwrap();
    enum_match(choice);

    print!("Enter temperature in Fahrenheit: ");
    io::stdout().flush();

    let mut fa_deg = String::new();
    io::stdin()
        .read_line(&mut fa_deg)
        .expect("Could not read line.");

    let fa_deg: f32 = match fa_deg.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{e}"),
    };

    let cel_deg = (fa_deg - 32.0) / (9.0 / 5.0);

    println!("{cel_deg} °C");

    print!("Enter temperature in Celsius: ");
    io::stdout().flush();

    let mut cel_deg = String::new();
    io::stdin()
        .read_line(&mut cel_deg)
        .expect("Could not read line.");

    let cel_deg: f32 = match cel_deg.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number! Try again."),
    };

    let fax_deg = cel_deg * (9.0 / 5.0) + 32.0;

    println!("{fax_deg} °F")
}
