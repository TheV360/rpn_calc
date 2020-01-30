/// This enum contains all the operators that can be used in the RPN calc.
/// 
/// When I say "Operator", I mean things like + and -. I also mean things like unary minus and sin().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
	Add, Sub,
	Mul, Div,
	Mod,
	Pow, Rot,
}

/*impl FromStr for Operator {
	type Err = std::num::ParseIntError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Operator::Add),
			"-" => Ok(Operator::Sub),
			"*" => Ok(Operator::Mul),
			"/" => Ok(Operator::Div),
			"%" => Ok(Operator::Mod),
			"^" => Ok(Operator::Pow),
			_ => panic!("Operator FromStr says: oh no sorry todo oh god sorry"), //TODO: make an error type? Not sure how this works.
		}
	}
}*/

// TODO: decide if I should have UnaryOperator and BinaryOperator. aaaaaaaaaaa

impl Operator {
	/// Gets the amount of arguments an operator requires.
	/// 
	/// This may look questionable for now, but it will be useful for unary operators, like unary minus and plus.
	pub fn get_arguments(&self) -> usize {
		match self {
			// there are exceptions to this, but for now just this.
			_ => 2,
		}
	}
	
	/// Gets the precedence of an operator.
	/// 
	/// This is extremely useful for coverting infix expressions into reverse polish notation ones. It controls which functions get evaluated first, essentially implementing PEMDAS.
	pub fn get_precedence(&self) -> usize {
		match self {
			Operator::Add | Operator::Sub => 2,
			Operator::Mul | Operator::Div | Operator::Mod => 3,
			Operator::Pow | Operator::Rot => 4,
			// Unary operators should have the highest precedence.
			// _ => 7,
		}
	}
	
	/// Gets the associativity of an operator.
	/// 
	/// For more information on operator associativity, check the OperatorAssociativity enum page.
	pub fn get_associativity(&self) -> OperatorAssociativity {
		match self {
			Operator::Pow | Operator::Rot => OperatorAssociativity::Right,
			_ => OperatorAssociativity::Left,
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
			// _ => Err("Unknown operator."),
		}
	}
}

/// Simple Operator Associativity enum.
/// 
/// Associativity dictates how an operator behaves in the absence of parenthesis. ([More info](https://en.wikipedia.org/wiki/Operator_associativity))
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorAssociativity {
	Left, Right,
}
