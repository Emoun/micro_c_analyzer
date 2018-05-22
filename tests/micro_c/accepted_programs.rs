
use analyzer::micro_c::ProgramParser;

#[test]
fn simple_assignment(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Constant(4)) }",
			   format!("{:?}",ProgramParser::new().parse("{x=4;}").unwrap()));
	
}

#[test]
fn simple_array_assignment(){
	assert_eq!("Block { declarations: None, statements: Assign(ArrayAccess(false, \"x\", Constant(2)), Constant(4)) }",
			   format!("{:?}",ProgramParser::new().parse("{x[2]=4;}").unwrap()));
	
}

#[test]
fn simple_if(){
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
							Assign(Variable(false, \"x\"), Constant(4)) \
					}, \
					None\
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
							Assign(Variable(false, \"x\"), Constant(2)) \
					}, \
					Some(\
						Block { \
							declarations: None, \
							statements: \
								Assign(Variable(false, \"y\"), Constant(3)) \
						}\
					)\
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
							Assign(Variable(false, \"x\"), Constant(2)) \
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
				Read(Variable(false, \"x\")) \
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
				Read(ArrayAccess(false, \"x\", Constant(1))) \
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
fn declare_mut_variable(){
	assert_eq!("Block { declarations: Some(Variable(Type { is_pointer: false, is_mutable: true, basic_type: Int }, \"x\")), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x; break;}").unwrap()));
	
}

#[test]
fn declare_const_variable(){
	assert_eq!("Block { declarations: Some(Variable(Type { is_pointer: false, is_mutable: false, basic_type: Int }, \"x\")), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{const int x; break;}").unwrap()));
	
}

#[test]
fn declare_variable_pointer(){
	assert_eq!("Block { declarations: Some(Variable(Type { is_pointer: true, is_mutable: true, basic_type: Int }, \"x\")), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int *x; break;}").unwrap()));
	
}

#[test]
fn declare_const_variable_pointer(){
	assert_eq!("Block { declarations: Some(Variable(Type { is_pointer: true, is_mutable: false, basic_type: Int }, \"x\")), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{const int *x; break;}").unwrap()));
	
}

#[test]
fn declare_mut_array(){
	assert_eq!("Block { declarations: Some(Array(Type { is_pointer: false, is_mutable: true, basic_type: Int }, \"x\", 1)), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x[1]; break;}").unwrap()));
	
}

#[test]
fn declare_const_array(){
	assert_eq!("Block { declarations: Some(Array(Type { is_pointer: false, is_mutable: false, basic_type: Int }, \"x\", 1)), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{const int x[1]; break;}").unwrap()));
	
}

#[test]
fn declare_array_pointer(){
	assert_eq!("Block { declarations: Some(Array(Type { is_pointer: true, is_mutable: true, basic_type: Int }, \"x\", 1)), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int *x[1]; break;}").unwrap()));
	
}

#[test]
fn declare_const_array_pointer(){
	assert_eq!("Block { declarations: Some(Array(Type { is_pointer: true, is_mutable: false, basic_type: Int }, \"x\", 1)), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{const int *x[1]; break;}").unwrap()));
	
}

#[test]
fn compisite_declaration(){
	assert_eq!("Block { declarations: Some(Composite(Variable(Type { is_pointer: false, is_mutable: true, basic_type: Int }, \"x\"), Variable(Type { is_pointer: false, is_mutable: true, basic_type: Void }, \"y\"))), statements: Break }",
			   format!("{:?}",ProgramParser::new().parse("{int x; void y; break;}").unwrap()));
	
}

#[test]
fn block_statement(){
	assert_eq!("Block { declarations: None, statements: Scope(Block { declarations: None, statements: Break }) }",
			   format!("{:?}",ProgramParser::new().parse("{{break;}}").unwrap()));
	
}

#[test]
fn variable_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Variable(\"y\")) }",
			   format!("{:?}",ProgramParser::new().parse("{x=y;}").unwrap()));
	
}

#[test]
fn array_access_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), ArrayAccess(\"y\", Constant(1))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=y[1];}").unwrap()));
	
}

#[test]
fn binary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Binary(Constant(1), Plus, Constant(2))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=1+2;}").unwrap()));
	
}

#[test]
fn unary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Unary(Negative, Constant(1))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=-1;}").unwrap()));
}

#[test]
fn unary_in_binary_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Binary(Constant(1), Multiply, Unary(Not, Constant(2)))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=1*!2;}").unwrap()));
}

#[test]
fn unary_precedence_expression(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Binary(Unary(Negative, Constant(1)), Division, Constant(2))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=-1/2;}").unwrap()));
}

#[test]
fn variable_deref(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Unary(Deref, Variable(\"y\"))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=*y;}").unwrap()));
}

#[test]
fn variable_borrow_mut(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Unary(BorrowMut(\"\\\'y\"), Variable(\"y\"))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=&'y y;}").unwrap()));
}

#[test]
fn variable_borrow_const(){
	assert_eq!("Block { declarations: None, statements: Assign(Variable(false, \"x\"), Unary(BorrowConst(\"\\\'y\"), Variable(\"y\"))) }",
			   format!("{:?}",ProgramParser::new().parse("{x=&'y const y;}").unwrap()));
}