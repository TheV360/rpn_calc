use raylib::prelude::*;

use crate::calculator::expression;

const SCALE: i32 = 4;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
	(1.0 - t) * a + t * b
}

fn inv_lerp(a: f64, b: f64, n: f64) -> f64 {
	(n - a) / (b - a)
}

pub struct Point { x: f64, y: f64, }

pub struct Window {
	minimum: Point,
	maximum: Point,
}

pub struct Graph {
	size: Point,
	window: Window,
	data: Vec<f64>,
}

impl Point {
	pub fn new(x: f64, y: f64) -> Point {
		Point {x, y}
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
		self.data.clear();
		
		let mut percent;
		for i in 0..points {
			percent = (i as f64) / (points as f64);
			
			expr.set_variable(x_axis, lerp(self.window.minimum.x, self.window.maximum.x, percent));
			match expr.calculate() {
				Ok(r) => self.data.push(r),
				Err(e) => println!("{}", e),
			}
		}
	}
	
	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		let points = self.data.len();
		
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
		
		let mut before_p;
		let mut after_p = 0.0;
		for i in 1..points {
			before_p = after_p;
			after_p = (i as f64) / (points as f64);
			
			// TODO: maybe try draw_line_strip?
			d.draw_line_ex(
				Vector2::new((before_p * self.size.x) as f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, self.data[i - 1]) * self.size.y) as f32),
				Vector2::new((after_p  * self.size.x) as f32, (inv_lerp(self.window.maximum.y, self.window.minimum.y, self.data[i    ]) * self.size.y) as f32),
				SCALE as f32, Color::ORANGE
			);
		}
	}
}
