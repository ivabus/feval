use std::{
	env::args,
	io::{stdin, stdout, Write},
};

use evalexpr;
use evalexpr::HashMapContext;

use crate::tasks::tasks::all;

mod tasks;

fn help() {
	println!(
		r#"feval: help
usage: provide exactly one argument (with expression) or no arguments for shell mode.
example: feval "math::sin(30 * math::pi / 180)""#
	);
}

fn main_loop() {
	print!(">>> ");
	stdout().flush().unwrap();
	let mut input = String::new();
	let mut context = HashMapContext::new();
	while stdin().read_line(&mut input).unwrap() != 0 {
		let result = evalexpr::eval_with_context_mut(&all(input.trim().to_string()), &mut context);
		match result {
			Ok(succ_res) => println!("{}", succ_res),
			Err(err) => println!("Error: {}", err),
		}
		input.clear();
		print!(">>> ");
		stdout().flush().unwrap();
	}
}

fn main() {
	let args: Vec<String> = args().collect();
	match args.len() {
		2 => {
			let expr = all(args[1].clone());
			let result = evalexpr::eval(&expr.trim());
			match result {
				Ok(succ_res) => println!("{}", succ_res),
				Err(err) => {
					println!("Error: {}", err);
					std::process::exit(127);
				}
			}
		}
		1 => main_loop(),
		_ => {
			if args.len() > 2 {
				println!("Too many args.");
				help();
			} else if args.len() < 1 {
				println!("Too few args.");
				help();
			}
		}
	}
}
