use std::convert::From;

use raylib::prelude::*;

use crate::calculator::expression;

const SCALE: i32 = 4;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
	(1.0 - t) * a + t * b
}

fn inv_lerp(a: f64, b: f64, n: f64) -> f64 {
	(n - a) / (b - a)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { x: f64, y: f64, }

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
	minimum: Point,
	maximum: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
	size: Point,
	window: Window,
	data: Vec<Point>,
}

impl Point {
	pub fn new(x: f64, y: f64) -> Point {
		Point { x, y }
	}
}

impl From<Vector2> for Point {
	fn from(item: Vector2) -> Point {
		Point { x: item.x as f64, y: item.y as f64 }
	}
}

impl From<Point> for Vector2 {
	fn from(item: Point) -> Vector2 {
		Vector2 { x: item.x as f32, y: item.y as f32 }
	}
}

impl Window {
	pub fn default() -> Window {
		Window {
			minimum: Point::new(-15.0, -10.0),
			maximum: Point::new( 15.0,  10.0),
		}
	}
}

impl Graph {
	pub fn new() -> Graph {
		Graph {
			size: Point::new(360.0 * (SCALE as f64), 240.0 * (SCALE as f64)),
			window: Window::default(),
			data: Vec::new(),
		}
	}
	
	pub fn calculate_expression(&mut self, expr: &mut expression::Expression, x_axis: char, points: usize) {
		let mut variables = expression::ExpressionVariables::new();
		
		self.data.clear();
		
		let mut x;
		for i in 0..points {
			x = lerp(self.window.minimum.x, self.window.maximum.x, (i as f64) / (points as f64));
			variables.insert(x_axis, x);
			
			match expr.calculate_with_variables(&variables) {
				Ok(r) => self.data.push(Point { x, y: r }),
				// Err(e) => println!("{}", e),
				Err(_) => {},
			}
		}
	}
	
	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		d.draw_line_ex(
			Vector2::new(            0.0f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, 0.0) * self.size.y) as f32),
			Vector2::new(self.size.x as f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, 0.0) * self.size.y) as f32),
			(SCALE as f32) / 2.0f32, Color::RED
		);
		
		d.draw_line_ex(
			Vector2::new((inv_lerp(self.window.minimum.x, self.window.maximum.x, 0.0) * self.size.x) as f32,             0.0f32),
			Vector2::new((inv_lerp(self.window.minimum.x, self.window.maximum.x, 0.0) * self.size.x) as f32, self.size.y as f32),
			(SCALE as f32) / 2.0f32, Color::BLUE
		);
		
		for i in 1..self.data.len() {
			d.draw_line_ex(
				Vector2::new((inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i - 1].x) * self.size.x) as f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, self.data[i - 1].y) * self.size.y) as f32),
				Vector2::new((inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i    ].x) * self.size.x) as f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, self.data[i    ].y) * self.size.y) as f32),
				SCALE as f32, Color::ORANGE
			);
		}
	}
}
