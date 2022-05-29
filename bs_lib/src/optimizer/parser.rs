use super::types::*;
use crate::utils::*;

/*
	Parser
	-----
	Parses a vector of tokens 
	into a vector of Instructions.
*/

pub fn optimized_parse(tokens: &[Token], repl_mode: &bool) -> Vec<Instruction> {
	let mut result: Vec<Instruction> = Vec::new();

	let mut loop_stack: usize = 0;
	let mut loop_start: usize = 0;

	// Cache for saving the tokens if they are not a part of a loop
	let mut cache: Vec<Token> = Vec::new();

	for (idx, token) in tokens.iter().enumerate() {
		if loop_stack == 0 {
			if token == &Token::LeftBracket {
				loop_stack += 1;
				loop_start = idx;
				
				// Append the result with combined cache then clear the cache
				// if cache is not empty
				if cache.len() > 0 {
					result.append(&mut combine_tokens(&cache).to_vec());
					cache.clear();
				}
			} else if token == &Token::RightBracket {
				BrainsuckError::throw_error(
					format!("Loop ending at {} has no beginning", idx),
					BrainsuckErrorType::SyntaxError,
					!repl_mode,
				);
			} else {
				cache.push(token.clone());
			}
		}
		else {
			match token {
                Token::LeftBracket => loop_stack += 1,
                Token::RightBracket => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
						result.push(
							Instruction::Loop(
								optimized_parse(&tokens[loop_start + 1..idx], repl_mode)
							)
						);
                    }
                }

                _ => (),
            }
		}

	}

	// If there is still tokens in the cache, append them to the result
	if cache.len() > 0 {
		result.append(&mut combine_tokens(&cache).to_vec());
	}

	if loop_stack != 0 {
        BrainsuckError::throw_error(
            format!("Loop starting at {} has no beginning", loop_start + 1),
            BrainsuckErrorType::SyntaxError,
            !repl_mode,
        );
    }

	result
	
}

/*
	Combine Tokens
	--------------

	Combines multiple repeating tokens into one Instruction
	to make the program faster. (Significantly, like 100x faster)

	Example:
		input = +++[-->>]<+
		output = [Add(3), Loop([Subtract(2), IncrementPointer(2)]), DecrementPointer(1), Add(1)]
*/

pub fn combine_tokens(tokens: &[Token]) -> Vec<Instruction> {
	let mut result: Vec<Instruction> = Vec::new();

	// Save the tokens as a tuple of (Token, usize)
	// to turn them into Instruction later
	let mut combined_tokens: Vec<(&Token, usize)> = Vec::new();

	// Variables for saving the last token and its amount
	let mut last_token: &Token = &tokens[0];
	let mut last_count: usize = 0;

	for token in tokens {
		if last_token == token {
			last_count += 1;
		} else {
			combined_tokens.push((last_token, last_count));
			last_token = token;
			last_count = 1;
		}
	}

	// Push the remaining token and its amount
	combined_tokens.push((last_token, last_count));

	// Turn (Token, usize) tuples into Instructions
	for (token, count) in combined_tokens {
		let instr: Option<Instruction> = match token {
			Token::Dot => Some(Instruction::Write),
			Token::Plus => Some(Instruction::Add(count)),
			Token::Minus => Some(Instruction::Subtract(count)),
			Token::Comma => Some(Instruction::Read),
			Token::LeftArrow => Some(Instruction::DecrementPointer(count)),
			Token::RightArrow => Some(Instruction::IncrementPointer(count)),
			Token::LeftBracket => None,
			Token::RightBracket => None,
		};

		if let Some(instr) = instr {
			result.push(instr);
		}
	}

	return result;
}


// TODO: Optimize Multiply and Clear loops
// pub fn parse_optimized_loops(tokens: Vec<Token>) -> Vec<Instruction> {
// 	let mut result: Vec<Instruction> = Vec::new();

// 	match tokens[..] {
// 		[Token::RightBracket, Token::Minus, Token::LeftBracket] => result.push(Instruction::Clear),
// 		_ => (),
// 	}

// 	return result;
// }

#[cfg(test)]
mod tests {
    use crate::optimizer::types::Instruction as I;
	use crate::optimizer::lexer::optimized_lex;

    #[test]
    fn test_parser() {
        let source = "+++[-->>]<+[[-+]]";
		let tokens = optimized_lex(source);
		let result = super::optimized_parse(&tokens, &true);
		assert_eq!(result, vec![
			I::Add(3), 
			I::Loop(vec![
				I::Subtract(2), I::IncrementPointer(2)
			]), 
			I::DecrementPointer(1), 
			I::Add(1), 
			I::Loop(vec![
				I::Loop(vec![
					I::Subtract(1), I::Add(1)
				])]
			)
		])
	}
}