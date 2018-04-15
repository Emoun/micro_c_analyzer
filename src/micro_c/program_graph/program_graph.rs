
use std;
use graphene::core::*;
use graphene::common::*;
use micro_c::program_graph::Action;

custom_graph!{
	pub struct ProgramGraph<'a>
	as AdjListGraph<u32, Action<'a>>
}
