#![allow(unused_variables)]
use micro_c::{
	Block, Declaration,
	Statement, Statement::*,
	Expression, Expression::*,
	Type, BinaryOperator, UnaryOperator};

pub trait AstVisitor{
//Enter methods
	fn enter_block(&mut self, block: &Block);
	fn enter_declaration(&mut self, decl: &Declaration);
	fn enter_declaration_variable(&mut self, t:Type, name: &str){}
	fn enter_declaration_array(&mut self, t:Type, name: &str, len: i32){}
	fn enter_declaration_composite(&mut self, c1: &Declaration, c2: &Declaration){}
	
	fn enter_statement(&mut self, stmt: &Statement);
	fn enter_statement_assign(&mut self, name: &str, expr: &Expression){}
	fn enter_statement_assign_array(&mut self, name: &str, index: &Expression, expr: &Expression){}
	fn enter_statement_if_else(&mut self,
							   cond: &Expression, if_true: &Block, if_false: Option<&Block>){}
	fn enter_statement_while(&mut self, cond: &Expression, body: &Block){}
	fn enter_statement_read(&mut self, var: &str){}
	fn enter_statement_read_array(&mut self, arr: &str, index: &Expression){}
	fn enter_statement_write(&mut self, value: &Expression){}
	fn enter_statement_break(&mut self){}
	fn enter_statement_continue(&mut self){}
	fn enter_statement_composite(&mut self, s1: &Statement, s2:&Statement){}
	fn enter_statement_scope(&mut self, block: &Block){}
	
	fn enter_expression(&mut self, expr: &Expression);
	fn enter_expression_constant(&mut self, value: i32){}
	fn enter_expression_variable(&mut self, name: &str){}
	fn enter_expression_array_access(&mut self, name: &str, index: &Expression){}
	fn enter_expression_binary(&mut self, lhs: &Expression, op: BinaryOperator, rhs: &Expression){}
	fn enter_expression_unary(&mut self, op: UnaryOperator, rhs: &Expression){}
	
//exit methods
	fn exit_block(&mut self, block: &Block);
	fn exit_declaration(&mut self, decl: &Declaration);
	fn exit_declaration_variable(&mut self, t:Type, name: &str){}
	fn exit_declaration_array(&mut self, t:Type, name: &str, len: i32){}
	fn exit_declaration_composite(&mut self, c1: &Declaration, c2: &Declaration){}
	
	fn exit_statement(&mut self, stmt: &Statement);
	fn exit_statement_assign(&mut self, name: &str, expr: &Expression){}
	fn exit_statement_assign_array(&mut self, name: &str, index: &Expression, expr: &Expression){}
	fn exit_statement_if_else(&mut self,
							   cond: &Expression, if_true: &Block, if_false: Option<&Block>){}
	fn exit_statement_while(&mut self, cond: &Expression, body: &Block){}
	fn exit_statement_read(&mut self, var: &str){}
	fn exit_statement_read_array(&mut self, arr: &str, index: &Expression){}
	fn exit_statement_write(&mut self, value: &Expression){}
	fn exit_statement_break(&mut self){}
	fn exit_statement_continue(&mut self){}
	fn exit_statement_composite(&mut self, s1: &Statement, s2:&Statement){}
	fn exit_statement_scope(&mut self, block: &Block){}
	
	fn exit_expression(&mut self, expr: &Expression);
	fn exit_expression_constant(&mut self, value: i32){}
	fn exit_expression_variable(&mut self, name: &str){}
	fn exit_expression_array_access(&mut self, name: &str, index: &Expression){}
	fn exit_expression_binary(&mut self, lhs: &Expression, op: BinaryOperator, rhs: &Expression){}
	fn exit_expression_unary(&mut self, op: UnaryOperator, rhs: &Expression){}
	
//Visit methods
	fn visit_block(&mut self, block: &Block){
		self.enter_block(block);
		if let Some(ref decl) = block.declarations {
			self.visit_declaration(decl);
		}
		self.visit_statement(block.statements.as_ref());
		self.exit_block(block);
	}
	fn visit_declaration(&mut self, decl: &Declaration){
		self.enter_declaration(decl);
		if let &Declaration::Composite(ref d1,ref d2) = decl {
			self.visit_declaration(d1);
			self.visit_declaration(d2);
		}
		self.exit_declaration(decl);
	}
	fn visit_declaration_variable(&mut self, t:Type, name: &str){
		self.enter_declaration_variable(t, name);
		self.exit_declaration_variable(t, name);
	}
	fn visit_declaration_array(&mut self, t:Type, name: &str, len: i32){
		self.enter_declaration_array(t, name, len);
		self.exit_declaration_array(t, name, len);
	}
	fn visit_declaration_composite(&mut self, c1: &Declaration, c2: &Declaration){
		self.enter_declaration_composite(c1, c2);
		self.visit_declaration(c1);
		self.visit_declaration(c2);
		self.exit_declaration_composite(c1, c2);
	}
	
