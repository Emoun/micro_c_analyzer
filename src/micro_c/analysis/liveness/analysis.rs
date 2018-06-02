
use progysis::core::{
	Analysis, CompleteLattice, SubLattice
};
use graphene::core::{
	BaseGraph, EdgeWeightedGraph,
	trait_aliases::IntoFromIter
};
use micro_c::{
	Action,
	analysis::liveness::{
		LiveVariables, transfer_function
	}
};
use std::marker::PhantomData;

pub struct LivenessAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a,G,L> Analysis<G,L> for LivenessAnalysis<'a>
	where
		G: EdgeWeightedGraph<EdgeWeight = Action<'a>> + BaseGraph<Vertex = u32>,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32, u32, <G as BaseGraph>::EdgeId)>,
		L: CompleteLattice + SubLattice<LiveVariables<'a>>,
{
	type Lattice = LiveVariables<'a>;
	type Action = Action<'a>;
	
	const FORWARD: bool = false;
	
	fn transfer(state: &L, _: &L, ac: &Action<'a>) -> LiveVariables<'a>
	{
		transfer_function(state.sub_lattice_ref(),ac)
	}
}