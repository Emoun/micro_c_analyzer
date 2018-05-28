
use progysis::core::{
	Analysis, Element, AnalysisDirection
};
use micro_c::{
	Action,
	analysis::detection_of_signs::{
		SignsTFSpace, transfer_function
	}
};
use std::marker::PhantomData;

pub struct DetectionOfSignsAnalysis<'a>{
	pha: PhantomData<&'a u8>
}

impl<'a> Analysis for DetectionOfSignsAnalysis<'a>
{
	type Lattice = SignsTFSpace<'a>;
	type Action = Action<'a>;
	
	fn transfer(state: &Element<SignsTFSpace<'a>>, ac: &Action<'a>) -> Element<SignsTFSpace<'a>>
	{
		transfer_function(state,ac)
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Forward
	}
}