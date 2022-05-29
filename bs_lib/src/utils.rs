use colored::Colorize;
use std::io;
use std::io::Write;
use std::process::exit;

pub struct BrainsuckError {
    message: String,
    error_type: BrainsuckErrorType,
}

pub enum BrainsuckErrorType {
    FileNotFoundError,
	CantCreateFileError,
    CantReadFileError,
	CompileError,

    SyntaxError,
    MemoryError,
    IOError,
}

impl BrainsuckError {
    pub fn new(message: String, error_type: BrainsuckErrorType) -> BrainsuckError {
        BrainsuckError {
            message,
            error_type,
        }
    }

    pub fn display(&self) {
        let err_type = match self.error_type {
            BrainsuckErrorType::FileNotFoundError => "File Not Found",
			BrainsuckErrorType::CantCreateFileError => "Can't Create File",
            BrainsuckErrorType::CantReadFileError => "Cant Read File",
			BrainsuckErrorType::CompileError => "Compile Error",

            BrainsuckErrorType::SyntaxError => "Syntax Error",
            BrainsuckErrorType::MemoryError => "Memory Overflow",
            BrainsuckErrorType::IOError => "IO Error",
        };

        let short_msg = format!(
            "\n{}\n---------\nError [{}]\n\nMessage: {}\n\n",
            "Brainsuck".bright_green(),
            err_type.bright_cyan(),
            &self.message.bright_yellow()
        )
        .bright_red();
        io::stdout().lock().write_all(short_msg.as_bytes()).unwrap();
    }

    pub fn throw_error(message: String, err_type: BrainsuckErrorType, do_exit: bool) {
        let new_error = BrainsuckError::new(message, err_type);
        new_error.display();
        if do_exit {
            exit(1);
        }
    }

	pub fn throw_error_with_help(message: String, help: String, err_type: BrainsuckErrorType, do_exit: bool) {
        let new_error = BrainsuckError::new(message, err_type);
        new_error.display();
		println!("{}: {}", "Help".bright_green(), help.bright_yellow());
        if do_exit {
            exit(1);
        }
    }
}

pub struct BrainsuckMessage {
    message: String,
    message_type: BrainsuckMessageType,
}

pub enum BrainsuckMessageType {
    Notification,
}

impl BrainsuckMessage {
    pub fn new(message: String, message_type: BrainsuckMessageType) -> BrainsuckMessage {
        BrainsuckMessage {
            message,
            message_type,
        }
    }

    pub fn display(&self) {
        let message_type = match self.message_type {
            BrainsuckMessageType::Notification => "Notification",
        };

        let short_msg = format!(
            "\n{}\n---------\nType: [{}]\n\nMessage: {}\n\n",
            "Brainsuck".bright_green(),
            message_type.bright_cyan(),
            &self.message.bright_yellow()
        )
        .bright_red();
        io::stdout().lock().write_all(short_msg.as_bytes()).unwrap();
    }

    pub fn throw_message(message: String, message_type: BrainsuckMessageType) {
        let new_error = BrainsuckMessage::new(message, message_type);
        new_error.display();
    }
}
