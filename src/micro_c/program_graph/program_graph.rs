
use micro_c::program_graph::Action;

use petgraph::Graph;

pub type ProgramGraph<'a> = Graph<(), Action<'a>>;