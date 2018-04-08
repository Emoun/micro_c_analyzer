
use micro_c::{Block, Declaration, Statement, Expression, Type, BinaryOperator, UnaryOperator};

pub trait AstVisitor{
	
	fn enter_block(&mut self, cxt: &Block);
	fn enter_declaration(&mut self, cxt: &Declaration);
	fn enter_statement(&mut self, cxt: &Statement);
	fn enter_expression(&mut self, cxt: &Expression);
	
	fn exit_block(&mut self, cxt: &Block);
	fn exit_declaration(&mut self, cxt: &Declaration);
	fn exit_statement(&mut self, cxt: &Statement);
	fn exit_expression(&mut self, cxt: &Expression);
	
	fn visit(&mut self, ast: &Block)
	{
		self.enter_block(ast);
		if let Some(ref decl) = ast.declarations {
			self.visit_declaration(decl);
		}
		self.visit_statement(ast.statements.as_ref());
		self.exit_block(ast);
	}
	
	fn visit_declaration(&mut self, decl: &Declaration)
	{
		self.enter_declaration(decl);
		if let &Declaration::Composite(ref d1,ref d2) = decl {
			self.visit_declaration(d1);
			self.visit_declaration(d2);
		}
		self.exit_declaration(decl);
	}
	
	fn visit_statement(&mut self, statement: &Statement)
	{
		self.enter_statement(statement);
		unimplemented!();
		self.exit_statement(statement);
	}
	fn visit_expression(&mut self, expr: &Expression)
	{
		self.enter_expression(expr);
		unimplemented!();
		self.exit_expression(expr);
	}
}

