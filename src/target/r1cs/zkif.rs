//! Export circ R1cs to zkinterface CS
use crate::target::r1cs::*;
use rug::{Integer, integer::Order};
use core::clone::Clone;
use std::collections::HashMap;
use zkinterface::{Witness, Variables, ConstraintSystem, BilinearConstraint, CircuitHeader};

/// circ R1cs -> zkif
pub fn r1cs_to_zkif<S: Eq + Hash + Clone + Display>(r1cs: R1cs<S>, field: &FieldT) -> (CircuitHeader, ConstraintSystem, Witness)
{

    // format mapper: CirC -> ZKIF
    let mut wit: HashMap<usize, Integer> = HashMap::new(); // CirC id -> non concated u8 vec
    let mut inp: HashMap<usize, Integer> = HashMap::new(); // CirC id -> "
    let mut pub_trans: HashMap<usize, u64> = HashMap::new(); // CirC -> zkif ids
    let mut priv_trans: HashMap<usize, u64> = HashMap::new(); // CirC -> zkif ids

    match r1cs.values {
        Some(_) =>
            for (k,v) in r1cs.values.as_ref().unwrap() { // CirC id, Integer

                let name = r1cs.idxs_signals.get(k).unwrap().to_string();

                if r1cs.public_idxs.contains(k) {
                    // input
                    println!("As public io: {}", name);

                    pub_trans.insert(*k, inp.len() as u64);
                    inp.insert(*k,v.i().clone());
                } else {
                     // witness
                    println!("As private witness: {}", name);

                    priv_trans.insert(*k, wit.len() as u64);
                    wit.insert(*k,v.i().clone());
                }

            }
        None    => panic!("Tried to output R1CS without inputs/witness"),
    }

    assert_eq!(wit.len() + inp.len(), r1cs.next_idx);

    // trans = (0, pub, priv)
    let mut trans: HashMap<usize,u64> = HashMap::new();
    for (cid, zid) in &pub_trans {
        trans.insert(cid.clone(), zid.clone()+1);
    }
    for (cid, zid) in &priv_trans {
        trans.insert(cid.clone(), zid.clone()+1+(inp.len() as u64));
    }

    let mut pub_ids: Vec<u64> = Vec::new();
    let mut pub_vars: Vec<&Integer> = Vec::new();
    for (cid, cvar) in inp {
        pub_ids.push(pub_trans.get(&cid).unwrap().clone());
        pub_vars.push(&cvar);
    }

    // instance (public io)
    let zkif_header = CircuitHeader {
        instance_variables: Variables {
            variable_ids: pub_ids, // vec of public ids
            values: Some(serialize(pub_vars, field)), // vals of public ids
        },
        free_variable_id: (r1cs.next_idx as u64)+1, // TODO ?
        field_maximum: Some(serialize(vec![&(field.modulus() - Integer::from(1))], field)), // TODO - largest elt of finit field (feild order - 1)
        configuration: None, // optional
    };

    // witness
    let mut priv_ids: Vec<u64> = Vec::new();
    let mut priv_vars: Vec<&Integer> = Vec::new();
    for (cid, cvar) in wit {
        priv_ids.push(priv_trans.get(&cid).unwrap().clone());
        priv_vars.push(&cvar);
    }

    let zkif_witness = Witness {
        assigned_variables: Variables { // private vars
            variable_ids: priv_ids,
            values: Some(serialize(priv_vars, field)),
        }
    };

    // put circuit together, witness = (1, private)
    let mut constraints_vec: Vec<BilinearConstraint> = Vec::new();

    for (lc_a, lc_b, lc_c) in r1cs.constraints.iter() {

        // circ Lc (const, monomials <Integer>) -> ZKIF (almost)
        let a = lc_to_v(&lc_a, &trans, field); // return Variables {vec<u64>,vec<Integer>}
        let b = lc_to_v(&lc_b, &trans, field);
        let c = lc_to_v(&lc_c, &trans, field);

        // make bilin constraint - order of these in constraints vec doesn't matter
        constraints_vec.push(BilinearConstraint {
            linear_combination_a: a,
            linear_combination_b: b,
            linear_combination_c: c,
        });

    }
    
    let zkif_cs = ConstraintSystem {
        constraints: constraints_vec,
    }; //ConstraintSystem::from(&constraints_vec);

    return (zkif_header, zkif_cs, zkif_witness);

}


// circ Lc (const, monomials <Integer>) -> (Vec<u64>, Vec<Vec<u8>>) 
fn lc_to_v(lc: &Lc, trans: &HashMap<usize,u64>, field: &FieldT) -> Variables {
    
    let mut ids: Vec<u64> = Vec::new();
    let mut vars: Vec<&Integer> = Vec::new();
 
    if lc.constant.i() != Integer::from(0 as u32) { // TODO: .cmp0() != Ordering::Equal
        ids.push(0);
        vars.push(&lc.constant.i());
    }

    for (k,m) in &lc.monomials {
        ids.push(trans.get(k).unwrap().clone());
        vars.push(&m.i());
    }

    return Variables{
        variable_ids: ids, 
        values: Some(serialize(vars, field)),
    };
}

// all big Integers become little endian vecs of u8, then are all concatenated together
fn serialize(vals: Vec<&Integer>, f: &FieldT) -> Vec<u8> {

    let mut lil: Vec<u8> = Vec::new();
    let elt_bits = (f.modulus().significant_bits() as f64 / 8.0).ceil() as usize;

    for i in vals {
        let mut vec = i.to_digits::<u8>(Order::Lsf); // ??
        while vec.len() < elt_bits { vec.push(0 as u8); }
        if vec.len() > elt_bits { panic!("Value too big for field"); }
        
        lil.append(&mut vec);
    }

    return lil;
}

