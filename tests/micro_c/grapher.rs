
use super::programs::*;
use analyzer::micro_c::{
	ProgramParser, ProgramGrapher, AstVisitor, ProgramGraph
};
use std::rc::Rc;

#[test]
fn test_program_1(){
	let g_expected = p1_program_graph();
	test_program_graph(&g_expected, P1);
}

#[test]
fn test_program_2(){
	let g_expected = p2_program_graph();
	test_program_graph(&g_expected, P2);
}

#[test]
fn test_program_3(){
	let g_expected = p3_program_graph();
	test_program_graph(&g_expected, P3);
}

#[test]
fn test_problem_1(){
	let g_expected = problem_1_program_graph();
	test_program_graph(&g_expected, PROBLEM_1);
}

// Helper functions

fn test_program_graph(g_expected: &ProgramGraph, src: &str)
{
	let ast = ProgramParser::new().parse(src).unwrap();
	
	let mut grapher = ProgramGrapher::new();
	grapher.visit(Rc::new(ast));
	
	let g_actual = grapher.get_graph();
	
	assert_eq!(format!("{:?}",g_expected), format!("{:?}",g_actual));
}