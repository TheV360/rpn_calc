use std::collections::HashMap;
use core::convert::TryFrom;

use super::operator;

/// The Token enum holds a variety of types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
	Constant(f64),
	Variable(char),
	Operator(operator::Operator),
	Function(operator::Function),
	Parenthesis(ParenthesisDirection),
	Comma,
}

/// Why not
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParenthesisDirection {
	Left, Right,
}
impl TryFrom<&str> for ParenthesisDirection {
	type Error = &'static str;
	
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"(" => Ok(ParenthesisDirection::Left),
			")" => Ok(ParenthesisDirection::Right),
			_ => Err("Scary error - could not parse parenthesis."),
		}
	}
}

impl Token {
	pub fn is_value(&self) -> bool {
		matches!(self, Token::Constant(_) | Token::Variable(_))
	}
	
	pub fn is_operator(&self) -> bool {
		matches!(self, Token::Operator(_))
	}
	
	pub fn is_function(&self) -> bool {
		matches!(self, Token::Function(_))
	}
}

pub type ExpressionVariables = HashMap<char, f64>;

/// The Expression struct holds a collection of tokens, and provides a variety of utility functions.
#[derive(Clone)]
pub struct Expression {
	tokens: Vec<Token>,
}

impl Expression {
	/// Makes an `Expression` from a vector of `Token`s.
	pub fn new(tokens: Vec<Token>) -> Expression {
		Expression { tokens }
	}
	
	/// Makes an `Expression` from a `vec!` of `Tokens` organized using infix notation.
	/// 
	/// If you're not familiar with infix notation, it's the one you use all the time for simple calculations, like `2 * 5 + 2`.
	/// That expression would be sent to this function like so:
	/// ```rust
	/// let f = Expression::new_from_infix(vec![
	/// 	Token::Constant(2.0),
	/// 	Token::Operator(operator::Operator::Mul),
	/// 	Token::Constant(5.0),
	/// 	Token::Operator(operator::Operator::Add),
	/// 	Token::Constant(2.0),
	/// ]);
	/// ```
	/// The above code will return the same expression, but in reverse polish notation, so functionally equivalent to this:
	/// ```rust
	/// let f = Expression::new(vec![
	/// 	Token::Constant(2.0),
	/// 	Token::Constant(5.0),
	/// 	Token::Operator(operator::Operator::Mul),
	/// 	Token::Constant(2.0),
	/// 	Token::Operator(operator::Operator::Add),
	/// ]);
	/// ```
	pub fn new_from_infix(tokens: Vec<Token>) -> Result<Expression, &'static str> {
		let mut op_stack: Vec<Token> = Vec::new();
		let mut result: Vec<Token> = Vec::new();
		
		let tokens = Expression::process_implicit_tokens(&tokens);
		
		for token in tokens.iter().cloned() {
			// Shunting-yard algorithm match statement
			match token {
				Token::Constant(_) => result.push(token),
				Token::Variable(_) => result.push(token),
				Token::Operator(o) => {
					if !op_stack.is_empty() {
						let mut next_token = op_stack[op_stack.len() - 1];
						
						// Very pretty code.
						while match next_token {
							Token::Function(_) => true,
							Token::Operator(o2) => 
								o2.get_precedence() > o.get_precedence() ||
								o2.get_precedence() == o.get_precedence() && o2.get_associativity() == operator::OperatorAssociativity::Left,
							_ => false,
						} {
							result.push(op_stack.pop().unwrap());
							if !op_stack.is_empty() {
								next_token = op_stack[op_stack.len() - 1];
							} else {
								break;
							}
						}
					}
					
					op_stack.push(token);
				},
				Token::Function(_) => op_stack.push(token),
				Token::Parenthesis(d) => match d {
					ParenthesisDirection::Left => op_stack.push(token),
					ParenthesisDirection::Right => {
						// Search through the stack for a left parenthesis.
						loop {
							let op = op_stack.pop().unwrap();
							
							if op == Token::Parenthesis(ParenthesisDirection::Left) {
								break;
							} else {
								result.push(op);
								if op_stack.is_empty() {
									return Err("Missing left parentheses.");
								}
							}
						}
					},
				},
				_ => {},
			}
		}
		
		// Dump rest of op_stack onto the result.
		while !op_stack.is_empty() {
			let op = op_stack.pop().unwrap();
			
			// If there was a parenthesis, somebody screwed up.
			// Those should've been consumed a long time ago.
			if op == Token::Parenthesis(ParenthesisDirection::Left) {
				return Err("Missing right parentheses.");
			} else {
				result.push(op);
			}
		}
		
