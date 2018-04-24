
use micro_c::program_graph::Action;
use graphene::common::AdjListGraph;

pub type ProgramGraph<'a> = AdjListGraph<u32,Action<'a>>;