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

// Include modules to parse json and trim JSON comments
use json_comments::StripComments;
use serde_json::{Result, Value};
use serde_json::error::Category as serde_err_category;
use serde::{Serialize, Deserialize};

// Include module locales...
mod locales;
// ...and include Localization struct and all the functions of the module
use locales::structures::Localization;
use locales::functions::*;

// Also, include modules commands and everything on it
mod commands;
use commands::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct JSONOptions {
	total_tries: u32,
	max_number: u32,
	min_number: u32,
	locale_name: String
}

impl Default for JSONOptions {
	fn default() -> Self {
		Self {
			// Edit these fields to change the defualt values for the game if they aren't found in "options.json"
			total_tries: 4,
			max_number: 100,
			min_number: 1,
			locale_name: String::from("No locale detected")
		}
	}
}

pub enum JSONResults {
	Value(Value),
	JSONOptions(JSONOptions)
}


fn get_json_info(embed_struct: bool) -> JSONResults {
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

	// Decode the JSON string to an object (if embed_struct is true)
	let stripped_str = StripComments::new(json_str.as_bytes());
	let json_result: Result<Value> = serde_json::from_reader(stripped_str);//json::parse(json_str.as_str());
	let json_struct = match json_result {
		Ok(value) => value,
		Err(error) => match error.classify() {
			serde_err_category::Io => {
				eprintln!("Couldn't read bytes due to an IO error. This error shouldn't occur normally, and if it keeps occuring, report it at https://github.com/Oakchris1955/rust-guessing-game/issues ");
				process::exit(1);
			},
			serde_err_category::Syntax => {
				eprintln!("There was an error in the syntax of \"options.json\". Exiting...");
				process::exit(1);
			},
			serde_err_category::Data => {
				eprintln!("There was an error embedding the JSON info to the \"JSONOptions\" struct because a field's values didn't match each other. Exiting...");
				process::exit(1);
			},
			serde_err_category::Eof => {
				eprintln!("Reached the end of the file while still waiting for more data. Exiting...");
				process::exit(1);
			}
		}
	};

	// then, return a JSONOptions struct with the specified options (if any)
	if embed_struct {
		JSONResults::JSONOptions(serde_json::from_value::<JSONOptions>(json_struct).unwrap_or_else(|err| {
			match err.classify() {
				serde_err_category::Io => {
					eprintln!("Couldn't read bytes due to an IO error. This error shouldn't occur normally, and if it keeps occuring, report it at https://github.com/Oakchris1955/rust-guessing-game/issues ");
					process::exit(1);
				},
				serde_err_category::Syntax => {
					eprintln!("There was an error in the syntax of \"options.json\". Exiting...");
					process::exit(1);
				},
				serde_err_category::Data => {
					eprintln!("There was an error embedding the JSON info to the \"JSONOptions\" struct because a field's values didn't match each other. Exiting...");
					process::exit(1);
				},
				serde_err_category::Eof => {
					eprintln!("Reached the end of the file while still waiting for more data. Exiting...");
					process::exit(1);
				}
			}
		}))
	} else {JSONResults::Value(json_struct)}
}

