
use progysis::core::{
	Analysis, Element, AnalysisDirection
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

impl<'a> Analysis for LivenessAnalysis<'a>
{
	type Lattice = LiveVariables<'a>;
	type Action = Action<'a>;
	
	fn transfer(state: &Element<LiveVariables<'a>>, ac: &Action<'a>) -> Element<LiveVariables<'a>>
	{
		transfer_function(state,ac)
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Backward
	}
}