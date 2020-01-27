use raylib::prelude::*;

use crate::calculator::{expression, operator};
use super::graph;

pub fn start() {
	let (mut rl, thread) = raylib::init()
		.size(360, 240)
		.title("Hello, world!")
		.build();
	
	let graph = graph::Graph::new();
	
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		
		d.clear_background(Color::BLACK);
		d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
	}
}
