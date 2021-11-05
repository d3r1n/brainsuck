extern crate clap;
use clap::{Arg, App};


use bs_lib;
use bs_lib::utils::{BrainsuckError, BrainsuckErrorType};
use std::io::Read;
use std::fs::File;

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
		.arg(Arg::with_name("MEM_SIZE")
		.help("Size of the memory buffer")
		.index(2)
		)
		.arg(Arg::with_name("PTR_LOC")
		.help("Location of the starting memory pointer")
		.index(3)
		)
		.get_matches();

    if matches.value_of("INPUT") == None {
        println!("Not implemented yet!")
    } else {
        let mut file = File::open(matches.value_of("INPUT").unwrap())
		.map_err(|err| BrainsuckError::throw_error(format!("Can't find input file '{}'", matches.value_of("INPUT").unwrap()), BrainsuckErrorType::FileNotFoundError));

        let mut src = String::new();

        file.unwrap().read_to_string(&mut src)
		.map_err(|err| BrainsuckError::throw_error(format!("Can't read the input file '{}'", matches.value_of("INPUT").unwrap()), BrainsuckErrorType::CantReadFileError));

        let ops = bs_lib::lexer::Lex(src);
        let program = bs_lib::parser::Parse(ops);

		let mem_str = matches.value_of("MEM_SIZE").unwrap();
		let mem_size = mem_str.parse::<usize>().unwrap();

		let ptr_str = matches.value_of("PTR_LOC").unwrap();
		let ptr_loc = ptr_str.parse::<usize>().unwrap();

        let mut memory: Vec<u8> = vec![0; mem_size];
        let mut memory_pointer: usize = ptr_loc;

        bs_lib::interpreter::Interpret(&program, &mut memory, &mut memory_pointer)
    }
}
