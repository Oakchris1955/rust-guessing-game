use std::io;
use std::io::Write;
use std::process;
use std::fs;
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
	let contents = fs::read_to_string("config/options.json");
	
	let json_str = match contents {
		Ok(pure_json) => pure_json,
		Err(e) => match e.kind() {
			ErrorKind::NotFound => String::from("{}"),
			_ => {
				eprintln!("{}", e);
				process::exit(1);
			}
		}
	};

	let json_content = json::parse(json_str.as_str());

	let json_content = match json_content {
		Ok(json_content) => json_content,
		Err(e) => {
			eprintln!("{}", e);
			process::exit(1);
		}
	};

	JSONOptions {
		total_tries: json_content["total_tries"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[0]),
		max_number: json_content["max_number"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[1]),
		min_number: json_content["min_number"].as_u32().unwrap_or(DEFAULT_JSON_OPTIONS[2]),
	}
}

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
	let options = get_json_info();
    // Edit these variables to modify the game
    let total_tries = options.total_tries;
    let max_number = options.max_number; // the number to iter last
    let min_number = options.min_number; // the number to begin from

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
			println!("You have {total_tries} tries.");
			println!("Also, the secret number is between {min_number} and {max_number}.");
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