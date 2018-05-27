
use progysis::common::lattices::HashPowerSet;

pub type LiveVariables<'a> = HashPowerSet<&'a str>;