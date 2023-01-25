use evalexpr;
use std::{env::args, io::{stdin, stdout, Write}};
mod tasks;
use crate::tasks::tasks::all;

fn help() {
    println!(r#"feval: evaluate expressions
usage: provide exactly one argument (with expression) or no arguments for shell mode.
example: feval "math::sin(30 * math::pi / 180)""#);
}

fn main_loop() {
    print!(">>> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    while stdin().read_line(&mut input).unwrap() != 0 {
        println!("{}", evalexpr::eval(&input.trim()).unwrap());
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
            println!("{}", evalexpr::eval(&expr).unwrap())
        },
        1 => {main_loop()},
        _ => {
            if args.len() > 2 {
                println!("Too many args.");
                help();
                return
            } else if args.len() < 1 {
                println!("Too few args.");
                help();
                return
            }
        }
    }
}
