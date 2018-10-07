
use progysis::core::{
	Analysis, SubLattice, Bottom
};
use graphene::core::{
	Graph,
	trait_aliases::IntoFromIter,
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

impl<'a,G,L> Analysis<'a,G,L> for DetectionOfSignsAnalysis<'a>
	where
		G: Graph<'a,EdgeWeight = Action<'a>>,
		G::Vertex: Hash,
		G::EdgeIter: IntoFromIter<(G::Vertex,G::Vertex,&'a G::EdgeWeight)>,
		G::EdgeMutIter: IntoFromIter<(G::Vertex,G::Vertex,&'a mut G::EdgeWeight)>,
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