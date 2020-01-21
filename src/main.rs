mod calculator;

use calculator::{expression, operator};

fn main() {
	println!("{}", expression::Expression::new(vec![
		expression::Token::Constant(2.0),
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Add),
	]).calculate().unwrap());
	
	let res = "0.25sroihjdskfdjiuehogfnh".parse::<f64>();
	match res {
		Ok(r) => println!("{}", r),
		Err(_) => println!("can't read"),
	}
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
