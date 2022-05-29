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
	},
	compiler::compiler::{Compiler, InstrType},
};

use crate::repl::repl;

use std::{
	fs::{File, self}, 
	io::{Read, Write}, 
	process::Command
};


pub fn handle() {
	// Command Line Handler
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
		.arg(
            Arg::with_name("compile")
                .short("c")
                .long("compile")
                .multiple(false)
                .help("If enabled, program will compiled and executed."),
        )
        .get_matches();

	// If the input file is not provided, we'll start the repl
    if matches.value_of("INPUT") == None {
		// Get optimize flag
		let optimize = matches.is_present("optimize");

		if optimize {
			// Notify user that optimization is enabled
			BrainsuckMessage::throw_message(
				"Optimization enabled".to_owned(),
				BrainsuckMessageType::Notification,
			);
		}
		
		// Display the welcome message
        print!("\n{}\n{}\nType \"{}\" or press CTRL + C to exit\n{}\n\n", 
			"Brainsuck Interactive Shell".bright_green(), 
			"-".repeat(40), 
			"quit".bright_magenta(), 
			"-".repeat(40)
		);

		// Start the repl
        repl(&optimize);
    } else {
		let input_file = matches.value_of("INPUT").unwrap();

		// Open the input file
        let mut file = File::open(input_file)
			.map_err(|_err| {
				// Throw an error if the file can't be opened
				BrainsuckError::throw_error(
					format!(
						"Can't find input file '{}'",
						input_file
					),
					BrainsuckErrorType::FileNotFoundError,
					true,
				)
			})
			.unwrap();

        let mut src = String::new();

		// Read the input file to "src" buffer
        file
            .read_to_string(&mut src)
            .map_err(|_err| {
				// Throw an error if file can't be read
                BrainsuckError::throw_error(
                    format!(
                        "Can't read the input file \"{}\"",
                        input_file
                    ),
                    BrainsuckErrorType::CantReadFileError,
                    true,
                )
            })
            .unwrap();

		// CLI inputs
        let mem_str = matches.value_of("mem-size").unwrap_or("1024");
        let ptr_str = matches.value_of("ptr-loc").unwrap_or("512");
		
		// Parsed CLI inputs
        let mem_size = mem_str.parse::<usize>().unwrap();
        let ptr_loc = ptr_str.parse::<usize>().unwrap();
		
		// CLI flags
        let auto_alloc = matches.is_present("auto");
		let optimize = matches.is_present("optimize");
		let compile = matches.is_present("compile");

		// If auto alloc is enabled and we're not compiling, we'll notify the user
        if auto_alloc && !compile {
            BrainsuckMessage::throw_message(
                "Auto allocation enabled".to_owned(),
                BrainsuckMessageType::Notification,
            );
        }

		if compile {
			// Output buffer (Rust) and "Compiler" instance
			let output: String;
			let mut compiler = Compiler::new(mem_size, ptr_loc);

			if optimize {
				// Notify user that optimization is enabled
				BrainsuckMessage::throw_message(
					"Optimization enabled".to_owned(),
					BrainsuckMessageType::Notification,
				);

				// Lex and parse the source code
				let tokens = optimized_lex(&src);
				let program = optimized_parse(&tokens, &false);

				// Compile the Optimized program and assign it to the output
				output = compiler.compile(InstrType::Optimized(program));
			}
			else {
				// Lex and parse the source code
				let tokens = lex(&src);
				let program = parse(&tokens, &false);

				// Compile the program and assign it to the output
				output = compiler.compile(InstrType::Normal(program));
			} 

			// Get the name of the file
			let filename = input_file.split('.').nth(0).unwrap();

			// Create a Rust file
			let mut rust_file = File::create(filename.to_string() + ".rs").map_err(|_err| {
				// Throw an error if the file can't be created
				BrainsuckError::throw_error(
					"Can't create output file".to_owned(),
					BrainsuckErrorType::CantCreateFileError,
					true,
				)
			}).unwrap();

			// Write the output to the Rust file
			rust_file.write_all(output.as_bytes()).map_err(|_err| {
				// Throw an error if the file can't be written
				BrainsuckError::throw_error(
					"Can't write to output file".to_owned(),
					BrainsuckErrorType::IOError,
					true,
				)
			}).unwrap();

			// Compile the Rust file via rustc
			Command::new("rustc")
			.arg("-A")
			.arg("warnings") // Disable all warnings
			.arg("-C")
			.arg("opt-level=3") // Custom optimization level
			.arg(filename.to_string() + ".rs") // filename of the rust file
			.spawn()
			.unwrap()
			.wait()
			.map_err(|_err| {
				// Throw an error if the file can't be compiled
				BrainsuckError::throw_error(
					"Can't compile output file".to_owned(),
					BrainsuckErrorType::CompileError,
					true,
				);

				// Remind the user to have Rust installed in their system
				BrainsuckMessage::throw_message(
					"If you don't have Rust installed in your system,\nYou have to install it in order to compile brainsuck".to_owned(), 
					BrainsuckMessageType::Notification
				)
			})
			.unwrap();

			// Remove the Rust file
			fs::remove_file(filename.to_string() + ".rs").map_err(|_err| {
				// Throw an error if the file can't be removed
				BrainsuckMessage::throw_message(
					"Can't remove output file".to_owned(),
					BrainsuckMessageType::Notification,
				);
			}).unwrap();
		}
		else {
			// Memory and pointer for interpreters
			let mut memory: Vec<u8> = vec![0; mem_size];
			let mut memory_pointer: usize = ptr_loc;

			if !optimize {
				// Lex and parse the source code
				let ops = lex(&src);
				let program = parse(&ops, &false);

				// Interpret the program
				interpret(
					&program,
					&mut memory,
					&mut memory_pointer,
					false,
					auto_alloc,
				)
			} else {
				// Lex, parse and optimize the source code
				let tokens = optimized_lex(&src);
				let program = optimized_parse(&tokens, &false);

				// Interpret the Optimized program
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
}
