use crate::{types::{OpCode as Op, Instruction as I}, utils::{BrainsuckError, BrainsuckErrorType}};

/*
	Parser
	------

	Turns a Vector of Ops
	to a Vector of Statements.

*/

pub fn parse(opcodes: &[Op], repl_mode: &bool) -> Vec<I> {
    let mut program: Vec<I> = Vec::new();
    let mut loop_start: usize = 0;
    let mut loop_stack: i32 = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let inst = match op {
                Op::IncrementPointer => Some(I::IncrementPointer),
                Op::DecrementPointer => Some(I::DecrementPointer),
                Op::Add => Some(I::Add),
                Op::Subtract => Some(I::Subtract),
                Op::Write => Some(I::Write),
                Op::Read => Some(I::Read),

                Op::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                Op::LoopEnd => {
                    BrainsuckError::throw_error(
                        format!("Loop ending at {} has no beginning", i),
                        BrainsuckErrorType::SyntaxError,
                        !repl_mode,
                    );
                    None
                }
            };

            if let Some(inst) = inst {
                program.push(inst);
            }
        } else {
            match op {
                Op::LoopBegin => loop_stack += 1,
                Op::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(I::Loop(parse(
                            &opcodes[loop_start + 1..i],
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

    program
}
