
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
	IncrementPointer,
	DecrementPointer,
	Add,
	Subtract,
	Write,
	Read,
	LoopBegin,
	LoopEnd,
}

/*
	Instructions
	------------

	Optimized Brainfuck Instructions
	and extra Brainsuck Instructions
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
	IncrementPointer,
	DecrementPointer,
	Add,
	Subtract,
	Write,
	Read,
	Loop(Vec<Instruction>),
}
