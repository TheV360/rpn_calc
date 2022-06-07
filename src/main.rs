#![allow(dead_code)]

// petty
#![allow(clippy::tabs_in_doc_comments)]

use std::io;

mod util;

mod calculator;
use crate::calculator::expression;

mod graph;
use crate::graph::window;

fn main() {
	/*let mut input_buffer = String::new();
	
	input_buffer.clear();
	io::stdin().read_line(&mut input_buffer)
		.expect("Can't read.");
	
	if input_buffer.to_lowercase().trim().starts_with("g") {*/
		window::start();
	/*} else {
		//worst_calculator();
		best_calc();
	}*/
}

fn best_calc() {
	println!("Slap an expression in. I evaluate it.");
	
	let mut input_buffer = String::new();
	loop {
		input_buffer.clear();
		io::stdin().read_line(&mut input_buffer)
			.expect("Can't read.");
		
		let tokens = match expression::Expression::infix_tokens_from_str(&input_buffer) {
			Ok(t) => {
				println!("infix tokens: {:?}", t);
				t
			},
			Err(e) => {
				println!("Couldn't read! Error: {}", e);
				continue;
			}
		};
		
		let expr = match expression::Expression::new_from_infix(tokens) {
			Ok(ex) => {
				println!("to expression: {:?}", ex);
				ex
			},
			Err(e) => {
				println!("Couldn't make expression! Error: {}", e);
				continue;
			}
		};
		
		match expr.calculate(None) {
			Ok(r) => println!("= {}", r),
			Err(e) => println!("Couldn't calculate! Error: {}", e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::calculator::operator;
	
	#[test]
	fn calculate_expression() {
		assert_eq!(expression::Expression::new(vec![
			expression::Token::Constant(2.0),
			expression::Token::Constant(4.0),
			expression::Token::Operator(operator::Operator::Mul),
		]).calculate(None).unwrap(), 8.0);
	}
	
	#[test]
	fn argument_order() {
		assert_eq!(expression::Expression::new(vec![
			expression::Token::Constant(1.0),
			expression::Token::Constant(2.0),
			expression::Token::Operator(operator::Operator::Div),
		]).calculate(None).unwrap(), 0.5);
	}
	
	#[test]
	fn infix_expression_order() {
		assert_eq!(expression::Expression::new_from_infix(vec![
			expression::Token::Constant(1.0),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Constant(2.0),
		]).unwrap().calculate(None).unwrap(), 0.5);
		
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
		
		assert_eq!(my_expression.calculate(None).unwrap(), 252.0);
		assert_eq!(my_other_expression.calculate(None).unwrap(), 182.0);
	}
	
	#[test]
	fn infix_expression_from_string() {
		let my_expression = expression::Expression::new_from_infix(
			expression::Expression::infix_tokens_from_str("2.0+(2^5)/8").unwrap()
		).unwrap();
		
		assert_eq!(my_expression.calculate(None).unwrap(), 6.0);
	}
	
	#[test]
	fn missing_left_paren() {
		assert!(
			expression::Expression::new_from_infix(
				expression::Expression::infix_tokens_from_str("2+2^5)/8").unwrap()
			).is_err()
		);
	}
	
	#[test]
	fn missing_right_paren() {
		assert!(
			expression::Expression::new_from_infix(
				expression::Expression::infix_tokens_from_str("2+(2^5/8").unwrap()
			).is_err()
		);
	}
	
	#[test]
	fn error_handling() {
		assert!(expression::Expression::new_from_infix(vec![
			expression::Token::Constant(1.0),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Operator(operator::Operator::Div),
			expression::Token::Constant(2.0),
		]).unwrap().calculate(None).is_err());
	}
	
	#[test]
	fn basic_simplify() {
		unimplemented!()
	}
}
