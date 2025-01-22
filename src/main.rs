use std::io;
use std::io::Write;

enum TempScale {
    C,
    F,
    K,
}


fn main () {
    println! ("Temperature Converter v 0.1.0");
    
    print! ("Please enter if you want ° F or ° C: ");
    io::stdout().flush();
    
    let mut which_scale = String::new();
    
    io::stdin().read_line(&mut which_scale)
            .expect("Could not read line.");

    
    print! ("Enter temperature in Fahrenheit: ");
    io::stdout().flush();
    
    let mut fa_deg = String::new();
    io::stdin().read_line(&mut fa_deg)
            .expect("Could not read line.");
    
    let fa_deg: f32 = match fa_deg.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number! Try again.")
    };
    
    let cel_deg  = (fa_deg - 32.0) / (9.0/5.0);
    
    println!("{cel_deg}");
    
    
    print! ("Enter temperature in Celsius: ");
    io::stdout().flush();
    
    let mut cel_deg = String::new();
    io::stdin().read_line(&mut cel_deg)
            .expect("Could not read line.");
    
    let cel_deg: f32 = match cel_deg.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number! Try again.")
    };
    
    let fax_deg  = (cel_deg * (9.0/5.0) + 32.0 );
    
    println!("{fax_deg}")
    
}
