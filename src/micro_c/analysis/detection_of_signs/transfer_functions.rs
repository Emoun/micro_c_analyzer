
use progysis::{
	core::{
		PowerSet, Bottom
	},
};
use micro_c::{
	Expression,
	analysis::detection_of_signs::{
		Sign, SignsPowerSet, SignsTFSpace, evaluate
	}
};

pub fn assign<'a>(state: &SignsTFSpace<'a>, id: &'a str, value: &Expression<'a>) -> SignsTFSpace<'a>
{
	let mut new_state = state.clone();
	new_state[id] = evaluate(state, &value);
	new_state
}

pub fn assign_array<'a>(state: &SignsTFSpace<'a>,
					id: &'a str, value: &Expression<'a>)
	-> SignsTFSpace<'a>
{
	let mut new_state = assign(state, id, value);
	new_state[id] += state[id].clone();
	new_state
}


pub fn set_to_top<'a>(state: &SignsTFSpace<'a>, id: &'a str) -> SignsTFSpace<'a>
{
	let mut new_state = state.clone();
	new_state[id] = SignsPowerSet::from_iter(vec![Sign::Plus, Sign::Minus, Sign::Zero]);
	new_state
}

pub fn set_to_bot<'a>(state: &SignsTFSpace<'a>, id: &'a str) -> SignsTFSpace<'a>
{
	let mut new_state = state.clone();
	new_state[id] = SignsPowerSet::bottom();
	new_state
}

///
/// Work for Break, Continue, Write
///
pub fn skip<'a>(state: &SignsTFSpace<'a>) -> SignsTFSpace<'a>
{
	state.clone()
}
/*
pub fn transfer_function<'a>(state: &Element<SignsTFSpace<'a>>, action: &Action<'a>) -> Element<SignsTFSpace<'a>>
{
	match *action {
		Assign(ref lvalue, ref expr) =>
			match **lvalue {
				Lvalue::Variable(false, id) => assign(state, id, expr),
				Lvalue::ArrayAccess(false, id,_) => assign_array(state, id, expr),
				_ => skip(state)
			},
		Read(ref lvalue) => match **lvalue {
			Lvalue::Variable(false, id) => set_to_top(state, id),
			Lvalue::ArrayAccess(false, id,_) => set_to_top(state, id),
			_ => skip(state)
		}
		DeclareVariable(_, id)
		| DeclareArray(_, id, _) => set_to_top(state, id),
		Skip
		| Write(_)
		| Condition(_) => skip(state),
		Drop(id) => set_to_bot(state, id)
	}
}
*/



