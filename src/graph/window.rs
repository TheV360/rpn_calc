use raylib::prelude::*;

use crate::calculator::expression;
use super::{graph3d, common};

const SCALE: i32 = 4;

pub fn start() {
	// no logging pls
	raylib::core::logging::set_trace_log(raylib::ffi::TraceLogType::LOG_NONE);
	
	let (mut rl, thread) = raylib::init()
		.size(360 * SCALE, 240 * SCALE)
		.title("Graphing Calculator")
		.resizable()
		.build();
	
	let mut angry_error = String::new();
	
	let mut function_input_strings = vec![
		String::from("10sin(13t)"), String::from("10cos(3t)"), String::from("10sin(17t)")
	];
	let mut active = 0;
	let mut expressions_to_graph: Vec<expression::Expression> = Vec::new();
	for i in 0..function_input_strings.len() {
		expressions_to_graph.push(expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&function_input_strings[i]).unwrap()).unwrap());
	}
	let mut graph3d = graph3d::Graph3D::new(graph3d::GraphArgs3D::Parametric(common::MinMax { min: -3.0, max: 3.0 }));
	match graph3d.calculate_expression(&expressions_to_graph, 1024) {
		Err(e) => println!("{}", e), _ => {},
	};
	
	let mut cam = raylib::core::camera::Camera3D::perspective(
		Vector3::new(0.0, 1.8, 0.0),
		Vector3::new(0.0, 0.0,-1.0),
		Vector3::new(0.0, 1.0, 0.0),
		80.0
	);
	
	rl.set_camera_mode(&cam, CameraMode::CAMERA_FREE);
	rl.set_camera_move_controls(
		KeyboardKey::KEY_UP,
		KeyboardKey::KEY_DOWN,
		KeyboardKey::KEY_RIGHT,
		KeyboardKey::KEY_LEFT,
		KeyboardKey::KEY_PAGE_UP,
		KeyboardKey::KEY_PAGE_DOWN,
	);
	rl.set_target_fps(60);
	
	while !rl.window_should_close() {
		// -- Updating -- \\
		
		let pressed_key = rl.get_key_pressed_number();
		
		// -- Wacky input -- \\
		
		if let Some(key) = pressed_key {
			if let Some(key_char) = std::char::from_u32(key) {
				function_input_strings[active].push(key_char);
			}
		}
		
		if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
			function_input_strings[active].pop();
		}
		
		if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
			if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
				if active < 1 {
					active = function_input_strings.len() - 1;
				} else {
					active -= 1;
				}
			} else {
				active = (active + 1) % function_input_strings.len();
			}
		}
		
		if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
			let mut is_okay = true;
			
			angry_error = String::new();
			
			for i in 0..function_input_strings.len() {
				
				match expression::Expression::infix_tokens_from_str(&function_input_strings[i]) {
					Ok(t) => {
						match expression::Expression::new_from_infix(t) {
							Ok(expr) => {
								expressions_to_graph[i] = expr;
								println!("{}: {:?}", i, expressions_to_graph[i]);
							},
							Err(e) => {
								angry_error = e.to_owned();
								is_okay = false;
							},
						}
					},
					Err(e) => {
						angry_error = e.to_owned();
						is_okay = false;
					},
				}
			}
			
			if is_okay {
				match graph3d.calculate_expression(&mut expressions_to_graph, 1024) {
					Err(e) => angry_error = e.to_owned(), _ => {},
				}
			}
		}
		
		rl.set_mouse_scale(1.0, 1.0);
		rl.update_camera(&mut cam);
		rl.set_mouse_scale((SCALE as f32).recip(), (SCALE as f32).recip());
		
		// -- Drawing -- \\
		
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		
		{
			let mut d = d.begin_mode_3D(cam);
			
			graph3d.draw(&mut d);
		}
		
		for i in 0..function_input_strings.len() {
			d.draw_text(&function_input_strings[i], if i == active { 20 } else { 12 }, 16 + 40 * (i as i32), 40, Color::PINK);
		}
		d.draw_text(">", 10, 26 + 40 * (active as i32), 20, Color::PINK);
		
		d.draw_text(&angry_error, 32, 128, 80, Color::RED);
	}
}
