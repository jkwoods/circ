//! Export circ R1cs to zkinterface CS
use crate::target::r1cs::*;
use rug::{Integer};
use core::clone::Clone;

use std::collections::HashMap;
use gmp_mpfr_sys::gmp::limb_t;
use lazy_static::lazy_static;

use zkinterface;


/// circ R1cs -> zkif
pub fn r1cs_to_zkif<S: Eq + Hash + Clone + Display>(r1cs: R1cs<S>) -> (zkinterface::CircuitHeader, zkinterface::ConstraintSystem, zkinterface::Witness)
{

    // format mapper: CirC -> ZKIF
    let mut wit = Vec::new();
    let mut inp = Vec::new();
    let mut pub_trans: HashMap<usize, usize> = HashMap::new(); // Circ -> zkif ids
    let mut priv_trans: HashMap<usize, usize> = HashMap::new(); // Circ -> zkif ids

    match r1cs.values {
        Some(_) =>
            for (k,v) in r1cs.values.as_ref().unwrap() { // CirC id, Integer

                let name = r1cs.idxs_signals.get(k).unwrap().to_string();
                let scalar = int_to_scalar(&v.i());

                if r1cs.public_idxs.contains(k) {
                    // input
                    println!("As public io: {}", name);

                    pub_trans.insert(*k, inp.len());
                    inp.push(scalar.to_bytes());
                } else {
                     // witness
                    println!("As private witness: {}", name);

                    priv_trans.insert(*k, wit.len());
                    wit.push(scalar.to_bytes());

                }

            }
        None    => panic!("Tried to output R1CS without inputs/witness"),
    }

    assert_eq!(wit.len() + inp.len(), r1cs.next_idx);

    // trans = (0, pub, priv)
    let mut trans: HashMap<usize,usize> = HashMap::new();
    for (cid, zid) in pub_trans {
        trans.insert(cid, zid+1);
    }
    for (cid, zid) in priv_trans {
        trans.insert(cid, zid+1+inp.len());
    }

    let zkif_header = zkinterface::CircuitHeader {
        instance_variables: Variables {
            variable_ids: // vec of public ids
            values: Some(serialize(&[ xx ])), // vals of public ids
        },
        free_variable_id: r1cs.next_idx, // TODO ?
        field_maximum: serialize(x), // TODO - largest elt of finit field (feild order - 1)
        configuration: None, // optional
    };

    let zkif_witness = zkinterface::Witness {
        assigned_variables: Variables { // private vars
            variable_ids: vev,
            values: Some(serialize(&[ xx ])),
        }
    };

    // put circuit together, witness = (1, private)
    let constraints_vec: [((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = Vec::new();

    for (lc_a, lc_b, lc_c) in r1cs.constraints.iter() {


        // circ Lc (const, monomials <Integer>) -> Vec<Variable>
        let a = lc_to_v(&lc_a, &trans); // return (vec<u64>,vec<u8>)
        let b = lc_to_v(&lc_b, &trans);
        let c = lc_to_v(&lc_c, &trans);

        // make bilin constraint - order of these in constraints vec doesn't matter
        constraints_vec.push((a,b,c));

    }
    
    let zkif_cs = zkinterface::ConstraintSystem::from(&constraints_vec);

    return (zkif_header, zkif_cs, zkif_wit);

}

fn int_to_u8(i: &Integer) -> Scalar {
    let mut accumulator = Scalar::zero();
    let limb_bits = (std::mem::size_of::<limb_t>() as u64) << 3;
    assert_eq!(limb_bits, 64);

    let two: u64 = 2;
    let mut m = Scalar::from(two.pow(63) as u64);
    m = m * Scalar::from(2 as u64);

    // as_ref yeilds a least-significant-first array.
    for digit in i.as_ref().iter().rev() {
        accumulator *= m;
        accumulator += Scalar::from(*digit as u64);
    }
    return accumulator;

}


// circ Lc (const, monomials <Integer>) -> (Vec<u64>, Vec<u8>) 
fn lc_to_v(lc: &Lc, trans: &HashMap<usize,usize>) -> (Vec<u64>, Vec<u8>) {
    
    let mut ids: Vec<u64> = Vec::new();
    let mut vars: Vec<Vec<u8>> = Vec::new();
 
    if lc.constant.i() != Integer::from(0 as u32) { // TODO: .cmp0() != Ordering::Equal
        ids.push(0);
        let scalar = int_to_u8(&lc.constant.i());
        vars.push(scalar);
    }

    for (k,m) in &lc.monomials {
        ids.push(trans.get(k).unwrap().clone());
        let scalar = int_to_u8(&m.i());
        vars.push(scalar);
    }
    
    return (ids, vars);
}

