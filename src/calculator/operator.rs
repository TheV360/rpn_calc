/// This enum contains all the operators that can be used in the RPN calc.
/// 
/// When I say "Operator", I mean things like + and -. Functions like sin() go in function.rs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
	Add, Sub,
	Mul, Div,
	Mod,
	Pow, Rot,
	Unp, Unm,
}

/// This enum contains all the functions that can be used in the RPN calc.
/// 
/// When I say "Function", I mean stuff like sin(). Unary operators don't quite fit here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
	Sin, Cos, Tan,
	Csc, Sec, Cot,
	Log, Ln,
	Abs, Sgn,
}

/// Simple Operator Associativity enum.
/// 
/// Associativity dictates how an operator behaves in the absence of parenthesis. ([More info](https://en.wikipedia.org/wiki/Operator_associativity))
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorAssociativity {
	Left, Right,
}

impl Operator {
	/// Gets the amount of parameters an operator requires.
	pub fn get_parameters(&self) -> usize {
		match self {
			// there are exceptions to this, but for now just this.
			Operator::Add | Operator::Sub |
			Operator::Mul | Operator::Div | Operator::Mod |
			Operator::Pow | Operator::Rot => 2,
			_ => 1,
		}
	}
	
	/// Gets the precedence of an operator.
	/// 
	/// This is extremely useful for coverting infix expressions into reverse polish notation ones. It controls which functions get evaluated first, essentially implementing PEMDAS.
	pub fn get_precedence(&self) -> usize {
		match self {
			Operator::Add | Operator::Sub => 2,
			Operator::Mul | Operator::Div | Operator::Mod => 3,
			Operator::Pow | Operator::Rot => 5,
			// Unary operators should have the highest precedence.
			Operator::Unm | Operator::Unp => 7,
		}
	}
	
	/// Gets the associativity of an operator.
	/// 
	/// For more information on operator associativity, check the OperatorAssociativity enum page.
	pub fn get_associativity(&self) -> OperatorAssociativity {
		match self {
			Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Mod => OperatorAssociativity::Left,
			_ => OperatorAssociativity::Right,
		}
	}
	
	/// Calculates the result of using this operator on a stack.
	/// 
	/// There's definitely a better way of doing this. I need to look into how to pass a variable amount of parameters.
	pub fn calculate(&self, args: Vec<f64>) -> Result<f64, &'static str> {
		match self {
			Operator::Add => Ok(args[0] + args[1]),
			Operator::Sub => Ok(args[0] - args[1]),
			Operator::Mul => Ok(args[0] * args[1]),
			Operator::Div => Ok(args[0] / args[1]),
			Operator::Mod => Ok(args[0] % args[1]),
			Operator::Pow => Ok(args[0].powf(args[1])),
			Operator::Rot => Ok(args[1].powf(args[0].recip())),
			Operator::Unp => Ok( args[0]),
			Operator::Unm => Ok(-args[0]),
		}
	}
}

impl Function {
	/// Gets the amount of parameters a function requires.
	/// 
	/// Sometimes functions will have parameters separated by commas, so we need a system in place for that.
	pub fn get_parameters(&self) -> usize {
		match self {
			_ => 1,
		}
	}
	
	/// Uh
	pub fn get_precedence(&self) -> usize {
		return 4;
	}
	
	/// Calculates the result of using this function on a stack.
	/// 
	/// See Operator comment for despair about code smell.
	pub fn calculate(&self, args: Vec<f64>) -> Result<f64, &'static str> {
		match self {
			Function::Sin => Ok(args[0].sin()),
			Function::Cos => Ok(args[0].cos()),
			Function::Tan => Ok(args[0].tan()),
			Function::Csc => Ok(args[0].sin().recip()),
			Function::Sec => Ok(args[0].cos().recip()),
			Function::Cot => Ok(args[0].tan().recip()),
			Function::Abs => Ok(args[0].abs()),
			Function::Sgn => Ok(args[0].signum()),
			_ => Err("Unknown function."),
		}
	}
}
