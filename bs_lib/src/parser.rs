use crate::types::Instruction;
use crate::types::OpCode;
use crate::utils::{BrainsuckError, BrainsuckErrorType};

/*
    Parser
    ------

    Turns Opcodes to
    Instructions that can be used
    by the interpreter
*/

pub fn Parse(opcodes: Vec<OpCode>, repl_mode: bool) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_start: usize = 0;
    let mut loop_stack: i32 = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let inst = match op {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                OpCode::LoopEnd => {
                    BrainsuckError::throw_error(
                        format!("Loop ending at {} has no beginning", i),
                        BrainsuckErrorType::SyntaxError,
                        !repl_mode,
                    );
                    None
                }
            };

            match inst {
                Some(inst) => {
                    program.push(inst);
                }
                None => (),
            }
        } else {
            match op {
                OpCode::LoopBegin => loop_stack += 1,
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(Parse(
                            opcodes[loop_start + 1..i].to_vec(),
                            repl_mode,
                        )));
                    }
                }

                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        BrainsuckError::throw_error(
            format!("Loop starting at {} has no beginning", loop_start + 1),
            BrainsuckErrorType::SyntaxError,
            !repl_mode,
        );
    }

    return program;
}