	fn visit_statement(&mut self, stmt: &Statement){
		self.enter_statement(stmt);
		
		match stmt {
			&Assign(name,ref expr) =>self.visit_statement_assign(name, expr),
			&AssignArray(name,ref index,ref expr) =>self.visit_statement_assign_array(name, index, expr),
			&IfElse(ref cond, ref if_true, ref if_false) =>
				if let &Some(ref block) = if_false {
					self.visit_statement_if_else(cond, if_true, Some(block))
				} else {
					self.visit_statement_if_else(cond, if_true, None)
				},
			&While(ref cond, ref body) => self.visit_statement_while(cond, body),
			&Read(var) => self.visit_statement_read(var),
			&ReadArray(arr, ref index) => self.visit_statement_read_array(arr, index),
			&Write(ref expr) => self.visit_statement_write(expr),
			&Break => self.visit_statement_break(),
			&Continue => self.visit_statement_continue(),
			&Composite(ref s1, ref s2) => self.visit_statement_composite(s1, s2),
			&Scope(ref block) => self.visit_statement_scope(block)
		}
		
		self.exit_statement(stmt);
	}
	fn visit_statement_assign(&mut self, name: &str, expr: &Expression){
		self.enter_statement_assign(name, expr);
		self.visit_expression(expr);
		self.exit_statement_assign(name, expr);
	}
	fn visit_statement_assign_array(&mut self, name: &str, index: &Expression, expr: &Expression){
		self.enter_statement_assign_array(name, index, expr);
		self.visit_expression(index);
		self.visit_expression(expr);
		self.exit_statement_assign_array(name, index, expr);
	}
	fn visit_statement_if_else(&mut self,
							   cond: &Expression, if_true: &Block, if_false: Option<&Block>){
		self.enter_statement_if_else(cond, if_true, if_false);
		self.visit_expression(cond);
		self.visit_block(if_true);
		if let Some(block) = if_false {
			self.visit_block(block);
		}
		self.exit_statement_if_else(cond, if_true, if_false);
	}
	fn visit_statement_while(&mut self, cond: &Expression, body: &Block){
		self.enter_statement_while(cond, body);
		self.visit_expression(cond);
		self.visit_block(body);
		self.exit_statement_while(cond, body);
	}
	fn visit_statement_read(&mut self, var: &str){
		self.enter_statement_read(var);
		self.exit_statement_read(var);
	}
	fn visit_statement_read_array(&mut self, arr: &str, index: &Expression){
		self.enter_statement_read_array(arr, index);
		self.visit_expression(index);
		self.exit_statement_read_array(arr, index);
	}
	fn visit_statement_write(&mut self, value: &Expression){
		self.enter_statement_write(value);
		self.visit_expression(value);
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
	fn visit_statement_composite(&mut self, s1: &Statement, s2:&Statement){
		self.enter_statement_composite(s1, s2);
		self.visit_statement(s1);
		self.visit_statement(s2);
		self.exit_statement_composite(s1, s2);
	}
	fn visit_statement_scope(&mut self, block: &Block){
		self.enter_statement_scope(block);
		self.visit_block(block);
		self.exit_statement_scope(block);
	}
	
	fn visit_expression(&mut self, expr: &Expression){
		self.enter_expression(expr);
		
		match expr {
			&Constant(n) => self.visit_expression_constant(n),
			&Variable(x) => self.visit_expression_variable(x),
			&ArrayAccess(x, ref index) => self.visit_expression_array_access(x, index),
			&Binary(ref lhs, op, ref rhs) => self.visit_expression_binary(lhs, op, rhs),
			&Unary(op, ref rhs) => self.visit_expression_unary(op, rhs),
		}
		
		self.exit_expression(expr);
	}
	fn visit_expression_constant(&mut self, value: i32){
		self.enter_expression_constant(value);
		self.exit_expression_constant(value);
	}
	fn visit_expression_variable(&mut self, name: &str){
		self.enter_expression_variable(name);
		self.exit_expression_variable(name);
	}
	fn visit_expression_array_access(&mut self, name: &str, index: &Expression){
		self.enter_expression_array_access(name, index);
		self.visit_expression(index);
		self.exit_expression_array_access(name, index);
	}
	fn visit_expression_binary(&mut self, lhs: &Expression, op: BinaryOperator, rhs: &Expression){
		self.enter_expression_binary(lhs, op, rhs);
		self.visit_expression(lhs);
		self.visit_expression(rhs);
		self.exit_expression_binary(lhs, op, rhs);
	}
	fn visit_expression_unary(&mut self, op: UnaryOperator, rhs: &Expression){
		self.enter_expression_unary(op, rhs);
		self.visit_expression(rhs);
		self.exit_expression_unary(op, rhs);
	}
}

