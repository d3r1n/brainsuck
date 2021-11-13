use colored::{Color, Colorize};
use std::io;
use std::io::Write;
use std::process::exit;

pub struct BrainsuckError {
    message: String,
    error_type: BrainsuckErrorType,
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
            message,
            error_type,
        };
    }

    pub fn display(&self) {
        let err_type: String = match &self.error_type {
            BrainsuckErrorType::FileNotFoundError => String::from("File Not Found"),
            BrainsuckErrorType::CantReadFileError => String::from("Cant Read File"),
            BrainsuckErrorType::SyntaxError => String::from("Syntax Error"),
            BrainsuckErrorType::MemoryError => String::from("Memory Overflow"),
            BrainsuckErrorType::IOError => String::from("IO Error"),
        };

        let short_msg = format!(
            "\n{}\n---------\nError [{}]\n\nMessage: {}\n\n",
            "Brainsuck".bright_green(),
            err_type.bright_cyan(),
            &self.message.bright_yellow()
        )
        .bright_red();
        io::stdout().lock().write(short_msg.as_bytes()).unwrap();
    }

    pub fn throw_error(message: String, err_type: BrainsuckErrorType, do_exit: bool) {
        let new_error = BrainsuckError::new(message, err_type);
        new_error.display();
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
        return BrainsuckMessage {
            message,
            message_type,
        };
    }

    pub fn display(&self) {
        let message_type: String = match &self.message_type {
            BrainsuckMessageType::Notification => String::from("Notification"),
        };

        let short_msg = format!(
            "\n{}\n---------\nType: [{}]\n\nMessage: {}\n\n",
            "Brainsuck".bright_green(),
            message_type.bright_cyan(),
            &self.message.bright_yellow()
        )
        .bright_red();
        io::stdout().lock().write(short_msg.as_bytes()).unwrap();
    }

    pub fn throw_message(message: String, message_type: BrainsuckMessageType) {
        let new_error = BrainsuckMessage::new(message, message_type);
        new_error.display();
    }
}
