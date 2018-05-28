
use progysis::core::{
	Analysis, Element, AnalysisDirection
};
use micro_c::{
	Action,
	analysis::detection_of_signs::{
		SignsTFSpace, transfer_function
	}
};

pub struct DetectionOfSignsAnalysis{}

impl<'a> Analysis<SignsTFSpace<'a>, Action<'a>> for DetectionOfSignsAnalysis
{
	fn transfer(state: &Element<SignsTFSpace<'a>>, ac: &Action<'a>) -> Element<SignsTFSpace<'a>>
	{
		transfer_function(state,ac)
	}
	
	fn direction() -> AnalysisDirection
	{
		AnalysisDirection::Forward
	}
}