		Ok(Expression { tokens: result })
	}
	
	/// Adds implicit multiplication stuff. Only used internally.
	fn process_implicit_tokens(input: &Vec<Token>) -> Vec<Token> {
		let mut result: Vec<Token> = Vec::new();
		
		for i in 0..input.len() {
			let token = input[i];
			let prev_token = if i > 0 {
				Some(input[i - 1])
			} else {
				None
			};
			
			if let Some(prev_token) = prev_token {
				// TODO: make this look less awkward. or maybe just split Pi and E off into a "constants" token type.
				if (prev_token.is_value() || prev_token == Token::Parenthesis(ParenthesisDirection::Right) ||
					(prev_token.is_function() && match prev_token { Token::Function(f) => f.get_parameters() < 1, _ => false, })
				)
				&& (token.is_value() || token.is_function() || token == Token::Parenthesis(ParenthesisDirection::Left)) {
					result.push(Token::Operator(operator::Operator::Mul));
				}
			}
			
			result.push(token);
		}
		
		result
	}
	
	/// Makes a vector of infix tokens from a string. Useful for user-facing things.
	pub fn infix_tokens_from_str(input: &str) -> Result<Vec<Token>, &'static str> {
		//TODO: don't initialize Regex stuff every time.
		let mut result: Vec<Token> = Vec::new();
		
		let big_regex = regex::Regex::new(r"(?x) # Order of these lines determines the priority.
			 (\(|\))                             # Matches any parenthesis.
			|((?:\d*\.\d+)|(?:\d+\.\d*)|(?:\d+)) # Matches any constants.
			|(\+|\-|\*|/|%|\^)                   # Matches any operators.
			|(sin|cos|tan|csc|sec|cot|log|ln|abs|sgn|Pi|E) # Matches any functions.
			|(\S)                                # Matches any variables.
		").unwrap();
		
		// TODO: this sucks. I didn't want to think about anything while doing this, and it shows.
		#[derive(Clone, Copy, PartialEq, Eq)]
		enum InfixStringRegexMatchesType {
			Parenthesis, Constant, Operator, Function, Variable,
		}
		struct InfixStringRegexMatches {
			start: usize,
			end: usize,
			token_type: InfixStringRegexMatchesType,
		}
		
		let mut matches: Vec<InfixStringRegexMatches> = Vec::new();
		for cap in big_regex.captures_iter(input) {
			for i in 1..cap.len() {
				if let Some(c) = cap.get(i) {
					matches.push(InfixStringRegexMatches {
						start: c.start(),
						end: c.end(),
						token_type: match i {
							// Ewwww
							1 => InfixStringRegexMatchesType::Parenthesis,
							2 => InfixStringRegexMatchesType::Constant, 3 => InfixStringRegexMatchesType::Operator,
							4 => InfixStringRegexMatchesType::Function, 5 => InfixStringRegexMatchesType::Variable,
							_ => panic!("uhhhhhh what"),
						},
					});
					break;
				}
			}
		}
		
		for cap in matches.iter() {
			let tmp = &input[cap.start..cap.end];
			match cap.token_type {
				InfixStringRegexMatchesType::Variable => {
					match tmp.chars().next() {
						Some(v) => result.push(Token::Variable(v)),
						None => return Err("Scary error - could not parse variable."),
					}
				},
				InfixStringRegexMatchesType::Constant => {
					match tmp.parse::<f64>() {
						Ok(c) => result.push(Token::Constant(c)),
						Err(_) => return Err("Could not parse f64."),
					}
				},
				InfixStringRegexMatchesType::Operator => result.push(Token::Operator(operator::Operator::try_from(tmp)?)),
				InfixStringRegexMatchesType::Function => result.push(Token::Function(operator::Function::try_from(tmp)?)),
				InfixStringRegexMatchesType::Parenthesis => result.push(Token::Parenthesis(ParenthesisDirection::try_from(tmp)?)),
			}
		}
		
		Ok(result)
	}
	
	// /// Simplifies an expression, looking for known values it can compute once.
	// pub fn simplify(&mut self) -> Result<(), &'static str> {
	// 	unimplemented!();
	// }
	
	/// Calculates an expression, returning a `f64`. If you want to use variables, you can also pass Some variables.
	pub fn calculate(&self, variables: Option<&ExpressionVariables>) -> Result<f64, &'static str> {
		let mut stack: Vec<f64> = Vec::new();
		
		for i in 0..self.tokens.len() {
			let token = self.tokens[i];
			
			match token {
				Token::Constant(c) => stack.push(c),
				Token::Operator(o) => {
					// TODO: move most of this back into operator impl.
					let arg_length = o.get_parameters(); // oops
					let mut args = Vec::new();
					
					if stack.len() < arg_length {
						return Err("Not enough arguments.");
					}
					
					// Get arguments from stack.
					for _ in 0..arg_length {
						args.push(stack.pop().unwrap());
					}
					// It has to be reversed because reasons.
					args.reverse();
					
					let result = o.calculate(args)?;
					stack.push(result);
				},
				Token::Function(f) => {
					// TODO: ewwwwww do not like just copy-pasting this
					let arg_length = f.get_parameters(); // oops
					let mut args = Vec::new();
					
					if stack.len() < arg_length {
						return Err("Not enough arguments.");
					}
					
					// Get arguments from stack.
					for _ in 0..arg_length {
						args.push(stack.pop().unwrap());
					}
					// It has to be reversed because reasons.
					args.reverse();
					
					let result = f.calculate(args)?;
					stack.push(result);
				},
				Token::Variable(v) => {
					if let Some(variables) = variables {
						match variables.get(&v) {
							Some(val) => stack.push(*val),
							None => return Err("Undefined variable."),
						}
					} else {
						return Err("Encountered variable without Some ExpressionVariables.");
					}
				},
				_ => {},
			}
		}
		
		if !stack.is_empty() {
			if stack.len() > 1 { // this is unnecessary but I think it helps sometimes
				Err("Too many leftover results.")
			} else {
				Ok(stack[0])
			}
		} else {
			Err("No calculation result.")
		}
	}
	
	/// Debug garbage
	pub fn print(&self) {
		println!("{:?}", self.tokens);
	}
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expr {:?}", self.tokens)
    }
}
