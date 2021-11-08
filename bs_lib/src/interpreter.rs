use crate::types::{Instruction};
use crate::utils::{BrainsuckError, BrainsuckErrorType};

use std::io::{self, Read};

pub fn Interpret(instructions: &Vec<Instruction>, memory: &mut Vec<u8>, memory_pointer: &mut usize, auto_alloc: bool, repl_mode: bool) {

	for inst in instructions {
		
		if *memory_pointer - 1 > memory.len() || *memory_pointer - 1 > memory.len() + 1 {
			if auto_alloc {
				memory.resize(memory.len() + 512, 0);
			}
			else {
				BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError, repl_mode);
			}
		}

		match inst {
			Instruction::IncrementPointer 	=> {
				match memory.get(*memory_pointer + 1) {
					Some(v) => *memory_pointer += 1,
					None => BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError, !repl_mode)
				}
			},
			Instruction::DecrementPointer 	=> {
				match memory.get(*memory_pointer - 1) {
					Some(v) => *memory_pointer -= 1,
					None => BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError, !repl_mode)
				}
			},
			Instruction::Increment 			=> {
				match memory.get(*memory_pointer - 1) {
					Some(v) => memory[*memory_pointer] += 1,
					None => BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError, !repl_mode)
				}
			},
			Instruction::Decrement 			=> {
				match memory.get(*memory_pointer - 1) {
					Some(v) => memory[*memory_pointer] -= 1,
					None => BrainsuckError::throw_error("Memory overflow!\nHelp: Give program more memory.".to_owned(), BrainsuckErrorType::MemoryError, !repl_mode)
				}
			},
			Instruction::Write				=> {
				if repl_mode {
					print!("\n{}\n\n", memory[*memory_pointer] as char)
				}
				else {
					print!("{}", memory[*memory_pointer] as char)
				}
			},
			Instruction::Read				=> {
				let mut input_byte: [u8; 1] = [0; 1];

				match io::stdin().read_exact(&mut input_byte) {
					Ok(()) => (),
					Err(e) => BrainsuckError::throw_error("Can't read input form stdin".to_owned(), BrainsuckErrorType::IOError, repl_mode)				
				}

				memory[*memory_pointer] = input_byte[0];
			},
			Instruction::Loop(loop_instructions) => {
				while memory[*memory_pointer] != 0 {
					Interpret(&loop_instructions, memory, memory_pointer, true, false)
				}
			}
		}
	}
}