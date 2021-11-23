use bs_lib::{interpreter::interpret, lexer::lex, parser::parse};

use colored::Colorize;

use std::io::{BufRead, BufReader, Write};

use std::{io, process::exit};

pub fn repl() {
    loop {
        let input = get_input(">>> ").unwrap();

        if input.is_empty() {
            continue;
        } else if input == "quit" {
            exit(0);
        } else if input == "clear" {
            print!("{esc}c", esc = 27 as char);
            print!("\n{}\n-------------------------------------\nType \"{}\" or press CTRL + C to exit\n-------------------------------------\n\n", "Brainsuck Interactive Shell".bright_green(), "quit".bright_magenta());
            continue;
        }

        let mut memory: Vec<u8> = vec![0; 1024];
        let mut memory_pointer: usize = 512;

        interpret(
            &parse(&lex(&input), true),
            &mut memory,
            &mut memory_pointer,
            true,
            true,
        )
    }
}

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    BufReader::new(io::stdin())
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Cannot read stdin"))
        .and_then(|inner| inner)
}
