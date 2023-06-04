use std::{
	env::args,
	io::{stdin, stdout, Write},
};

use evalexpr;
use evalexpr::HashMapContext;
use linefeed::{Interface, ReadResult};

use crate::tasks::tasks::all;

mod tasks;

fn help() {
	println!(
		r#"feval: help
usage: provide expression as arguments or don't provide arguments for shell mode.
example: feval "math::sin(30 * math::pi / 180)""#
	);
}

fn main_loop() {
	let mut reader = Interface::new("feval").unwrap();
	reader.set_prompt(">>> ").unwrap();
	let mut context = HashMapContext::new();
	while let ReadResult::Input(input) = reader.read_line().unwrap() {
		let result = evalexpr::eval_with_context_mut(&all(input.trim().to_string()), &mut context);
		match result {
			Ok(succ_res) => println!("{}", succ_res),
			Err(err) => println!("Error: {}", err),
		}
	}
}

fn main() {
	let args: Vec<String> = args().collect();
	if args.contains(&"--help".to_string()) {
		help();
		std::process::exit(0);
	}
	if args.len() >= 2 {
		let mut expr = String::new();
		for i in args[1..].to_vec() {
			expr += &i;
			expr += " ";
		}
		let expr = all(expr);
		let result = evalexpr::eval(&expr.trim());
		match result {
			Ok(succ_res) => println!("{}", succ_res),
			Err(err) => {
				println!("Error: {}", err);
				std::process::exit(127);
			}
		}
	} else if args.len() == 1 {
		main_loop();
	}
}