fn get_user_input(msg: &str, secret_number: u32, locale: &Localization) -> u32 {
	// we supply the secret number and the locale in case the program exits

	// begin a loop
    loop {
		// print the message supplied
        print!("{msg}");

		// this match statement is so that input and msg are on the same line
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

		// before doing anything else, check if user entered a command or "q"
		let command_entered = if validate_command_name(&user_input.as_str()) {execute_command(&user_input.as_str(), locale, secret_number)} else {false};
		// If entered a command, skip rest of loop
		if command_entered {continue;}

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
	// Firstly, get the json options and save them as a variable
	let options = if let JSONResults::JSONOptions(option) = get_json_info(true) {
		Some(option)
	} else {None}.unwrap();

	// Check if locale_name exists or not and save corresponding Localization object
	let selected_locale = if options.locale_name == String::from("No locale detected") {
		// Firstly, get a list with all the valid locales
		let locales_list = get_locales_list("locales");
		// Then, prompt the user to select a locale
		let selected_locale_name = select_locale(&locales_list, "locales");
		// Then, get the Localization object for the selected locale
		let selected_locale = change_locale(&selected_locale_name);

		// This will be only executed without the --release flag
		#[cfg(debug_assertions)]{
		println!("The list of locales is: {:?}", locales_list);
		println!("And the localization object for the selected locale ({}) is: {:?}", selected_locale_name, selected_locale);}

		selected_locale
	} else {
		// Check if found locale is valid
		if get_locales_list("locales").iter().find(|locale_name| locale_name == &&options.locale_name).is_some() {
			get_localization_info(&options.locale_name, "locales")
		} else {
			eprintln!("Invalid locale detected. Exiting...");
			process::exit(1);
		}
	};

	// The localization will be automatically implemented throughout the program

    // Main code begins here.
    let secret_number: u32 = rand::thread_rng().gen_range(options.min_number..options.max_number+1);

    // This will be only executed without the --release flag
    #[cfg(debug_assertions)]
	println!("{}", format_once(selected_locale.messages.info_messages.debug.secret_number.as_str(), secret_number.to_string().as_str()));

    println!("{}", selected_locale.messages.info_messages.welcome_message);

    for current_try in 1..options.total_tries+1 {
		#[cfg(debug_assertions)]
        println!("{}", format_once(selected_locale.messages.info_messages.debug.current_try.as_str(), current_try.to_string().as_str()));

        if current_try == options.total_tries {
            println!("{}", selected_locale.messages.info_messages.game_messages.last_try);
        } else if current_try == 1 {
            println!("{}",  selected_locale.messages.info_messages.game_messages.first_try_messages.beginning_message);
			println!("{}", format_once(selected_locale.messages.info_messages.game_messages.first_try_messages.total_tries_announcement.as_str(), options.total_tries.to_string().as_str()));
			println!("{}", format_twice(selected_locale.messages.info_messages.game_messages.first_try_messages.secret_number_range.as_str(),[options.min_number.to_string().as_str(), options.max_number.to_string().as_str()]));
        } else {
            println!("{}", selected_locale.messages.info_messages.game_messages.other_tries);
        }

        let guess: u32 = get_user_input(selected_locale.messages.info_messages.game_messages.input_prompt_message.as_str(), secret_number, &selected_locale);

        // Check if found the correct number
		// If yes, print a message and break the loop, essentially ending the program
        if guess == secret_number {
            println!("{}", format_once(selected_locale.messages.info_messages.game_messages.guess_results.correct.as_str(), current_try.to_string().as_str()));
            break;
        }

		// Check if game ends here
		// If not, print a hint about the guess (if it is higher or lower than the current one)
        if current_try!=options.total_tries {
            println!("{}", selected_locale.messages.info_messages.game_messages.guess_results.wrong.announcement);
            if guess < secret_number {
                println!("{}", selected_locale.messages.info_messages.game_messages.guess_results.wrong.higher_hint);
            } else {
                println!("{}", selected_locale.messages.info_messages.game_messages.guess_results.wrong.lower_hint);
            }
			let remaining_tries = options.total_tries - current_try;
            println!("{}", format_twice(selected_locale.messages.info_messages.game_messages.guess_results.wrong.remaining_tries.as_str(), [remaining_tries.to_string().as_str(), if remaining_tries == 1 {selected_locale.messages.info_messages.game_messages.guess_results.wrong.try_singular.as_str()} else {selected_locale.messages.info_messages.game_messages.guess_results.wrong.try_plural.as_str()}]));
        } else {
			// If yes, print a message. The loop won't repeat and thus the program will end
            println!("{}", format_once(selected_locale.messages.info_messages.game_messages.guess_results.wrong.no_tries_remaining.as_str(), secret_number.to_string().as_str()));
        }
    }

}