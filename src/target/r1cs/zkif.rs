//! Export circ R1cs to zkinterface CS
use crate::target::r1cs::*;
use rug::{integer::Order, Integer};
use zkinterface::{BilinearConstraint, CircuitHeader, ConstraintSystem, Variables, Witness};

/// circ R1cs -> zkif
pub fn r1cs_to_zkif(
    prover_data: &ProverData,
    inputs_map: &HashMap<String, Value>,
    field: &FieldT,
) -> (CircuitHeader, ConstraintSystem, Witness) {
    let mut inp: Vec<(u64, Integer)> = Vec::new();
    let mut wit: Vec<(u64, Integer)> = Vec::new();

    let values = eval_inputs(inputs_map, prover_data);

    for (sig, idx) in &prover_data.r1cs.signal_idxs {
        //let name = r1cs.idxs_signals.get(k).unwrap().to_string();

        let v = match values.get(&sig.to_string()) {
            Some(v) => v,
            None => panic!("Input/witness variable does not have matching evaluation"),
        };

        let fi = match v.sort() {
            Sort::Field(_) => v.as_pf().i(),
            _ => panic!("Value should be a field"),
        };

        if prover_data.r1cs.public_idxs.contains(&idx) {
            // input
            //println!("As public io: {}", name);

            inp.push(((idx + 1) as u64, fi));
        } else {
            // witness
            //println!("As private witness: {}", name);

            wit.push(((idx + 1) as u64, fi));
        }
    }

    // todo where do constants go??

    assert_eq!(inp.len() + wit.len(), prover_data.r1cs.next_idx);
    inp.sort_by(|a, b| a.0.cmp(&b.0));
    wit.sort_by(|a, b| a.0.cmp(&b.0));

    let (pub_ids, pub_vars): (Vec<u64>, Vec<Integer>) = inp.into_iter().unzip();
    let (priv_ids, priv_vars): (Vec<u64>, Vec<Integer>) = wit.into_iter().unzip();

    //println!("{:#?}, {:#?}", pub_ids, pub_vars);
    //println!("{:#?}, {:#?}", priv_ids, priv_vars);

    //let field = &FieldT::from(DFL_T.modulus().clone());

    // instance (public io)
    let zkif_header = CircuitHeader {
        instance_variables: Variables {
            variable_ids: pub_ids,                     // vec of public ids
            values: Some(serialize(&pub_vars, field)), // vals of public ids
        },
        free_variable_id: (prover_data.r1cs.next_idx as u64) + 1, // TODO ?
        field_maximum: Some(serialize(
            &vec![(field.modulus() - Integer::from(1))],
            field,
        )), // TODO - largest elt of finit field (feild order - 1)
        configuration: None,                                      // optional
    };

    //println!("{:#?}", zkif_header);

    // witness
    let zkif_witness = Witness {
        assigned_variables: Variables {
            // private vars
            variable_ids: priv_ids,
            values: Some(serialize(&priv_vars, field)),
        },
    };

    //println!("{:#?}", zkif_witness);

    // put circuit together, witness = (1, private)
    let mut constraints_vec: Vec<BilinearConstraint> = Vec::new();

    for (lc_a, lc_b, lc_c) in prover_data.r1cs.constraints.iter() {
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

    //println!("{:#?}", zkif_cs);

    return (zkif_header, zkif_cs, zkif_witness);
}

fn eval_inputs(
    inputs_map: &HashMap<String, Value>,
    prover_data: &ProverData,
) -> HashMap<String, Value> {
    for (input, sort) in &prover_data.precompute_inputs {
        let value = inputs_map
            .get(input)
            .unwrap_or_else(|| panic!("No input for {}", input));
        let sort2 = value.sort();
        assert_eq!(
            sort, &sort2,
            "Sort mismatch for {}. Expected\n\t{} but got\n\t{}",
            input, sort, sort2
        );
    }
    let new_map = prover_data.precompute.eval(inputs_map);
    prover_data.r1cs.check_all(&new_map);

    new_map
}

// circ Lc (const, monomials <Integer>) -> (Vec<u64>, Vec<Vec<u8>>)
fn lc_to_v(lc: &Lc, field: &FieldT) -> Variables {
    let mut ids: Vec<u64> = Vec::new();
    let mut vars: Vec<Integer> = Vec::new();

    if lc.constant.i() != Integer::from(0 as u32) {
        // TODO: .cmp0() != Ordering::Equal
        ids.push(0);
        vars.push(lc.constant.i());
    }

    for (k, m) in &lc.monomials {
        ids.push((k + 1) as u64);
        vars.push(m.i());
    }

    let vals = serialize(&vars, field);
    let opt = match vals.len() {
        0 => None,
        _ => Some(vals),
    };

    return Variables {
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
        while vec.len() < elt_bits {
            vec.push(0 as u8);
        }
        if vec.len() > elt_bits {
            panic!("Value too big for field");
        }

        lil.append(&mut vec);
    }

    return lil;
}
