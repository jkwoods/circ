//! Export circ R1cs to zkinterface CS
use crate::target::r1cs::*;
use rug::{Integer, integer::Order};
use core::clone::Clone;
use zkinterface::{Witness, Variables, ConstraintSystem, BilinearConstraint, CircuitHeader};

/// circ R1cs -> zkif
pub fn r1cs_to_zkif<S: Eq + Hash + Clone + Display>(r1cs: R1cs<S>, field: &FieldT) -> (CircuitHeader, ConstraintSystem, Witness)
{


    let mut inp: Vec<(u64, Integer)> = Vec::new();
    let mut wit: Vec<(u64, Integer)> = Vec::new();

    match r1cs.values {
        Some(_) =>
            for (k,v) in r1cs.values.as_ref().unwrap() { // CirC id, Integer

                let name = r1cs.idxs_signals.get(k).unwrap().to_string();

                if r1cs.public_idxs.contains(k) {
                    // input
                    println!("As public io: {}", name);
                    println!("k = {}, k+1 = {}", k, k+1);
                    println!("{:#?}",v.i());

                    inp.push(((k+1) as u64, v.i()));
                } else {
                    // witness
                    println!("As private witness: {}", name);
                    println!("k = {}, k+1 = {}", k, k+1);
                    println!("{:#?}", v.i());
                    
                    wit.push(((k+1) as u64, v.i()));
                }

            }
        None    => panic!("Tried to output R1CS without inputs/witness"),
    }

    // todo where do constants go??

    assert_eq!(inp.len() + wit.len(), r1cs.next_idx);
    inp.sort_by(|a,b| a.0.cmp(&b.0));
    wit.sort_by(|a,b| a.0.cmp(&b.0));

    let (pub_ids, pub_vars): (Vec<u64>, Vec<Integer>) = inp.into_iter().unzip();
    let (priv_ids, priv_vars): (Vec<u64>, Vec<Integer>) = wit.into_iter().unzip();
    
    println!("{:#?}, {:#?}", pub_ids, pub_vars);
    println!("{:#?}, {:#?}", priv_ids, priv_vars);

    // instance (public io)
    let zkif_header = CircuitHeader {
        instance_variables: Variables {
            variable_ids: pub_ids, // vec of public ids
            values: Some(serialize(&pub_vars, field)), // vals of public ids
        },
        free_variable_id: (r1cs.next_idx as u64)+1, // TODO ?
        field_maximum: Some(serialize(&vec![(field.modulus() - Integer::from(1))], field)), // TODO - largest elt of finit field (feild order - 1)
        configuration: None, // optional
    };

    println!("{:#?}", zkif_header);

    // witness
    let zkif_witness = Witness {
        assigned_variables: Variables { // private vars
            variable_ids: priv_ids,
            values: Some(serialize(&priv_vars, field)),
        }
    };

    println!("{:#?}", zkif_witness);

    // put circuit together, witness = (1, private)
    let mut constraints_vec: Vec<BilinearConstraint> = Vec::new();

    for (lc_a, lc_b, lc_c) in r1cs.constraints.iter() {

        // circ Lc (const, monomials <Integer>) -> ZKIF (almost)
        let a = lc_to_v(&lc_a, field); // return Variables {vec<u64>,vec<Integer>}
        let b = lc_to_v(&lc_b, field);
        let c = lc_to_v(&lc_c, field);

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

    println!("{:#?}", zkif_cs);

    return (zkif_header, zkif_cs, zkif_witness);

}


// circ Lc (const, monomials <Integer>) -> (Vec<u64>, Vec<Vec<u8>>) 
fn lc_to_v(lc: &Lc, field: &FieldT) -> Variables {
    
    let mut ids: Vec<u64> = Vec::new();
    let mut vars: Vec<Integer> = Vec::new();
 
    if lc.constant.i() != Integer::from(0 as u32) { // TODO: .cmp0() != Ordering::Equal
        ids.push(0);
        vars.push(lc.constant.i());
    }

    for (k,m) in &lc.monomials {
        ids.push((k+1) as u64);
        vars.push(m.i());
    }

    let vals = serialize(&vars, field);
    let opt = match vals.len() {
        0 => None,
        _ => Some(vals)
    };

    return Variables{
        variable_ids: ids, 
        values: opt,
    };
}

// all big Integers become little endian vecs of u8, then are all concatenated together
fn serialize(vals: &Vec<Integer>, f: &FieldT) -> Vec<u8> {

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

