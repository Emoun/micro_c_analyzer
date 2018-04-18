#[allow(dead_code)]
use micro_c::{
	AstVisitor,
	Block, Declaration,
	Statement, Statement::*,
	Expression, Expression::*,
	Type, BinaryOperator, UnaryOperator
};
use std::rc::Rc;

struct ProgramGrapher{

}

impl AstVisitor for ProgramGrapher{
	//Enter methods
	fn enter_block(&mut self, block: Rc<Block>){
		unimplemented!()
	}
	fn enter_declaration(&mut self, decl: Rc<Declaration>){
		unimplemented!()
	}
	
	fn enter_statement(&mut self, stmt: Rc<Statement>){
		unimplemented!()
	}
	
	fn enter_expression(&mut self, expr: Rc<Expression>){
		unimplemented!()
	}
	
	//exit methods
	fn exit_block(&mut self, block: Rc<Block>){
		unimplemented!()
	}
	fn exit_declaration(&mut self, decl: Rc<Declaration>){
		unimplemented!()
	}
	
	fn exit_statement(&mut self, stmt: Rc<Statement>){
		unimplemented!()
	}
	
	fn exit_expression(&mut self, expr: Rc<Expression>){
		unimplemented!()
	}
	
}
