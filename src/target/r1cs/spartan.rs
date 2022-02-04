//! Export circ R1cs to Spartan
use libspartan::*;
use crate::target::r1cs::*;
use curve25519_dalek::scalar::Scalar;
use rug::{Assign, Integer};
use core::clone::Clone;
use core::ops::Shr;

use log::debug;
use std::collections::HashMap;
use gmp_mpfr_sys::gmp::limb_t;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPARTAN_MODULUS: Integer = Integer::from_str_radix(
        "7237005577332262213973186563042994240857116359379907606001950938285454250989",
        // "52435875175126190479447740508185965837690552500527637822603658699938581184513",
         10
    ).unwrap();

}


#[derive(Debug)]
pub struct Variable {
    id: usize,
    value: [u8; 32],
}


// circ R1cs -> spartan R1CSInstance
pub fn r1cs_to_spartan<S: Eq + Hash + Clone + Display>(r1cs: R1cs<S>) -> (Instance, Assignment, Assignment, usize, usize, usize)
{

    // assume no public inputs for now
    assert!(r1cs.public_idxs.len() == 0);
    let num_inputs = 0; //r1cs.vals.len();

    let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
    let assn_inputs = InputsAssignment::new(&inputs).unwrap();

    // witness
    let num_vars = r1cs.next_idx;

    let mut wit: Vec<Variable> = Vec::new();
    let mut witness = vec![Scalar::zero().to_bytes(); num_vars];

    // TODO if not input?
    match r1cs.values {
	Some(_) => 
	    for (k,v) in r1cs.values.as_ref().unwrap() { // CirC id, Integer
	
		let scalar = int_to_scalar(&v);
        	let var = Variable {
            	    id: *k, //translate(k),
            	    value: scalar.to_bytes(),
        	};	
	    	wit.push(var);
 	    	witness[*k] = scalar.to_bytes(); // ordering (?) - TODO
	    }
	None 	=> panic!("Tried to run Spartan without inputs/witness"),
    }

    let assn_witness = VarsAssignment::new(&witness).unwrap();
    let const_id = r1cs.next_idx;
 
    // circuit
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let mut i = 0; // constraint #
    for (lc_a, lc_b, lc_c) in r1cs.constraints.iter() {

        // circ Lc (const, monomials <Integer>) -> Vec<Integer> -> Vec<Variable>
	let a = lc_to_v(&lc_a, const_id);
        let b = lc_to_v(&lc_b, const_id);
	let c = lc_to_v(&lc_c, const_id);

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
    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    // check if the instance we created is satisfiable
    let res = inst.is_sat(&assn_witness, &assn_inputs);
    assert_eq!(res.unwrap(), true);

    (inst, assn_witness, assn_inputs, num_cons, num_vars, num_inputs)

}

fn int_to_scalar(i: &Integer) -> Scalar {
    let mut accumulator = Scalar::zero();
    let limb_bits = (std::mem::size_of::<limb_t>() as u64) << 3;
    assert_eq!(limb_bits, 64);

    let two: u64 = 2;
    let mut m = Scalar::from(two.pow(63) as u64);
    m = m * Scalar::from(2 as u64);
    //println!("in int2scal i={:#?}", i); 

    // as_ref yeilds a least-significant-first array.
    for digit in i.as_ref().iter().rev() {
	//println!("digit: {:#?}", digit);
        accumulator *= m;
        accumulator += Scalar::from(*digit as u64);
    }
    return accumulator; 

}

// figure out input or witness, put into Spartan format: z = [vars, 1, inputs]
fn translate(k: &usize) -> usize {
    return 0; // TODO
    // i think it may be a direct mapping ?

}

// circ Lc (const, monomials <Integer>) -> Vec<Variable>
fn lc_to_v(lc: &Lc, const_id: usize) -> Vec<Variable> {
    let mut v: Vec<Variable> = Vec::new();

    for (k,m) in &lc.monomials {
	if *k >= const_id { panic!("Error: variable number off") }

        let scalar = int_to_scalar(&m);
        //println!("int to scalar test: {:#?} -> {:#?}", m, scalar.to_bytes());
	let var = Variable {
            id: *k, //translate(k),
            value: scalar.to_bytes(),
        };
	v.push(var);
    }
    if lc.constant != Integer::from(0 as u32) {
	let scalar = int_to_scalar(&lc.constant);
        let var = Variable {
            id: const_id,
            value: scalar.to_bytes(),
        };
        v.push(var);
    }
    v
}



