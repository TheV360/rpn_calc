use raylib::prelude::*;
use std::ffi::{CStr, CString};

use crate::calculator::{expression, operator};
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
	
	let mut overlay = rl.load_render_texture(&thread, 360, 240).unwrap();
	overlay.texture_mut().set_texture_filter(raylib::consts::TextureFilterMode::FILTER_POINT);
	
	// TODO: complain to raylib-rs dev about lack of support for GUILoadStyle
	/*
	unsafe {
		raylib::ffi::GuiLoadStyle(CString::new("resources/cyber.rgs").unwrap().as_ptr());
	}
	
	let mut wb = raylib::rgui::WindowBox {
		bounds: Rectangle::new(0.0, 0.0, 240.0, 120.0),
		text: CString::new("Hello, world!").unwrap(),
	};
	
	let btn = raylib::rgui::Button {
		bounds: Rectangle::new(8.0, 8.0, 64.0, 16.0),
		text: CString::new("aaaaaa").unwrap(),
	};
	
	// ALLOCATING MEMORY
	let mut st = CString::default();
	let mut blank = raylib::rgui::TextBox {
		bounds: Rectangle::new(8.0, 32.0, 128.0, 16.0),
		text: st,
		text_size: 1,
		edit_mode: true,
	};
	*/
	
	// TODO: complain to raylib-rs dev about lack of support for BMFonts.
	// let font = rl.load_font(&thread, "resources/6x8_font.fnt").expect("Failed to load font.");
	
	// let mut function_input_string = String::from("sin(x)cos(z)");
	// let mut expression_to_graph = expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&function_input_string).unwrap()).unwrap();
	// let mut graph3d = graph3d::Graph3D::new(common::GraphArgs3D::Cartesian);
	// match graph3d.calculate_expression(vec![&mut expression_to_graph], 8) {
	// 	Err(e) => println!("{}", e), _ => {},
	// };
	
	let mut function_input_strings = vec![
		String::from("8sin(t)"), String::from("8cos(t)"), String::from("8cos(32t)")
	];
	let mut active = 0;
	let mut expressions_to_graph: Vec<expression::Expression> = Vec::new();
	for i in 0..function_input_strings.len() {
		expressions_to_graph.push(expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&function_input_strings[i]).unwrap()).unwrap());
	}
	let mut graph3d = graph3d::Graph3D::new(common::GraphArgs3D::Parametric(common::MinMax { min: -3.0, max: 3.0 }));
	match graph3d.calculate_expression(&mut expressions_to_graph, 8) {
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
	
	let mut closed = false;
	while !rl.window_should_close() && !closed {
		// -- Updating -- \\
		
		let time = rl.get_time();
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
		
		if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
			/*match expression::Expression::infix_tokens_from_str(&function_input_string) {
				Ok(t) => {
					match expression::Expression::new_from_infix(t) {
						Ok(e) => {
							expression_to_graph = e.clone();
							println!("{:?}", expression_to_graph);
							match graph3d.calculate_expression(vec![&mut expression_to_graph], 32) {
								Err(e) => println!("{}", e), _ => {},
							};
						},
						Err(_) => println!("error message"),
					}
				},
				Err(_) => println!("Failed to make expression."),
			}*/
			
			expressions_to_graph = Vec::new();
			for i in 0..function_input_strings.len() {
				expressions_to_graph.push(expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&function_input_strings[i]).unwrap()).unwrap());
			}
			match graph3d.calculate_expression(&mut expressions_to_graph, 64*64) {
				Err(e) => println!("{}", e), _ => {},
			}
		}
		
		// graph3d.position.x = (time as f64).sin() * 5.0;
		// graph3d.position.z = (time as f64).cos() * 5.0;
		
		rl.set_mouse_scale(1.0, 1.0);
		rl.update_camera(&mut cam);
		rl.set_mouse_scale((SCALE as f32).recip(), (SCALE as f32).recip());
		
		// -- Drawing -- \\
		
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		
		/*
		{
			let mut d = d.begin_texture_mode(&mut overlay);
			d.clear_background(Color::BLANK);
			
			if let raylib::rgui::DrawResult::Bool(b) = d.draw_gui(&wb) {
				if b { closed = true; }
			}
			if let raylib::rgui::DrawResult::Bool(b) = d.draw_gui(&btn) {
				if b { wb.bounds.x += 8.0; }
			}
			if let raylib::rgui::DrawResult::Text(txt, edit) = d.draw_gui(&blank) {
				blank.text = txt;
				blank.edit_mode = edit;
			}
		}
		d.draw_texture_pro(
			overlay.texture(),
			Rectangle::new(0.0, 0.0, 360.0, -240.0),
			Rectangle::new(0.0, 0.0, 360.0 * (SCALE as f32), 240.0 * (SCALE as f32)),
			Vector2::zero(),
			0.0,
			Color::WHITE
		);
		*/
		
		{
			let mut d = d.begin_mode_3D(cam);
			
			d.draw_grid(8, 1.0);
			graph3d.draw(&mut d);
		}
		
		for i in 0..function_input_strings.len() {
			d.draw_text(&function_input_strings[i], 12, 16 + 40 * (i as i32), 40, Color::PINK);
		}
	}
}
