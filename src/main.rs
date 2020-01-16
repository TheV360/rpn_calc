mod operator;
mod expression;

#[test]
fn calculate_expression() {
	assert_eq!(expression::Expression::new(vec!(
		expression::Token::Constant(2.0),
		expression::Token::Constant(4.0),
		expression::Token::Operator(operator::Operator::Mul),
	)).calculate(), 8.0);
}

#[test]
fn argument_order() {
	assert_eq!(expression::Expression::new(vec!(
		expression::Token::Constant(1.0),
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Div),
	)).calculate(), 0.5);
}

fn main() {
	println!("{}", expression::Expression::new(vec!(
		expression::Token::Constant(2.0),
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Add),
	)).calculate());
	
	let res = "0.25sroihjdskfdjiuehogfnh".parse::<f64>();
	if res.is_ok() {
		println!("{}", res.unwrap());
	} else {
		println!("can't read");
	}
}
