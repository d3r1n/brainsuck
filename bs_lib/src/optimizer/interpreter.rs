use super::types::Instruction;
use crate::utils::*;
use std::{
    io::{self, Read},
    vec,
};

pub fn alloc(memory: &mut Vec<u8>, amount: usize) {
    memory.resize(memory.len() + amount, 0);
}

pub fn optimized_interpret(
    instructions: &Vec<Instruction>,
    memory: &mut Vec<u8>,
    pointer: &mut usize,
    repl_mode: bool,
    auto_allocate: bool,
) {
    for instr in instructions.iter() {
        match instr {
            Instruction::IncrementPointer(x) => {
                if *pointer + *x > memory.len() - 1 {
                    if auto_allocate {
                        alloc(memory, *x);
                        *pointer += *x;
                    } else {
                        BrainsuckError::throw_error_with_help(
                            format!("Memory owerflow at {}", pointer),
                            "Give program more memory or\nenable auto-allocation".to_string(),
                            BrainsuckErrorType::MemoryError,
                            !repl_mode,
                        );
                    }
                } else {
                    *pointer += *x;
                }
            }

            Instruction::DecrementPointer(x) => {
                if (*pointer as i32) - (*x as i32) < 0 {
                    BrainsuckError::throw_error_with_help(
                        "Memory pointer can't be negative".to_string(),
                        format!(
                            "Decrementing pointer by {}\nat pointer location {}",
                            x, pointer
                        ),
                        BrainsuckErrorType::MemoryError,
                        true,
                    );
                } else {
                    *pointer -= *x;
                }
            }

            Instruction::Add(x) => {
                if (memory[*pointer] as i32) + (*x as i32) > 255 {
                    memory[*pointer] = ((memory[*pointer] as i32) + (*x as i32) - 255) as u8;
                } else {
                    memory[*pointer] += *x as u8;
                }
            }

            Instruction::Subtract(x) => {
                if (memory[*pointer] as i32) - (*x as i32) < 0 {
                    memory[*pointer] = ((memory[*pointer] as i32) - (*x as i32) + 255) as u8;
                } else {
                    memory[*pointer] -= *x as u8;
                }
            }

            Instruction::Write => print!("{}", memory[*pointer] as char),

            Instruction::Read => {
                let mut input = vec![0; 1];
                io::stdin().read(&mut input).unwrap();
                memory[*pointer] = input[0];
            }

            Instruction::Loop(loop_instructions) => {
                while memory[*pointer] != 0 {
                    optimized_interpret(
                        &loop_instructions,
                        memory,
                        pointer,
                        repl_mode,
                        auto_allocate,
                    );
                }
            }

            Instruction::Clear => memory[*pointer] = 0,
        }
    }
}
