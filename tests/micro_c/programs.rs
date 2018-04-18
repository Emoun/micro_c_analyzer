
use analyzer::micro_c::{
	ProgramGraph, Action::*, Type::Int,
	Expression, BinaryOperator, UnaryOperator};
use std::rc::Rc;

pub const P1: &'static str =
	"{
		int i; int x; int y; int z;
		int A[10];
		while (i<10){
			read A[i];
			i=i+1;
		}
		while (i<10){
			if (A[i]+1>=0){
				x=x+A[i];
				i=i+1;
			} else {
				i=i+1;
				break;
			}
			y=y+1;
		}
		write x/y;
		read z;
	}";

pub fn p1_program_graph<'a>() -> ProgramGraph<'a> {
	
	let mut g = ProgramGraph::new();
	let mut v = Vec::new();
	for i in 0..18{
		v.push(g.add_node(()));
	}
	
	let e_0 = Rc::new(Expression::Constant(0));
	let e_1 = Rc::new(Expression::Constant(1));
	let e_10 = Rc::new(Expression::Constant(10));
	let e_i = Rc::new(Expression::Variable("i"));
	let e_x = Rc::new(Expression::Variable("x"));
	let e_y = Rc::new(Expression::Variable("y"));
	let e_i_lt_10 = Rc::new(Expression::Binary(e_i.clone(), BinaryOperator::LessThan, e_10.clone()));
	let e_i_ge_10 = Rc::new(Expression::Unary(UnaryOperator::Not, e_i_lt_10.clone()));
	let e_i_plus_1 = Rc::new(Expression::Binary(e_i.clone(), BinaryOperator::Plus, e_1.clone()));
	let e_a_i = Rc::new(Expression::ArrayAccess("A", e_i.clone()));
	let e_a_i_plus_1 = Rc::new(Expression::Binary(e_a_i.clone(), BinaryOperator::Plus, e_1.clone()));
	let e_a_i_plus_1_ge_0 = Rc::new(Expression::Binary(
		e_a_i_plus_1.clone(), BinaryOperator::GreaterOrEqual, e_0.clone()));
	let e_a_i_plus_1_lt_0 = Rc::new(Expression::Unary(UnaryOperator::Not, e_a_i_plus_1_ge_0.clone()));
	let r_x_plus_a_i = Rc::new(Expression::Binary(e_x.clone(), BinaryOperator::Plus, e_a_i.clone()));
	let e_y_plus_1 = Rc::new(Expression::Binary(e_y.clone(), BinaryOperator::Plus, e_1.clone()));
	let r_x_div_y = Rc::new(Expression::Binary(e_x.clone(), BinaryOperator::Division, e_y.clone()));
	
	let inc_i = Assign("i", e_i_plus_1);
	let while_cond = Condition(e_i_lt_10.clone());
	let while_not_cond = Condition(e_i_ge_10.clone());
	let if_cond = Condition(e_a_i_plus_1_ge_0.clone());
	let if_not_cond = Condition(e_a_i_plus_1_lt_0.clone());
	let x_ass_x_plus_a_i = Assign("x", r_x_plus_a_i);
	let inc_y = Assign("y", e_y_plus_1);
	let write_x_div_y = Write(r_x_div_y);
	
	g.add_edge(v[0],v[3],DeclareVariable(Int, "i"));
	g.add_edge(v[3],v[4],DeclareVariable(Int, "x"));
	g.add_edge(v[4],v[5],DeclareVariable(Int, "y"));
	g.add_edge(v[5],v[6],DeclareVariable(Int, "z"));
	g.add_edge(v[6],v[2],DeclareArray(Int, "A", 10));
	g.add_edge(v[2], v[8], while_cond.clone());
	g.add_edge(v[2], v[7], while_not_cond.clone());
	g.add_edge(v[8],v[9],ReadArray("A", e_i.clone()));
	g.add_edge(v[9],v[2],inc_i.clone());
	g.add_edge(v[7], v[10], while_not_cond.clone());
	g.add_edge(v[7], v[11], while_cond.clone());
	g.add_edge(v[11], v[13], if_cond);
	g.add_edge(v[11], v[14], if_not_cond);
	g.add_edge(v[13], v[15], x_ass_x_plus_a_i);
	g.add_edge(v[15],v[12],inc_i.clone());
	g.add_edge(v[14],v[16],inc_i.clone());
	g.add_edge(v[16],v[10],Skip);
	g.add_edge(v[12],v[7],inc_y.clone());
	g.add_edge(v[10],v[17],write_x_div_y);
	g.add_edge(v[17],v[1],Read("z"));
	g
}