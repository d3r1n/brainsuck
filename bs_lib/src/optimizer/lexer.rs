use crate::optimizer::types::*;

/*
    Lexer
    -----

    Turns String based keywordss
    to a Vector of Tokens.
*/

pub fn optimized_lex(source: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();

	for token in source.chars() {
		let token = match token {
			'.' => Some(Token::Dot),
			'+' => Some(Token::Plus),
			'-' => Some(Token::Minus),
			',' => Some(Token::Comma),
			'<' => Some(Token::LeftArrow),
			'>' => Some(Token::RightArrow),
			'[' => Some(Token::LeftBracket),
			']' => Some(Token::RightBracket),
			_ => None,
		};

		if let Some(token) = token {
			tokens.push(token);
		}
	}

	tokens
}