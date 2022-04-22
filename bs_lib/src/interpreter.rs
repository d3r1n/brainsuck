use crate::{
	types::Instruction as I, 
	utils::{BrainsuckError, BrainsuckErrorType}
};
use std::{io::{self, Read}, vec};

pub fn alloc(memory: &mut Vec<u8>, amount: usize) {
	memory.resize(memory.len() + amount, 0);
}


pub fn interpret(instructions: &Vec<I>, memory: &mut Vec<u8>, pointer: &mut usize, repl_mode: bool, auto_allocate: bool) {
	
	for (_idx, instr) in instructions.iter().enumerate() {
		match instr {
			I::IncrementPointer => if *pointer + 1 > memory.len() - 1 {
				if auto_allocate {
					alloc(memory, 1);
					*pointer += 1;
				} 
				else {
					BrainsuckError::throw_error_with_help(
						format!("Memory owerflow at {}", pointer),
						"Give program more memory or\nenable auto-allocation".to_string(),
						BrainsuckErrorType::MemoryError,
						!repl_mode,
					);
				}
			} else {
				*pointer += 1;
			}

			I::DecrementPointer => if (*pointer as i32) - 1 < 0 {
				BrainsuckError::throw_error_with_help(
					"Memory pointer can't be negative".to_string(),
					format!("Decrementing pointer by 1\nat pointer location {}", pointer),
					BrainsuckErrorType::MemoryError,
					true,
				);
			} else {
				*pointer -= 1;
			}

			I::Add => if (memory[*pointer] as i32) + 1 > 255 {
				memory[*pointer] = 0;
			} else {
				memory[*pointer] += 1;
			},

			I::Subtract => if (memory[*pointer] as i32) - 1 < 0 {
				memory[*pointer] = 255;
			} else {
				memory[*pointer] -= 1;
			},

			I::Write => print!("{}", memory[*pointer] as char),

			I::Read => {
				let mut input = vec![0; 1];
				io::stdin().read(&mut input).unwrap();
				memory[*pointer] = input[0];
			},

			I::Loop(loop_instructions) => {
				while memory[*pointer] != 0 {
					interpret(&loop_instructions, memory, pointer, repl_mode, auto_allocate);
				}
			}
		}
	}
}