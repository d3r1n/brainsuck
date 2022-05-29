use bs_lib::{
	interpreter::interpret,
	lexer::lex, 
	parser::parse, 
	optimizer::interpreter::optimized_interpret, 
	optimizer::lexer::optimized_lex, 
	optimizer::parser::optimized_parse,
	utils::{
		BrainsuckError,
		BrainsuckErrorType
	}
};

use colored::Colorize;

use std::io::{BufRead, BufReader, Write};

use std::{io, process::exit};

pub fn repl(optimize: &bool) {
    loop {
        let input = get_input(">>> ").unwrap();

        if input.is_empty() {
            continue;
        } else if input == "quit" {
            exit(0);
        } else if input == "clear" {
            print!("{esc}c", esc = 27 as char);
			print!("\n{}\n{}\nType \"{}\" or press CTRL + C to exit\n{}\n\n", 
				"Brainsuck Interactive Shell".bright_green(), 
				"-".repeat(40), 
				"quit".bright_magenta(), 
				"-".repeat(40)
			);
			continue;
        }

        let mut memory: Vec<u8> = vec![0; 1024];
        let mut memory_pointer: usize = 512;

        if *optimize {
			optimized_interpret(
				&optimized_parse(&optimized_lex(&input), &true),
				&mut memory,
				&mut memory_pointer,
				true,
				true,
			)
		}
		else {
			interpret(
				&parse(&lex(&input), &true),
				&mut memory,
				&mut memory_pointer,
				true,
				true,
			)
		}
    }
}

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    BufReader::new(io::stdin())
        .lines()
        .next()
        .ok_or_else(|| 
				BrainsuckError::throw_error(
					"Reading input failed".to_string(), 
					BrainsuckErrorType::IOError, 
					true
				)
		)
		.unwrap()
}
