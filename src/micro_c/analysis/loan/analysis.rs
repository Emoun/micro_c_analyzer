
use crate::micro_c::{
	Action, Lvalue, Expression, UnaryOperator,
	analysis::{
		lifetime::{
			LifetimeTFSpace, LifetimePowerSet
		},
		loan::lattice::{
			LoanPowerSet, Loan,
		},
	}
};
use progysis::{
	core::{
		TFSpace, PowerSet, Analysis, SubLattice, Bottom
	},
};
use graphene::core::{Graph, Directed};
use std::{
	hash::Hash,
	marker::PhantomData,
};

pub fn live<'a>(lfs: &LifetimeTFSpace<'a>, lifetime: &str) -> bool
{
	lfs.keys().into_iter().any(|v| lfs[v] >= LifetimePowerSet::singleton(lifetime))
}

pub struct LoanAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a,G,L> Analysis<G,L> for LoanAnalysis<'a>
	where
		G: Graph<Directedness=Directed, EdgeWeight = Action<'a>>,
		G::Vertex: Hash,
		L: Bottom + SubLattice<LoanPowerSet<'a>> + SubLattice<LifetimeTFSpace<'a>>,
{
	type Lattice = LoanPowerSet<'a>;
	
	const FORWARD: bool = true;
	
	fn transfer(dependency: &L, target: &L, ac: &Action<'a>) -> Self::Lattice
	{
		let dep = dependency.sub_lattice_ref();
		let tar = target.sub_lattice_ref();
		
		if let Action::Assign(ref l, ref e) = *ac {
			if let (
				Lvalue::Variable(false, _),
				Expression::Unary(op, l_1)
			) = (l.as_ref(), e.as_ref()){
				match op {
					UnaryOperator::BorrowConst(a) => {
						let with_loan = LoanPowerSet::singleton
							(Loan { lifetime: a, shared: true, lvalue: l_1.clone() })
							+ dep;
						return only_live(&with_loan, tar);
					},
					UnaryOperator::BorrowMut(a) =>{
						let with_loan = LoanPowerSet::singleton
							(Loan { lifetime: a, shared: false, lvalue: l_1.clone() })
							+ dep
						;
						return only_live(&with_loan, tar);
					},
					_ => (),
				}
			}
		}
		only_live(dep, tar)
	}
}

// Helper functions

fn only_live<'a>(loans: &LoanPowerSet<'a>, lfs: &LifetimeTFSpace<'a>) -> LoanPowerSet<'a>
{
	let mut result = LoanPowerSet::bottom();
	
	for loan in loans.all() {
		if live(lfs, loan.lifetime){
			result += LoanPowerSet::singleton(loan);
		}
	}
	result
}






