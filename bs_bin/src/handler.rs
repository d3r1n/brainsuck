extern crate clap;
use clap::{App, Arg};
use colored::Colorize;

use bs_lib::{
	utils::*,
	lexer::lex,
	parser::parse,
	interpreter::interpret,
	optimizer::{
		lexer::optimized_lex,
		parser::optimized_parse,
		interpreter::optimized_interpret,
	}
};

use std::fs::File;
use std::io::Read;

use crate::repl::repl;

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
                .help("Size of the memory buffer"),
        )
        .arg(
            Arg::with_name("ptr-loc")
                .short("p")
                .long("ptr-loc")
                .value_name("LOCATION")
                .takes_value(true)
                .help("Location of the memory pointer"),
        )
        .arg(
            Arg::with_name("auto")
                .short("a")
                .long("auto")
                .multiple(false)
                .help("If enabled, program will automatically allocate memory size"),
        )
		.arg(
            Arg::with_name("optimize")
                .short("o")
                .long("optimize")
                .multiple(false)
                .help("If enabled, program will optimize the application to make it faster"),
        )
        .get_matches();

    if matches.value_of("INPUT") == None {
        print!("\n{}\n-------------------------------------\nType \"{}\" or press CTRL + C to exit\n-------------------------------------\n\n", "Brainsuck Interactive Shell".bright_green(), "quit".bright_magenta());

        repl();
    } else {
        let file = File::open(matches.value_of("INPUT").unwrap()).map_err(|_err| {
            BrainsuckError::throw_error(
                format!(
                    "Can't find input file '{}'",
                    matches.value_of("INPUT").unwrap()
                ),
                BrainsuckErrorType::FileNotFoundError,
                true,
            )
        });

        let mut src = String::new();

        file.unwrap()
            .read_to_string(&mut src)
            .map_err(|_err| {
                BrainsuckError::throw_error(
                    format!(
                        "Can't read the input file '{}'",
                        matches.value_of("INPUT").unwrap()
                    ),
                    BrainsuckErrorType::CantReadFileError,
                    true,
                )
            })
            .unwrap();

        let mem_str = matches.value_of("mem-size").unwrap_or("1024");
        let mem_size = mem_str.parse::<usize>().unwrap();

        let ptr_str = matches.value_of("ptr-loc").unwrap_or("512");
        let ptr_loc = ptr_str.parse::<usize>().unwrap();

        let auto_alloc = matches.is_present("auto");
		let optimize = matches.is_present("optimize");

		let mut memory: Vec<u8> = vec![0; mem_size];
		let mut memory_pointer: usize = ptr_loc;

        if auto_alloc {
            BrainsuckMessage::throw_message(
                "Auto allocation enabled".to_owned(),
                BrainsuckMessageType::Notification,
            );
        }

		if !optimize {
			let ops = lex(&src);
			let program = parse(&ops, &false);

			interpret(
				&program,
				&mut memory,
				&mut memory_pointer,
				false,
				auto_alloc,
			)
		} else {
			let tokens = optimized_lex(&src);
			let program = optimized_parse(&tokens, &false);

			optimized_interpret(
				&program,
				&mut memory,
				&mut memory_pointer,
				false,
				auto_alloc,
			)
		}
		
    }
}
