use crate::types::OpCode;

/*
    Lexer
    -----

    Turns String based keywords
    to a Vector of OpCodes.
*/

pub fn lex(source: &str) -> Vec<OpCode> {
    let mut operations: Vec<OpCode> = Vec::new();

    for current_char in source.chars() {
        let op = match current_char {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None,
        };

        if let Some(op) = op {
            operations.push(op);
        }
    }

    operations
}
