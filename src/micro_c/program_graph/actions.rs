
use std::rc::Rc;

use crate::micro_c::{
	Expression,
	Type, Lvalue
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action<'a>{
	Assign(Rc<Lvalue<'a>>, Rc<Expression<'a>>),
	Read(Rc<Lvalue<'a>>),
	Write(Rc<Expression<'a>>),
	DeclareVariable(Type, &'a str),
	DeclareArray(Type, &'a str, i32),
	Condition(Rc<Expression<'a>>),
	Drop(&'a str),
	Skip
}
