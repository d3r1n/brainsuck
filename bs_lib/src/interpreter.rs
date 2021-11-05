use crate::types::{Instruction};
use crate::utils::{BrainsuckError, BrainsuckErrorType};

use std::io::{self, Read};

pub fn Interpret(instructions: &Vec<Instruction>, memory: &mut Vec<u8>, memory_pointer: &mut usize) {
	for inst in instructions {
		
		if *memory_pointer > memory.len() || *memory_pointer > memory.len() + 1 {
			BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError);
		}

		match inst {
			Instruction::IncrementPointer 	=> *memory_pointer += 1,
			Instruction::DecrementPointer 	=> *memory_pointer -= 1,
			Instruction::Increment 			=> memory[*memory_pointer] += 1,
			Instruction::Decrement 			=> memory[*memory_pointer] -= 1,
			Instruction::Write				=> print!("{}", memory[*memory_pointer] as char),
			Instruction::Read				=> {
				let mut input_byte: [u8; 1] = [0; 1];

				match io::stdin().read_exact(&mut input_byte) {
					Ok(()) => (),
					Err(e) => BrainsuckError::throw_error("Can't read input form stdin".to_owned(), BrainsuckErrorType::IOError)
				}

				memory[*memory_pointer] = input_byte[0];
			},
			Instruction::Loop(loop_instructions) => {
				while memory[*memory_pointer] != 0 {
					Interpret(&loop_instructions, memory, memory_pointer)
				}
			}
		}
	}
}