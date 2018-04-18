#[allow(dead_code)]
use micro_c::{
	AstVisitor,
	Block, Declaration,
	Statement,Expression,
	Type, UnaryOperator,
	ProgramGraph, Action
};
use std::rc::Rc;
use petgraph::graph::NodeIndex;

pub struct ProgramGrapher<'a>{
	graph: ProgramGraph<'a>,
	start_state_stack: Vec<NodeIndex>,
	end_state_stack: Vec<NodeIndex>,
	break_stack: Vec<NodeIndex>,
	continue_stack: Vec<NodeIndex>,
}

impl<'a> ProgramGrapher<'a>
{
	pub fn new() -> Self
	{
		let mut graph= ProgramGraph::new();
		let mut start_state_stack = Vec::new();
		let mut end_state_stack = Vec::new();
		let break_stack = Vec::new();
		let continue_stack = Vec::new();
		
		//Create initial and final nodes for the first enter_block
		let qs = graph.add_node(());
		let qt = graph.add_node(());
		
		start_state_stack.push(qs);
		end_state_stack.push(qt);
		
		ProgramGrapher{
			graph,
			start_state_stack,
			end_state_stack,
			break_stack,
			continue_stack,
		}
	}

	fn pop_stack(&mut self, call_msg: &'a str) -> (NodeIndex, NodeIndex)
	{
		let m_start = &format!("{} expected start state on stack, found none", call_msg);
		let m_end = &format!("{} expected end state on stack, found none", call_msg);
		(
			self.start_state_stack.pop().expect(m_start),
			self.end_state_stack.pop().expect(m_end)
		)
	}
	
	fn peek_stack(&mut self, call_msg: &'a str) -> (NodeIndex, NodeIndex)
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
			// Create a new node if there are declarations
			let q = self.graph.add_node(());
			// Push for the statement
			self.start_state_stack.push(q);
			self.end_state_stack.push(qt);
			// Push for the declaration
			self.start_state_stack.push(qs);
			self.end_state_stack.push(q);
		}else{
			self.start_state_stack.push(qs);
			self.end_state_stack.push(qt);
		}
	}
	
	fn enter_declaration_composite(&mut self, _: Rc<Declaration<'a>>, _: Rc<Declaration<'a>>)
	{
		let q = self.graph.add_node(());
		let (qs,qt) = self.pop_stack("Enter Composite declaration");
		
		// push states for the second declaration
		self.start_state_stack.push(q);
		self.end_state_stack.push(qt);
		// Push states for the first declaration
		self.start_state_stack.push(qs);
		self.end_state_stack.push(q);
	}
	
	fn enter_statement_composite(&mut self, _: Rc<Statement<'a>>, _:Rc<Statement<'a>>)
	{
		// Create a new state between them
		let q = self.graph.add_node(());
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
		let q_true = self.graph.add_node(());
		
		//push the true branch's start state for the exit_statement_if_else to use
		self.start_state_stack.push(q_true);
		
		if let Some(_) = if_false {
			// Create a new state for the else branch
			let q_false = self.graph.add_node(());
			
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
		
		let q_body = self.graph.add_node(());
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
		self.graph.add_edge(qs,qt,Action::DeclareVariable(t,name));
	}
	fn exit_declaration_array(&mut self, t:Type, name: &'a str, len: i32){
		let (qs,qt) = self.pop_stack("Exit variable declaration");
		self.graph.add_edge(qs,qt,Action::DeclareArray(t,name,len));
	}
	
	fn exit_statement(&mut self, _: Rc<Statement<'a>>){
		// start/end stacks popped by child statements
	}
	fn exit_statement_assign(&mut self, name: &'a str, expr: Rc<Expression<'a>>){
		let (qs,qt) = self.pop_stack("Exit assign statement");
		
		self.graph.add_edge(qs,qt,Action::Assign(name, expr));
		
	}
	fn exit_statement_assign_array(&mut self, name: &'a str,
								   index: Rc<Expression<'a>>,
								   expr: Rc<Expression<'a>>)
	{
		let (qs,qt) = self.pop_stack("AssignArray statement");
		
		self.graph.add_edge(qs,qt,Action::AssignArray(name, index, expr));
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
			self.graph.add_edge(qs, q_else, Action::Condition(false_cond));
		}else{
			// No else branch
			self.graph.add_edge(qs,qt, Action::Condition(false_cond));
		}
		
		// Add true branch
		let q_true = self.start_state_stack.pop().expect("Exit if statement expected start state of true branch, found none.");
		self.graph.add_edge(qs, q_true, Action::Condition(cond));
	}
	fn exit_statement_while(&mut self, cond: Rc<Expression<'a>>, _: Rc<Block<'a>>)
	{
		let qs = self.start_state_stack.pop().expect("Exit while statement expected start state on stack, found none.");
		let qt = self.end_state_stack.pop().expect("Exit while statement end state on stack, found none.");
		let q_body = self.start_state_stack.pop().expect("Exit while state expected body state on stack, found none");
		
		// Pop break/continue targets off their stacks
		self.break_stack.pop().expect("Exit while statement expexted break state on stack, found none");
		self.continue_stack.pop().expect("Exit while statement expexted continue state on stack, found none");
		
		let false_cond = Rc::new(Expression::Unary(UnaryOperator::Not, cond.clone()));
		
		// add body branch
		self.graph.add_edge(qs, q_body, Action::Condition(cond));
		// add false branch
		self.graph.add_edge(qs, qt, Action::Condition(false_cond));
	}
	fn exit_statement_read(&mut self, var: &'a str){
		let (qs,qt) = self.pop_stack("Exit read statement");
		self.graph.add_edge(qs,qt, Action::Read(var));
	}
	fn exit_statement_read_array(&mut self, arr: &'a str, index: Rc<Expression<'a>>){
		let (qs,qt) = self.pop_stack("Exit read array statement");
		self.graph.add_edge(qs,qt, Action::ReadArray(arr,index));
	}
	fn exit_statement_write(&mut self, value: Rc<Expression<'a>>){
		let (qs,qt) = self.pop_stack("Exit write statement");
		self.graph.add_edge(qs,qt, Action::Write(value));
	}
	fn exit_statement_break(&mut self){
		let (qs,_) = self.pop_stack("Exit break statement");
		let q_break = *self.break_stack.last().expect("Exit break statement expected break state on stakc, found none");
		self.graph.add_edge(qs, q_break, Action::Skip);
	}
	fn exit_statement_continue(&mut self){
		let (qs,_) = self.pop_stack("Exit continue statement");
		let q_continue = *self.break_stack.last().expect("Exit continue statement expected break state on stakc, found none");
		self.graph.add_edge(qs, q_continue, Action::Skip);
	}
}
