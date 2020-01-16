use super::operator;

/// The Token enum holds either a constant (stored as a 64-bit float) or an operator.
#[derive(Clone, Copy)]
pub enum Token {
	Constant(f64),
	Operator(operator::Operator),
}

/// The Expression struct holds a collection of tokens, and provides a variety of utility functions.
pub struct Expression {
	tokens: Vec<Token>,
}

impl Expression {
	/// Makes an `Expression` from a `Vec!` of `Tokens`.
	pub fn new(tokens: Vec<Token>) -> Expression {
		Expression { tokens }
	}
	
	/// Makes an `Expression` from a `Vec!` of `Tokens` organized using infix notation.
	/// 
	/// If you're not familiar with infix notation, it's the one you use all the time for simple calculations, like `2 * 5 + 2`.
	/// That expression would be sent to this function like so:
	/// ```rust
	/// let f = Expression::new_from_infix(Vec!(
	/// 	Token::Constant(2.),
	/// 	Token::Operator(Operator::Mul),
	/// 	Token::Constant(5.),
	/// 	Token::Operator(Operator::Add),
	/// 	Token::Constant(2.),
	/// ));
	/// ```
	/// The above code will return the same expression, but in reverse polish notation, so functionally equivalent to this:
	/// ```rust
	/// let f = Expression::new(Vec!(
	/// 	Token::Constant(2.),
	/// 	Token::Constant(5.),
	/// 	Token::Operator(Operator::Mul),
	/// 	Token::Constant(2.),
	/// 	Token::Operator(Operator::Add),
	/// ));
	/// ```
	pub fn new_from_infix(tokens: Vec<Token>) -> Expression {
		let mut result: Vec<Token> = Vec::new();
		
		for i in 0..tokens.len() {
			let token = tokens[i];
			
			match token {
				Token::Constant(c) => result.push(token),
				Token::Operator(o) => {
					let arg_length = o.get_arguments();
					
					// oops oh no
					
					// todo: actually write shunting-yard algorithm correctly
				}
			}
		}
		
		Expression { tokens: result }
	}
	
	/// Calculates an expression, returning a `f64`.
	pub fn calculate(&mut self) -> f64 {
		let mut stack: Vec<f64> = Vec::new();
		
		for i in 0..self.tokens.len() {
			let token = self.tokens[i];
			
			match token {
				Token::Constant(c) => stack.push(c),
				Token::Operator(o) => {
					let r = o.calculate(&mut stack);
					stack.push(r);
				},
			}
		}
		
		if stack.len() > 0 {
			stack[0]
		} else {
			0.0 // todo; replace with error
		}
	}
}
