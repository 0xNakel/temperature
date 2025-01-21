use std::io;
use std::io::Write;

fn main () {
    println! ("Temperature Converter v 0.1.0");
    
    let mut input = String::new();
    
    print! ("Enter temperature in Fahrenheit: ");
    io::stdout().flush();
    
    io::stdin().read_line(&mut input)
            .expect("Could not read line.");
    
    let temp: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number! Try again.")
    };
    
    let celsius  = ((temp - 32.0) / (9.0/5.0));
    
    println!("{celsius}");
    
}
