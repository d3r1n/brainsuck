use crate::types::OpCode;

/*
    Lexer
    -----

    Turns String based keywords
    to a Vector of OpCodes.
*/

pub fn lex(source: String) -> Vec<OpCode> {
    let mut operations: Vec<OpCode> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut current_char: usize = 0;

    while current_char < chars.len() {
        let op = match chars[current_char] {
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

        current_char += 1;
    }

    operations
}
