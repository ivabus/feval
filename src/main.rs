use evalexpr;
use std::env::args;
mod tasks;
use crate::tasks::tasks::all;

fn help() {
    println!(r#"feval: evaluate expressions
usage: provide exactly one argument (with expression)
example: feval "math::sin(30 * math::pi / 180)""#);
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Too many args.");
        help();
        return
    } else if args.len() < 2 {
        println!("Too few args.");
        help();
        return
    }
    let expr = all(args[1].clone());
    println!("{}", evalexpr::eval(&expr).unwrap())
}
