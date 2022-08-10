// Include locale module's Localization struct
use crate::locales::structures::Localization;
use std::str::Split;

struct Command {
	name: &'static str,
	aliases: Vec<&'static str>,
	command: fn(&Localization, u32, Split<&str>),
	description: String
}

impl Command {
	pub fn get_names(&self) -> Vec<&str> {
		let mut aliases = self.aliases.clone();
		aliases.push(self.name);
		aliases
	}
}

fn get_user_commands(locale: &Localization) -> Vec<Command> {
	// Edit this variable to alter the commands
	let commands_vec: Vec<(&'static str, &'static [&'static str], (fn(&Localization, u32, Split<&str>), &str))> = vec![
		("quit", &["q"], (comm_funcs::QUIT, &locale.commands.descriptions.quit)),
		("reset", &["r"], (comm_funcs::RESET, &locale.commands.descriptions.reset)),
		("help", &[], (comm_funcs::HELP, &locale.commands.descriptions.help))
	];
	// For those wondering, no, unfortunately the variable can't be a static because we need the locale
	
	let mut commands_struct_vec: Vec<Command> = Vec::new();

	for comm in commands_vec.iter() {
		let local_command = Command {
			name: comm.0,
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
	// Include process module to terminate program when needed
	use std::process;
	// Include built-in module fs to edit files and directories
	use std::fs;
	// Include struct Split in case a command need parameters, for example "change locale"
	use std::str::Split;

	pub static HELP: fn(&Localization, u32, Split<&str>) = |locale: &Localization, _secret_number: u32, _split: Split<&str>| {
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

	pub static QUIT: fn(&Localization, u32, Split<&str>) = |locale: &Localization, secret_number: u32, _split: Split<&str>| {
		println!("{}", format_once(locale.messages.info_messages.user_exit.as_str(), secret_number.to_string().as_str()));
		process::exit(0);
	};

	pub static RESET: fn(&Localization, u32, Split<&str>) = |locale: &Localization, _secret_number: u32, _split: Split<&str>| {
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
		println!("{}", locale.commands.text.reset.as_str());
		process::exit(0);
	};
}


pub fn execute_command(raw_input: &str, locale: &Localization, secret_number: u32) -> bool {
	let commands = get_user_commands(locale);
	// Begin by trimming the input
	let input = raw_input.trim();
	// Then, check if the string entered exists in COMMANDS_MAP
	match commands.iter().find(|comm| comm.get_names().iter().find(|name| name == &&input).is_some()) {
		// If yes, run the command
		Some(content) => {(content.command)(locale, secret_number, input.split(" "));true},
		None => false
	}
}