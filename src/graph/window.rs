use raylib::prelude::*;
// use std::ffi::{CStr, CString};

use crate::calculator::{expression, operator};
use super::graph;

const SCALE: i32 = 4;

pub fn start() {
	// no logging pls
	raylib::core::logging::set_trace_log(raylib::ffi::TraceLogType::LOG_NONE);
	
	let (mut rl, thread) = raylib::init()
		.size(360 * SCALE, 240 * SCALE)
		.title("Graphing Calculator")
		.build();
	
	// TODO: complain to raylib-rs dev about lack of support for GUILoadStyle
	// unsafe {
	// 	raylib::ffi::GuiLoadStyle(CString::new("resources/cyber.rgs").unwrap().as_ptr());
	// }
	
	// let mut wb = raylib::rgui::WindowBox {
	// 	bounds: Rectangle::new(0.0, 0.0, 240.0, 120.0),
	// 	text: CString::new("Hello, world!").unwrap(),
	// };
	// 
	// let btn = raylib::rgui::Button {
	// 	bounds: Rectangle::new(8.0, 8.0, 64.0, 16.0),
	// 	text: CString::new("aaaaaa").unwrap(),
	// };
	
	// TODO: complain to raylib-rs dev about lack of support for BMFonts.
	// let font = rl.load_font(&thread, "resources/6x8_font.fnt").expect("Failed to load font.");
	
	let exp = expression::Expression::new_from_infix(vec![
		expression::Token::Constant(2.0),
		expression::Token::Operator(operator::Operator::Add),
		expression::Token::Constant(3.0),
	]).expect("math machine broke");
	let a_very_smart_way_to_do_things = format!("Did you know? {:?} = {}", exp, exp.calculate().expect("OH NO"));
	
	let mut graph = graph::Graph::new();
	
	// let mut expression_to_graph = expression::Expression::new(vec![
	// 	expression::Token::Variable('😂'),
	// 	expression::Token::Constant(2.0),
	// 	expression::Token::Operator(operator::Operator::Pow),
	// ]);
	// graph.calculate_expression(&mut expression_to_graph, '😂', 65);
	
	let mut function_input_string = String::from("0.2x(x-5)(x+5)");
	
	let mut expression_to_graph = expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_str(&function_input_string).unwrap()).unwrap();
	graph.calculate_expression(&mut expression_to_graph, 'x', 65);
	
	while !rl.window_should_close() {
		let pressed_key = rl.get_key_pressed_number();
		
		if let Some(key) = pressed_key {
			if let Some(key_char) = std::char::from_u32(key) {
				function_input_string.push(key_char);
			}
		}
		
		if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
			function_input_string.pop();
		}
		
		if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
			match expression::Expression::infix_tokens_from_str(&function_input_string) {
				Ok(t) => {
					match expression::Expression::new_from_infix(t) {
						Ok(e) => {
							expression_to_graph = e.clone();
							graph.calculate_expression(&mut expression_to_graph, 'x', 65);
						},
						Err(_) => println!("error message"),
					}
				},
				Err(_) => println!("Failed to make expression."),
			}
		}
		
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		
		graph.draw(&mut d);
		
		d.draw_text(&a_very_smart_way_to_do_things, 12, 48, 20, Color::RED);
		d.draw_text(&function_input_string, 12, 180, 40, Color::PINK);
		
		// d.draw_gui(&wb);
		// if let raylib::rgui::DrawResult::Bool(b) = d.draw_gui(&btn) {
		// 	if b { wb.bounds.x += 8.0; }
		// }
	}
}
