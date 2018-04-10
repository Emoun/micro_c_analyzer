
use micro_c::{
	AstVisitor,
	Block, Declaration,
	Statement, Statement::*,
	Expression, Expression::*,
	Type, BinaryOperator, UnaryOperator
};

struct ProgramGrapher{

}

impl AstVisitor for ProgramGrapher{
	//Enter methods
	fn enter_block(&mut self, block: &Block){
		unimplemented!()
	}
	fn enter_declaration(&mut self, decl: &Declaration){
		unimplemented!()
	}
	
	fn enter_statement(&mut self, stmt: &Statement){
		unimplemented!()
	}
	
	fn enter_expression(&mut self, expr: &Expression){
		unimplemented!()
	}
	
	//exit methods
	fn exit_block(&mut self, block: &Block){
		unimplemented!()
	}
	fn exit_declaration(&mut self, decl: &Declaration){
		unimplemented!()
	}
	
	fn exit_statement(&mut self, stmt: &Statement){
		unimplemented!()
	}
	
	fn exit_expression(&mut self, expr: &Expression){
		unimplemented!()
	}
	
}
