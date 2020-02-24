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
pub struct Point3D { pub x: f64, pub y: f64, pub z: f64, }

#[derive(Debug, Clone, PartialEq)]
pub struct Window3D {
	pub minimum: Point3D,
	pub maximum: Point3D,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph3D {
	pub position: Point3D,
	pub size: Point3D,
	pub window: Window3D,
	data: Vec<Point3D>,
	tmp_data_width: usize
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
			position: Point3D::new(0.0, 0.0, 0.0),
			size: Point3D::new(10.0, 7.5, 10.0),
			window: Window3D::default(),
			data: Vec::new(),
			tmp_data_width: 1,
		}
	}
	
	pub fn calculate_expression(&mut self, expr: &mut expression::Expression, x_axis: char, z_axis: char, points: usize) {
		let mut variables = expression::ExpressionVariables::new();
		
		self.data.clear();
		self.tmp_data_width = points + 1;
		
		let mut x;
		let mut z;
		for i in 0..=points {
			x = lerp(self.window.minimum.x, self.window.maximum.x, (i as f64) / (points as f64));
			variables.insert(x_axis, x);
			
			for j in 0..=points {
				z = lerp(self.window.minimum.z, self.window.maximum.z, (j as f64) / (points as f64));
				variables.insert(z_axis, z);
				
				match expr.calculate_with_variables(&variables) {
					Ok(r) => self.data.push(Point3D { x, y: r, z }),
					Err(e) => {
						println!("Error! {}", e);
						return;
					},
				}
			}
		}
	}
	
	pub fn graph_to_screen_x(&self, x: f64) -> f64 { self.position.x - self.size.x / 2.0 + (inv_lerp(self.window.minimum.x, self.window.maximum.x, x) * self.size.x) }
	pub fn graph_to_screen_y(&self, y: f64) -> f64 { self.position.y - self.size.y / 2.0 + (inv_lerp(self.window.minimum.y, self.window.maximum.y, y) * self.size.y) }
	pub fn graph_to_screen_z(&self, z: f64) -> f64 { self.position.z - self.size.z / 2.0 + (inv_lerp(self.window.minimum.z, self.window.maximum.z, z) * self.size.z) }
	pub fn graph_to_screen_clamp_x(&self, x: f64) -> f64 { self.graph_to_screen_x(clamp(x, self.window.minimum.x, self.window.maximum.x)) }
	pub fn graph_to_screen_clamp_y(&self, y: f64) -> f64 { self.graph_to_screen_y(clamp(y, self.window.minimum.y, self.window.maximum.y)) }
	pub fn graph_to_screen_clamp_z(&self, z: f64) -> f64 { self.graph_to_screen_z(clamp(z, self.window.minimum.z, self.window.maximum.z)) }
	pub fn graph_to_screen_point3d(&self, p: &Point3D) -> Point3D {
		Point3D {
			x: self.graph_to_screen_x(p.x),
			y: self.graph_to_screen_y(p.y),
			z: self.graph_to_screen_z(p.z),
		}
	}
	pub fn graph_to_screen_clamp_point3d(&self, p: &Point3D) -> Point3D {
		Point3D {
			x: self.graph_to_screen_clamp_x(p.x),
			y: self.graph_to_screen_clamp_y(p.y),
			z: self.graph_to_screen_clamp_z(p.z),
		}
	}
	
	pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
		let x_axis = self.graph_to_screen_x(0.0) as f32;
		let y_axis = self.graph_to_screen_y(0.0) as f32;
		let z_axis = self.graph_to_screen_z(0.0) as f32;
		
		for i in 1..self.data.len() {
			if i % self.tmp_data_width == 0 { continue; }
			d.draw_line_3d(
				Vector3::from(self.graph_to_screen_point3d(&self.data[i - 1])),
				Vector3::from(self.graph_to_screen_point3d(&self.data[i    ])),
				Color::WHITE//Color::color_from_hsv(Vector3::new( (i as f32) / (self.data.len() as f32) * 360.0, 1.0, 1.0))
			);
		}
		
		for i in self.tmp_data_width..self.data.len() {
			d.draw_line_3d(
				Vector3::from(self.graph_to_screen_point3d(&self.data[i - self.tmp_data_width])),
				Vector3::from(self.graph_to_screen_point3d(&self.data[i                      ])),
				Color::WHITE//Color::color_from_hsv(Vector3::new( (i as f32) / (self.data.len() as f32) * 360.0, 1.0, 1.0))
			);
		}
		
		let thickness = 0.05;
		d.draw_cube(Vector3::new(self.position.x as f32, y_axis, z_axis), self.size.x as f32, thickness, thickness, Color::RED);
		d.draw_cube(Vector3::new(x_axis, self.position.y as f32, z_axis), thickness, self.size.y as f32, thickness, Color::LIME);
		d.draw_cube(Vector3::new(x_axis, y_axis, self.position.z as f32), thickness, thickness, self.size.z as f32, Color::BLUE);
	}
}
