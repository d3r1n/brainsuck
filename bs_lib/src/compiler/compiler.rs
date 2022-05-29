use crate::compiler::codegen::CodeGen;
use crate::types::Instruction;
use crate::optimizer::types::Instruction as OpInstruction;

pub struct Compiler {
	pub memory_size: usize,
	pub pointer: usize,
}

pub enum InstrType {
	Optimized(Vec<OpInstruction>),
	Normal(Vec<Instruction>),
}

impl Compiler {
	pub fn new(memory_size: usize, pointer: usize) -> Compiler {
		Compiler {
			memory_size,
			pointer,
		}
	}

	pub fn compile(&mut self, input: InstrType) -> String {
		let mut codegen = CodeGen::new();

		codegen.start(self.memory_size, self.pointer);
		codegen.program.push_str(&self.generate_code(&input).program);
		codegen.close();
		
		codegen.program
	}

	pub fn generate_code(&self, input: &InstrType) -> CodeGen {
		let mut codegen = CodeGen::new();

		match input {
			InstrType::Optimized(instruction) => {
				for instr in instruction.iter() {
					match instr {
						OpInstruction::Add(value) => codegen.add(value),
						OpInstruction::Subtract(value) => codegen.subtract(value),

						OpInstruction::IncrementPointer(value) => codegen.increment_pointer(value),
						OpInstruction::DecrementPointer(value) => codegen.decrement_pointer(value),

						OpInstruction::Write => codegen.write(),
						OpInstruction::Read => 	codegen.read(),
						
						OpInstruction::Loop(body) => {
							let code = self.generate_code(&InstrType::Optimized(body.clone()));
							
							codegen.add_loop(&format!("memory[pointer] != 0"), &code.program);
						},

						OpInstruction::Clear => codegen.clear(),
					}
				}
			},
			InstrType::Normal(instruction) => {
				for instr in instruction.iter() {
					match instr {
						Instruction::Add => codegen.add(&1),
						Instruction::Subtract => codegen.subtract(&1),
						
						Instruction::IncrementPointer => codegen.increment_pointer(&1),
						Instruction::DecrementPointer => codegen.decrement_pointer(&1),
						
						Instruction::Write => codegen.write(),
						Instruction::Read => codegen.read(),

						Instruction::Loop(body) => {
							let code = self.generate_code(&InstrType::Normal(body.clone()));
							
							codegen.add_loop(&format!("memory[pointer] != 0"), &code.program);
						},
					}
				}
			}
		}

		codegen
	}
}