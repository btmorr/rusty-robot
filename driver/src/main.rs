//extern crate serial;
extern crate iron;
extern crate clap;

//use std::env;
use std::io;
use std::io::Write;

use clap::{Arg, App};

use iron::prelude::*;
use iron::status;
//use std::time::Duration;

fn main() {
	let matches = App::new("app")
		.arg(Arg::with_name("hostname")
			.short("h")
			.long("hostname")
			.takes_value(true))
		.arg(Arg::with_name("port")
			.short("p")
			.long("port")
			.takes_value(true))
		.arg(Arg::with_name("cli")
			.long("cli")
			.multiple(false))
		.arg(Arg::with_name("v")
			.short("v")
			.multiple(true))
		.get_matches();

	let verbosity = match matches.occurrences_of("v") {
		0 => "WARN",
		1 => "INFO",
		2 | _ => "DEBUG",
	};

	println!("Debug level: {}", verbosity);

	match matches.occurrences_of("cli") {
		0 => {
			let hostname = matches.value_of("hostname")
				.unwrap_or("localhost");
			let port = matches.value_of("port")
				.unwrap_or("80");
			server_main(hostname, port);			
		},
		_ => {
			println!("Starting REPL");
			cli_main()
		},
	}
}

fn server_main(hostname: &str, port: &str) {
	let server_uri = format!("{}:{}", hostname, port);
	println!("Starting server on {}", server_uri);
	Iron::new(|_: &mut Request| {
		Ok(Response::with((status::Ok, "Hello World!")))
	}).http(server_uri).unwrap();
}

fn cli_main() {
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
