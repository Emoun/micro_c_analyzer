
use progysis::core::{
	Analysis, SubLattice, Bottom
};
use graphene::core::{
	EdgeWeightedGraph,
};
use micro_c::{
	Action,Lvalue,
	analysis::detection_of_signs::*
};
use std::{
	marker::PhantomData,
	hash::Hash,
};

pub struct DetectionOfSignsAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a,G,L> Analysis<G,L> for DetectionOfSignsAnalysis<'a>
	where
		G: EdgeWeightedGraph<EdgeWeight = Action<'a>>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<SignsTFSpace<'a>>,
{
	type Lattice = SignsTFSpace<'a>;
	
	const FORWARD: bool = true;
	
	fn transfer(dependency: &L, _: &L, action: &Action<'a>) -> SignsTFSpace<'a>
	{
		let state = dependency.sub_lattice_ref();
		use self::Action::*;
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
	
}