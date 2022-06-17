use std::io;
use std::io::Write;

use rand::Rng;

fn get_user_input(msg: &str) -> u32 {
    print!("{msg}");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");


    user_input.trim().parse().expect("Please type a number!")

}

fn main() {
    // Edit these variables to modify the game
    let remaining_tries = 4;
    let max_number = 100; // the number to iter last
    let min_number = 1; // the number to begin from

    // Main code begins here. DO NOT MODIFY IF YOU DON'T KNOW WHAT YOU ARE DOING
    let secret_number: u32 = rand::thread_rng().gen_range(min_number..max_number+1);

    // Uncomment line below for debug
    // println!("The secret number is: {secret_number}", );

    println!("A simple guessing game. Made by Oakchris1955");

    for current_guess in 1..remaining_tries+1 {
        // println!("Number of current guess: {current_guess}");

        if current_guess == remaining_tries {
            println!("Last try. ");
        } else if current_guess == 1 {
            println!("Alright, let's begin. ");
        } else {
            println!("Let's retry");
        }

        let guess: u32 = get_user_input("Enter a number: ");

        // Check if found the correct number
        if guess == secret_number {
            println!("Congratulations. You found the secret number after {} guesses", current_guess);
            break;
        }
        println!("I'm sorry, but your guess wasn't correct.");
        if guess < secret_number {
            println!("Next time try a higher number");
        } else {
            println!("Next time try a lower number");
        }
        println!("You have {} tries remaining", remaining_tries - current_guess);
    }

}