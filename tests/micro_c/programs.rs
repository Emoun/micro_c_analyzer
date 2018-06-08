
use analyzer::micro_c::{
	ProgramGraph, Action::*, Type, BasicType::Int,
	Expression, BinaryOperator, UnaryOperator, Lvalue
};

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
	
	for i in 0..23{
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
	let lv_x = Rc::new(Lvalue::Variable(false, "x"));
	let lv_i = Rc::new(Lvalue::Variable(false, "i"));
	let lv_y = Rc::new(Lvalue::Variable(false, "y"));
	let lv_z = Rc::new(Lvalue::Variable(false, "z"));
	let lv_a_i = Rc::new(Lvalue::ArrayAccess(false, "A", e_i.clone()));
	
	
	let inc_i = Assign(lv_i.clone(), e_i_plus_1);
	let while_cond = Condition(e_i_lt_10.clone());
	let while_not_cond = Condition(e_i_ge_10.clone());
	let if_cond = Condition(e_a_i_plus_1_ge_0.clone());
	let if_not_cond = Condition(e_a_i_plus_1_lt_0.clone());
	let x_ass_x_plus_a_i = Assign(lv_x.clone(), r_x_plus_a_i);
	let inc_y = Assign(lv_y.clone(), e_y_plus_1);
	let write_x_div_y = Write(r_x_div_y);
	let mut_int = Type{is_pointer: false, is_mutable: true, basic_type: Int};
	
	g.add_edge_weighted((0,4),DeclareVariable(mut_int, "i")).unwrap();
	g.add_edge_weighted((5,1),Drop("i")).unwrap();
	g.add_edge_weighted((4,6),DeclareVariable(mut_int, "x")).unwrap();
	g.add_edge_weighted((7,5),Drop("x")).unwrap();
	g.add_edge_weighted((6,8),DeclareVariable(mut_int, "y")).unwrap();
	g.add_edge_weighted((9,7),Drop("y")).unwrap();
	g.add_edge_weighted((8,10),DeclareVariable(mut_int, "z")).unwrap();
	g.add_edge_weighted((11,9),Drop("z")).unwrap();
	g.add_edge_weighted((10,2),DeclareArray(mut_int, "A", 10)).unwrap();
	g.add_edge_weighted((3,11),Drop("A")).unwrap();
	g.add_edge_weighted((13,14),Read(lv_a_i.clone())).unwrap();
	g.add_edge_weighted((14,2),inc_i.clone()).unwrap();
	g.add_edge_weighted((2, 13), while_cond.clone()).unwrap();
	g.add_edge_weighted((2, 12), while_not_cond.clone()).unwrap();
	g.add_edge_weighted((18, 20), x_ass_x_plus_a_i).unwrap();
	g.add_edge_weighted((20,17),inc_i.clone()).unwrap();
	g.add_edge_weighted((19,21),inc_i.clone()).unwrap();
	g.add_edge_weighted((21,15),Skip).unwrap();
	g.add_edge_weighted((16, 19), if_not_cond).unwrap();
	g.add_edge_weighted((16, 18), if_cond).unwrap();
	g.add_edge_weighted((17,12),inc_y.clone()).unwrap();
	g.add_edge_weighted((12, 16), while_cond.clone()).unwrap();
	g.add_edge_weighted((12, 15), while_not_cond.clone()).unwrap();
	g.add_edge_weighted((15,22),write_x_div_y).unwrap();
	g.add_edge_weighted((22,3),Read(lv_z)).unwrap();
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
	for i in 0..10{
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
	let lv_x = Rc::new(Lvalue::Variable(false, "x"));
	let lv_y = Rc::new(Lvalue::Variable(false, "y"));
	
	let mut_int = Type{is_pointer: false, is_mutable: true, basic_type: Int};
	
	g.add_edge_weighted((0,4),DeclareVariable(mut_int, "x")).unwrap();
	g.add_edge_weighted((5,1),Drop("x")).unwrap();
	g.add_edge_weighted((4,2),DeclareVariable(mut_int, "y")).unwrap();
	g.add_edge_weighted((3,5),Drop("y")).unwrap();
	g.add_edge_weighted((2,6),Assign(lv_y.clone(), e_minus_1.clone())).unwrap();
	g.add_edge_weighted((6,7),Assign(lv_x.clone(), e_0.clone())).unwrap();
	g.add_edge_weighted((8,9),Assign(lv_x.clone(), e_x_plus_1.clone())).unwrap();
	g.add_edge_weighted((9,7),Read(lv_y.clone())).unwrap();
	g.add_edge_weighted((7,8),Condition(e_y_lt_0.clone())).unwrap();
	g.add_edge_weighted((7,3),Condition(e_not_y_lt_0.clone())).unwrap();
	
	g
}

pub const P3: &'static str =
	"{\
		int x; int y;\
		const int *p;\
		read x; read y;\
		p = &'x const x;\
		if(x<1){\
			write *p;\
			p = &'y const y;\
		}\
		write *p;\
	}\
	";

pub fn p3_program_graph<'a>() -> ProgramGraph<'a> {
	let mut g = ProgramGraph::empty_graph();
	for i in 0..14{
		g.add_vertex(i).unwrap();
	}
	
	let e_1 = Rc::new(Expression::Constant(1));
	let e_x = Rc::new(Expression::Variable("x"));
	let e_y = Rc::new(Expression::Variable("y"));
	let e_p = Rc::new(Expression::Variable("p"));
	let e_brw_const_x = Rc::new(Expression::Unary(UnaryOperator::BorrowConst("'x"),e_x.clone()));
	let e_brw_const_y = Rc::new(Expression::Unary(UnaryOperator::BorrowConst("'y"),e_y.clone()));
	let e_x_lt_1 = Rc::new(Expression::Binary(e_x.clone(),BinaryOperator::LessThan, e_1.clone()));
	let e_not_x_lt_1 = Rc::new(Expression::Unary(UnaryOperator::Not, e_x_lt_1.clone()));
	let e_deref_p = Rc::new(Expression::Unary(UnaryOperator::Deref, e_p.clone()));
	let lv_x = Rc::new(Lvalue::Variable(false, "x"));
	let lv_y = Rc::new(Lvalue::Variable(false, "y"));
	let lv_p = Rc::new(Lvalue::Variable(false, "p"));
	
	let mut_int = Type{is_pointer: false, is_mutable: true, basic_type: Int};
	let const_int_p = Type{is_pointer: true, is_mutable: false, basic_type: Int};
	
	g.add_edge_weighted((0,4),DeclareVariable(mut_int, "x")).unwrap();
	g.add_edge_weighted((5,1),Drop("x")).unwrap();
	g.add_edge_weighted((4,6),DeclareVariable(mut_int, "y")).unwrap();
	g.add_edge_weighted((7,5),Drop("y")).unwrap();
	g.add_edge_weighted((6,2),DeclareVariable(const_int_p, "p")).unwrap();
	g.add_edge_weighted((3,7),Drop("p")).unwrap();
	g.add_edge_weighted((2,8),Read(lv_x.clone())).unwrap();
	g.add_edge_weighted((8,9),Read(lv_y.clone())).unwrap();
	g.add_edge_weighted((9,10),Assign(lv_p.clone(), e_brw_const_x.clone())).unwrap();
	g.add_edge_weighted((12,13),Write(e_deref_p.clone())).unwrap();
	g.add_edge_weighted((13,11),Assign(lv_p.clone(), e_brw_const_y.clone())).unwrap();
	g.add_edge_weighted((10,11),Condition(e_not_x_lt_1.clone())).unwrap();
	g.add_edge_weighted((10,12),Condition(e_x_lt_1.clone())).unwrap();
	g.add_edge_weighted((11,3),Write(e_deref_p.clone())).unwrap();
	g
}

pub const P4: &'static str =
	"{\
		int data[4];\
		int *p[4];\
		p = &'a data;\
		*p[1] = 4;\
		data[1] = 1;\
	}\
	";
pub fn p4_program_graph<'a>() -> ProgramGraph<'a>
{
	let mut g = ProgramGraph::empty_graph();
	for i in 0..=7{
		g.add_vertex(i).unwrap();
	}
	
	let e_1 = Rc::new(Expression::Constant(1));
	let e_4 = Rc::new(Expression::Constant(4));
	let e_data = Rc::new(Expression::Variable("data"));
	let e_brw_mut_data = Rc::new(Expression::Unary(UnaryOperator::BorrowMut("'a"),e_data.clone()));
	
	let l_deref_p_1 = Rc::new(Lvalue::ArrayAccess(true, "p", e_1.clone()));
	let l_data_1 = Rc::new(Lvalue::ArrayAccess(false, "data", e_1.clone()));
	let l_p = Rc::new(Lvalue::Variable(false, "p"));
	
	let mut_int = Type{is_pointer: false, is_mutable: true, basic_type: Int};
	let mut_int_p = Type{is_pointer: true, is_mutable: true, basic_type: Int};
	
	g.add_edge_weighted((0,4),DeclareArray(mut_int, "data", 4)).unwrap();
	g.add_edge_weighted((5,1),Drop("data")).unwrap();
	g.add_edge_weighted((4,2),DeclareArray(mut_int_p, "p", 4)).unwrap();
	g.add_edge_weighted((3,5),Drop("p")).unwrap();
	g.add_edge_weighted((2,6),Assign(l_p, e_brw_mut_data)).unwrap();
	g.add_edge_weighted((6,7),Assign(l_deref_p_1, e_4.clone())).unwrap();
	g.add_edge_weighted((7,3),Assign(l_data_1, e_1)).unwrap();
	g
}

pub const P5: &'static str =
	"{\
		int x;\
		int *p;\
		p = &'a x;\
		if(1 < *p){\
			*p = 1;\
		}else{\
			x = 2;\
		}\
	}\
	";
