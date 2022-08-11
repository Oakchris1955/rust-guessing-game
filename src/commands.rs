// Include locale module's Localization struct
use crate::locales::structures::Localization;
use std::str::SplitWhitespace;

struct Command {
	name: String,
	aliases: Vec<String>,
	command: fn(&Localization, u32, SplitWhitespace),
	description: String
}

impl Command {
	pub fn get_names(&self) -> Vec<String> {
		let mut aliases = self.aliases.clone();
		aliases.push(self.name.to_string());
		aliases
	}
}

fn get_user_commands(locale: &Localization) -> Vec<Command> {
	// Edit these variables to alter the commands
	let fast_info = &locale.commands.info;
	let commands_vec: &[(&String, &Vec<String>, (fn(&Localization, u32, SplitWhitespace), &String))] = &[
		(&fast_info.quit.name, &fast_info.quit.aliases, (comm_funcs::QUIT, &fast_info.quit.description)),
		(&fast_info.reset.name, &fast_info.reset.aliases, (comm_funcs::RESET, &fast_info.reset.description)),
		(&fast_info.change.name, &fast_info.change.aliases, (comm_funcs::CHANGE, &fast_info.change.description)),
		(&fast_info.help.name, &fast_info.help.aliases, (comm_funcs::HELP, &fast_info.help.description))
	];
	// For those wondering, no, unfortunately the variable can't be a static because we need the locale
	
	let mut commands_struct_vec: Vec<Command> = Vec::new();

	for comm in commands_vec.iter() {
		let local_command = Command {
			name: comm.0.to_owned(),
			aliases: comm.1.to_vec(),
			command: comm.2.0,
			description: comm.2.1.to_owned()
		};
		commands_struct_vec.push(local_command);
	}
	commands_struct_vec
}

// This module contains the functions that execute the commands
// comms_funcs stands for COMMandS FUNCtionS
mod comm_funcs {
	// Include locale module's functions and Localization struct
	use crate::locales::{functions::*, structures::Localization};
	use std::io::Write;
// Include process module to terminate program when needed
	use std::process;
	// Include built-in module fs to edit files and directories
	use std::fs;
	// Include struct SplitWhitespace in case a command needs parameters, for example "change locale"
	use std::str::SplitWhitespace;

	use std::io;
	use std::num::IntErrorKind;

	use crate::{get_json_info, JSONResults};
	use serde_json::{self, json};

	pub static HELP: fn(&Localization, u32, SplitWhitespace) = |locale: &Localization, _secret_number: u32, _split: SplitWhitespace| {
		for entry in super::get_user_commands(locale) {
			// Print them
			if entry.aliases.is_empty() {
				println!("{} - {}", entry.name, entry.description);
			} else {
				let mut aliases_str = entry.aliases.iter().map(|alias| format!("{}, ", alias)).collect::<String>();
				let aliases_str_len = aliases_str.chars().count();
				aliases_str.truncate(aliases_str_len-2);
				println!("{} ({}) - {}", entry.name, aliases_str, entry.description);
			}
		};
	};

