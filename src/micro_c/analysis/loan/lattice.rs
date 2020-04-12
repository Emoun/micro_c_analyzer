
use crate::micro_c::{
	Expression
};
use progysis::{
	common::lattices::HashPowerSet,
};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Loan<'a>{
	pub lifetime: &'a str,
	pub shared: bool,
	pub lvalue: Rc<Expression<'a>>,
}

pub type LoanPowerSet<'a> = HashPowerSet<Loan<'a>>;




