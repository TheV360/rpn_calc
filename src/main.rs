use std::io;

mod calculator;

use calculator::{expression, operator};

fn main() {
	println!("Welcome to the worst calculator ever! Enter an infix expression.");
	println!("Enter = to calculate and < to delete a token.");
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
						_ => {
							println!("Enter a valid operator. you entered \"{}\"", input_buffer);
							continue;
						},
					};
					
					tokens.push(expression::Token::Operator(operator));
				},
				"(" => {
					tokens.push(expression::Token::ParenthesisLeft);
				},
				")" => {
					tokens.push(expression::Token::ParenthesisRight);
				},
				"=" => {
					let resulting_expression = expression::Expression::new_from_infix(tokens);
					
					match resulting_expression {
						Ok(mut exp) => {
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
				"<" => {
					if !tokens.is_empty() {
						tokens.pop().unwrap();
					}
				},
				"exit" => {
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
	use super::calculator::{expression, operator};
	
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
			expression::Token::ParenthesisLeft,
			expression::Token::Constant(2.0),
			expression::Token::Operator(operator::Operator::Add),
			expression::Token::Constant(5.0),
			expression::Token::ParenthesisRight,
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
	fn error_handling() {
		assert!(!expression::Expression::new_from_infix(vec![
			expression::Token::Constant(1.0),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Constant(2.0),
		]).unwrap().calculate().is_ok());
	}
}
