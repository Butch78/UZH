pub mod machine_parser;

use crate::machine_parser::parser::Machine;
use std::fs;
use std::process;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename: filename })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let input = fs::read_to_string(config.filename);

    match machine_parser::parser::parser(input.unwrap()) {
        Ok(machine) => return machine_cli(machine),
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            return Err("Problem parsing arguments");
        }
    };
}

pub fn machine_cli(machine: Machine) -> Result<(), &'static str> {
    let mut machine = machine;

    loop {
        match machine.current_state.print_state() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        match machine.execute_autoforwarding() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid Input!");

        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        print!("{esc}c", esc = 27 as char);

        // Execute Transition
        match machine.execute_transition(&input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        match machine.current_transition.print_transition() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
}
