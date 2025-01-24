use std::io::{self, Write};


fn main () {
    println! ("Temperature Converter v 0.1.0");

    print! ("Select starting unit [F]ahrenheit [C]elsius [K]elvin or [Q]uit: ");
    io::stdout().flush();

    fn prompt_choice() -> Result <String, io::Error> {
        let mut sel_choice = String::new();
        io::stdin().read_line(&mut sel_choice)?;
        Ok(sel_choice)
    }

    


    match Result {
        1 => println!("Fahrenheit chosen"),
        2 => println!("Celsius chosen"),
        3 => println!("Not yet implemented"),
        4 => return,
        _ => println! ("what")
    }
    
    print! ("Enter temperature in Fahrenheit: ");
    io::stdout().flush();
    
    let mut fa_deg = String::new();
    io::stdin().read_line(&mut fa_deg)
            .expect("Could not read line.");
    
    let fa_deg: f32 = match fa_deg.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{e}")
    };
    
    let cel_deg  = (fa_deg - 32.0) / (9.0/5.0);
    
    println!("{cel_deg} °C");
    
    
    print! ("Enter temperature in Celsius: ");
    io::stdout().flush();
    
    let mut cel_deg = String::new();
    io::stdin().read_line(&mut cel_deg)
            .expect("Could not read line.");
    
    let cel_deg: f32 = match cel_deg.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number! Try again.")
    };
    
    let fax_deg  = cel_deg * (9.0/5.0) + 32.0;
    
    println!("{fax_deg} °F")
    
}
