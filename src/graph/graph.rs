use std::convert::From;
use raylib::prelude::*;
use crate::util;
use crate::calculator::expression;
use crate::graph::common;

const SCALE: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphArgs2D {
	Cartesian,
	Parametric(common::MinMax),
	Polar(common::MinMax),
}

impl Default for GraphArgs2D {
	fn default() -> Self { GraphArgs2D::Cartesian }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { pub x: f64, pub y: f64, }

impl From<Vector2> for Point {
	fn from(item: Vector2) -> Self {
		Point { x: item.x as f64, y: item.y as f64 }
	}
}
impl From<Point> for Vector2 {
	fn from(item: Point) -> Self {
		Vector2 { x: item.x as f32, y: item.y as f32 }
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
	pub minimum: Point,
	pub maximum: Point,
}

impl Default for Window {
	fn default() -> Self {
		Window {
			minimum: Point { x: -15.0, y: -10.0},
			maximum: Point { x:  15.0, y:  10.0},
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
	pub args: GraphArgs2D,
	pub size: Point,
	pub window: Window,
	data: Vec<Point>,
}

impl Graph {
	pub fn new(args: GraphArgs2D) -> Graph {
		Graph {
			args,
			size: Point { x: 360.0 * (SCALE as f64), y: 240.0 * (SCALE as f64) },
			window: Window::default(),
			data: Vec::new(),
		}
	}
	
	pub fn calculate_expression(&mut self, expr: &Vec<expression::Expression>, points: usize) -> Result<(), &'static str> {
		let mut variables = expression::ExpressionVariables::new();
		
		self.data.clear();
		
		match self.args {
			GraphArgs2D::Cartesian => {
				assert!(expr.len() == 1);
				
				let mut x;
				for i in 0..=points {
					x = util::lerp(self.window.minimum.x, self.window.maximum.x, (i as f64) / (points as f64));
					variables.insert('x', x);
					
					let y = expr[0].calculate_with_variables(&variables)?;
					
					self.data.push(Point { x, y });
				}
			},
			GraphArgs2D::Parametric(t_minmax) => {
				assert!(expr.len() == 2);
				
				let mut t;
				for i in 0..=points {
					t = util::lerp(t_minmax.min, t_minmax.max, (i as f64) / (points as f64));
					variables.insert('t', t);
					
					let x = expr[0].calculate_with_variables(&variables)?;
					let y = expr[1].calculate_with_variables(&variables)?;
					
					self.data.push(Point { x, y });
				}
			},
			GraphArgs2D::Polar(t_minmax) => {
				assert!(expr.len() == 1);
				
				let mut theta;
				for i in 0..=points {
					theta = util::lerp(t_minmax.min, t_minmax.max, (i as f64) / (points as f64));
					variables.insert('t', theta);
					
					let r = expr[0].calculate_with_variables(&variables)?;
					
					self.data.push(Point { x: r * theta.cos(), y: r * theta.sin(), });
				}
			}
		}
		
		Ok(())
	}
	
	pub fn graph_to_screen_x(&self, x: f64) -> f64 { util::inv_lerp(self.window.minimum.x, self.window.maximum.x, x) * self.size.x }
	pub fn graph_to_screen_y(&self, y: f64) -> f64 { util::inv_lerp(self.window.maximum.y, self.window.minimum.y, y) * self.size.y }
	pub fn graph_to_screen_point(&self, p: &Point) -> Point {
		Point {
			x: self.graph_to_screen_x(p.x),
			y: self.graph_to_screen_y(p.y),
		}
	}
	
	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		let x_axis = self.graph_to_screen_x(0.0) as f32;
		let y_axis = self.graph_to_screen_y(0.0) as f32;
		
		for i in 1..self.data.len() {
			d.draw_line_ex(
				Vector2::from(self.graph_to_screen_point(&self.data[i - 1])),
				Vector2::from(self.graph_to_screen_point(&self.data[i    ])),
				SCALE as f32, Color::ORANGE
			);
		}
		
		d.draw_line_ex(Vector2::new(0.0f32, y_axis), Vector2::new(self.size.x as f32, y_axis), (SCALE as f32) / 2.0f32, Color::RED);
		d.draw_line_ex(Vector2::new(x_axis, 0.0f32), Vector2::new(x_axis, self.size.y as f32), (SCALE as f32) / 2.0f32, Color::LIME);
	}
}
