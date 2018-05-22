
use std::rc::Rc;

#[derive(Clone,Debug, Eq, PartialEq)]
pub struct Block<'a> {
	pub(crate) declarations: Option<Rc<Declaration<'a>>>,
	pub(crate) statements: Rc<Statement<'a>>,
}

impl<'a> Block<'a> {
	pub fn new(declarations: Option<Rc<Declaration<'a>>>, statements: Rc<Statement<'a>>) -> Self
	{
		Block{declarations, statements}
	}
}

#[derive(Clone,Debug, Eq, PartialEq)]
pub enum Declaration<'a> {
	Variable(Type, &'a str),
	Array(Type, &'a str, i32),
	Composite(Rc<Declaration<'a>>, Rc<Declaration<'a>>)
}

#[derive(Clone,Debug, Eq, PartialEq)]
pub enum Statement<'a> {
	Assign(Rc<Lvalue<'a>>, Rc<Expression<'a>>),
	Scope(Rc<Block<'a>>),
	IfElse(Rc<Expression<'a>>, Rc<Block<'a>>, Option<Rc<Block<'a>>>),
	While(Rc<Expression<'a>>, Rc<Block<'a>>),
	Read(Rc<Lvalue<'a>>),
	Write(Rc<Expression<'a>>),
	Break,
	Continue,
	Composite(Rc<Statement<'a>>, Rc<Statement<'a>>)
}

#[derive(Clone,Debug, Eq, PartialEq)]
pub enum Expression<'a> {
	Constant(i32),
	Variable(&'a str),
	ArrayAccess(&'a str, Rc<Expression<'a>>),
	Binary(Rc<Expression<'a>>, BinaryOperator, Rc<Expression<'a>>),
	Unary(UnaryOperator<'a>, Rc<Expression<'a>>)
}

#[derive(Clone,Debug, Eq, PartialEq)]
pub enum Lvalue<'a> {
	Variable(bool,&'a str),
	ArrayAccess(bool,&'a str, Rc<Expression<'a>>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Type {
	pub is_pointer: bool,
	pub is_mutable: bool,
	pub basic_type: BasicType
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BasicType{
	Int,
	Void
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOperator {
	Plus,
	Minus,
	Multiply,
	Division,
	LessThan,
	GreaterThan,
	LessOrEqual,
	GreaterOrEqual,
	Equal,
	NotEqual,
	And,
	Or
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator<'a> {
	Negative,
	Not,
	Deref,
	BorrowMut(&'a str),
	BorrowConst(&'a str),
}

