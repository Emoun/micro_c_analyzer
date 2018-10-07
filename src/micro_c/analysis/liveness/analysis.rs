
use progysis::core::{
	Analysis, SubLattice, Bottom
};
use graphene::core::{
	Graph,trait_aliases::IntoFromIter,
};
use micro_c::{
	Action,
	analysis::liveness::{
		LiveVariables, transfer_function
	}
};
use std::{
	marker::PhantomData,
	hash::Hash,
};

pub struct LivenessAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a,G,L> Analysis<'a,G,L> for LivenessAnalysis<'a>
	where
		G: Graph<'a,EdgeWeight = Action<'a>>,
		G::Vertex: Hash,
		G::EdgeIter: IntoFromIter<(G::Vertex,G::Vertex,&'a G::EdgeWeight)>,
		G::EdgeMutIter: IntoFromIter<(G::Vertex,G::Vertex,&'a mut G::EdgeWeight)>,
		L: Bottom + SubLattice<LiveVariables<'a>>,
{
	type Lattice = LiveVariables<'a>;
	
	const FORWARD: bool = false;
	
	fn transfer(state: &L, _: &L, ac: &Action<'a>) -> Self::Lattice
	{
		transfer_function(state.sub_lattice_ref(),ac)
	}
}