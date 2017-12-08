//extern crate serial;

use std::env;
use std::io;
use std::io::Write;
//use std::time::Duration;

//use std::io::prelude::*;
//use serial::prelude::*;

fn main() {
	println!("Initialization arguments:");
	for arg in env::args_os().skip(1) {
		arg.to_str()
			.map(|s| println!("- {}", s));
	}

	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut input = String::new();

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");
		send(&input).unwrap();

		match input.trim() {
			"exit" => break,
			_ => continue,
		}
	}
}

fn send(msg: &str) -> io::Result<()> {
	println!("{}", msg.trim());
	Ok(())
}
