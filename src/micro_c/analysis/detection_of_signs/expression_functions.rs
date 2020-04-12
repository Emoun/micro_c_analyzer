
use crate::micro_c::{
	UnaryOperator, Expression,
	analysis::detection_of_signs::{
		Sign, SignsPowerSet, SignsTFSpace
	}
};
use progysis::core::{
	PowerSet, Bottom
};

macro_rules! element{
	(+)=>{
		$crate::micro_c::analysis::detection_of_signs::SignsPowerSet
		::singleton($crate::micro_c::analysis::detection_of_signs::Sign::Plus)
	};
	(0)=>{
		$crate::micro_c::analysis::detection_of_signs::SignsPowerSet
		::singleton($crate::micro_c::analysis::detection_of_signs::Sign::Zero)
	};
	(-)=>{
		$crate::micro_c::analysis::detection_of_signs::SignsPowerSet
		::singleton($crate::micro_c::analysis::detection_of_signs::Sign::Minus)
	};
	(
		$($mz_signs:tt)*
	)=>{
		<$crate::micro_c::analysis::detection_of_signs::SignsPowerSet as ::progysis::core::Bottom>
		::bottom() $(+ element!($mz_signs))*
	};
}

macro_rules! binary_operator_mapping_macro{

	(
		$(
			$bin_op:ident,
			($($mm_signs:tt)*) | ($($mz_signs:tt)*) | ($($mp_signs:tt)*)
			($($zm_signs:tt)*) | ($($zz_signs:tt)*) | ($($zp_signs:tt)*)
			($($pm_signs:tt)*) | ($($pz_signs:tt)*) | ($($pp_signs:tt)*)
		)*
	)=>{
		
		pub fn binary_operator_mapping(
			lhs: $crate::micro_c::analysis::detection_of_signs::Sign,
			op: $crate::micro_c::BinaryOperator,
			rhs: $crate::micro_c::analysis::detection_of_signs::Sign)
			-> $crate::micro_c::analysis::detection_of_signs::SignsPowerSet
		{
			
			match op{
				$(
					$crate::micro_c::BinaryOperator::$bin_op =>{
						match lhs {
							$crate::micro_c::analysis::detection_of_signs::Sign::Plus => match rhs{
								$crate::micro_c::analysis::detection_of_signs::Sign::Plus => element!($($pp_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Zero => element!($($pz_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Minus => element!($($pm_signs)*),
							},
							$crate::micro_c::analysis::detection_of_signs::Sign::Zero => match rhs{
								$crate::micro_c::analysis::detection_of_signs::Sign::Plus => element!($($zp_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Zero => element!($($zz_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Minus => element!($($zm_signs)*),
							},
							$crate::micro_c::analysis::detection_of_signs::Sign::Minus => match rhs{
								$crate::micro_c::analysis::detection_of_signs::Sign::Plus => element!($($mp_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Zero => element!($($mz_signs)*),
								$crate::micro_c::analysis::detection_of_signs::Sign::Minus => element!($($mm_signs)*),
							},
						}
					},
				)*
				
				//_ => unimplemented!("Haven't implemented operator: {:?}", op),
			}
		}
	};
}

binary_operator_mapping_macro!{
	/*
	The format below matches table:
			opr	|	-	|	0	|	+		(rhs)
			-	|	v11	|	v21	|	v31
	(lhs)	0	|	v12	|	v22	|	v32
			+	|	v13	|	v23	|	v31
	*/
	Plus,
	(-)		| (-)	| (+0-)
	(-)		| (0)	| (+)
	(+0-)	| (+)	| (+)
	
	Minus,
	(+0-)	| (-)	| (-)
	(+)		| (0)	| (-)
	(+)		| (0)	| (+0-)
	
	Multiply,
	(+)		| (0)	| (-)
	(0)		| (0)	| (0)
	(-)		| (0)	| (+)
	
	Division,
	(+)		| (+0-)	| (-)
	(0)		| (+0-)	| (0)
	(-)		| (+0-)	| (+)
	
	LessThan,
	(+0)	| (+)	| (+)
	(0)		| (0)	| (+)
	(0)		| (0)	| (+0)
	
	GreaterThan,
	(+0)	| (0)	| (+)
	(+)		| (0)	| (0)
	(+)		| (+)	| (+0)
	
	LessOrEqual,
	(+0)	| (+)	| (+)
	(0)		| (+)	| (+)
	(0)		| (0)	| (+0)
	
	GreaterOrEqual,
	(+0)	| (0)	| (+)
	(0)		| (+)	| (+)
	(0)		| (+)	| (+0)
	
	Equal,
	(+0)	| (0)	| (0)
	(0)		| (+)	| (0)
	(0)		| (0)	| (+0)
	
	NotEqual,
	(+0)	| (+)	| (+)
	(+)		| (0)	| (+)
	(+)		| (+)	| (+0)
	
	And,
	(+)		| (0)	| (+)
	(0)		| (0)	| (0)
	(+)		| (0)	| (+)
	
	Or,
	(+)		| (+)	| (+)
	(+)		| (0)	| (+)
	(+)		| (+)	| (+)
	
}

pub fn unary_operator_mapping(op: UnaryOperator, rhs: Sign) -> SignsPowerSet
{
	match op{
		UnaryOperator::Negative => match rhs {
			Sign::Plus => element!(-),
			Sign::Minus => element!(+),
			Sign::Zero => element!(0),
		},
		UnaryOperator::Not => match rhs {
			Sign::Plus => element!(0),
			Sign::Minus => element!(0),
			Sign::Zero =>element!(+),
		},
		_ => unimplemented!()
	}
}

pub fn evaluate<'a>(state: &SignsTFSpace<'a>, expr: &'a Expression<'a>) -> SignsPowerSet
{
	match *expr{
		Expression::Constant(n) => if n>0 {element!(+)}else if n<0 {element!(-)}else{element!(0)},
		Expression::Variable(id)
		| Expression::ArrayAccess(id, _)=> state[id].clone(),
		Expression::Binary(ref lhs, op, ref rhs) => {
			let mut result = SignsPowerSet::bottom();
			for s1 in evaluate(state, lhs).all(){
				for s2 in evaluate(state, rhs).all(){
					result += binary_operator_mapping(s1, op, s2);
				}
			}
			result
		},
		Expression::Unary(op, ref rhs) => {
			let mut result = SignsPowerSet::bottom();
			for s in evaluate(state, rhs).all(){
				result += unary_operator_mapping(op, s);
			}
			result
		}
	}
}


