/*
	Operation Codes
	---------------

	Normal Brainfuck opcodes 
	and extra Brainsuck opcodes
*/

#[derive(Debug)]
#[derive(Clone)]
pub enum OpCode {
	IncrementPointer,
	DecrementPointer,
	Increment,
	Decrement,
	Write,
	Read,
	LoopBegin,
	LoopEnd,

	// Comment,
}

/*
	Instructions
	------------

	Normal Brainfuck Instructions 
	and extra Brainsuck Instructions
*/

#[derive(Debug)]
#[derive(Clone)]
pub enum Instruction {
	IncrementPointer,
	DecrementPointer,
	Increment,
	Decrement,
	Write,
	Read,
	Loop(Vec<Instruction>)	
}