//! Export circ R1cs to Spartan
use libspartan::*

use log::debug;
use std::collections::HashMap;


// circ R1cs -> spartan R1CSInstance
fn r1cs_to_spartan(r1cs: &mut R1cs) -> (R1CSInstance, Vec<Scalar>, Vec<Scalar>)
{

    let num_cons =
    let num_vars =
    let num_inputs = 

    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    let inst = R1CSInstance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();

    let mut i = 0;
    for (lc_a, lc_b, lc_c) in r1cs.constraints {

        

        A.push((i, self.translate(id), value));
        



        for Variable { id, value } in b {
            B.push((i, self.translate(id), value));
        }
        for Variable { id, value } in c {
            C.push((i, self.translate(id), value));
        }
        i += 1;

    }





     (inst, vars, inputs, num_cons, num_vars, num_inputs)
}






