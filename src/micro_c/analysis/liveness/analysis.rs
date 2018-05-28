
use progysis::core::{
	Analysis, Element, AnalysisDirection
};
use micro_c::{
	Action,
	analysis::liveness::{
		LiveVariables, transfer_function
	}
};

pub struct LivenessAnalysis{}

impl<'a> Analysis<LiveVariables<'a>, Action<'a>> for LivenessAnalysis
{
	fn transfer(state: &Element<LiveVariables<'a>>, ac: &Action<'a>) -> Element<LiveVariables<'a>>
	{
		transfer_function(state,ac)
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Backward
	}
}