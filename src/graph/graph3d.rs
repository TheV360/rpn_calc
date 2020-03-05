// TODO: both graph.rs and graph3d.rs should rely on the same code.
// maybe have some way to list independent vars and dependent vars, to make a graph of arbitrary dimensions?

use std::convert::From;
use raylib::prelude::*;
use crate::util;
use crate::calculator::expression;
use crate::graph::common;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphArgs3D {
	Cartesian,
	Parametric(common::MinMax),
}

impl Default for GraphArgs3D {
	fn default() -> Self { GraphArgs3D::Cartesian }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D { pub x: f64, pub y: f64, pub z: f64, }

impl From<Vector3> for Point3D {
	fn from(item: Vector3) -> Self {
		Point3D { x: item.x as f64, y: item.y as f64, z: item.z as f64 }
	}
}
impl From<Point3D> for Vector3 {
	fn from(item: Point3D) -> Self {
		Vector3 { x: item.x as f32, y: item.y as f32, z: item.z as f32 }
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window3D {
	pub minimum: Point3D,
	pub maximum: Point3D,
}

impl Window3D {
	pub fn default() -> Window3D {
		Window3D {
			minimum: Point3D { x: -15.0, y: -10.0, z: -15.0 },
			maximum: Point3D { x:  15.0, y:  10.0, z:  15.0 },
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph3D {
	pub args: GraphArgs3D,
	pub position: Point3D,
	pub size: Point3D,
	pub window: Window3D,
	data: Vec<Point3D>,
	tmp_data_width: usize,
}

impl Graph3D {
	pub fn new(args: GraphArgs3D) -> Graph3D {
		Graph3D {
			args,
			position: Point3D { x: 0.0, y: 0.0, z: 0.0 },
			size: Point3D { x: 10.0, y: 7.5, z: 10.0 },
			window: Window3D::default(),
			data: Vec::new(),
			tmp_data_width: 1,
		}
	}
	
	pub fn calculate_expression(&mut self, expr: &Vec<expression::Expression>, points: usize) -> Result<(), &'static str> {
		let mut variables = expression::ExpressionVariables::new();
		
		self.data.clear();
		self.tmp_data_width = points + 1;
		
		match self.args {
			GraphArgs3D::Cartesian => {
				assert!(expr.len() == 1);
				
				let mut x;
				let mut z;
				for i in 0..=points {
					x = util::lerp(self.window.minimum.x, self.window.maximum.x, (i as f64) / (points as f64));
					variables.insert('x', x);
					
					for j in 0..=points {
						z = util::lerp(self.window.minimum.z, self.window.maximum.z, (j as f64) / (points as f64));
						variables.insert('z', z);
						
						let y = expr[0].calculate_with_variables(&variables)?;
						
						self.data.push(Point3D { x, y, z });
					}
				}
			},
			GraphArgs3D::Parametric(t_minmax) => {
				assert!(expr.len() == 3);
				
				let mut t;
				for i in 0..=points {
					t = util::lerp(t_minmax.min, t_minmax.max, (i as f64) / (points as f64));
					variables.insert('t', t);
					
					let x = expr[0].calculate_with_variables(&variables)?;
					let y = expr[1].calculate_with_variables(&variables)?;
					let z = expr[2].calculate_with_variables(&variables)?;
					
					self.data.push(Point3D { x, y, z });
				}
			},
		}
		
		Ok(())
	}
	
	pub fn graph_to_screen_x(&self, x: f64) -> f64 { self.position.x - self.size.x / 2.0 + (util::inv_lerp(self.window.minimum.x, self.window.maximum.x, x) * self.size.x) }
	pub fn graph_to_screen_y(&self, y: f64) -> f64 { self.position.y - self.size.y / 2.0 + (util::inv_lerp(self.window.minimum.y, self.window.maximum.y, y) * self.size.y) }
	pub fn graph_to_screen_z(&self, z: f64) -> f64 { self.position.z - self.size.z / 2.0 + (util::inv_lerp(self.window.minimum.z, self.window.maximum.z, z) * self.size.z) }
	pub fn graph_to_screen_clamp_x(&self, x: f64) -> f64 { self.graph_to_screen_x(util::clamp(x, self.window.minimum.x, self.window.maximum.x)) }
	pub fn graph_to_screen_clamp_y(&self, y: f64) -> f64 { self.graph_to_screen_y(util::clamp(y, self.window.minimum.y, self.window.maximum.y)) }
	pub fn graph_to_screen_clamp_z(&self, z: f64) -> f64 { self.graph_to_screen_z(util::clamp(z, self.window.minimum.z, self.window.maximum.z)) }
	pub fn graph_to_screen_point3d(&self, p: &Point3D) -> Point3D {
		Point3D { x: self.graph_to_screen_x(p.x), y: self.graph_to_screen_y(p.y), z: self.graph_to_screen_z(p.z), }
	}
	pub fn graph_to_screen_clamp_point3d(&self, p: &Point3D) -> Point3D {
		Point3D { x: self.graph_to_screen_clamp_x(p.x), y: self.graph_to_screen_clamp_y(p.y), z: self.graph_to_screen_clamp_z(p.z), }
	}
	
	pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
		let axis = self.graph_to_screen_point3d(&Point3D { x: 0.0, y: 0.0, z: 0.0 });
		
		fn grey(n: f64) -> Color {
			return Color::new(255, 255, 255, (n * 255.) as u8);
		}
		for i in 1..self.data.len() {
			// if i % self.tmp_data_width == 0 { continue; }
			d.draw_line_3d(
				Vector3::from(self.graph_to_screen_point3d(&self.data[i - 1])),
				Vector3::from(self.graph_to_screen_point3d(&self.data[i    ])),
				grey(1.0 - ((i as f64 + d.get_time()) / (self.data.len() as f64)) / 2.0)
			);
		}
		
		// for i in self.tmp_data_width..self.data.len() {
		// 	d.draw_line_3d(
		// 		Vector3::from(self.graph_to_screen_point3d(&self.data[i - self.tmp_data_width])),
		// 		Vector3::from(self.graph_to_screen_point3d(&self.data[i                      ])),
		// 		Color::WHITE
		// 	);
		// }
		
		let thickness = 0.05;
		d.draw_cube(Vector3::new(self.position.x as f32, axis.y as f32, axis.z as f32), self.size.x as f32, thickness, thickness, Color::RED);
		d.draw_cube(Vector3::new(axis.x as f32, self.position.y as f32, axis.z as f32), thickness, self.size.y as f32, thickness, Color::LIME);
		d.draw_cube(Vector3::new(axis.x as f32, axis.y as f32, self.position.z as f32), thickness, thickness, self.size.z as f32, Color::BLUE);
	}
}
