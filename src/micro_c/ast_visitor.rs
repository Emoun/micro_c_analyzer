#![allow(unused_variables)]
use crate::micro_c::{
	Block, Declaration,
	Statement, Statement::*,
	Expression, Expression::*,
	Type, BinaryOperator, UnaryOperator, Lvalue
};
use std::rc::Rc;

pub trait AstVisitor<'a>{
//Enter methods
	fn enter_block(&mut self, block: Rc<Block<'a>>){}
	fn enter_declaration(&mut self, decl: Rc<Declaration<'a>>){}
	fn enter_declaration_variable(&mut self, t:Type, name: &'a str){}
	fn enter_declaration_array(&mut self, t:Type, name: &'a str, len: i32){}
	fn enter_declaration_composite(&mut self, c1: Rc<Declaration<'a>>, c2: Rc<Declaration<'a>>){}
	
	fn enter_statement(&mut self, stmt: Rc<Statement<'a>>){}
	fn enter_statement_assign(&mut self, lvalue: Rc<Lvalue<'a>>, expr: Rc<Expression<'a>>){}
	fn enter_statement_if_else(&mut self,
							   cond: Rc<Expression<'a>>, if_true: Rc<Block<'a>>, if_false: Option<Rc<Block<'a>>>){}
	fn enter_statement_while(&mut self, cond: Rc<Expression<'a>>, body: Rc<Block<'a>>){}
	fn enter_statement_read(&mut self, lvalue: Rc<Lvalue<'a>>){}
	fn enter_statement_write(&mut self, value: Rc<Expression<'a>>){}
	fn enter_statement_break(&mut self){}
	fn enter_statement_continue(&mut self){}
	fn enter_statement_composite(&mut self, s1: Rc<Statement<'a>>, s2:Rc<Statement<'a>>){}
	fn enter_statement_scope(&mut self, block: Rc<Block<'a>>){}
	
	fn enter_expression(&mut self, expr: Rc<Expression<'a>>){}
	fn enter_expression_constant(&mut self, value: i32){}
	fn enter_expression_variable(&mut self, name: &'a str){}
	fn enter_expression_array_access(&mut self, name: &'a str, index: Rc<Expression<'a>>){}
	fn enter_expression_binary(&mut self, lhs: Rc<Expression<'a>>, op: BinaryOperator, rhs: Rc<Expression<'a>>){}
	fn enter_expression_unary(&mut self, op: UnaryOperator, rhs: Rc<Expression<'a>>){}
	
//exit methods
	fn exit_block(&mut self, block: Rc<Block<'a>>){}
	fn exit_declaration(&mut self, decl: Rc<Declaration<'a>>){}
	fn exit_declaration_variable(&mut self, t:Type, name: &'a str){}
	fn exit_declaration_array(&mut self, t:Type, name: &'a str, len: i32){}
	fn exit_declaration_composite(&mut self, c1: Rc<Declaration<'a>>, c2: Rc<Declaration<'a>>){}
	
	fn exit_statement(&mut self, stmt: Rc<Statement<'a>>){}
	fn exit_statement_assign(&mut self, lvalue: Rc<Lvalue<'a>>, expr: Rc<Expression<'a>>){}
	fn exit_statement_if_else(&mut self,
							   cond: Rc<Expression<'a>>, if_true: Rc<Block<'a>>, if_false: Option<Rc<Block<'a>>>){}
	fn exit_statement_while(&mut self, cond: Rc<Expression<'a>>, body: Rc<Block<'a>>){}
	fn exit_statement_read(&mut self, lvalue: Rc<Lvalue<'a>>){}
	fn exit_statement_write(&mut self, value: Rc<Expression<'a>>){}
	fn exit_statement_break(&mut self){}
	fn exit_statement_continue(&mut self){}
	fn exit_statement_composite(&mut self, s1: Rc<Statement<'a>>, s2:Rc<Statement<'a>>){}
	fn exit_statement_scope(&mut self, block: Rc<Block<'a>>){}
	
	fn exit_expression(&mut self, expr: Rc<Expression<'a>>){}
	fn exit_expression_constant(&mut self, value: i32){}
	fn exit_expression_variable(&mut self, name: &'a str){}
	fn exit_expression_array_access(&mut self, name: &'a str, index: Rc<Expression<'a>>){}
	fn exit_expression_binary(&mut self, lhs: Rc<Expression<'a>>, op: BinaryOperator, rhs: Rc<Expression<'a>>){}
	fn exit_expression_unary(&mut self, op: UnaryOperator, rhs: Rc<Expression<'a>>){}
	
//Visit methods
	fn visit(&mut self, ast: Rc<Block<'a>>){
		self.visit_block(ast);
	}

	fn visit_block(&mut self, block: Rc<Block<'a>>){
		self.enter_block(block.clone());
		if let Some(ref decl) = block.declarations {
			self.visit_declaration(decl.clone());
		}
		self.visit_statement(block.statements.clone());
		self.exit_block(block);
	}
	fn visit_declaration(&mut self, decl: Rc<Declaration<'a>>){
		self.enter_declaration(decl.clone());
		match *decl {
			Declaration::Variable(t, name) =>
				self.visit_declaration_variable(t, name),
			Declaration::Array(t, n, i) =>
				self.visit_declaration_array(t,n,i),
			Declaration::Composite(ref d1, ref d2) =>
				self.visit_declaration_composite(d1.clone(), d2.clone()),
		}
		self.exit_declaration(decl);
	}
	fn visit_declaration_variable(&mut self, t:Type, name: &'a str){
		self.enter_declaration_variable(t, name);
		self.exit_declaration_variable(t, name);
	}
	fn visit_declaration_array(&mut self, t:Type, name: &'a str, len: i32){
		self.enter_declaration_array(t, name, len);
		self.exit_declaration_array(t, name, len);
	}
	fn visit_declaration_composite(&mut self, c1: Rc<Declaration<'a>>, c2: Rc<Declaration<'a>>){
		self.enter_declaration_composite(c1.clone(), c2.clone());
		self.visit_declaration(c1.clone());
		self.visit_declaration(c2.clone());
		self.exit_declaration_composite(c1, c2);
	}
	
	fn visit_statement(&mut self, stmt: Rc<Statement<'a>>){
		self.enter_statement(stmt.clone());
		
		match *stmt {
			Assign(ref lvalue, ref expr) =>self.visit_statement_assign(lvalue.clone(), expr.clone()),
			IfElse( ref cond,  ref if_true,  ref if_false) =>
				if let Some(ref block) = *if_false {
					self.visit_statement_if_else(cond.clone(), if_true.clone(), Some(block.clone()))
				} else {
					self.visit_statement_if_else(cond.clone(), if_true.clone(), None)
				},
			While(ref cond, ref body) => self.visit_statement_while(cond.clone(), body.clone()),
			Read(ref lvalue) => self.visit_statement_read(lvalue.clone()),
			Write(ref expr) => self.visit_statement_write(expr.clone()),
			Break => self.visit_statement_break(),
			Continue => self.visit_statement_continue(),
			Composite(ref s1, ref s2) => self.visit_statement_composite(s1.clone(), s2.clone()),
			Scope(ref block) => self.visit_statement_scope(block.clone())
		}
		
		self.exit_statement(stmt);
	}
	fn visit_statement_assign(&mut self, lvalue: Rc<Lvalue<'a>>, expr: Rc<Expression<'a>>){
		self.enter_statement_assign(lvalue.clone(), expr.clone());
		self.visit_expression(expr.clone());
		self.exit_statement_assign(lvalue, expr);
	}
	fn visit_statement_if_else(&mut self,
							   cond: Rc<Expression<'a>>, if_true: Rc<Block<'a>>, if_false: Option<Rc<Block<'a>>>){
		self.enter_statement_if_else(cond.clone(), if_true.clone(), if_false.clone());
		self.visit_expression(cond.clone());
		self.visit_block(if_true.clone());
		if let Some(ref block) = if_false {
			self.visit_block(block.clone());
		}
		self.exit_statement_if_else(cond, if_true, if_false);
	}
	fn visit_statement_while(&mut self, cond: Rc<Expression<'a>>, body: Rc<Block<'a>>){
		self.enter_statement_while(cond.clone(), body.clone());
		self.visit_expression(cond.clone());
		self.visit_block(body.clone());
		self.exit_statement_while(cond, body);
	}
	fn visit_statement_read(&mut self, lvalue: Rc<Lvalue<'a>>){
		self.enter_statement_read(lvalue.clone());
		self.exit_statement_read(lvalue);
	}
	fn visit_statement_write(&mut self, value: Rc<Expression<'a>>){
		self.enter_statement_write(value.clone());
		self.visit_expression(value.clone());
		self.exit_statement_write(value);
	}
	fn visit_statement_break(&mut self){
		self.enter_statement_break();
		self.exit_statement_break();
	}
	fn visit_statement_continue(&mut self){
		self.enter_statement_continue();
		self.exit_statement_continue();
	}
	fn visit_statement_composite(&mut self, s1: Rc<Statement<'a>>, s2:Rc<Statement<'a>>){
		self.enter_statement_composite(s1.clone(), s2.clone());
		self.visit_statement(s1.clone());
		self.visit_statement(s2.clone());
		self.exit_statement_composite(s1, s2);
	}
	fn visit_statement_scope(&mut self, block: Rc<Block<'a>>){
		self.enter_statement_scope(block.clone());
		self.visit_block(block.clone());
		self.exit_statement_scope(block);
	}
	
	fn visit_expression(&mut self, expr: Rc<Expression<'a>>){
		self.enter_expression(expr.clone());
		
		match *expr {
			Constant(n) => self.visit_expression_constant(n),
			Variable(x) => self.visit_expression_variable(x),
			ArrayAccess(x, ref index) => self.visit_expression_array_access(x, index.clone()),
			Binary(ref lhs, op, ref rhs) => self.visit_expression_binary(lhs.clone(), op, rhs.clone()),
			Unary(op, ref rhs) => self.visit_expression_unary(op, rhs.clone()),
		}
		
		self.exit_expression(expr);
	}
	fn visit_expression_constant(&mut self, value: i32){
		self.enter_expression_constant(value);
		self.exit_expression_constant(value);
	}
	fn visit_expression_variable(&mut self, name: &'a str){
		self.enter_expression_variable(name);
		self.exit_expression_variable(name);
	}
	fn visit_expression_array_access(&mut self, name: &'a str, index: Rc<Expression<'a>>){
		self.enter_expression_array_access(name, index.clone());
		self.visit_expression(index.clone());
		self.exit_expression_array_access(name, index);
	}
	fn visit_expression_binary(&mut self, lhs: Rc<Expression<'a>>, op: BinaryOperator, rhs: Rc<Expression<'a>>){
		self.enter_expression_binary(lhs.clone(), op, rhs.clone());
		self.visit_expression(lhs.clone());
		self.visit_expression(rhs.clone());
		self.exit_expression_binary(lhs, op, rhs);
	}
	fn visit_expression_unary(&mut self, op: UnaryOperator, rhs: Rc<Expression<'a>>){
		self.enter_expression_unary(op, rhs.clone());
		self.visit_expression(rhs.clone());
		self.exit_expression_unary(op, rhs);
	}
}

