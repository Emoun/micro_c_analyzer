
use super::programs::*;
use analyzer::micro_c::{ProgramParser, ProgramGrapher, AstVisitor};
use std::rc::Rc;
#[test]
fn test_program_1(){
	let g_expected = p1_program_graph();
	let ast = ProgramParser::new().parse(P1).unwrap();
	
	let mut grapher = ProgramGrapher::new();
	grapher.visit(Rc::new(ast));
	
	let g_actual = grapher.get_graph();
	
	assert_eq!(format!("{:?}",g_expected), format!("{:?}",g_actual));
}