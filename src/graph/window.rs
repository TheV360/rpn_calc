use raylib::prelude::*;

use crate::calculator::{expression, operator};
use super::graph;

const SCALE: i32 = 4;

pub fn start() {
	// no logging pls
	raylib::core::logging::set_trace_log(raylib::ffi::TraceLogType::LOG_NONE);
	
	let (mut rl, thread) = raylib::init()
		.size(360 * SCALE, 240 * SCALE)
		.title("Hello, world!")
		.build();
	
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
	
	let mut expression_to_graph = expression::Expression::new_from_infix(expression::Expression::infix_tokens_from_string("x(x-5)(x+2)").unwrap()).unwrap();
	graph.calculate_expression(&mut expression_to_graph, 'x', 65);
	
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::BLACK);
		
		graph.draw(&mut d);
		
		d.draw_text("Hello, world!", 12, 12, 32, Color::WHITE);
		d.draw_text(&a_very_smart_way_to_do_things[..], 12, 48, 20, Color::RED);
	}
}
