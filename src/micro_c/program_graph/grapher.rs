#[allow(dead_code)]
use micro_c::{
	AstVisitor,
	Block, Declaration,
	Statement,Expression,
	Type, UnaryOperator,
	ProgramGraph, Action, Lvalue
};
use std::rc::Rc;
use graphene::{
	core::{
		BaseGraph,EdgeWeightedGraph
	},
	common::AdjListGraph
};



pub struct ProgramGrapher<'a>{
	graph: ProgramGraph<'a>,
	next_vertex: u32,
	start_state_stack: Vec<u32>,
	end_state_stack: Vec<u32>,
	break_stack: Vec<u32>,
	continue_stack: Vec<u32>,
}

impl<'a> ProgramGrapher<'a>
{
	pub fn add_state(&mut self) -> u32
	{
		self.graph.add_vertex(self.next_vertex).unwrap();
		self.next_vertex += 1;
		self.next_vertex - 1
	}
	
	pub fn new() -> Self
	{
		let graph= AdjListGraph::empty_graph();
		let start_state_stack = Vec::new();
		let end_state_stack = Vec::new();
		let break_stack = Vec::new();
		let continue_stack = Vec::new();
		
		let mut result = ProgramGrapher{
			graph,
			next_vertex: 0,
			start_state_stack,
			end_state_stack,
			break_stack,
			continue_stack,
		};
		
		//Create initial and final nodes for the first enter_block
		let qs = result.add_state();
		let qt = result.add_state();
		
		result.start_state_stack.push(qs);
		result.end_state_stack.push(qt);
		result
	}

