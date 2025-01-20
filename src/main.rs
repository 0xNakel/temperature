use std::io;
use std::io::Write;


fn main() {
    println!("Temperature Converter!");

    print!("Enter the temperature in Fahrenheit: ");
    io::stdout().flush().unwrap();

    let mut faren = String::new();

    io::stdin()
        .read_line(&mut faren)
        .expect("Could not read line");

    let faren:f64 = faren.trim()
        .parse()
        .expect("Only numbers are accepted!");

    
    let cel:f64 = (({faren} - 32.0) * 5.0) / 9.0;

    println!("The temperature in Celcius is: {cel}")






}
