use std::io;


fn main() {
    println!("Welcome to a Fibonnaci program");

    loop {
        println!("Please enter the number of fibonnaci elements you want");

        let mut fibonnanic_count: String = String::new();

        io::stdin()
            .read_line(&mut fibonnanic_count)
            .expect("Error when trying to read content");
    
            
        let fibonnanic_count: usize = match fibonnanic_count.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Error when trying to read number, please enter a number");
                continue;
            }
        };

        let mut fibonnaci_numbers: Vec<i32> = vec![0, 1];

        for index in 0..(fibonnanic_count - 2) {
            let fibonnaci_number: i32 = fibonnaci_numbers[index] + fibonnaci_numbers[index];
            fibonnaci_numbers.push(fibonnaci_number) 
        };
        
        for number in fibonnaci_numbers {
            println!("{number}");
        }

        break;
    }
}
