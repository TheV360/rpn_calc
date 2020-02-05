// TODO: both graph.rs and graph3d.rs should rely on the same code.
// maybe have some way to list independent vars and dependent vars, to make a graph of arbitrary dimensions?
// too complex? at least put lerp and inv_lerp somewhere else

use std::convert::From;

use raylib::prelude::*;

use crate::calculator::expression;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
	(1.0 - t) * a + t * b
}

fn inv_lerp(a: f64, b: f64, n: f64) -> f64 {
	(n - a) / (b - a)
}

fn clamp(n: f64, min: f64, max: f64) -> f64 {
	if n < min {
		min
	} else if n > max {
		max
	} else {
		n
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D { x: f64, y: f64, z: f64, }

#[derive(Debug, Clone, PartialEq)]
pub struct Window3D {
	minimum: Point3D,
	maximum: Point3D,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph3D {
	position: Point3D,
	size: Point3D,
	window: Window3D,
	data: Vec<Point3D>,
}

impl Point3D {
	pub fn new(x: f64, y: f64, z: f64) -> Point3D {
		Point3D { x, y, z }
	}
}

impl From<Vector3> for Point3D {
	fn from(item: Vector3) -> Point3D {
		Point3D { x: item.x as f64, y: item.y as f64, z: item.z as f64 }
	}
}

impl From<Point3D> for Vector3 {
	fn from(item: Point3D) -> Vector3 {
		Vector3 { x: item.x as f32, y: item.y as f32, z: item.z as f32 }
	}
}

impl Window3D {
	pub fn default() -> Window3D {
		Window3D {
			minimum: Point3D::new(-15.0, -10.0, -15.0),
			maximum: Point3D::new( 15.0,  10.0,  15.0),
		}
	}
}

impl Graph3D {
	pub fn new() -> Graph3D {
		Graph3D {
			position: Point3D::new(0.0, 1.5, 0.0),
			size: Point3D::new(2.0, 1.5, 2.0),
			window: Window3D::default(),
			data: Vec::new(),
		}
	}
	
	pub fn calculate_expression(&mut self, expr: &mut expression::Expression, x_axis: char, z_axis: char, points: usize) {
		let mut variables = expression::ExpressionVariables::new();
		
		self.data.clear();
		
		let mut x;
		let mut z;
		for i in 0..points {
			x = lerp(self.window.minimum.x, self.window.maximum.x, (i as f64) / (points as f64));
			variables.insert(x_axis, x);
			
			for j in 0..points {
				z = lerp(self.window.minimum.z, self.window.maximum.z, (j as f64) / (points as f64));
				variables.insert(z_axis, z);
				
				match expr.calculate_with_variables(&variables) {
					Ok(r) => self.data.push(Point3D { x, y: r, z }),
					// Err(e) => println!("{}", e),
					Err(_) => {},
				}
			}
		}
	}
	
	pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
		d.draw_line_3d(
			Vector3::new(
				(self.position.x) as f32,
				(self.position.y + (inv_lerp(self.window.minimum.y, self.window.maximum.y, 0.0) * self.size.y)) as f32,
				(self.position.z + (inv_lerp(self.window.maximum.z, self.window.minimum.z, 0.0) * self.size.z)) as f32
			),
			Vector3::new(
				(self.position.x + self.size.x) as f32,
				(self.position.y + (inv_lerp(self.window.maximum.y, self.window.minimum.y, 0.0) * self.size.y)) as f32,
				(self.position.z + (inv_lerp(self.window.maximum.z, self.window.minimum.z, 0.0) * self.size.z)) as f32
			),
			Color::RED
		);
		d.draw_line_3d(
			Vector3::new(
				(self.position.x + (inv_lerp(self.window.maximum.x, self.window.minimum.x, 0.0) * self.size.x)) as f32,
				(self.position.y) as f32,
				(self.position.z + (inv_lerp(self.window.maximum.z, self.window.minimum.z, 0.0) * self.size.z)) as f32
			),
			Vector3::new(
				(self.position.x + (inv_lerp(self.window.maximum.x, self.window.minimum.x, 0.0) * self.size.x)) as f32,
				(self.position.y + self.size.y) as f32,
				(self.position.z + (inv_lerp(self.window.maximum.z, self.window.minimum.z, 0.0) * self.size.z)) as f32
			),
			Color::LIME
		);
		d.draw_line_3d(
			Vector3::new(
				(self.position.x + (inv_lerp(self.window.maximum.x, self.window.minimum.x, 0.0) * self.size.x)) as f32,
				(self.position.y + (inv_lerp(self.window.minimum.y, self.window.maximum.y, 0.0) * self.size.y)) as f32,
				(self.position.z) as f32
			),
			Vector3::new(
				(self.position.x + (inv_lerp(self.window.maximum.x, self.window.minimum.x, 0.0) * self.size.x)) as f32,
				(self.position.y + (inv_lerp(self.window.minimum.y, self.window.maximum.y, 0.0) * self.size.y)) as f32,
				(self.position.z + self.size.z) as f32
			),
			Color::BLUE
		);
		
		for i in 1..self.data.len() {
			if i % 65 == 0 { continue; }
			d.draw_line_3d(
				Vector3::new(
					(self.position.x + clamp(inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i - 1].x), 0.0, 1.0) * self.size.x) as f32,
					(self.position.y + clamp(inv_lerp(self.window.minimum.y, self.window.maximum.y, self.data[i - 1].y), 0.0, 1.0) * self.size.y) as f32,
					(self.position.z + clamp(inv_lerp(self.window.maximum.z, self.window.minimum.z, self.data[i - 1].z), 0.0, 1.0) * self.size.z) as f32
				),
				Vector3::new(
					(self.position.x + clamp(inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i    ].x), 0.0, 1.0) * self.size.x) as f32,
					(self.position.y + clamp(inv_lerp(self.window.minimum.y, self.window.maximum.y, self.data[i    ].y), 0.0, 1.0) * self.size.y) as f32,
					(self.position.z + clamp(inv_lerp(self.window.maximum.z, self.window.minimum.z, self.data[i    ].z), 0.0, 1.0) * self.size.z) as f32
				),
				Color::ORANGE
			);
		}
		
		for i in 65..self.data.len() {
			d.draw_line_3d(
				Vector3::new(
					(self.position.x + clamp(inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i - 65].x), 0.0, 1.0) * self.size.x) as f32,
					(self.position.y + clamp(inv_lerp(self.window.minimum.y, self.window.maximum.y, self.data[i - 65].y), 0.0, 1.0) * self.size.y) as f32,
					(self.position.z + clamp(inv_lerp(self.window.maximum.z, self.window.minimum.z, self.data[i - 65].z), 0.0, 1.0) * self.size.z) as f32
				),
				Vector3::new(
					(self.position.x + clamp(inv_lerp(self.window.maximum.x, self.window.minimum.x, self.data[i     ].x), 0.0, 1.0) * self.size.x) as f32,
					(self.position.y + clamp(inv_lerp(self.window.minimum.y, self.window.maximum.y, self.data[i     ].y), 0.0, 1.0) * self.size.y) as f32,
					(self.position.z + clamp(inv_lerp(self.window.maximum.z, self.window.minimum.z, self.data[i     ].z), 0.0, 1.0) * self.size.z) as f32
				),
				Color::ORANGE
			);
		}
	}
}
