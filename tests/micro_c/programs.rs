
use analyzer::micro_c::{
	ProgramGraph, Action, Action::*, Type::Int,
	Expression, BinaryOperator, UnaryOperator};
use graphene::core::{BaseGraph, BaseEdge as Edge};

const P1: &'static str =
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

fn p1_program_graph<'a>()-> (Vec<Edge<u32, Action<'a>>>, Vec<Box<Expression<'a>>>){
	
	let mut edges = Vec::new();
	let mut expr = Vec::new();
	
	edges.push(Edge::new(0,3,DeclareVariable(Int, "i")));
	edges.push(Edge::new(3,4,DeclareVariable(Int, "x")));
	edges.push(Edge::new(4,5,DeclareVariable(Int, "y")));
	edges.push(Edge::new(5,6,DeclareVariable(Int, "z")));
	edges.push(Edge::new(6,2,DeclareArray(Int, "z", 10)));
	
	let e_10 = Box::new(Expression::Constant(10));
	let e_i = Box::new(Expression::Variable("i"));
	let e_i_lt_10 = Box::new(Expression::Binary(e_i, BinaryOperator::LessThan, e_10));
	expr.push(Box::new(Expression::Unary(UnaryOperator::Not, e_i_lt_10)));
	
	edges.push(Edge::new(2,8,Condition(expr[0].as_ref())));
	
	(edges, expr)
}