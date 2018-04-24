
use progysis::{ core::{Element, PowerSet}, common::MonotoneFunction};
use micro_c::{Expression, Action, Action::*, analysis::detection_of_signs::{Sign, SignsTFSpace, evaluate}};

fn assign<'a>(state: &SignsTFSpace<'a>, id: &'a str, value: &Expression<'a>) -> SignsTFSpace<'a>
{
	let mut new_state = state.clone();
	new_state[id] = evaluate(state, &value);
	new_state
}

fn assign_array<'a>(state: &SignsTFSpace<'a>,
					id: &'a str, value: &Expression<'a>)
	-> SignsTFSpace<'a>
{
	let mut new_state = assign(state, id, value);
	new_state[id] += state[id].clone();
	new_state
}

///
/// Works for Read, ReadArray, DeclareVariable, DeclareArray
///
fn set_to_top<'a>(state: &SignsTFSpace<'a>, id: &'a str) -> SignsTFSpace<'a>
{
	let mut new_state = state.clone();
	new_state[id] = Element::from_iter(vec![Sign::Plus, Sign::Minus, Sign::Zero]);
	new_state
}

///
/// Work for Break, Continue, Write
///
fn skip<'a>(state: &SignsTFSpace<'a>) -> SignsTFSpace<'a>
{
	state.clone()
}

pub fn transfer_function<'a>(state: &SignsTFSpace<'a>, action: &Action<'a>) -> SignsTFSpace<'a>
{
	match *action {
		Assign(id, ref expr) => assign(state, id, expr),
		AssignArray(id, _, ref expr) =>assign_array(state, id, expr),
		Read(id)
		| ReadArray(id,_)
		| DeclareVariable(_, id)
		| DeclareArray(_, id, _) => set_to_top(state, id),
		Skip
		| Write(_)
		| Condition(_) => skip(state),
	}
}




