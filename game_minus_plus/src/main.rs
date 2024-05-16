use std::cmp::Ordering; 
use std::io;

use rand::Rng;

fn main() {
    println!("GAME : Try to find the number");

    // Générer un nombre aléatoire entre 1 et 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Enter a number");
        
        // Créer une variable de type String et vide
        let mut supposition: String = String::new();
        
        // Lire l'entrée utilisateur
        io::stdin()
            .read_line(&mut supposition)
            .expect("Error when trying to read input value");

        // Retirer les caractères vides de l'entrée et la convertir en u32
        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Input is incorrent, try again");
                continue;
            }
        };

        println!("Input number is {}", supposition);
        
        // Checker si la proposition est égale au nombre secret
        match supposition.cmp(&secret_number) {
            Ordering::Less => println!("It's higher"),
            Ordering::Equal => {
                println!("You win");
                break;
            },        
            Ordering::Greater => println!("It's lower"),        
        }
    }
}
