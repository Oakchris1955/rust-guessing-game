// Include locale module's functions
use crate::locales::functions::*;
use std::process;

pub fn validate_command_name(raw_input: &str) -> bool {
	//! Note: If this function returns true, this doesn't necesarily mean that the inputted "str" is a command

	// Check if all chars all alphabetic and return the result value
	raw_input.trim().chars().all(|ch| ch.is_ascii_alphabetic() || ch == ' ')
}

pub fn execute_command(raw_input: &str) {
	// Begin by trimming the input
	let input = raw_input.trim();

	// The match statement executes the command
	let mut split_input = input.split(" ");
	let f = split_input.next().unwrap_or("");
	match f {
		"change" => {
			match split_input.next().unwrap_or("") {
				"locale" => {
					// Firstly, get a list with all the valid locales
					let locales_list = get_locales_list("locales");
					// Then, prompt the user to select a locale
					let selected_locale_name = select_locale(&locales_list, "locales");
					// Lastly, save selected locale to "options.json" and exit
					change_locale(&selected_locale_name);
					println!("Changed locale, restarting program...");
					process::exit(0);
				},
				_ => ()
			}
		},
		_ => ()
	}
}