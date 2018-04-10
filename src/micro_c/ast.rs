
#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a> {
	pub(crate) declarations: Option<Box<Declaration<'a>>>,
	pub(crate) statements: Box<Statement<'a>>,
}

impl<'a> Block<'a> {
	pub fn new(declarations: Option<Box<Declaration<'a>>>, statements: Box<Statement<'a>>) -> Self
	{
		Block{declarations, statements}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Declaration<'a> {
	Variable(Type, &'a str),
	Array(Type, &'a str, i32),
	Composite(Box<Declaration<'a>>, Box<Declaration<'a>>)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement<'a> {
	Assign(&'a str, Box<Expression<'a>>),
	AssignArray(&'a str, Box<Expression<'a>>, Box<Expression<'a>>),
	Scope(Box<Block<'a>>),
	IfElse(Box<Expression<'a>>, Box<Block<'a>>, Option<Box<Block<'a>>>),
	While(Box<Expression<'a>>, Box<Block<'a>>),
	Read(&'a str),
	ReadArray(&'a str, Box<Expression<'a>>),
	Write(Box<Expression<'a>>),
	Break,
	Continue,
	Composite(Box<Statement<'a>>, Box<Statement<'a>>)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression<'a> {
	Constant(i32),
	Variable(&'a str),
	ArrayAccess(&'a str, Box<Expression<'a>>),
	Binary(Box<Expression<'a>>, BinaryOperator, Box<Expression<'a>>),
	Unary(UnaryOperator, Box<Expression<'a>>)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
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
pub enum UnaryOperator {
	Negative,
	Not
}

