extern crate analyzer;

use analyzer::micro_c;

fn main() {
	println!("{:?}", micro_c::ProgramParser::new().parse("{}").unwrap());
}
