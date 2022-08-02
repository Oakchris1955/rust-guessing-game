// Include locale module's functions and Localization struct
use crate::locales::{functions::*, structures::Localization};
// Include process module to terminate program when needed
use std::process;
// Include macro to create Maps at compile time
use phf::phf_map;

// Constant that stores all the commands the user can input
const COMMANDS_MAP: phf::Map<&'static str, fn(&Localization, u32)> = phf_map! {
	"q" => |locale, secret_number| {format_once(locale.messages.info_messages.user_exit.as_str(), secret_number.to_string().as_str());process::exit(0)},
	"quit" => |locale, secret_number| {format_once(locale.messages.info_messages.user_exit.as_str(), secret_number.to_string().as_str());process::exit(0)},
	"help" => |_, _| {println!("Placeholder command")}
};

pub fn validate_command_name(raw_input: &str) -> bool {
	//! Note: If this function returns true, this doesn't necesarily mean that the inputted "str" is a command

	// Check if all chars all alphabetic and return the result value
	raw_input.trim().chars().all(|ch| ch.is_ascii_alphabetic() || ch == ' ')
}

pub fn execute_command(raw_input: &str, locale: &Localization, secret_number: u32) -> bool {
	// Begin by trimming the input
	let input = raw_input.trim();
	// Then, check if the string entered exists in COMMANDS_MAP
	match COMMANDS_MAP.get(input) {
		// If yes, run the command
		Some(func) => {func(locale, secret_number);true},
		None => false
	}
}