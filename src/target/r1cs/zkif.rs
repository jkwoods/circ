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
    let mut wit = HashMap<usize, Integer>::new(); // CirC id -> non concated u8 vec
    let mut inp = HashMap<usize, Integer>::new(); // CirC id -> "
    let mut pub_trans: HashMap<usize, usize> = HashMap::new(); // CirC -> zkif ids
    let mut priv_trans: HashMap<usize, usize> = HashMap::new(); // CirC -> zkif ids

    match r1cs.values {
        Some(_) =>
            for (k,v) in r1cs.values.as_ref().unwrap() { // CirC id, Integer

                let name = r1cs.idxs_signals.get(k).unwrap().to_string();
                let scalar = &v.i();

                if r1cs.public_idxs.contains(k) {
                    // input
                    println!("As public io: {}", name);

                    pub_trans.insert(*k, inp.len());
                    inp.insert(*k,scalar);
                } else {
                     // witness
                    println!("As private witness: {}", name);

                    priv_trans.insert(*k, wit.len());
                    wit.insert(*k,scalar);
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

    let pub_ids: Vec<u64> = Vec::new();
    let pub_vars: Vec<Integer> = Vec::new();
    for (cid, cvar) in inp {
        pub_ids.push(pub_trans.get(cid).unwrap().clone());
        pub_vars.push(&cvar.i());
    }

    // instance (public io)
    let zkif_header = zkinterface::CircuitHeader {
        instance_variables: Variables {
            variable_ids: pub_ids, // vec of public ids
            values: Some(serialize(pub_vars)), // vals of public ids
        },
        free_variable_id: r1cs.next_idx+1, // TODO ?
        field_maximum: serialize(x), // TODO - largest elt of finit field (feild order - 1)
        configuration: None, // optional
    };

    // witness
    let priv_ids: Vec<u64> = Vec::new();
    let priv_vars: Vec<Integer> = Vec::new();
    for (cid, cvar) in wit {
        priv_ids.push(priv_trans.get(cid).unwrap().clone());
        priv_vars.push(&cvar.i());
    }

    let zkif_witness = zkinterface::Witness {
        assigned_variables: Variables { // private vars
            variable_ids: priv_ids,
            values: Some(serialize(priv_vars)),
        }
    };

    // put circuit together, witness = (1, private)
    let constraints_vec: [((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = Vec::new();

    for (lc_a, lc_b, lc_c) in r1cs.constraints.iter() {

        // circ Lc (const, monomials <Integer>) -> ZKIF (almost)
        let a = lc_to_v(&lc_a, &trans); // return (vec<u64>,vec<Integer>)
        let b = lc_to_v(&lc_b, &trans);
        let c = lc_to_v(&lc_c, &trans);

        // make bilin constraint - order of these in constraints vec doesn't matter
        constraints_vec.push((a,b,c));

    }
    
    let zkif_cs = zkinterface::ConstraintSystem::from(&constraints_vec);

    return (zkif_header, zkif_cs, zkif_wit);

}


// circ Lc (const, monomials <Integer>) -> (Vec<u64>, Vec<Vec<u8>>) 
fn lc_to_v(lc: &Lc, trans: &HashMap<usize,usize>) -> (Vec<u64>, Vec<Vec<u8>>) {
    
    let mut ids: Vec<u64> = Vec::new();
    let mut vars: Vec<Integer> = Vec::new();
 
    if lc.constant.i() != Integer::from(0 as u32) { // TODO: .cmp0() != Ordering::Equal
        ids.push(0);
        let scalar = &lc.constant.i();
        vars.push(scalar);
    }

    for (k,m) in &lc.monomials {
        ids.push(trans.get(k).unwrap().clone());
        let scalar = &m.i();
        vars.push(scalar);
    }

    vars = serialize(vars);
    
    return (ids, vars);
}

// all big Integers become little endian vecs of u8, then are all concatenated together
fn serialize(vals: Vec<&Integer>) -> Vec<u8> {

    let mut lil = Vec<u8> = Vec::new();
    let elt_bits = ceil(max bits of feild / 8);

    for i in vals {
        let vec = i.to_digits::<u8>(Order::Lsf); // ??
        while vec.len() < elt_bits { vec.push(0 as u8); }
        if vec.len() > elt_bits { panic!("Value too big for feild"); }
        
        lil.append(&mut vec);
    }
    
    return lil;
}

