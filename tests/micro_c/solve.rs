
use super::programs::*;
use progysis::{
	core::{Element, CompleteLattice, ConstraintSystem, PowerSet, TFSpace},
	common::{worklist::FifoWorklist}
};
use analyzer::micro_c::analysis::detection_of_signs::{transfer_function, Sign::*};
use std::collections::HashMap;

#[test]
fn test_p2_signs_analysis(){
	
	let program = p2_program_graph();
	let cs = ConstraintSystem::new(program, transfer_function, true);
	let mut initial = HashMap::new();
	initial.insert(0,Element::bottom());
	
	cs.solve::<FifoWorklist>(&mut initial);
	
	let top = Element::from_iter(vec![Plus,Minus,Zero]);
	let plus_zero = Element::from_iter(vec![Plus, Zero]);
	let bot = Element::bottom();
	let minus = Element::singleton(Minus);
	let plus = Element::singleton(Plus);
	
	assert_eq!(false, initial[&0].has_key("x"));	assert_eq!(false, initial[&0].has_key("y"));
	// x and y are not present because the previous state set them to bot, which means they were not merged.
	assert_eq!(false, initial[&1].has_key("x"));	assert_eq!(false, initial[&1].has_key("y"));
	assert_eq!(top, initial[&2]["x"]);				assert_eq!(top, initial[&2]["y"]);
	assert_eq!(plus_zero, initial[&3]["x"]);		assert_eq!(top, initial[&3]["y"]);
	assert_eq!(top, initial[&4]["x"]);				assert_eq!(false, initial[&4].has_key("y"));
	assert_eq!(plus_zero, initial[&5]["x"]);		assert_eq!(bot, initial[&5]["y"]);
	assert_eq!(top, initial[&6]["x"]);				assert_eq!(minus, initial[&6]["y"]);
	assert_eq!(plus_zero, initial[&7]["x"]);		assert_eq!(top, initial[&7]["y"]);
	assert_eq!(plus_zero, initial[&8]["x"]);		assert_eq!(top, initial[&8]["y"]);
	assert_eq!(plus, initial[&9]["x"]);				assert_eq!(top, initial[&9]["y"]);
}