	pub static CHANGE: fn(&Localization, u32, SplitWhitespace) = |locale: &Localization, _secret_number: u32, mut split: SplitWhitespace| {
		// Begin by checking how many parameters were entered (doesn't include command name)
		let split_size = split.clone().count();
		if split_size == 0 {
			// If no parameters were entered, print a message
			println!("{}", locale.commands.various_texts.no_params);
		} else if split_size >= 1 {
			// If 1 parameter or more were inputted, save the 1st parameter as variable "main_arg"
			let main_arg = split.next().unwrap();
			// Now, process that argument
			if main_arg == "lang" || main_arg == "locale" {
				// Firstly, get a list with all the valid locales
				let locales_list = get_locales_list("locales");
				// Then, prompt the user to select a locale
				let selected_locale_options = select_locale(&locales_list, "locales");
				let selected_locale_name = match selected_locale_options {
					Some(name) => name,
					None => {println!("{}", locale.commands.various_texts.no_selection);return}
				};
				// Then, save it at options.json after getting the current contents of options.json
				let mut json_options = if let JSONResults::Value(option) = get_json_info(false) {
					Some(option)
				} else {None}.unwrap();
				json_options["locale_name"] = json!(selected_locale_name);
				match fs::write("config/options.json", json_options.to_string()) {
					Ok(_) => {println!("{}", locale.commands.various_texts.change);process::exit(0)},
					Err(_err) => {eprintln!("{}", format_once(locale.commands.errors.cant_write_to_file.as_str(), "config/options.json"));process::exit(1)}
				}
			} else if ["max", "min", "tries"].iter().find(|elem| elem == &&main_arg).is_some() { // The process for editing all these 3 is pretty much the same, that's why this statement if for all of them
				// Get "config/options.json" contents
				let mut json_options = if let JSONResults::Value(option) = get_json_info(false) {
					Some(option)
				} else {None}.unwrap();
				// Then, get a user input
				let number = loop {
					// Begin by printing a message
					let prompt = &locale.commands.various_texts.selection_prompts[main_arg]; //if main_arg == "max" {&locale.commands.various_texts.selection_prompts.max} else if main_arg == "min" {&locale.commands.various_texts.selection_prompts.min} else {&locale.commands.various_texts.selection_prompts.tries};
					print!("{}: ", prompt);
					match io::stdout().flush() {
						Ok(_) => (),
						Err(_error) => continue
					}
					let mut user_input = String::new();
					
					match io::stdin().read_line(&mut user_input) {
						Ok(_) => {if user_input.trim() == "q" {println!("{}", locale.commands.various_texts.no_selection);return} else {match user_input.trim().parse::<u32>() {
							Ok(value) => break value,
							Err(err) => match err.kind() {
								IntErrorKind::Empty => println!("{}", locale.messages.error_messages.input_errors.empty),
								IntErrorKind::InvalidDigit => println!("{}", locale.messages.error_messages.input_errors.invalid_digit),
								IntErrorKind::PosOverflow => println!("{}", locale.messages.error_messages.input_errors.pos_overflow),
								IntErrorKind::NegOverflow => println!("{}", locale.messages.error_messages.input_errors.neg_overflow),
								IntErrorKind::Zero => println!("{}", locale.messages.error_messages.input_errors.zero),
								_ => panic!("{}", format_once(locale.messages.error_messages.input_errors.unknown_err.as_str(), format!("{err}").as_str()))
							}
						}}},
						Err(_error) => {
							// if an error occured while reading line, then skip rest of loop using continue
							continue;
						}
					}
				};
				json_options[main_arg] = json!(number);
				match fs::write("config/options.json", json_options.to_string()) {
					Ok(_) => {println!("{}", locale.commands.various_texts.change);process::exit(0)},
					Err(_err) => {eprintln!("{}", format_once(locale.commands.errors.cant_write_to_file.as_str(), "config/options.json"));process::exit(1)}
				}
			} else {

			}
		}
	};

	pub static QUIT: fn(&Localization, u32, SplitWhitespace) = |locale: &Localization, secret_number: u32, _split: SplitWhitespace| {
		println!("{}", format_once(locale.commands.various_texts.quit.as_str(), secret_number.to_string().as_str()));
		process::exit(0);
	};

	pub static RESET: fn(&Localization, u32, SplitWhitespace) = |locale: &Localization, _secret_number: u32, _split: SplitWhitespace| {
		// Define some variables to know what to remove and what not
		let to_remove_paths: Vec<&str> = ["config"].to_vec();
		let to_exclude_paths: Vec<&str> = [].to_vec();

		// Collect the path of each file (except to_exclude_paths)
		let to_remove_files = to_remove_paths.iter().map(|path|
			match fs::read_dir(path) {
				Ok(entries) => entries.map(|entry| match entry {
					Ok(unwrapped_entry) => {
						let filepath = unwrapped_entry.path();
						let unwrapped_filepath = filepath.to_str().unwrap_or("");
						if to_exclude_paths.contains(&unwrapped_filepath) {
							String::new()
						} else {
							unwrapped_filepath.to_string()
						}
					},
					Err(_) => {
						eprintln!("{}", format_once(locale.commands.errors.cant_read_file_in_path.as_str(), path));
						process::exit(1);
					}
				}).collect::<Vec<String>>(),
				Err(_) => {
					eprintln!("{}", format_once(locale.commands.errors.cant_read_dir.as_str(), path));
					process::exit(1);
				}
			}
		).flatten().filter(|string| string != "").collect::<Vec<String>>();

		// Iterate through the Vec and delete all files present on it
		for filename in to_remove_files {
			match fs::remove_file(&filename) {
				Ok(_) => (),
				Err(_err) => {
					eprintln!("{}", format_once(locale.commands.errors.cant_remove_file.as_str(), filename.as_str()));
					process::exit(1);
				}
			}
		}
		// Print a small text and exit program
		println!("{}", locale.commands.various_texts.reset.as_str());
		process::exit(0);
	};
}


pub fn execute_command(raw_input: &str, locale: &Localization, secret_number: u32) -> bool {
	let commands = get_user_commands(locale);
	// Begin by trimming the input
	let input = raw_input.trim();
	// Store a SplitWhitespace of the input
	let mut input_split = input.split_whitespace();
	// And the first element of the input_split (the command) as a variable
	let command_name = match input_split.next() {
		Some(name) => name,
		None => return false // In case nothing was entered, return false early
	};
	// Then, check if the string entered exists in COMMANDS_MAP
	match commands.iter().find(|comm| comm.get_names().iter().find(|name| name == &&command_name).is_some()) {
		// If yes, run the command
		Some(content) => {(content.command)(locale, secret_number, input_split);true},
		None => false
	}
}