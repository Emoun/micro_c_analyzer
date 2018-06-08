
use super::programs::*;
use progysis::{
	core::{
		PowerSet, TFSpace, Analysis, SubLattice, Bottom
	},
	common::{
		worklist::FifoWorklist
	},
};
use analyzer::micro_c::{
	Expression, ProgramParser, ProgramGrapher, AstVisitor,
	analysis::{
		detection_of_signs::{
			DetectionOfSignsAnalysis, Sign::*, SignsTFSpace, SignsPowerSet
		},
		liveness::{
			LivenessAnalysis, LiveVariables
		},
		lifetime::{
			LifetimeAnalysis, LifetimePowerSet, LifetimeTFSpace
		},
		loan::{
			Loan, LoanPowerSet, LoanAnalysis,
		}
	},
};
use std::{
	collections::{
		HashMap
	},
	rc::Rc,
};

#[test]
fn test_p2_signs_analysis(){
	
	let program = p2_program_graph();
	let mut initial = HashMap::new();
	initial.insert(0,SignsTFSpace::bottom());
	
	DetectionOfSignsAnalysis::analyze::<FifoWorklist<_>>(&program,&mut initial);
	
	let top = SignsPowerSet::from_iter(vec![Plus,Minus,Zero]);
	let plus_zero = SignsPowerSet::from_iter(vec![Plus, Zero]);
	let bot = SignsPowerSet::bottom();
	let minus = SignsPowerSet::singleton(Minus);
	let plus = SignsPowerSet::singleton(Plus);
	
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

#[test]
fn test_p3_liveness_analysis(){
	let program = p3_program_graph();
	let mut initial: HashMap<_,LiveVariables>  = HashMap::new();
	
	LivenessAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	
	for i in 0..=7{
		assert!(initial[&i].all().is_empty(), "State {} was not empty: {:?}", i, initial[&i]);
	}
	
	let expected = vec![
		["x"].iter().cloned().collect(),
		["x","y"].iter().cloned().collect(),
		["x","y","p"].iter().cloned().collect(),
		["p"].iter().cloned().collect(),
		["y","p"].iter().cloned().collect(),
		["y"].iter().cloned().collect(),
	];
	
	for i in 8..=13{
		assert_eq!(initial[&i].all(), expected[(i-8) as usize]);
	}
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct LifeLiveLattice<'a>(LifetimeTFSpace<'a>, LiveVariables<'a>);

impl<'a> Bottom for LifeLiveLattice<'a>
{
	fn bottom() -> Self
	{
		LifeLiveLattice(LifetimeTFSpace::bottom(), LiveVariables::bottom())
	}
}

impl<'a> SubLattice<LifetimeTFSpace<'a>> for LifeLiveLattice<'a>
{
	fn sub_lattice(self) -> LifetimeTFSpace<'a>
	{
		self.0
	}
	
	fn sub_lattice_ref(&self) -> &LifetimeTFSpace<'a>
	{
		&self.0
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut LifetimeTFSpace<'a>
	{
		&mut self.0
	}
}

impl<'a> SubLattice<LiveVariables<'a>> for LifeLiveLattice<'a>
{
	fn sub_lattice(self) -> LiveVariables<'a>
	{
		self.1
	}
	
	fn sub_lattice_ref(&self) -> &LiveVariables<'a>
	{
		&self.1
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut LiveVariables<'a>
	{
		&mut self.1
	}
}

#[test]
fn test_p3_lifetime_analysis(){
	let program = p3_program_graph();
	let mut initial: HashMap<_, LifeLiveLattice>  = HashMap::new();
	
	LivenessAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LifetimeAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	
	for i in 0..=7{
		assert!(initial[&i].1.all().is_empty(), "State {} was not empty: {:?}", i, initial[&i]);
	}
	
	let expected = vec![
		["x"].iter().cloned().collect(),
		["x","y"].iter().cloned().collect(),
		["x","y","p"].iter().cloned().collect(),
		["p"].iter().cloned().collect(),
		["y","p"].iter().cloned().collect(),
		["y"].iter().cloned().collect(),
	];
	
	for i in 8..=13{
		assert_eq!(initial[&i].1.all(), expected[(i-8) as usize]);
	}
	
	for i in 0..=9{
		assert_eq!(0, initial[&i].0.keys().len())
	}
	for i in 10..=12{
		assert_eq!(1, initial[&i].0.keys().len())
	}
	assert_eq!(LifetimePowerSet::singleton("\'x"), initial[&10].0["p"]);
	assert_eq!(LifetimePowerSet::from_iter(vec!["\'x","\'y"]), initial[&11].0["p"]);
	assert_eq!(LifetimePowerSet::singleton("\'x"), initial[&12].0["p"]);
	assert_eq!(0, initial[&13].0.keys().len());
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct LoanLifeLiveLattice<'a>(LoanPowerSet<'a>, LifetimeTFSpace<'a>, LiveVariables<'a>);

impl<'a> Bottom for LoanLifeLiveLattice<'a>
{
	fn bottom() -> Self
	{
		LoanLifeLiveLattice(
			LoanPowerSet::bottom(),
			LifetimeTFSpace::bottom(),
			LiveVariables::bottom())
	}
}

impl<'a> SubLattice<LoanPowerSet<'a>> for LoanLifeLiveLattice<'a>
{
	fn sub_lattice(self) -> LoanPowerSet<'a>
	{
		self.0
	}
	
	fn sub_lattice_ref(&self) -> &LoanPowerSet<'a>
	{
		&self.0
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut LoanPowerSet<'a>
	{
		&mut self.0
	}
}

impl<'a> SubLattice<LifetimeTFSpace<'a>> for LoanLifeLiveLattice<'a>
{
	fn sub_lattice(self) -> LifetimeTFSpace<'a>
	{
		self.1
	}
	
	fn sub_lattice_ref(&self) -> &LifetimeTFSpace<'a>
	{
		&self.1
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut LifetimeTFSpace<'a>
	{
		&mut self.1
	}
}

impl<'a> SubLattice<LiveVariables<'a>> for LoanLifeLiveLattice<'a>
{
	fn sub_lattice(self) -> LiveVariables<'a>
	{
		self.2
	}
	
	fn sub_lattice_ref(&self) -> &LiveVariables<'a>
	{
		&self.2
	}
	
	fn sub_lattice_ref_mut(&mut self) -> &mut LiveVariables<'a>
	{
		&mut self.2
	}
}

#[test]
fn test_p3_loan_analysis(){
	let program = p3_program_graph();
	let mut initial: HashMap<_, LoanLifeLiveLattice>  = HashMap::new();
	
	LivenessAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LifetimeAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LoanAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	
	for i in 0..=7{
		assert!(initial[&i].2.all().is_empty(), "State {} was not empty: {:?}", i, initial[&i].2);
	}
	
	let expected = vec![
		["x"].iter().cloned().collect(),
		["x","y"].iter().cloned().collect(),
		["x","y","p"].iter().cloned().collect(),
		["p"].iter().cloned().collect(),
		["y","p"].iter().cloned().collect(),
		["y"].iter().cloned().collect(),
	];
	
	for i in 8..=13{
		assert_eq!(initial[&i].2.all(), expected[(i-8) as usize]);
	}
	
	for i in 0..=9{
		assert_eq!(0, initial[&i].1.keys().len())
	}
	for i in 10..=12{
		assert_eq!(1, initial[&i].1.keys().len())
	}
	assert_eq!(LifetimePowerSet::singleton("\'x"), initial[&10].1["p"]);
	assert_eq!(LifetimePowerSet::from_iter(vec!["\'x","\'y"]), initial[&11].1["p"]);
	assert_eq!(LifetimePowerSet::singleton("\'x"), initial[&12].1["p"]);
	assert_eq!(0, initial[&13].1.keys().len());
	
	for i in 0..=9{
		assert!(initial[&i].0.all().is_empty(), "State {} was not empty: {:?}", i, initial[&i].0);
	}
	
	let x = Rc::new(Expression::Variable("x"));
	let y = Rc::new(Expression::Variable("y"));
	
	assert_eq!(LoanPowerSet::singleton(Loan{lifetime: "'x", shared: true, lvalue: x.clone()}),
			   initial[&10].0);
	assert_eq!(LoanPowerSet::from_iter(vec![
			Loan{lifetime: "'x", shared: true, lvalue: x.clone()},
			Loan{lifetime: "'y", shared: true, lvalue: y}
		]),
		initial[&11].0);
	assert_eq!(LoanPowerSet::singleton(Loan{lifetime: "'x", shared: true, lvalue: x}),
			   initial[&12].0);
	
}

#[test]
fn test_p4_loan_analysis(){
	let program = p4_program_graph();
	let mut initial: HashMap<_, LoanLifeLiveLattice>  = HashMap::new();
	
	LivenessAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LifetimeAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LoanAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	
	for i in 0..=5{
		assert!(initial[&i].0.all().is_empty(), "State {} was not empty: {:?}", i, initial[&i].0);
	}
	
	let data = Rc::new(Expression::Variable("data"));
	
	assert_eq!(LoanPowerSet::singleton(
		Loan{lifetime: "'a", shared: false, lvalue: data}), initial[&6].0);
	assert!(initial[&7].0.all().is_empty(), "State {} was not empty: {:?}", 7, initial[&7].0);
}

#[test]
fn test_p5_loan_analysis() {
	let ast = ProgramParser::new().parse(P5).unwrap();
	let mut grapher = ProgramGrapher::new();
	grapher.visit(Rc::new(ast));
	let program = grapher.get_graph();
	
	let mut initial: HashMap<_, LoanLifeLiveLattice>  = HashMap::new();
	
	LivenessAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LifetimeAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	LoanAnalysis::analyze::<FifoWorklist<_>>(&program, &mut initial);
	
	for i in 0..=5{
		assert!(initial[&i].0.all().is_empty(), "State {} was not empty: {:?}", i, initial[&i].0);
	}
	
	let x = Rc::new(Expression::Variable("x"));
	
	assert_eq!(LoanPowerSet::singleton(
		Loan{lifetime: "'a", shared: false, lvalue: x.clone()}), initial[&6].0);
	assert_eq!(LoanPowerSet::singleton(
		Loan{lifetime: "'a", shared: false, lvalue: x}), initial[&7].0);
	
	assert!(initial[&8].0.all().is_empty(), "State {} was not empty: {:?}", 8, initial[&8].0);
}