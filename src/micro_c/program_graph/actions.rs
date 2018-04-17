
use std::rc::Rc;

use micro_c::{
	Expression,
	Type,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action<'a>{
	Assign(&'a str, Rc<Expression<'a>>),
	AssignArray(&'a str, Rc<Expression<'a>>, Rc<Expression<'a>>),
	Read(&'a str),
	ReadArray(&'a str, Rc<Expression<'a>>),
	Write(Rc<Expression<'a>>),
	DeclareVariable(Type, &'a str),
	DeclareArray(Type, &'a str, i32),
	Condition(Rc<Expression<'a>>),
	Skip
}
