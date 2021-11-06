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

		let short_msg= format!("\n{}\n---------\nError [{}]\n\nMessage: {}\n\n", "Brainsuck".bright_green(), err_type.bright_cyan(), &self.Message.bright_yellow()).bright_red();
		io::stdout().lock().write(short_msg.as_bytes()).unwrap();
	}

	pub fn throw_error(message: String, err_type: BrainsuckErrorType, do_exit: bool) {
		let new_error = BrainsuckError::new(message, err_type);
		new_error.display();
		if do_exit {exit(1);}
	}
}


pub struct BrainsuckMessage {
	Message: 	String,
	MessageType: BrainsuckMessageType,
}

pub enum BrainsuckMessageType {
	Notification
}

impl BrainsuckMessage {
	pub fn new(message: String, message_type: BrainsuckMessageType) -> BrainsuckMessage {
		return BrainsuckMessage {
			Message: message,
			MessageType: message_type,
		}
	}

	pub fn display(&self) {
		let message_type: String = match &self.MessageType {
			BrainsuckMessageType::Notification => String::from("Notification"),
		};

		let short_msg= format!("\n{}\n---------\nType: [{}]\n\nMessage: {}\n\n", "Brainsuck".bright_green(), message_type.bright_cyan(), &self.Message.bright_yellow()).bright_red();
		io::stdout().lock().write(short_msg.as_bytes()).unwrap();
	}

	pub fn throw_message(message: String, message_type: BrainsuckMessageType) {
		let new_error = BrainsuckMessage::new(message, message_type);
		new_error.display();
	}
}