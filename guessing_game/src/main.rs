use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // use mut keyword to make a mutable variable
        let mut guess = String::new();

        io::stdin()
        // readline takes users input and appends it to specified buffer. Takes a mutable reference to string without taking ownership of it
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            // underscore is a catchall variable that will match any type of error
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        };
    }
}
