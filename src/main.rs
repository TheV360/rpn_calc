pub mod calculator {
	pub mod operator;
	pub mod expression;
}

use calculator::operator;
use calculator::expression;

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
fn infix_expression() {
	assert_eq!(expression::Expression::new_from_infix(vec![
		expression::Token::Constant(1.0),
		expression::Token::Operator(operator::Operator::Div),
		expression::Token::Constant(2.0),
	]).unwrap().calculate().unwrap(), 0.5);
	
	assert_eq!(expression::Expression::new_from_infix(vec![ // (2 + 5) * 6 ^ 2 = 252
		expression::Token::ParenthesisLeft,
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Add),
		expression::Token::Constant(5.0),
		expression::Token::ParenthesisRight,
		expression::Token::Operator(operator::Operator::Mul),
		expression::Token::Constant(6.0),
		expression::Token::Operator(operator::Operator::Pow),
		expression::Token::Constant(2.0),
	]).unwrap().calculate().unwrap(), 252.0);
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

fn main() {
	println!("{}", expression::Expression::new(vec![
		expression::Token::Constant(2.0),
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Add),
	]).calculate().unwrap());
	
	let res = "0.25sroihjdskfdjiuehogfnh".parse::<f64>();
	if res.is_ok() {
		println!("{}", res.unwrap());
	} else {
		println!("can't read");
	}
}
