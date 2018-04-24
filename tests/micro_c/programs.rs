
use analyzer::micro_c::{
	ProgramGraph, Action::*, Type::Int,
	Expression, BinaryOperator, UnaryOperator};

use graphene::{
	core::{
		BaseGraph, EdgeWeightedGraph
	},
};

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
			if ((A[i]+1)>=0){
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
	
	let mut g = ProgramGraph::empty_graph();
	
	for i in 0..18{
		g.add_vertex(i).unwrap();
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
	
	g.add_edge_weighted((0,3),DeclareVariable(Int, "i")).unwrap();
	g.add_edge_weighted((3,4),DeclareVariable(Int, "x")).unwrap();
	g.add_edge_weighted((4,5),DeclareVariable(Int, "y")).unwrap();
	g.add_edge_weighted((5,6),DeclareVariable(Int, "z")).unwrap();
	g.add_edge_weighted((6,2),DeclareArray(Int, "A", 10)).unwrap();
	g.add_edge_weighted((8,9),ReadArray("A", e_i.clone())).unwrap();
	g.add_edge_weighted((9,2),inc_i.clone()).unwrap();
	g.add_edge_weighted((2, 8), while_cond.clone()).unwrap();
	g.add_edge_weighted((2, 7), while_not_cond.clone()).unwrap();
	g.add_edge_weighted((13, 15), x_ass_x_plus_a_i).unwrap();
	g.add_edge_weighted((15,12),inc_i.clone()).unwrap();
	g.add_edge_weighted((14,16),inc_i.clone()).unwrap();
	g.add_edge_weighted((16,10),Skip).unwrap();
	g.add_edge_weighted((11, 14), if_not_cond).unwrap();
	g.add_edge_weighted((11, 13), if_cond).unwrap();
	g.add_edge_weighted((12,7),inc_y.clone()).unwrap();
	g.add_edge_weighted((7, 11), while_cond.clone()).unwrap();
	g.add_edge_weighted((7, 10), while_not_cond.clone()).unwrap();
	g.add_edge_weighted((10,17),write_x_div_y).unwrap();
	g.add_edge_weighted((17,1),Read("z")).unwrap();
	g
}

pub const P2: &'static str =
	"{\
		int x; int y;\
		y = -1;\
		x = 0;\
		while(y<0){\
			x = x + 1;\
			read y;\
		}\
	}\
	";

pub fn p2_program_graph<'a>() -> ProgramGraph<'a> {
	let mut g = ProgramGraph::empty_graph();
	for i in 0..8{
		g.add_vertex(i).unwrap();
	}
	
	let e_0 = Rc::new(Expression::Constant(0));
	let e_1 = Rc::new(Expression::Constant(1));
	let e_x = Rc::new(Expression::Variable("x"));
	let e_y = Rc::new(Expression::Variable("y"));
	let e_minus_1 = Rc::new(Expression::Unary(UnaryOperator::Negative, e_1.clone()));
	let e_y_lt_0 = Rc::new(Expression::Binary(e_y.clone(), BinaryOperator::LessThan, e_0.clone()));
	let e_not_y_lt_0 = Rc::new(Expression::Unary(UnaryOperator::Not, e_y_lt_0.clone()));
	let e_x_plus_1 = Rc::new(Expression::Binary(e_x.clone(), BinaryOperator::Plus, e_1.clone()));
	
	g.add_edge_weighted((0,3),DeclareVariable(Int, "x")).unwrap();
	g.add_edge_weighted((3,2),DeclareVariable(Int, "y")).unwrap();
	g.add_edge_weighted((2,4),Assign("y", e_minus_1.clone())).unwrap();
	g.add_edge_weighted((4,5),Assign("x", e_0.clone())).unwrap();
	g.add_edge_weighted((6,7),Assign("x", e_x_plus_1.clone())).unwrap();
	g.add_edge_weighted((7,5),Read("y")).unwrap();
	g.add_edge_weighted((5,6),Condition(e_y_lt_0.clone())).unwrap();
	g.add_edge_weighted((5,1),Condition(e_not_y_lt_0.clone())).unwrap();
	
	g
}