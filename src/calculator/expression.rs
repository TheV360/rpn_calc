use std::collections::HashMap;
use regex::Regex;

use super::operator;

/// The Token enum holds either a constant (stored as a 64-bit float) or an operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
	Constant(f64),
	Operator(operator::Operator),
	Variable(char),
	Parenthesis(ParenthesisDirection),
	None,
}

/// Why not
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParenthesisDirection {
	Left, Right,
}

impl Token {
	pub fn is_value(&self) -> bool {
		match self {
			Token::Constant(_) | Token::Variable(_) => true,
			_ => false,
		}
	}
	
	pub fn is_operator(&self) -> bool {
		match self {
			Token::Operator(_) => true,
			_ => false,
		}
	}
}

/// The Expression struct holds a collection of tokens, and provides a variety of utility functions.
#[derive(Clone)]
pub struct Expression {
	tokens: Vec<Token>,
	variables: HashMap<char, f64>,
}

impl Expression {
	/// Makes an `Expression` from a `vec!` of `Tokens`.
	pub fn new(tokens: Vec<Token>) -> Expression {
		Expression { tokens, variables: HashMap::new() }
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
		
		for i in 0..tokens.len() {
			let token = tokens[i];
			let prev_token = { if i > 0 { tokens[i-1] } else { Token::None } };
			
			// Implicit multiplication match statement
			if (prev_token.is_value() || prev_token == Token::Parenthesis(ParenthesisDirection::Right))
			&& (token.is_value() || token == Token::Parenthesis(ParenthesisDirection::Left)) {
				op_stack.push(Token::Operator(operator::Operator::Mul));
			}
			
			// Shunting-yard algorithm match statement
			match token {
				Token::Constant(_) => result.push(token),
				Token::Variable(_) => result.push(token),
				Token::Operator(o) => {
					if !op_stack.is_empty() {
						let mut next_token = op_stack[op_stack.len() - 1];
						
						while match next_token {
							// Token::Function(_) => true,
							Token::Operator(o2) => 
								o2.get_precedence() > o.get_precedence() ||
								o2.get_precedence() == o.get_precedence() && o2.get_associativity() == operator::OperatorAssociativity::Left,
							_ => false,
						} {
							result.push(op_stack.pop().unwrap());
							if !op_stack.is_empty() {
								// Eek! This won't work.
								// or no, it will because of Copy and Clone.
								next_token = op_stack[op_stack.len() - 1];
							} else {
								break;
							}
						}
					}
					
					op_stack.push(token);
				},
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
		
		Ok(Expression { tokens: result, variables: HashMap::new() })
	}
	
	/// Makes a vec of infix tokens from a string. Useful for user-facing things.
	pub fn infix_tokens_from_str(input: &str) -> Result<Vec<Token>, &'static str> {
		//TODO: don't initialize Regex stuff every time.
		//TODO: variables could be better
		let mut result: Vec<Token> = Vec::new();
		
		let regex_variables = Regex::new("[^\\.\\d\\s\\+\\-\\*/%\\^\\(\\)]").unwrap(); // amazing
		// let regex_constants = Regex::new(r"(?:\-?)(?:(?:\d*\.\d+)|(?:\d+\.\d*)|(?:\d))").unwrap();
		let regex_constants = Regex::new(r"(?:\d*\.\d+)|(?:\d+\.\d*)|(?:\d+)").unwrap();
		let regex_operators = Regex::new(r"[\+\-\*/%\^]").unwrap();
		let regex_parenthesis = Regex::new("[\\(\\)]").unwrap(); //TODO: why doesn't this work as a raw string?
		
		// nope. not how you do this
		#[derive(Clone, Copy, PartialEq, Eq)]
		enum InfixStringRegexMatchesType {
			Variable, Constant, Operator, Parenthesis
		}
		struct InfixStringRegexMatches {
			start: usize,
			end: usize,
			token_type: InfixStringRegexMatchesType,
		}
		
		let mut matches: Vec<InfixStringRegexMatches> = Vec::new();
		let get_matches_for = |input: &str, matches: &mut Vec<InfixStringRegexMatches>, r: &Regex, token: InfixStringRegexMatchesType| {
			for cap in r.find_iter(input) {
				matches.push(InfixStringRegexMatches {
					start: cap.start(), end: cap.end(), token_type: token,
				});
			}
		};
		
		get_matches_for(input, &mut matches, &regex_variables, InfixStringRegexMatchesType::Variable);
		get_matches_for(input, &mut matches, &regex_constants, InfixStringRegexMatchesType::Constant);
		get_matches_for(input, &mut matches, &regex_operators, InfixStringRegexMatchesType::Operator);
		get_matches_for(input, &mut matches, &regex_parenthesis, InfixStringRegexMatchesType::Parenthesis);
		
		matches.sort_by(|m1: &InfixStringRegexMatches, m2: &InfixStringRegexMatches| m1.start.cmp(&m2.start));
		
		for cap in matches.iter() {
			match cap.token_type {
				InfixStringRegexMatchesType::Variable => {
					let variable = match (&input[cap.start..cap.end]).chars().next() {
						Some(v) => v,
						None => return Err("I think I saw a variable but it's gone now. How?!?!?!"),
					};
					
					result.push(Token::Variable(variable));
				},
				InfixStringRegexMatchesType::Constant => {
					let constant = match &input[cap.start..cap.end].parse::<f64>() {
						Ok(c) => *c,
						Err(_) => return Err("Could not parse f64."),
					};
					
					result.push(Token::Constant(constant));
				},
				InfixStringRegexMatchesType::Operator => {
					let operator = match &input[cap.start..cap.end] {
						"+" => operator::Operator::Add,
						"-" => operator::Operator::Sub,
						"*" => operator::Operator::Mul,
						"/" => operator::Operator::Div,
						"%" => operator::Operator::Mod,
						"^" => operator::Operator::Pow,
						_ => return Err("Could not parse operator. Unknown operator?"),
					};
					
					result.push(Token::Operator(operator));
				},
				InfixStringRegexMatchesType::Parenthesis => {
					result.push(Token::Parenthesis(match &input[cap.start..cap.end] {
						"(" => ParenthesisDirection::Left,
						")" => ParenthesisDirection::Right,
						_ => return Err("ah. very cool. Could not parse parenthesis."),
					}));
				},
			}
		}
		
		Ok(result)
	}
	
	// TODO: make builder?
	/// Sets the variable specified by the identifier to a f64 value. All variables must be set before calculation.
	pub fn set_variable(&mut self, identifier: char, value: f64) {
		self.variables.insert(identifier, value);
	}
	
	/// Simplifies an expression, looking for known values it can compute once.
	pub fn simplify(&mut self) -> Result<(), &'static str> {
		unimplemented!();
	}
	
	/// Calculates an expression, returning a `f64`.
	pub fn calculate(&self) -> Result<f64, &'static str> {
		let mut stack: Vec<f64> = Vec::new();
		
		for i in 0..self.tokens.len() {
			let token = self.tokens[i];
			
			match token {
				Token::Constant(c) => stack.push(c),
				Token::Operator(o) => {
					// todo: move most of this back into operator impl.
					let arg_length = o.get_arguments();
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
				Token::Variable(v) => {
					match self.variables.get(&v) {
						Some(val) => stack.push(*val),
						None => return Err("Undefined variable."),
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
