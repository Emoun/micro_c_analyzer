
use micro_c::{
	Expression,
	Type,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action<'a>{
	Assign(&'a str, &'a Expression<'a>),
	AssignArray(&'a str, &'a Expression<'a>, &'a Expression<'a>),
	Read(&'a str),
	ReadArray(&'a str, &'a Expression<'a>),
	Write(&'a Expression<'a>),
	DeclareVariable(Type, &'a str),
	DeclareArray(Type, &'a str, i32),
	Condition(&'a Expression<'a>),
	Skip
}
