//! Export circ R1cs to Spartan
use libspartan::*;
use crate::target::r1cs::*;

use log::debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    id: usize,
    value: [u8; 32],
}


// circ R1cs -> spartan R1CSInstance
fn r1cs_to_spartan(r1cs: &mut R1cs<S>) //, inputs??) -> (R1CSInstance, Vec<Scalar>, Vec<Scalar>)
{

//    let inputs = get_variables(??);
    
//    let witness = get_variables(r1cs.values)


    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let mut i = 0; // constraint #
    for (lc_a, lc_b, lc_c) in r1cs.constraints {

        // circ Lc (const, monomials <Integer>) -> Vec<Integer> -> Vec<Variable>
	let a = lc_to_v(lc_a, &i);
	let b = lc_to_v(lc_b, &i);
	let c = lc_to_v(lc_c, &i);

	// TODO - deal with constants

	// constraint # x identifier (vars, 1, inp)
        A.push((i, translate(a.id), a.value));
  	B.push((i, translate(b.id), a.value));
        C.push((i, translate(c.id), a.value));


        i += 1;

    }

    /*
    let num_cons = i;
    let num_vars = witness.len();
    let num_inputs = inputs.len();

    let inst = R1CSInstance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    // check if the instance we created is satisfiable
    let res = inst.is_sat(&vars, &inputs);
    assert_eq!(res.unwrap(), true);

    (inst, vars, inputs, num_cons, num_vars, num_inputs)
*/
}


// circ Lc (const, monomials <Integer>) -> Vec<Integer> -> Vec<Variable>
fn lc_to_v(lc: &Lc, i: &usize) -> Variable {
    let mut val = [0; 32];
    for (k,m) in lc.monomials {
        val[k] = m;
    }

    let v = Variable {
        id: i,
        value: val,
    };

    v
}

fn translate(id: &usize) -> usize {


}

fn get_variables() -> Vec<Variable> {

}



