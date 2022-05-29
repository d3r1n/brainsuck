pub struct CodeGen {
	pub program: String
}

impl CodeGen{
	pub fn new() -> CodeGen {
		CodeGen {
			program: String::new()
		}
	}

	pub fn start(&mut self, memory_size: usize, pointer: usize) {
		let boilerplate = format!("
use std::{{io::{{self, Read}}}};

fn main() {{
	let mut memory: Vec<u8> = vec![0; {}];
	let mut pointer: usize = {};
	let mut input: Vec<u8>;

", memory_size, pointer);

		self.program.push_str(&boilerplate);
	}

	pub fn close(&mut self) {
		self.program.push_str("}");
	}

	pub fn add_loop(&mut self, condition: &str, body: &str) {
		self.program.push_str(&format!("while {} {{\n{}\n}}\n", condition, body));
	}

	pub fn add(&mut self, value: &usize) {
		self.program.push_str(&format!("memory[pointer] += {};\n", value));
	}

	pub fn subtract(&mut self, value: &usize) {
		self.program.push_str(&format!("memory[pointer] -= {};\n", value));
	}

	pub fn increment_pointer(&mut self, value: &usize) {
		self.program.push_str(&format!("pointer += {};\n", value));
	}

	pub fn decrement_pointer(&mut self, value: &usize) {
		self.program.push_str(&format!("pointer -= {};\n", value));
	}

	pub fn clear(&mut self) {
		self.program.push_str("memory[pointer] = 0;\n");
	}

	pub fn write(&mut self) {
		self.program.push_str("print!(\"{}\", memory[pointer] as char);\n");
	}

	pub fn read(&mut self) {
		self.program.push_str("input = vec![0; 1];\n");
		self.program.push_str("io::stdin().read(&mut input).unwrap();\n");
		self.program.push_str("memory[pointer] = input[0];\n");
	}
}