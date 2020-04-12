
use progysis::common::lattices::{HashPowerSet, HashTFSpace};


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Sign{
	Plus, Minus, Zero
}

pub type SignsPowerSet = HashPowerSet<Sign>;

pub type SignsTFSpace<'a> = HashTFSpace<&'a str, SignsPowerSet>;