	fn pop_stack(&mut self, call_msg: &'a str) -> (u32, u32)
	{
		let m_start = &format!("{} expected start state on stack, found none", call_msg);
		let m_end = &format!("{} expected end state on stack, found none", call_msg);
		(
			self.start_state_stack.pop().expect(m_start),
			self.end_state_stack.pop().expect(m_end)
		)
	}
	
	fn peek_stack(&mut self, call_msg: &'a str) -> (u32, u32)
	{
		let m_start = &format!("{} expected start state on stack, found none", call_msg);
		let m_end = &format!("{} expected end state on stack, found none", call_msg);
		(
			*self.start_state_stack.last().expect(m_start),
			*self.end_state_stack.last().expect(m_end)
		)
	}

	pub fn get_graph(self) -> ProgramGraph<'a>
	{
		self.graph
	}
}


impl<'a> AstVisitor<'a> for ProgramGrapher<'a>{

//Enter methods
	fn enter_block(&mut self, block: Rc<Block<'a>>){
		let (qs,qt) = self.pop_stack("Block");
	
		if let Some(_) = block.declarations{
			// Create a new nodes if there are declarations
			let q_1 = self.add_state();
			let q_2 = self.add_state();
			// Push for the statement
			self.start_state_stack.push(q_1);
			self.end_state_stack.push(q_2);
			// Push for the declaration
			self.start_state_stack.push(qs);
			self.end_state_stack.push(q_1);
			// Use break/continue stack for drop states
			self.continue_stack.push(q_2); // start state
			self.break_stack.push(qt); // end state
			
		}else{
			self.start_state_stack.push(qs);
			self.end_state_stack.push(qt);
		}
	}
	
	fn enter_declaration_composite(&mut self, _: Rc<Declaration<'a>>, _: Rc<Declaration<'a>>)
	{
		let q_1 = self.add_state();
		let q_2 = self.add_state();
		let (qs,qt) = self.pop_stack("Enter Composite declaration");
		let qd = self.continue_stack.pop()
			.expect("Enter Composite declaration expected drop start state on stack, found none");
		let qp = self.break_stack.pop()
			.expect("Enter Composite declaration expected drop end state on stack, found none");
		
		// push states for the second declaration
		self.start_state_stack.push(q_1);
		self.end_state_stack.push(qt);
		self.continue_stack.push(qd);
		self.break_stack.push(q_2);
		// Push states for the first declaration
		self.start_state_stack.push(qs);
		self.end_state_stack.push(q_1);
		self.continue_stack.push(q_2);
		self.break_stack.push(qp);
	}
	
	fn enter_statement_composite(&mut self, _: Rc<Statement<'a>>, _:Rc<Statement<'a>>)
	{
		// Create a new state between them
		let q = self.add_state();
		let (qs,qt) = self.pop_stack("Enter Composite statement");
		
		// Push states for the second statement
		self.start_state_stack.push(q);
		self.end_state_stack.push(qt);
		// push states for the first declaration
		self.start_state_stack.push(qs);
		self.end_state_stack.push(q);
	}
	fn enter_statement_if_else(&mut self,
							   _: Rc<Expression<'a>>, _: Rc<Block<'a>>,
							   if_false: Option<Rc<Block<'a>>>)
	{
		let (qs,qt) = self.pop_stack("If statement");
		
		// Create a new state for the true branch
		let q_true = self.add_state();
		
		//push the true branch's start state for the exit_statement_if_else to use
		self.start_state_stack.push(q_true);
		
		if let Some(_) = if_false {
			// Create a new state for the else branch
			let q_false = self.add_state();
			
			// push its start state for exit_statement_if_else to use
			self.start_state_stack.push(q_false);
			
			//push the start/end states of the if to use on exit
			self.start_state_stack.push(qs);
			self.end_state_stack.push(qt);
			
			//push both start/end states for the else branch to use
			self.start_state_stack.push(q_false);
			self.end_state_stack.push(qt);
		}else{
			//push the start/end states of the if to use on exit
			self.start_state_stack.push(qs);
			self.end_state_stack.push(qt);
		}
		
		// push the true branch's start/end state again for the true branch to use
		self.start_state_stack.push(q_true);
		self.end_state_stack.push(qt);
	}
	fn enter_statement_while(&mut self, _: Rc<Expression<'a>>, _: Rc<Block<'a>>)
	{
		let (qs,qt) = self.peek_stack("While statement");
		
		let q_body = self.add_state();
		// Add new state to stack for exit_statement_while to use
		self.start_state_stack.push(q_body);
		
		// Add start/end states for the body
		self.start_state_stack.push(q_body);
		self.end_state_stack.push(qs);
		
		// Add break and continue targets for the body
		self.break_stack.push(qt);
		self.continue_stack.push(qs);
		
	}
	
//exit methods
	
	fn exit_declaration_variable(&mut self, t:Type, name: &'a str){
		let (qs,qt) = self.pop_stack("Exit variable declaration");
		let qd = self.continue_stack.pop()
			.expect("Exit variable declaration expected drop start state on stack, found none");
		let qp = self.break_stack.pop()
			.expect("Exit variable declaration expected drop end state on stack, found none");
		
		self.graph.add_edge_weighted((qs,qt),Action::DeclareVariable(t,name)).unwrap();
		self.graph.add_edge_weighted((qd,qp),Action::Drop(name)).unwrap();
	}
	fn exit_declaration_array(&mut self, t:Type, name: &'a str, len: i32){
		let (qs,qt) = self.pop_stack("Exit array declaration");
		let qd = self.continue_stack.pop()
			.expect("Exit array declaration expected drop start state on stack, found none");
		let qp = self.break_stack.pop()
			.expect("Exit array declaration expected drop end state on stack, found none");
		
		self.graph.add_edge_weighted((qs,qt),Action::DeclareArray(t,name,len)).unwrap();
		self.graph.add_edge_weighted((qd,qp),Action::Drop(name)).unwrap();
	}
	
	fn exit_statement(&mut self, _: Rc<Statement<'a>>){
		// start/end stacks popped by child statements
	}
	fn exit_statement_assign(&mut self, lvalue: Rc<Lvalue<'a>>, expr: Rc<Expression<'a>>){
		let (qs,qt) = self.pop_stack("Exit assign statement");
		
		self.graph.add_edge_weighted((qs,qt),Action::Assign(lvalue, expr)).unwrap();
		
	}
	fn exit_statement_if_else(&mut self,
							  cond: Rc<Expression<'a>>, _: Rc<Block<'a>>,
							  if_false: Option<Rc<Block<'a>>>)
	{
		let (qs,qt) = self.pop_stack("Exit if statement");
		let false_cond = Rc::new(Expression::Unary(UnaryOperator::Not, cond.clone()));
		
		if let Some(_) = if_false{
			//add else branch
			let q_else = self.start_state_stack.pop().expect("Exit if statement expected start state of else branch, found none.");
			self.graph.add_edge_weighted((qs, q_else), Action::Condition(false_cond)).unwrap();
		}else{
			// No else branch
			self.graph.add_edge_weighted((qs,qt), Action::Condition(false_cond)).unwrap();
		}
		
		// Add true branch
		let q_true = self.start_state_stack.pop().expect("Exit if statement expected start state of true branch, found none.");
		self.graph.add_edge_weighted((qs, q_true), Action::Condition(cond)).unwrap();
	}
	fn exit_statement_while(&mut self, cond: Rc<Expression<'a>>, _: Rc<Block<'a>>)
	{
		let q_body = self.start_state_stack.pop().expect("Exit while state expected body state on stack, found none");
		let (qs,qt) = self.pop_stack("Exit While statement");
		
		// Pop break/continue targets off their stacks
		self.break_stack.pop().expect("Exit while statement expexted break state on stack, found none");
		self.continue_stack.pop().expect("Exit while statement expexted continue state on stack, found none");
		
		let false_cond = Rc::new(Expression::Unary(UnaryOperator::Not, cond.clone()));
		
		// add body branch
		self.graph.add_edge_weighted((qs, q_body), Action::Condition(cond)).unwrap();
		// add false branch
		self.graph.add_edge_weighted((qs, qt), Action::Condition(false_cond)).unwrap();
	}
	fn exit_statement_read(&mut self, lvalue: Rc<Lvalue<'a>>){
		let (qs,qt) = self.pop_stack("Exit read statement");
		self.graph.add_edge_weighted((qs,qt), Action::Read(lvalue)).unwrap();
	}
	fn exit_statement_write(&mut self, value: Rc<Expression<'a>>){
		let (qs,qt) = self.pop_stack("Exit write statement");
		self.graph.add_edge_weighted((qs,qt), Action::Write(value)).unwrap();
	}
	fn exit_statement_break(&mut self){
		let (qs,_) = self.pop_stack("Exit break statement");
		let q_break = *self.break_stack.last().expect("Exit break statement expected break state on stakc, found none");
		self.graph.add_edge_weighted((qs, q_break), Action::Skip).unwrap();
	}
	fn exit_statement_continue(&mut self){
		let (qs,_) = self.pop_stack("Exit continue statement");
		let q_continue = *self.break_stack.last().expect("Exit continue statement expected break state on stakc, found none");
		self.graph.add_edge_weighted((qs, q_continue), Action::Skip).unwrap();
	}
}
