
use progysis::{
	core::{
		TFSpace, PowerSet, CompleteLattice, Analysis, SubLattice
	},
	common::lattices::{
		HashPowerSet, HashTFSpace,
	}
};
use graphene::core::{
	BaseGraph, EdgeWeightedGraph,
	trait_aliases::IntoFromIter
};
use micro_c::{
	Expression, UnaryOperator, Action, Lvalue,
	analysis::liveness::LiveVariables
};
use std::marker::PhantomData;

pub type LifetimePowerSet<'a> = HashPowerSet<&'a str>;

pub type LifetimeTFSpace<'a> = HashTFSpace<'a, &'a str, LifetimePowerSet<'a>>;

pub fn lifetimes<'a>(state: &LifetimeTFSpace<'a>, e: &Expression<'a>) -> LifetimePowerSet<'a>
{
	use self::Expression::*;
	match e {
		Variable(p) => if state.has_key(p) {return state[p].clone();},
		Unary(UnaryOperator::Deref, e_1) => {
			if let &Variable(p) = e_1.as_ref(){
				if state.has_key(p){
					return state[p].clone();
				}
			}
		},
		Unary(UnaryOperator::BorrowMut(a), e_1)
		| Unary(UnaryOperator::BorrowConst(a), e_1) =>{
			return LifetimePowerSet::singleton(a) + lifetimes(state, e_1.as_ref());
		},
		_ => (),
	}
	LifetimePowerSet::bottom()
}

pub struct LifetimeAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a,G,L> Analysis<G,L> for LifetimeAnalysis<'a>
	where
		G: EdgeWeightedGraph<EdgeWeight = Action<'a>> + BaseGraph<Vertex = u32>,
		<G as BaseGraph>::VertexIter: IntoFromIter<u32>,
		<G as BaseGraph>::EdgeIter: IntoFromIter<(u32, u32, <G as BaseGraph>::EdgeId)>,
		L: CompleteLattice + SubLattice<LifetimeTFSpace<'a>> + SubLattice<LiveVariables<'a>>,
{
	type Lattice = LifetimeTFSpace<'a>;
	type Action = Action<'a>;
	
	const FORWARD: bool = true;
	
	fn transfer(dependency: &L, target: &L, ac: &Action<'a>) -> Self::Lattice
	{
		let dep = dependency.sub_lattice_ref();
		let tar = target.sub_lattice_ref();
		//println!("transfer({:?},{:?},{:?})",dep,tar,ac);
		
		if let &Action::Assign(ref l,ref e) = ac{
			if let Lvalue::Variable(false,x) = l.as_ref(){
				let lifetimes = lifetimes(dep,e.as_ref());
				if !lifetimes.is_bottom() {
					let mut with_x = dep.clone();
					with_x[x] = lifetimes;
					return shared(&with_x, tar);
				}
			}
		}
		shared(dep, tar)
	}
}

// Helper functions

fn shared<'a>(f: &LifetimeTFSpace<'a>, v: &LiveVariables<'a>) -> LifetimeTFSpace<'a>
{
	let mut result = LifetimeTFSpace::bottom();
	for y in f.keys().filter(|k| {
		//println!("{:?} <= {:?}",k,v);
		LifetimePowerSet::singleton(k) <= *v
	}) {
		result[y] = f[y].clone()
	}
	//println!("{:?}", result);
	result
}

