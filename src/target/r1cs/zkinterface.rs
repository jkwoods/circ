//! Export circ R1cs to ZkInterface
use zkinterface::{Variables, BilinearConstraint};

use log::debug;
use std::collections::HashMap;



pub fn to_zkif()
{


}

pub fn to_zkif_constraint(
	a: LC?, 
	b: LC, 
	c: LC
) -> BilinearConstraint {
	BilinearConstraint {
		lc_a: to_zkif_lc(a),
		lc_b: to_zkif_lc(b),
		lc_c: to_zkif_lc(c),
	}
}


pub fn to_zkif_lc(lc: LC?,) -> Variables {
	xxxxx


	Variables { variable_ids, values: Some(coeff) }
}


