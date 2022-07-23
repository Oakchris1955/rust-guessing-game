use std::io;
use std::num::IntErrorKind;
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

// Include module locales...
mod locales;
// ...and include Localization struct and all the functions of the module
use locales::structures::Localization;
use locales::functions::*;

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

fn get_user_input(msg: &str, secret_number: u32, locale: &Localization) -> u32 {
	// we supply the secret number and the locale in case the program exits

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
				println!("{}", locale.messages.error_messages.cant_read_line);
				continue;
			}
		}

		// before doing anything else, check if user entered "q"
		if user_input.trim() == "q" {
			// if yes, successfully exit the program and display a message
			println!("{}", format_once(locale.messages.info_messages.user_exit.as_str(), secret_number.to_string().as_str()));
			process::exit(0);
		}

		// try turning the input string to a u32 type
        match user_input.trim().parse::<u32>() {
			// if did succesfully, return it early, effectively stopping the loop
            Ok(number) => return number,
			// else, print a message telling the user to input a number next time and repeat
            Err(error) => match error.kind() {
				IntErrorKind::Empty => println!("{}", locale.messages.error_messages.input_errors.empty),
				IntErrorKind::InvalidDigit => println!("{}", locale.messages.error_messages.input_errors.invalid_digit),
				IntErrorKind::PosOverflow => println!("{}", locale.messages.error_messages.input_errors.pos_overflow),
				IntErrorKind::NegOverflow => println!("{}", locale.messages.error_messages.input_errors.neg_overflow),
				IntErrorKind::Zero => println!("{}", locale.messages.error_messages.input_errors.zero),
				_ => panic!("{}", format_once(locale.messages.error_messages.input_errors.unknown_err.as_str(), format!("{error}").as_str()))
			}
        };
    }
}

fn main() {
	// Begin by getting the localization info
	// Firstly, get a list with all the valid locales
	let locales_list = get_locales_list("locales");
	// Then, get the Localization object for the first locale (right now should be an English locale)
	let english_locale = get_localization_info(&locales_list[0], "locales");
	
	// The localization will be automatically implemented throughout the program

	// This will be only executed without the --release flag
	#[cfg(debug_assertions)]{
	println!("The list of locales is: {:?}", locales_list);
	println!("And the localization object for the first locale is: {:?}", english_locale);}

	// Save the options as a variable
	let options = get_json_info();

    // Main code begins here. DO NOT MODIFY IF YOU DON'T KNOW WHAT YOU ARE DOING
    let secret_number: u32 = rand::thread_rng().gen_range(options.min_number..options.max_number+1);

    // This will be only executed without the --release flag
    #[cfg(debug_assertions)]
	println!("{}", format_once(english_locale.messages.info_messages.debug.secret_number.as_str(), secret_number.to_string().as_str()));
	//println!("The secret number is: {}", secret_number);

    println!("{}", english_locale.messages.info_messages.welcome_message);

    for current_try in 1..options.total_tries+1 {
		#[cfg(debug_assertions)]
        println!("{}", format_once(english_locale.messages.info_messages.debug.current_try.as_str(), current_try.to_string().as_str()));

        if current_try == options.total_tries {
            println!("{}", english_locale.messages.info_messages.game_messages.last_try);
        } else if current_try == 1 {
            println!("{}",  english_locale.messages.info_messages.game_messages.first_try_messages.beginning_message);
			println!("{}", format_once(english_locale.messages.info_messages.game_messages.first_try_messages.total_tries_announcement.as_str(), options.total_tries.to_string().as_str()));
			println!("{}", format_twice(english_locale.messages.info_messages.game_messages.first_try_messages.secret_number_range.as_str(),[options.min_number.to_string().as_str(), options.max_number.to_string().as_str()]));
        } else {
            println!("{}", english_locale.messages.info_messages.game_messages.other_tries);
        }

        let guess: u32 = get_user_input(english_locale.messages.info_messages.game_messages.input_prompt_message.as_str(), secret_number, &english_locale);

        // Check if found the correct number
		// If yes, print a message and break the loop, essentially ending the program
        if guess == secret_number {
            println!("{}", format_once(english_locale.messages.info_messages.game_messages.guess_results.correct.as_str(), current_try.to_string().as_str()));
            break;
        }

		// Check if game ends here
		// If not, print a hint about the guess (if it is higher or lower than the current one)
        if current_try!=options.total_tries {
            println!("{}", english_locale.messages.info_messages.game_messages.guess_results.wrong.announcement);
            if guess < secret_number {
                println!("{}", english_locale.messages.info_messages.game_messages.guess_results.wrong.higher_hint);
            } else {
                println!("{}", english_locale.messages.info_messages.game_messages.guess_results.wrong.lower_hint);
            }
			let remaining_tries = options.total_tries - current_try;
            println!("{}", format_twice(english_locale.messages.info_messages.game_messages.guess_results.wrong.remaining_tries.as_str(), [remaining_tries.to_string().as_str(), if remaining_tries == 1 {english_locale.messages.info_messages.game_messages.guess_results.wrong.try_singular.as_str()} else {english_locale.messages.info_messages.game_messages.guess_results.wrong.try_plural.as_str()}]));
        } else {
			// If yes, print a message. The loop won't repeat and thus the program will end
            println!("{}", format_once(english_locale.messages.info_messages.game_messages.guess_results.wrong.no_tries_remaining.as_str(), secret_number.to_string().as_str()));
        }
    }

}