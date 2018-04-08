
use analyzer::micro_c::ProgramParser;

#[test]
fn empty_block(){
	assert!(ProgramParser::new().parse("{}").is_err());
}
#[test]
fn no_statements(){
	assert!(ProgramParser::new().parse("{int x;}").is_err());
}
