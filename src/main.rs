use std::io;
use std::io::Write;
use std::process;

// Include built-in module to read file
use std::fs;
// Include built-in error types
use std::io::ErrorKind;

// Include module to randomly select number
use rand::Rng;

// Include module to parse json
use json;


struct JSONOptions {
	total_tries: u32,
	max_number: u32,
	min_number: u32
}

const DEFAULT_JSON_OPTIONS: [u32; 3] = [4, 100, 1];
// 1st is total tries, 2nd is max number and 3rd is the min number


fn get_json_info() -> JSONOptions {
	// Read file contents
	let contents = fs::read_to_string("config/options.json");
	
	let json_str = match contents {
		// if everything is fine, save as a variable the file contents
		Ok(pure_json) => pure_json,
		Err(e) => match e.kind() {
			// else, if file doesn't exist, return an empty JSON string
			ErrorKind::NotFound => String::from("{}"),
			_ => {
				// else, print the error and exit
				eprintln!("{}", e);
				process::exit(1);
			}
		}
	};

	// Decode the JSON string to an object
	let json_content = json::parse(json_str.as_str());

	let json_content = match json_content {
		// if everything is fine, save as a variable the object
		Ok(json_content) => json_content,
		Err(e) => {
			// else, print the error and exit
			eprintln!("{}", e);
			process::exit(1);
		}
	};

	// then, return a JSONOptions struct with the specified options (if any)
	JSONOptions {
		total_tries: json_content["total_tries"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[0]),
		max_number: json_content["max_number"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[1]),
		min_number: json_content["min_number"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[2]),
	}
}

fn get_user_input(msg: &str, secret_number: u32) -> u32 {
	// we supply the secret number in case the program exits

	// begin a loop
    loop {
		// print the message supplied
        print!("{msg}");

		// this match staement is so that input and msg are on the same line
        match io::stdout().flush() {
			Ok(_) => (),
			Err(_error) => continue
		}

		// create an empty string
        let mut user_input = String::new();

		// check if an error occured
		match io::stdin().read_line(&mut user_input) {
			Ok(_) => (),
			Err(_error) => {
				// if an error occured while reading line, then skip rest of loop using continue
				println!("An error occured while reading line. Please try again");
				continue;
			}
		}

		// before doing anything else, check if user entered "q"
		if user_input.trim() == "q" {
			// if yes, successfully exit the program and display a message
			println!("Exiting program. The secret number was {secret_number}");
			process::exit(0);
		}

		// try turning the input string to a u32 type
        match user_input.trim().parse::<u32>() {
			// if did succesfully, return it early, effectively stopping the loop
            Ok(number) => return number,
			// else, print a message telling the user to input a number next time and repeat
            Err(_error) => println!("Please type a number!"),
        };
    }
}

fn main() {
	// Save the options as a variable
	let options = get_json_info();

    // Main code begins here. DO NOT MODIFY IF YOU DON'T KNOW WHAT YOU ARE DOING
    let secret_number: u32 = rand::thread_rng().gen_range(options.min_number..options.max_number+1);

    // This will be only executed without the --release flag
    #[cfg(debug_assertions)]
    println!("The secret number is: {secret_number}");

    println!("A simple guessing game. Made by Oakchris1955");

    for current_try in 1..options.total_tries+1 {
        // println!("Number of current guess: {current_try}");

        if current_try == options.total_tries {
            println!("Last try.");
        } else if current_try == 1 {
            println!("Alright, let's begin.");
			println!("You have {} tries.", options.total_tries);
			println!("Also, the secret number is between {} and {}.", options.min_number, options.max_number);
        } else {
            println!("Let's retry");
        }

        let guess: u32 = get_user_input("Enter a number (or \"q\" to exit): ", secret_number);

        // Check if found the correct number
        if guess == secret_number {
            println!("Congratulations. You found the secret number after {} guesses", current_try);
            break;
        }

        if current_try!=options.total_tries {
            println!("I'm sorry, but your guess wasn't correct.");
            if guess < secret_number {
                println!("Next time try a higher number");
            } else {
                println!("Next time try a lower number");
            }
			let remaining_tries = options.total_tries - current_try;
            println!("You have {} {} remaining", remaining_tries, if remaining_tries == 1 {"try"} else {"tries"});
        } else {
            println!("I'm sorry, you lost. The secret number was {secret_number}");
        }
    }

}