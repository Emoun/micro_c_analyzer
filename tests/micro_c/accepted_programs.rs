
use analyzer::micro_c::ProgramParser;

#[test]
fn simple_assignment(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Constant(4)) }",
			   format!("{:?}",ProgramParser::new().parse("{x=4;}").unwrap()));
	
}

#[test]
fn simple_array_assignment(){
	assert_eq!("Block { declarations: None, statements: AssignArray(\"x\", Constant(2), Constant(4)) }",
			   format!("{:?}",ProgramParser::new().parse("{x[2]=4;}").unwrap()));
	
}

#[test]
fn simple_if(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				If(\
					Constant(1), \
					Block { \
						declarations: None, \
						statements: \
							Assign(\"x\", Constant(4)) \
					}\
				) \
		}",
		format!("{:?}",ProgramParser::new().parse("{if(1){x=4;}}").unwrap()));
}

#[test]
fn simple_if_else(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				IfElse(\
					Constant(1), \
					Block { \
						declarations: None, \
						statements: \
							Assign(\"x\", Constant(2)) \
					}, \
					Block { \
						declarations: None, \
						statements: \
							Assign(\"y\", Constant(3)) \
					}\
				) \
		}",
		format!("{:?}",ProgramParser::new().parse("{if(1){x=2;}else{y=3;}}").unwrap()));
}

#[test]
fn simple_while(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				While(\
					Constant(1), \
					Block { \
						declarations: None, \
						statements: \
							Assign(\"x\", Constant(2)) \
					}\
				) \
		}",
		format!("{:?}",ProgramParser::new().parse("{while(1){x=2;}}").unwrap()));
}

#[test]
fn simple_read(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				Read(\"x\") \
		}",
		format!("{:?}",ProgramParser::new().parse("{read x;}").unwrap()));
}

#[test]
fn simple_read_array(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				ReadArray(\"x\", Constant(1)) \
		}",
		format!("{:?}",ProgramParser::new().parse("{read x[1];}").unwrap()));
}

#[test]
fn simple_write(){
	assert_eq!(
		"Block { \
			declarations: \
				None, \
			statements: \
				Write(Constant(1)) \
		}",
		format!("{:?}",ProgramParser::new().parse("{write 1;}").unwrap()));
}

#[test]
fn simple_break(){
	assert_eq!("Block { declarations: None, statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{break;}").unwrap()));
	
}

#[test]
fn simple_continue(){
	assert_eq!("Block { declarations: None, statements: Continue }",
			   format!("{:?}",ProgramParser::new().parse("{continue;}").unwrap()));
	
}

#[test]
fn simple_composite(){
	assert_eq!("Block { declarations: None, statements: Composite(Break, Continue) }",
			   format!("{:?}",ProgramParser::new().parse("{break;continue;}").unwrap()));
	
}

#[test]
fn simple_declare_variable(){
	assert_eq!("Block { declarations: Some(Variable(Int, \"x\")), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x; break;}").unwrap()));
	
}

#[test]
fn simple_declare_array(){
	assert_eq!("Block { declarations: Some(Array(Int, \"x\", 1)), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x[1]; break;}").unwrap()));
	
}

#[test]
fn compisite_declaration(){
	assert_eq!("Block { declarations: Some(Composite(Variable(Int, \"x\"), Variable(Void, \"y\"))), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x; void y; break;}").unwrap()));
	
}

#[test]
fn block_statement(){
	assert_eq!("Block { declarations: None, statements: Block(Block { declarations: None, statements: Break }) }",
			   format!("{:?}",ProgramParser::new().parse("{{break;}}").unwrap()));
	
}

#[test]
fn variable_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Variable(\"y\")) }",
			   format!("{:?}",ProgramParser::new().parse("{x=y;}").unwrap()));
	
}

#[test]
fn array_access_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", ArrayAccess(\"y\", Constant(1))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=y[1];}").unwrap()));
	
}

#[test]
fn binary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Binary(Constant(1), Plus, Constant(2))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=1+2;}").unwrap()));
	
}

#[test]
fn unary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Unary(Negative, Constant(1))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=-1;}").unwrap()));
}

#[test]
fn unary_in_binary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Binary(Constant(1), Multiply, Unary(Not, Constant(2)))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=1*!2;}").unwrap()));
}

#[test]
fn unary_precedence_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(\"x\", Binary(Unary(Negative, Constant(1)), Division, Constant(2))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=-1/2;}").unwrap()));
}