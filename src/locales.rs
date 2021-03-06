// Create a public module named "structures"
pub mod structures {
	// Include serde_json
	use serde::Deserialize;
	// Define some structs
	#[derive(Deserialize, Debug)]
	pub struct InputErrors {
		pub empty: String,
		pub invalid_digit: String,
		pub pos_overflow: String,
		pub neg_overflow: String,
		pub zero: String,
		pub unknown_err: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct ErrorMessages {
		pub cant_read_line: String,
		pub input_errors: InputErrors,
	}

	#[derive(Deserialize, Debug)]
	pub struct DebugMessages {
		pub secret_number: String,
		pub current_try: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct FirstTryMessages {
		pub beginning_message: String,
		pub total_tries_announcement: String,
		pub secret_number_range: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct WrongGuessResult {
		pub announcement: String,
		pub higher_hint: String,
		pub lower_hint: String,
		pub remaining_tries: String,
		pub try_singular: String,
		pub try_plural: String,
		pub no_tries_remaining: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct GuessResults {
		pub correct: String,
		pub wrong: WrongGuessResult,
	}

	#[derive(Deserialize, Debug)]
	pub struct GameMessages {
		pub first_try_messages: FirstTryMessages,
		pub last_try: String,
		pub other_tries: String,
		pub input_prompt_message: String,
		pub guess_results: GuessResults,
	}

	#[derive(Deserialize, Debug)]
	pub struct InfoMessages {
		pub debug: DebugMessages,
		pub welcome_message: String,
		pub user_exit: String,
		pub game_messages: GameMessages,
	}

	#[derive(Deserialize, Debug)]
	pub struct Messages {
		pub error_messages: ErrorMessages,
		pub info_messages: InfoMessages,
	}

	// This will be the only module part accessible from outside
	#[derive(Deserialize, Debug)]
	pub struct Localization {
		pub lang_title: String,
		pub messages: Messages,
	}
}

// Create a public module named "functions"
pub mod functions {
	// Include some modules
	use std::fs;
	use std::process;
	use super::structures::Localization;
	use serde_json::error::Category as serde_err_category;

	pub fn get_locales_list(directory: &str) -> Vec<String> {
		// Get all files inside directory and unwrap it
		let dir_paths = fs::read_dir(directory).unwrap_or_else(|err| {
			eprintln!("An error occured while trying to get {directory}'s contents. The error message is:\n{err}\nExiting...");
			process::exit(1);
		});
	
		// Initialize a new empty Vector
		let mut filenames: Vec<String> = Vec::new();
	
		for path in dir_paths {
			// Unwrap the path and turn it into a OsString
			let unwrapped_path = match path {
				Ok(obj) => obj,
				Err(err) => {
					// If a error occurs, skip file
					eprintln!("An error occured while unwrapping a path. The error message is:\n{err}\nSkipping path...");
					continue;
				}
			};
	
			// Check if path is a file
			match unwrapped_path.file_type() {
				 Ok(filetype) => if !filetype.is_file() {
					// If it isn't a file, skip file
					continue;
				 }
				 Err(err) => {
					// If a error occurs, skip file
					eprintln!("An error occured while checking if path is a file. The error message is:\n{err}\nSkipping path...");
					continue;
				}
			};
	
			// If everything is fine, do some check and then turn OsString to a String...
			let path_file_name = match unwrapped_path.file_name().into_string() {
				Ok(name) => {
					let name_str = name.as_str();
					let name_chars = name_str.chars();
					// If no error occured, do some checks (check locales' path README for more info)
					if {name_chars.clone().count() == 10 &&
						name_chars.clone().skip(5).collect::<String>() == String::from(".json") &&
						name_chars.clone().take(2).all(|x| x.is_lowercase()) &&
						name_chars.clone().skip(2).take(1).all(|x| x=='-') &&
						name_chars.clone().skip(3).take(2).all(|x| x.is_uppercase())
					} {name_chars.take(5).collect::<String>()} else {continue;}
				},
				Err(_err) => {
					eprintln!("An error occured while turning a path to a readable String. Skipping path...");
					continue;
				}
			};
			// and push it to filenames Vector
			filenames.push(path_file_name);
		}
	
		// Once done, return the filenames Vector
		filenames
	}
	
	pub fn get_localization_info(locale_name: &String, locale_path: &str) -> Localization {
		let result_locale = fs::read_to_string(format!("{locale_path}/{locale_name}.json"));
	
		let json_locale = match result_locale {
			Ok(locale) => locale,
			Err(error) => {
				eprintln!("An error has occured. Error message:\n\n{error}");
				process::exit(1);
			} 
			// Ingnore this comment
			/*match error.into() {
				ErrorKind::NotFound => {
					eprintln!("Locale not found. This error shouldn't occur by default. Submit it to: https://github.com/Oakchris1955/rust-guessing-game/issues ");
					process::exit(1);
				},
				ErrorKind::PermissionDenied  => {
					eprintln!("Can't read locale file for locale {locale_name}. Please make sure that the program has sufficient permissions and then run it again");
					process::exit(1);
				}
				_ => {
					eprint!("An unidentified error has occured. If you don't know wht it is, report it at: https://github.com/Oakchris1955/rust-guessing-game/issues. The error message is the following:\n\n{error}");
					process::exit(1);
				}
			}*/
		};
	
		let constructed_locale = serde_json::from_str(json_locale.as_str());
	
		let unwrapped_locale: Localization = match constructed_locale {
			Ok(locale) => locale,
			Err(error) => {
				eprintln!("An error occured while unwrapping locale {locale_name} at line {}, column {}", error.line(), error.column());
				match error.classify() {
				serde_err_category::Io => {
					eprintln!("Couldn't read bytes due to an IO error. This error shouldn't occur normally, and if it keeps occuring, report it at https://github.com/Oakchris1955/rust-guessing-game/issues ");
					process::exit(1);
				},
				serde_err_category::Syntax => {
					eprintln!("There was an error in the syntax of {locale_name} locale. Exiting...");
					process::exit(1);
				},
				serde_err_category::Data => {
					eprintln!("There was an error embedding the extracted struct into the Localization struct because a field's values didn't match each other. Exiting...");
					process::exit(1);
				},
				serde_err_category::Eof => {
					eprintln!("Reached the end of the file while still waiting for more data. Exiting...");
					process::exit(1);
				}
			}}
		};
	
		// return the unwrapped locale JSON data
		unwrapped_locale
	
	}

	pub fn format_once(to_format: &str, argument: &str) -> String {
		// Begin by making a test to see if the string is valid
		let test_result = to_format.matches("{}").count();
		// Check if result equals 1
		if test_result != 1 {
			// If not, exit
			eprintln!("Invalid string \"{to_format}\" received. Exiting...");
			process::exit(1);
		}

		// Format the string and return it
		to_format.replace("{}", argument)
	}

	pub fn format_twice(to_format: &str, arguments: [&str; 2]) -> String {
		//! This is basically [`format_once`], but for 2 arguments
		//! (Check locales' `README.md` for more info)
		
		// Begin by making a test to see if the string is valid
		for num in 1..2 {
			let test_result = to_format.matches(format!("{{{num}}}").as_str()).count();
			// Check if result equals 1
			if test_result != 1 {
				// If not, exit
				eprintln!("Invalid string \"{to_format}\" received. Exiting...");
				process::exit(1);
			}
		}
		// If the test passed successfully, begin formatting
		// Do the first format and save as a variable
		let to_format = to_format.replace("{1}", arguments[0]);
		// Then, do the second and return
		to_format.as_str().replace("{2}", arguments[1])
	}
}