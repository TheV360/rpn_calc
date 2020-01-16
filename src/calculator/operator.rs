/// This enum contains all the operators that can be used.
#[derive(Clone, Copy)]
pub enum Operator {
	Add, Sub,
	Mul, Div,
	Mod, Pow,
}

impl Operator {
	/// Gets the amount of arguments an operator requires.
	/// 
	/// This may look questionable for now, but it will be useful for unary operators, like unary minus and plus.
	pub fn get_arguments(self) -> usize {
		match self {
			// there are exceptions to this, but for now just this.
			_ => 2,
		}
	}
	
	/// Gets the precedence of an operator.
	/// 
	/// This is extremely useful for coverting infix expressions into reverse polish notation ones. It controls which functions get evaluated first, essentially implementing PEMDAS.
	pub fn get_precedence(self) -> usize {
		match self {
			Operator::Add | Operator::Sub => 2,
			Operator::Mul | Operator::Div | Operator::Mod => 3,
			Operator::Pow => 4,
			// If there's ever a negative operator, it should be like:
			// Operator::Neg => 5,
		}
	}
	
	/// Gets the associativity of an operator.
	/// 
	/// For more information on operator associativity, check the OperatorAssociativity enum page.
	pub fn get_associativity(self) -> OperatorAssociativity {
		match self {
			Operator::Pow => OperatorAssociativity::Right,
			_ => OperatorAssociativity::Left,
		}
	}
	
	/// Calculates the result of using this operator on a stack.
	/// 
	/// There's definitely a better way of doing this. I need to look into how to pass a variable amount of parameters.
	pub fn calculate(self, stack: &mut Vec<f64>) -> f64 {
		// todo: see if you can do this without mutably borrowing the entire stack. Slices maybe? Tuples?
		let arg_length = self.get_arguments();
		
		if stack.len() < arg_length {
			return 0.0; // todo; replace with error?
		}
		
		let mut args: Vec<f64> = vec![0.0; arg_length];
		
		for i in (0..arg_length).rev() {
			args[i] = stack.pop().unwrap();
		}
		
		match self {
			Operator::Add => args[0] + args[1],
			Operator::Sub => args[0] - args[1],
			Operator::Mul => args[0] * args[1],
			Operator::Div => args[0] / args[1],
			Operator::Mod => args[0] % args[1],
			Operator::Pow => args[0].powf(args[1]),
			_ => 0.0, // todo; replace with error?
		}
	}
}

/// Simple Operator Associativity enum.
/// 
/// Associativity dictates how an operator behaves in the absence of parenthesis. ([More info](https://en.wikipedia.org/wiki/Operator_associativity))
#[derive(Clone, Copy)]
pub enum OperatorAssociativity {
	Left, Right,
}
