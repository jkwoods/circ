//! Export circ R1cs to Spartan
use libspartan::*

use log::debug;
use std::collections::HashMap;


// circ R1cs -> spartan R1CSInstance
fn r1cs_to_spartan(r1cs: &mut R1cs, inputs??) -> (R1CSInstance, Vec<Scalar>, Vec<Scalar>)
{

    let inputs = get_variables(??);
    
    let witness = get_variables(values??)


    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let mut i = 0;
    for (lc_a, lc_b, lc_c) in constraints {

	// to Lef's reader IR
	vec_a = single_vec(lc_a);

	// to Spartan
        let a = get_variables(vec_a);


	for Variable { id, value } in b {
            A.push((i, self.translate(id), value));
        }
        for Variable { id, value } in b {
            B.push((i, self.translate(id), value));
        }
        for Variable { id, value } in c {
            C.push((i, self.translate(id), value));
        }
        i += 1;

    }

    
    let num_cons = i;
    let num_vars = witness.len();
    let num_inputs = inputs.len();

    let inst = R1CSInstance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    // check if the instance we created is satisfiable
    let res = inst.is_sat(&vars, &inputs);
    assert_eq!(res.unwrap(), true);

    (inst, vars, inputs, num_cons, num_vars, num_inputs)
}

fn get_variables(){ -> Vec<Variable>

}

// circ Lc (const, monomials) -> Vec<Variable>
fn single_vec(){

}



