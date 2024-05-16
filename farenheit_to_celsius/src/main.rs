use std::io;

fn main() {
    println!("Welcome to a Farenheit to Celsius temperature converter");

    loop {
        println!("Please enter a farenheit temperature");

        let mut farenheit_temperature: String = String::new();

        io::stdin()
            .read_line(&mut farenheit_temperature)
            .expect("Error when trying to read input");
        
        let farenheit_temperature: u64 = match farenheit_temperature.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Error when trying to convert value, please enter a number");
                continue;
            }
        };

        let celsius_temperature: f64 = farenheit_to_celsius(&farenheit_temperature);

        println!("In celsius, {} farenheit degrees is {:.2} degrees", farenheit_temperature, celsius_temperature);

        break;
    }
    
}

fn farenheit_to_celsius(&farenheit_temperature: &u64) -> f64 {
    let celsius_temperature: f64 = (farenheit_temperature as f64 - 32.0) * (5.0/9.0);
    celsius_temperature 
}
