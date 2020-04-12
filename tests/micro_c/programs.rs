
use analyzer::micro_c::{
	ProgramGraph, Action::*, Type, BasicType::Int,
	Expression, BinaryOperator, UnaryOperator, Lvalue
};

use graphene::{
	core::{
		Graph
	},
};

use std::rc::Rc;
use graphene::core::property::{AddEdge, NewVertex};

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
	
	let mut g = ProgramGraph::new();
	let mut verts = Vec::new();
	for _ in 0..23{
		verts.push(g.new_vertex().unwrap());
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
	
	g.add_edge_weighted((verts[0],verts[4],DeclareVariable(mut_int, "i"))).unwrap();
	g.add_edge_weighted((verts[5],verts[1],Drop("i"))).unwrap();
	g.add_edge_weighted((verts[4],verts[6],DeclareVariable(mut_int, "x"))).unwrap();
	g.add_edge_weighted((verts[7],verts[5],Drop("x"))).unwrap();
	g.add_edge_weighted((verts[6],verts[8],DeclareVariable(mut_int, "y"))).unwrap();
	g.add_edge_weighted((verts[9],verts[7],Drop("y"))).unwrap();
	g.add_edge_weighted((verts[8],verts[10],DeclareVariable(mut_int, "z"))).unwrap();
	g.add_edge_weighted((verts[11],verts[9],Drop("z"))).unwrap();
	g.add_edge_weighted((verts[10],verts[2],DeclareArray(mut_int, "A", 10))).unwrap();
	g.add_edge_weighted((verts[3],verts[11],Drop("A"))).unwrap();
	g.add_edge_weighted((verts[13],verts[14],Read(lv_a_i.clone()))).unwrap();
	g.add_edge_weighted((verts[14],verts[2],inc_i.clone())).unwrap();
	g.add_edge_weighted((verts[2], verts[13], while_cond.clone())).unwrap();
	g.add_edge_weighted((verts[2], verts[12], while_not_cond.clone())).unwrap();
	g.add_edge_weighted((verts[18], verts[20], x_ass_x_plus_a_i)).unwrap();
	g.add_edge_weighted((verts[20],verts[17],inc_i.clone())).unwrap();
	g.add_edge_weighted((verts[19],verts[21],inc_i.clone())).unwrap();
	g.add_edge_weighted((verts[21],verts[15],Skip)).unwrap();
	g.add_edge_weighted((verts[16], verts[19], if_not_cond)).unwrap();
	g.add_edge_weighted((verts[16], verts[18], if_cond)).unwrap();
	g.add_edge_weighted((verts[17],verts[12],inc_y.clone())).unwrap();
	g.add_edge_weighted((verts[12], verts[16], while_cond.clone())).unwrap();
	g.add_edge_weighted((verts[12], verts[15], while_not_cond.clone())).unwrap();
	g.add_edge_weighted((verts[15],verts[22],write_x_div_y)).unwrap();
	g.add_edge_weighted((verts[22],verts[3],Read(lv_z))).unwrap();
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
	let mut g = ProgramGraph::new();
	let mut verts = Vec::new();
	for _ in 0..10{
		verts.push(g.new_vertex().unwrap());
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
	
	g.add_edge_weighted((verts[0],verts[4],DeclareVariable(mut_int, "x"))).unwrap();
	g.add_edge_weighted((verts[5],verts[1],Drop("x"))).unwrap();
	g.add_edge_weighted((verts[4],verts[2],DeclareVariable(mut_int, "y"))).unwrap();
	g.add_edge_weighted((verts[3],verts[5],Drop("y"))).unwrap();
	g.add_edge_weighted((verts[2],verts[6],Assign(lv_y.clone(), e_minus_1.clone()))).unwrap();
	g.add_edge_weighted((verts[6],verts[7],Assign(lv_x.clone(), e_0.clone()))).unwrap();
	g.add_edge_weighted((verts[8],verts[9],Assign(lv_x.clone(), e_x_plus_1.clone()))).unwrap();
	g.add_edge_weighted((verts[9],verts[7],Read(lv_y.clone()))).unwrap();
	g.add_edge_weighted((verts[7],verts[8],Condition(e_y_lt_0.clone()))).unwrap();
	g.add_edge_weighted((verts[7],verts[3],Condition(e_not_y_lt_0.clone()))).unwrap();
	
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
	let mut g = ProgramGraph::new();
	let mut verts = Vec::new();
	for _ in 0..14{
		verts.push(g.new_vertex().unwrap());
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
	
	g.add_edge_weighted((verts[0],verts[4],DeclareVariable(mut_int, "x"))).unwrap();
	g.add_edge_weighted((verts[5],verts[1],Drop("x"))).unwrap();
	g.add_edge_weighted((verts[4],verts[6],DeclareVariable(mut_int, "y"))).unwrap();
	g.add_edge_weighted((verts[7],verts[5],Drop("y"))).unwrap();
	g.add_edge_weighted((verts[6],verts[2],DeclareVariable(const_int_p, "p"))).unwrap();
	g.add_edge_weighted((verts[3],verts[7],Drop("p"))).unwrap();
	g.add_edge_weighted((verts[2],verts[8],Read(lv_x.clone()))).unwrap();
	g.add_edge_weighted((verts[8],verts[9],Read(lv_y.clone()))).unwrap();
	g.add_edge_weighted((verts[9],verts[10],Assign(lv_p.clone(), e_brw_const_x.clone()))).unwrap();
	g.add_edge_weighted((verts[12],verts[13],Write(e_deref_p.clone()))).unwrap();
	g.add_edge_weighted((verts[13],verts[11],Assign(lv_p.clone(), e_brw_const_y.clone()))).unwrap();
	g.add_edge_weighted((verts[10],verts[11],Condition(e_not_x_lt_1.clone()))).unwrap();
	g.add_edge_weighted((verts[10],verts[12],Condition(e_x_lt_1.clone()))).unwrap();
	g.add_edge_weighted((verts[11],verts[3],Write(e_deref_p.clone()))).unwrap();
	g
}

pub const PROBLEM_1: &'static str =
	"{\
		int data; int *r;\
		r = &'a data;\
		*r = 4;\
		data = 1;\
	}\
	";
pub fn problem_1_program_graph<'a>() -> ProgramGraph<'a>
{
	let mut g = ProgramGraph::new();
	let mut verts = Vec::new();
	for _ in 0..=7{
		verts.push(g.new_vertex().unwrap());
	}
	
	let e_1 = Rc::new(Expression::Constant(1));
	let e_4 = Rc::new(Expression::Constant(4));
	let e_data = Rc::new(Expression::Variable("data"));
	let e_brw_mut_data = Rc::new(Expression::Unary(UnaryOperator::BorrowMut("'a"),e_data.clone()));
	
	let l_deref_r = Rc::new(Lvalue::Variable(true, "r"));
	let l_data = Rc::new(Lvalue::Variable(false, "data"));
	let l_r = Rc::new(Lvalue::Variable(false, "r"));
	
	let mut_int = Type{is_pointer: false, is_mutable: true, basic_type: Int};
	let mut_int_p = Type{is_pointer: true, is_mutable: true, basic_type: Int};
	
	g.add_edge_weighted((verts[0],verts[4],DeclareVariable(mut_int, "data"))).unwrap();
	g.add_edge_weighted((verts[5],verts[1],Drop("data"))).unwrap();
	g.add_edge_weighted((verts[4],verts[2],DeclareVariable(mut_int_p, "r"))).unwrap();
	g.add_edge_weighted((verts[3],verts[5],Drop("r"))).unwrap();
	g.add_edge_weighted((verts[2],verts[6],Assign(l_r, e_brw_mut_data))).unwrap();
	g.add_edge_weighted((verts[6],verts[7],Assign(l_deref_r, e_4.clone()))).unwrap();
	g.add_edge_weighted((verts[7],verts[3],Assign(l_data, e_1))).unwrap();
	g
}

pub const PROBLEM_2: &'static str =
	"{\
		int data; int *r;\
		r = &'a data;\
		if(1 < *r){\
			*r = 1;\
		}else{\
			data = 2;\
		}\
	}\
	";

pub const INVALID_PROGRAM: &'static str =
	"{\
		int data; const int *r; int *p;\
		r = &'a const data;\
		if(1 < *r){\
			p = &'b data;\
		}else{\
			data = 1;\
		}\
		write *r;
		*p = 2;
	}\
	";