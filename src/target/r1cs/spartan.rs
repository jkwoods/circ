//! Export circ R1cs to Spartan
use libspartan::*;
use crate::target::r1cs::*;
use curve25519_dalek::scalar::Scalar;

use log::debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    id: usize,
    value: [u8; 32],
}


// circ R1cs -> spartan R1CSInstance
fn r1cs_to_spartan(r1cs: &mut R1cs<S>, inputs: XXX, witness: XX) -> (R1CSInstance, Vec<Scalar>, Vec<Scalar>)
{

    let num_inputs = inputs.len();

    
    // witness
    let mut wit: Vec<Variable> = Vec::new();
    let mut witness: Vec<Scalar> = Vec::new();

    for m in r1cs.values {
	match m {
	    Some (k, v) => { // CirC id, Integer
		let scalar = int_to_scalar(v);
        	let var = Variable {
            	    id: translate(k),
            	    value: scalar.to_bytes(),
        	};
	    wit.push(var);
	    witness.push(scalar); // ordering (?) - TODO

	    }
	    None => {}
	}
    }

    let num_vars = witness.length();
 
    // circuit
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let mut i = 0; // constraint #
    for (lc_a, lc_b, lc_c) in r1cs.constraints {

        // circ Lc (const, monomials <Integer>) -> Vec<Integer> -> Vec<Variable>
	let a = lc_to_v(lc_a, num_vars);
        let b = lc_to_v(lc_b, num_vars);
	let c = lc_to_v(lc_c, num_vars);

	// constraint # x identifier (vars, 1, inp)
        for Variable { id, value } in a {
            A.push((i, id, value));
	}
	for Variable { id, value } in b { 
  	    B.push((i, id, value));
	}
	for Variable { id, value } in c {
            C.push((i, id, value));
	}

        i += 1;

    }

    
    let num_cons = i;
    let inst = R1CSInstance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    // check if the instance we created is satisfiable
    let res = inst.is_sat(&wintess, &inputs);
    assert_eq!(res.unwrap(), true);

    (inst, vars, inputs, num_cons, num_vars, num_inputs)

}

fn int_to_scalar(in: &Integer) -> Scalar {


}

// figure out input or witness, put into Spartan format: z = [vars, 1, inputs]
fn translate(k: &usize) -> usize {


}

// circ Lc (const, monomials <Integer>) -> Vec<Variable>
fn lc_to_v(lc: &Lc, const_id: &usize) -> Vec<Variable> {
    let mut v: Vec<Variable> = Vec::new();

    for (k,m) in lc.monomials {
	if (k >= const_id) { panic!("Error: variable number off") }

        let scalar = int_to_scalar(m);
        let var = Variable {
            id: translate(k),
            value: scalar.to_bytes(),
        };
	v.push(var);
    }
    if (lc.constant != &Integer::from(0)) {
	let scalar = int_to_scalar(m);
        let var = Variable {
            id: const_id,
            value: scalar.to_bytes(),
        };
        v.push(var);
    }
    v
}

fn get_variables() -> Vec<Variable> {

}



