#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Dot,
	Plus,
	Minus,
	Comma,
	LeftArrow,
	RightArrow,
	LeftBracket,
	RightBracket,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
	Read,
	Write,
	Clear,
	Add(usize),
	Subtract(usize),
	Loop(Vec<Instruction>),
	IncrementPointer(usize),
	DecrementPointer(usize),
}