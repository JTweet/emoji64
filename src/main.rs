/**
 * Copyright 2016 Oliver "JTweet" Springer
 *
 *	This file is part of emoji64.
 *
 *	emoji64 is free software: you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation, either version 3 of the License, or
 *	(at your option) any later version.
 *
 *	emoji64 is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *	GNU General Public License for more details.
 *
 *	You should have received a copy of the GNU General Public License
 *	along with emoji64.  If not, see <http://www.gnu.org/licenses/>.
 */


use std::env;
use std::io;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut decode = false;

	for i in 0..args.len() - 1 {
		match &*args[i + 1] {
			"-d" => { decode = true; }
			"--decode" => { decode = true; }
			"-h" => { show_help(); }
			"--help" => { show_help(); }
			"" => { }
			_ => { show_invalid_command(); }
		}
	}
	emoji_conversion(decode);
}

// Can convert to and from emojis. "decode" bools sets its mode.
// Unused variables are required return vars by Rust that I don't check.
#[allow(unused_variables)]
fn emoji_conversion(decode: bool) {
	let mut buf = String::new();
	match io::stdin().read_line(&mut buf) {
		Ok(n) => {
			let mut chars = buf.chars();
			for i in 0..buf.len() {
				match chars.next() {
					Some(c) => {
						if !is_special_char(c) {
							if decode {
								print!("{}", emoji_to_char(c));
							} else {
								print!("{}", char_to_emoji(c));
							}
						}
					}
					// None => { panic!("Unknown char specified."); }
					None => { }
				}
			}
		}
		Err(error) => {
			panic!("Invalid input.");
		}
	}
}

fn show_help() {
	println!("Usage: emoji64 [OPTION]");
	println!("Convert a base64 input, provided through stdin, to an Emoji output.");
	println!("");
	println!("\t-d --decode\tDecode an Emoji input back to base64.");
	println!("\t-h --help\tDisplay this help message.");
	process::exit(0);
}

fn show_invalid_command() {
	println!("You have passed an invalid argument.");
	show_help();
}

fn char_to_emoji(c: char) -> char {
	let emoji_value = c as u32 -47 +0x1F600;
	let emoji_char: Option<char> = std::char::from_u32(emoji_value);
	match emoji_char {
		Some(emoji_hex) => { emoji_hex }
		_ => { panic!("Invalid value!") }
	}
}

fn emoji_to_char(c: char) -> char {
	let char_value = c as u32 +47 -0x1F600;
	let chr: Option<char> = std::char::from_u32(char_value);
	match chr {
		Some(chr_hex)	=> { chr_hex }
		_ => { panic!("Invalid value!") }
	}
}

fn is_special_char(c: char) -> bool {
	match c {
		'\n' => true,
		'\r' => true,
		'\'' => true,
		'\"' => true,
		'\\' => true,
		_ => false,
	}
}

