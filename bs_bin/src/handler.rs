extern crate clap;
use clap::{Arg, App};
use colored::{Colorize};

use bs_lib;
use bs_lib::utils::{BrainsuckError, BrainsuckErrorType, BrainsuckMessage, BrainsuckMessageType};

use std::io;
use std::io::Write;
use std::io::Read;
use std::fs::File;

use crate::repl::Repl;

pub fn handle() {
    let matches = App::new("Brainsuck")
        .version("1.0")
        .author("d3r1n <github.com/d3r1n>")
        .about("Brainfuck but with more commands, interpeter and a repl...")
        .arg(
            Arg::with_name("INPUT")
                .help("Brainsuck file to be executed.")
                .index(1),
        )
		.arg(
			Arg::with_name("mem-size")
				.short("m")
				.long("mem-size")
				.value_name("SIZE")
				.takes_value(true)
				.help("Size of the memory buffer")
		)
		.arg(
			Arg::with_name("ptr-loc")
				.short("p")
				.long("ptr-loc")
				.value_name("LOCATION")
				.takes_value(true)
				.help("Location of the memory pointer")
		)
		.arg(
			Arg::with_name("auto")
				.short("a")
				.long("auto")
				.multiple(false)
				.help("If enabled, program will automatically allocate memory size")
		)
		.get_matches();

    if matches.value_of("INPUT") == None {
		
		print!("\n{}\n-------------------------------------\nType \"{}\" or press CTRL + C to exit\n-------------------------------------\n\n", "Brainsuck Interactive Shell".bright_green(), "quit".bright_magenta());

		Repl();

    } else {
        let mut file = File::open(matches.value_of("INPUT").unwrap())
		.map_err(|err| BrainsuckError::throw_error(format!("Can't find input file '{}'", matches.value_of("INPUT").unwrap()), BrainsuckErrorType::FileNotFoundError, true));

        let mut src = String::new();

        file.unwrap().read_to_string(&mut src)
		.map_err(|err| BrainsuckError::throw_error(format!("Can't read the input file '{}'", matches.value_of("INPUT").unwrap()), BrainsuckErrorType::CantReadFileError, true));

        let ops = bs_lib::lexer::Lex(src);
        let program = bs_lib::parser::Parse(ops, false);

		let mem_str = matches.value_of("mem-size").unwrap_or("1024");
		let mem_size = mem_str.parse::<usize>().unwrap();

		let ptr_str = matches.value_of("ptr-loc").unwrap_or("512");
		let ptr_loc = ptr_str.parse::<usize>().unwrap();

		let auto_alloc = matches.is_present("auto");

		if auto_alloc {
			BrainsuckMessage::throw_message("Auto allocation enabled".to_owned(), BrainsuckMessageType::Notification);
		}

        let mut memory: Vec<u8> = vec![0; mem_size];
        let mut memory_pointer: usize = ptr_loc;

        bs_lib::interpreter::Interpret(&program, &mut memory, &mut memory_pointer, auto_alloc, false)
    }
}
