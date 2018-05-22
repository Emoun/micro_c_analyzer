
use progysis::{core::{Element, PowerSet, CompleteLattice}};
use micro_c::{
	Expression, Action, Action::*, Lvalue,
	analysis::detection_of_signs::{
		Sign, SignsTFSpace, evaluate
	}
};

fn assign<'a>(state: &Element<SignsTFSpace<'a>>, id: &'a str, value: &Expression<'a>) -> Element<SignsTFSpace<'a>>
{
	let mut new_state = state.clone();
	new_state[id] = evaluate(state, &value);
	new_state
}

fn assign_array<'a>(state: &Element<SignsTFSpace<'a>>,
					id: &'a str, value: &Expression<'a>)
	-> Element<SignsTFSpace<'a>>
{
	let mut new_state = assign(state, id, value);
	new_state[id] += state[id].clone();
	new_state
}


fn set_to_top<'a>(state: &Element<SignsTFSpace<'a>>, id: &'a str) -> Element<SignsTFSpace<'a>>
{
	let mut new_state = state.clone();
	new_state[id] = Element::from_iter(vec![Sign::Plus, Sign::Minus, Sign::Zero]);
	new_state
}

fn set_to_bot<'a>(state: &Element<SignsTFSpace<'a>>, id: &'a str) -> Element<SignsTFSpace<'a>>
{
	let mut new_state = state.clone();
	new_state[id] = Element::bottom();
	new_state
}

///
/// Work for Break, Continue, Write
///
fn skip<'a>(state: &Element<SignsTFSpace<'a>>) -> Element<SignsTFSpace<'a>>
{
	state.clone()
}

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




