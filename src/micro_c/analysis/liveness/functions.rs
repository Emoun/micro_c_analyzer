
use micro_c::{
	Expression, Lvalue, Action,
	analysis::liveness::LiveVariables
};
use std::collections::HashSet;

///
/// Calculates the set of free variable of an expression.
///
/// Any variable whos value must be known to evaluate the expression is called a free variable.
///
fn free_variables<'a>(e: &Expression<'a>) -> HashSet<&'a str>
{
	use self::Expression::*;
	let mut result = HashSet::new();
	
	match *e {
		Constant(_) => (),
		Variable(id) => {result.insert(id);},
		ArrayAccess(id, ref e) => {
			result.insert(id);
			result = &result | &free_variables(e.as_ref());
		},
		Binary(ref e_1, _, ref e_2) => {
			result = &free_variables(e_1.as_ref()) | &free_variables(e_2.as_ref());
		},
		Unary(_, ref e) => {
			result = free_variables(e.as_ref());
		}
	}
	
	result
}

#[cfg(test)]
mod free_variables_tests {
	use super::*;
	use super::Expression::*;
	use std::rc::Rc;
	use micro_c::{
		BinaryOperator, UnaryOperator
	};
	
	#[test]
	fn constant(){
		let _4 = Rc::new(Constant(4));
		assert!(free_variables(_4.as_ref()).is_empty());
	}
	
	#[test]
	fn variable(){
		let x = Rc::new(Variable("x"));
		let fv = free_variables(x.as_ref());
		assert_eq!(1, fv.len());
		assert!(fv.contains("x"));
	}
	
	#[test]
	fn array_access(){
		let y = Rc::new(Variable("y"));
		let x_y = Rc::new(ArrayAccess("x", y.clone()));
		let fv = free_variables(x_y.as_ref());
		assert_eq!(2, fv.len());
		assert!(fv.contains("x"));
		assert!(fv.contains("y"));
	}
	
	#[test]
	fn binary(){
		let x = Rc::new(Variable("x"));
		let y = Rc::new(Variable("y"));
		let x_plus_y = Rc::new(Binary(x.clone(), BinaryOperator::Plus, y.clone()));
		let fv = free_variables(x_plus_y.as_ref());
		assert_eq!(2, fv.len());
		assert!(fv.contains("x"));
		assert!(fv.contains("y"));
	}
	
	#[test]
	fn unary(){
		let x = Rc::new(Variable("x"));
		let minus_x = Rc::new(Unary(UnaryOperator::Negative, x.clone()));
		let fv = free_variables(minus_x.as_ref());
		assert_eq!(1, fv.len());
		assert!(fv.contains("x"));
	}
	
	#[test]
	fn compound(){
		let _4 = Rc::new(Constant(4));
		let x = Rc::new(Variable("x"));
		let y = Rc::new(Variable("y"));
		let x_plus_4 = Rc::new(Binary(x.clone(), BinaryOperator::Plus, _4.clone()));
		let x_plus_4_minus_y = Rc::new(
			Binary(x_plus_4.clone(), BinaryOperator::Minus, y.clone()));
		
		let fv = free_variables(x_plus_4_minus_y.as_ref());
		assert_eq!(2, fv.len());
		assert!(fv.contains("x"));
		assert!(fv.contains("y"));
	}
}

///
/// Calculates the set of free variable of an lvalue.
///
/// Any variable whos value must be known to evaluate the lvalue is called a free variable.
///
pub fn free_variables_lvalue<'a>(l: &Lvalue<'a>) -> HashSet<&'a str>
{
	use self::Lvalue::*;
	let mut result = HashSet::new();
	
	match *l {
		Variable(false, _) => (),
		Variable(true, id) => {result.insert(id);},
		ArrayAccess(false, _, ref e) => result = free_variables(e),
		ArrayAccess(true, id, ref e) => {
			result.insert(id);
			result = &result | &free_variables(e);
		}
	}
	
	result
}

#[cfg(test)]
mod free_variables_lvalue_tests{
	
	use super::*;
	use super::Lvalue::*;
	use std::rc::Rc;
	
	#[test]
	fn variable(){
		let x = Rc::new(Variable(false, "x"));
		
		assert!(free_variables_lvalue(x.as_ref()).is_empty());
	}
	
	#[test]
	fn deref_variable(){
		let deref_x = Rc::new(Variable(true, "x"));
		let fv = free_variables_lvalue(deref_x.as_ref());
		
		assert_eq!(1, fv.len());
		assert!(fv.contains("x"));
	}
	
	#[test]
	fn array_access(){
		let y = Rc::new(Expression::Variable("y"));
		let x_y = Rc::new(ArrayAccess(false, "x", y));
		let fv = free_variables_lvalue(x_y.as_ref());
		
		assert_eq!(1, fv.len());
		assert!(fv.contains("y"));
	}
	
	#[test]
	fn deref_array_access(){
		let y = Rc::new(Expression::Variable("y"));
		let x_y = Rc::new(ArrayAccess(true, "x", y));
		let fv = free_variables_lvalue(x_y.as_ref());
		
		assert_eq!(2, fv.len());
		assert!(fv.contains("x"));
		assert!(fv.contains("y"));
	}
}

fn dead_variables<'a>(l: &Lvalue<'a>) -> HashSet<&'a str>
{
	let mut result = HashSet::new();
	
	match *l {
		Lvalue::Variable(false, id) => {result.insert(id);},
		_ => (),
	}
	
	result
}

fn kill<'a>(a: &Action<'a>) -> HashSet<&'a str>
{
	use self::Action::*;
	match *a {
		Assign(ref l, _)
		| Read(ref l) => dead_variables(l.as_ref()),
		Drop(id) => {let mut r = HashSet::new(); r.insert(id); r},
		_ => HashSet::new()
	}
}

fn gen<'a>(a: &Action<'a>) -> HashSet<&'a str>
{
	use self::Action::*;
	match *a {
		Assign(ref l, ref e) => &free_variables(e.as_ref()) | &free_variables_lvalue(l.as_ref()),
		Read(ref l) => free_variables_lvalue(l.as_ref()),
		Write(ref e)
		| Condition(ref e) => free_variables(e.as_ref()),
		_ => HashSet::new(),
	}
}

pub fn transfer_function<'a>(state: &LiveVariables<'a>, action: &Action<'a>)
	-> LiveVariables<'a>
{
	let state_set:HashSet<&'a str> = state.clone().into();
	let new_set = &(&state_set - &kill(action)) | &gen(action);
	LiveVariables::from(new_set)
}
