use std::io;

mod calculator;
use crate::calculator::{expression, operator};

mod graph;
use crate::graph::window;

fn main() {
	println!("'graph' or 'calc'?");
	let mut input_buffer = String::new();
	
	input_buffer.clear();
	io::stdin().read_line(&mut input_buffer)
		.expect("Can't read.");
	
	if input_buffer.to_lowercase().trim().starts_with("g") {
		window::start();
	} else {
		worst_calculator();
	}
}

fn worst_calculator() {
	println!("Welcome to the worst calculator ever! Enter an infix expression.");
	println!("Enter = to calculate and end to restart.");
	loop {
		let mut tokens = Vec::<expression::Token>::new();
		let mut undefined_variables = Vec::<char>::new();
		let mut input_buffer = String::new();
		
		loop {
			println!("Enter the token type.");
			println!("Current tokens: {:?}", tokens);
			
			input_buffer.clear();
			io::stdin().read_line(&mut input_buffer)
				.expect("Can't read.");
			
			match input_buffer.trim() {
				"const" => {
					println!("Enter a constant.");
					
					input_buffer.clear();
					io::stdin().read_line(&mut input_buffer)
						.expect("Can't read.");
					
					let constant = match input_buffer.trim().parse::<f64>() {
						Ok(c) => c,
						Err(_) => {
							println!("Enter a valid constant.");
							continue;
						},
					};
					
					tokens.push(expression::Token::Constant(constant));
				},
				"var" => {
					println!("Enter a variable identifier.");
					
					input_buffer.clear();
					io::stdin().read_line(&mut input_buffer)
						.expect("Can't read.");
					
					let var_identifier = match input_buffer.trim().chars().next() {
						Some(v) => v,
						None => {
							println!("Enter a valid single-character identifier.");
							continue;
						}
					};
					
					tokens.push(expression::Token::Variable(var_identifier));
					
					if !undefined_variables.contains(&var_identifier) {
						undefined_variables.push(var_identifier);
					}
				},
				"op" => {
					println!("Enter an operator.");
					
					input_buffer.clear();
					io::stdin().read_line(&mut input_buffer).expect("Can't read.");
					
					let operator = match input_buffer.trim() {
						"+" => operator::Operator::Add,
						"-" => operator::Operator::Sub,
						"*" => operator::Operator::Mul,
						"/" => operator::Operator::Div,
						"%" => operator::Operator::Mod,
						"^" => operator::Operator::Pow,
						"v" => operator::Operator::Rot,
						_ => {
							println!("Enter a valid operator. you entered \"{}\"", input_buffer);
							continue;
						},
					};
					
					tokens.push(expression::Token::Operator(operator));
				},
				"(" => {
					tokens.push(expression::Token::Parenthesis(expression::ParenthesisDirection::Left));
				},
				")" => {
					tokens.push(expression::Token::Parenthesis(expression::ParenthesisDirection::Right));
				},
				"=" => {
					input_buffer.clear();
					io::stdin().read_line(&mut input_buffer)
						.expect("Can't read.");
					
					let resulting_expression = expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&input_buffer).unwrap());
					
					match resulting_expression {
						Ok(mut exp) => {
							println!("After converting to postfix:");
							exp.print();
							
							undefined_variables.reverse();
							while !undefined_variables.is_empty() {
								let i = undefined_variables.last().unwrap();
								println!("Enter a value for the {} variable.", i);
								
								input_buffer.clear();
								io::stdin().read_line(&mut input_buffer)
									.expect("Can't read.");
								
								let value = match input_buffer.trim().parse::<f64>() {
									Ok(v) => v,
									Err(_) => {
										println!("Enter a valid value.");
										continue;
									},
								};
								
								exp.set_variable(undefined_variables.pop().unwrap(), value);
							}
							
							match exp.calculate() {
								Ok(n) => {
									println!("expr = {}", n);
								},
								Err(e) => println!("Could not calculate expression. Error: {}", e),
							}
						},
						Err(e) => println!("Could not make expression. Error: {}", e),
					}
					
					break;
				},
				"end" => {
					break;
				},
				_ => {
					println!("Enter a valid token type.");
					continue;
				}
			}
		}
		
		println!("Calculate again? (y/N)");
		
		input_buffer.clear();
		io::stdin().read_line(&mut input_buffer)
			.expect("Can't read.");
		
		match input_buffer.to_lowercase().trim() {
			"y" => continue,
			_ => break,
		}
	}
	println!("Alright, bye!");
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn calculate_expression() {
		assert_eq!(expression::Expression::new(vec![
			expression::Token::Constant(2.0),
			expression::Token::Constant(4.0),
			expression::Token::Operator(operator::Operator::Mul),
		]).calculate().unwrap(), 8.0);
	}
	
	#[test]
	fn argument_order() {
		assert_eq!(expression::Expression::new(vec![
			expression::Token::Constant(1.0),
			expression::Token::Constant(2.0),
			expression::Token::Operator(operator::Operator::Div),
		]).calculate().unwrap(), 0.5);
	}
	
	#[test]
	fn infix_expression_order() {
		assert_eq!(expression::Expression::new_from_infix(vec![
			expression::Token::Constant(1.0),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Constant(2.0),
		]).unwrap().calculate().unwrap(), 0.5);
		
		let my_expression = expression::Expression::new_from_infix(vec![ // (2 + 5) * 6 ^ 2 = 252
			expression::Token::Parenthesis(expression::ParenthesisDirection::Left),
			expression::Token::Constant(2.0),
			expression::Token::Operator(operator::Operator::Add),
			expression::Token::Constant(5.0),
			expression::Token::Parenthesis(expression::ParenthesisDirection::Right),
			expression::Token::Operator(operator::Operator::Mul),
			expression::Token::Constant(6.0),
			expression::Token::Operator(operator::Operator::Pow),
			expression::Token::Constant(2.0),
		]).unwrap();
		my_expression.print();
		
		let my_other_expression = expression::Expression::new_from_infix(vec![ // 2 + 5 * 6 ^ 2 = 182
			expression::Token::Constant(2.0),
			expression::Token::Operator(operator::Operator::Add),
			expression::Token::Constant(5.0),
			expression::Token::Operator(operator::Operator::Mul),
			expression::Token::Constant(6.0),
			expression::Token::Operator(operator::Operator::Pow),
			expression::Token::Constant(2.0),
		]).unwrap();
		my_other_expression.print();
		
		assert_eq!(my_expression.calculate().unwrap(), 252.0);
		assert_eq!(my_other_expression.calculate().unwrap(), 182.0);
	}
	
	#[test]
	fn infix_expression_from_string() {
		let my_expression = expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str("2.0+(2^5)/8").unwrap()).unwrap();
		
		assert_eq!(my_expression.calculate().unwrap(), 6.0);
	}
	
	#[test]
	fn missing_left_paren() {
		assert!(expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str("2+2^5)/8").unwrap()).is_err());
	}
	
	#[test]
	fn missing_right_paren() {
		assert!(expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str("2+(2^5/8").unwrap()).is_err());
	}
	
	#[test]
	fn error_handling() {
		assert!(!(expression::Expression::new_from_infix(vec![
			expression::Token::Constant(1.0),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Constant(2.0),
		]).unwrap().calculate()).is_ok());
	}
	
	#[test]
	fn basic_simplify() {
	}
}
