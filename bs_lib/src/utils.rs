use colored::{Color, Colorize};
use std::io::Write;
use std::io;
use std::process::exit;

pub struct BrainsuckError {
	Message: 	String,
	ErrorType: 		BrainsuckErrorType
}

pub enum BrainsuckErrorType {
	FileNotFoundError,
	CantReadFileError,

	SyntaxError,
	MemoryError,
	IOError,
}

impl BrainsuckError {
	pub fn new(message: String, error_type: BrainsuckErrorType) -> BrainsuckError {
		return BrainsuckError {
			Message: message,
			ErrorType: error_type,
		}
	}

	pub fn display(&self) {
		let err_type: String = match &self.ErrorType {
			BrainsuckErrorType::FileNotFoundError => String::from("File Not Found"),
			BrainsuckErrorType::CantReadFileError => String::from("Cant Read File"),
			BrainsuckErrorType::SyntaxError => String::from("Syntax Error"),
			BrainsuckErrorType::MemoryError => String::from("Memory Overflow"),
			BrainsuckErrorType::IOError => String::from("IO Error"),
		};

		let short_msg= format!("\n{}\n---------\nError [{}]\n\nMessage: {}\n\nExiting...\n", "Brainsuck".bright_green(), err_type.bright_cyan(), &self.Message.bright_yellow()).bright_red();
		io::stdout().write(short_msg.as_bytes()).unwrap();
	}

	pub fn throw_error(message: String, err_type: BrainsuckErrorType) {
		let new_error = BrainsuckError::new(message, err_type);
		new_error.display();
		exit(1);
	}
}