use std::io;
use std::io::Write;

// Include module to randomly select number
use rand::Rng;

fn get_user_input(msg: &str) -> u32 {
    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(number) => return number,
            Err(_error) => println!("Please type a number!"),
        };
    }
}

fn main() {
    // Edit these variables to modify the game
    let total_tries = 4;
    let max_number = 100; // the number to iter last
    let min_number = 1; // the number to begin from

    // Main code begins here. DO NOT MODIFY IF YOU DON'T KNOW WHAT YOU ARE DOING
    let secret_number: u32 = rand::thread_rng().gen_range(min_number..max_number+1);

    // This will be only executed without the --release flag
    #[cfg(debug_assertions)]
    println!("The secret number is: {secret_number}");

    println!("A simple guessing game. Made by Oakchris1955");

    for current_try in 1..total_tries+1 {
        // println!("Number of current guess: {current_try}");

        if current_try == total_tries {
            println!("Last try.");
        } else if current_try == 1 {
            println!("Alright, let's begin.");
        } else {
            println!("Let's retry");
        }

        let guess: u32 = get_user_input("Enter a number: ");

        // Check if found the correct number
        if guess == secret_number {
            println!("Congratulations. You found the secret number after {} guesses", current_try);
            break;
        }

        if current_try!=total_tries {
            println!("I'm sorry, but your guess wasn't correct.");
            if guess < secret_number {
                println!("Next time try a higher number");
            } else {
                println!("Next time try a lower number");
            }
			let remaining_tries = total_tries - current_try;
            println!("You have {} {} remaining", remaining_tries, if remaining_tries == 1 {"try"} else {"tries"});
        } else {
            println!("I'm sorry, you lost. The secret number was {secret_number}");
        }
    }

}