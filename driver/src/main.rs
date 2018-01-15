extern crate clap;
extern crate iron;
extern crate router;

//use std::env;
//use std::time::Duration;
use std::io;
use std::io::{Read, Write};

use clap::{Arg, App};

use iron::prelude::*;
use iron::status;
use router::Router;

#[derive(Debug)]
struct ServerURI<'a> {
    hostname: &'a str,
    port: &'a str,
}

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
			let uri = ServerURI { 
				hostname: matches.value_of("hostname").unwrap_or("localhost"),
				port: matches.value_of("port").unwrap_or("80")
			};
			server_main(uri);
		},
		_ => {
			println!("Starting REPL");
			cli_main()
		},
	}
}

fn server_main(uri: ServerURI) {
	let uri = format!("{}:{}", uri.hostname, uri.port);
	println!("Starting server on {}", uri);

	let mut router = Router::new();

	router.get(
		"/ping", |_: &mut Request| { 
			Ok(Response::with((status::Ok, "pong")))
		}, 
		"ping");

	router.get(
		"/hello/:name", 
		hello_world, 
		"hello");

	router.post(
		"/echo",
		echo,
		"echo");

	Iron::new(router).http(uri).unwrap();
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

fn hello_world(req: &mut Request) -> IronResult<Response> {
	println!("Request: {:?}", req);

	let name = req.extensions.get::<Router>()
		.unwrap().find("name").unwrap_or("World");

	// let mut buffer = String::new();
	// println!("Body: {:?}", req.body.read_to_string(&mut buffer));

	Ok(Response::with((status::Ok, format!("Hello, {}!", name))))
}

fn echo(req: &mut Request) -> IronResult<Response> {
	let mut payload = String::new();
	req.body.read_to_string(&mut payload).unwrap();
	println!("Received: {}", payload);
	Ok(Response::with((status::Ok, payload)))
}
