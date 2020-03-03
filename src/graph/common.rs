use crate::calculator::expression;

// TODO: hah this really isn't "common" anymore. move into their respective files.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMax { pub min: f64, pub max: f64, }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphArgs2D {
	Cartesian,
	Parametric(MinMax),
	Polar(MinMax),
}

impl Default for GraphArgs2D {
	fn default() -> Self { GraphArgs2D::Cartesian }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphArgs3D {
	Cartesian,
	Parametric(MinMax),
}

impl Default for GraphArgs3D {
	fn default() -> Self { GraphArgs3D::Cartesian }
}
