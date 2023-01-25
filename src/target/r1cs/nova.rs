#![allow(missing_docs)]
type G1 = pasta_curves::pallas::Point;
type G2 = pasta_curves::vesta::Point;
use crate::target::r1cs::bellman::*;
use crate::target::r1cs::R1cs;
use ::bellperson::{Circuit, ConstraintSystem, LinearCombination, SynthesisError, Variable};
use bincode::{deserialize_from, serialize_into};
// use ff::PrimeField;
use ff::{Field, PrimeField, PrimeFieldBits};
use fxhash::FxHashMap;
use gmp_mpfr_sys::gmp::limb_t;
use group::WnafGroup;
use log::debug;
use pairing::{Engine, MultiMillerLoop};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use rug::integer::{IsPrime, Order};
use rug::Integer;

use super::*;

/// Convert a (rug) integer to a prime field element.
fn int_to_ff<F: PrimeField>(i: Integer) -> F {
    let mut accumulator = F::from(0);
    let limb_bits = (std::mem::size_of::<limb_t>() as u64) << 3;
    let limb_base = F::from(2).pow_vartime([limb_bits]);
    // as_ref yeilds a least-significant-first array.
    for digit in i.as_ref().iter().rev() {
        accumulator *= limb_base;
        accumulator += F::from(*digit);
    }
    accumulator
}

/// Convert one our our linear combinations to a bellman linear combination.
/// Takes a zero linear combination. We could build it locally, but bellman provides one, so...
fn lc_to_bellman<F: PrimeField, CS: ConstraintSystem<F>>(
    vars: &HashMap<usize, Variable>,
    lc: &Lc,
    zero_lc: LinearCombination<F>,
) -> LinearCombination<F> {
    let mut lc_bellman = zero_lc;
    // This zero test is needed until https://github.com/zkcrypto/bellman/pull/78 is resolved
    if !lc.constant.is_zero() {
        lc_bellman = lc_bellman + (int_to_ff((&lc.constant).into()), CS::one());
    }
    for (v, c) in &lc.monomials {
        // ditto
        if !c.is_zero() {
            lc_bellman = lc_bellman + (int_to_ff(c.into()), *vars.get(v).unwrap());
        }
    }
    lc_bellman
}

// hmmm... this should work essentially all the time, I think
fn get_modulus<F: Field + PrimeField>() -> Integer {
    let neg_1_f = -F::one();
    let p_lsf: Integer = Integer::from_digits(neg_1_f.to_repr().as_ref(), Order::Lsf) + 1;
    let p_msf: Integer = Integer::from_digits(neg_1_f.to_repr().as_ref(), Order::Msf) + 1;
    if p_lsf.is_probably_prime(30) != IsPrime::No {
        p_lsf
    } else if p_msf.is_probably_prime(30) != IsPrime::No {
        p_msf
    } else {
        panic!("could not determine ff::Field byte order")
    }
}

pub fn circ_to_bellperson<F: PrimeField>(
    circ_r1cs: R1cs<String>,
    circ_values: Option<FxHashMap<String, Value>>,
) -> Vec<LinearCombination<F>> {
    let f_mod = get_modulus::<F>();
    assert_eq!(
        circ_r1cs.modulus.modulus(),
        &f_mod,
        "\nR1CS has modulus \n{},\n but Bellman CS expects \n{}",
        circ_r1cs.modulus,
        f_mod
    );
    let mut uses = HashMap::with_capacity(circ_r1cs.next_idx);
    for (a, b, c) in circ_r1cs.constraints.iter() {
        [a, b, c].iter().for_each(|y| {
            y.monomials.keys().for_each(|k| {
                uses.get_mut(k)
                    .map(|i| {
                        *i += 1;
                    })
                    .or_else(|| {
                        uses.insert(*k, 1);
                        None
                    });
            })
        });
    }

    //let mut vars = HashMap::with_capacity(circ_r1cs.next_idx);
    for i in 0..circ_r1cs.next_idx {
        if let Some(s) = circ_r1cs.idxs_signals.get(&i) {
            //for (_i, s) in circ_r1cs.idxs_signals.get() {
            let public = circ_r1cs.public_idxs.contains(&i);
            if uses.get(&i).is_some() || public {
                let name_f = || s.to_string();
                let val_f = || {
                    Ok({
                        let i_val = circ_values
                            .as_ref()
                            .expect("missing values")
                            .get(s)
                            .unwrap();
                        let ff_val: F = int_to_ff(i_val.as_pf().into());
                        debug!("value : {} -> {:?} ({})", s, ff_val, i_val);
                        ff_val
                    })
                };
                debug!("var: {}, public: {}", s, public);

                /*
                let v = if public {
                    cs.alloc_input(name_f, val_f)?
                } else {
                    cs.alloc(name_f, val_f)?
                };
                */

                // vars.insert(i, v);
            } else {
                debug!("drop dead var: {}", s);
            }
        }
    }
    /*
    for (i, (a, b, c)) in circ_r1cs.constraints.iter().enumerate() {
        cs.enforce(
            || format!("con{}", i),
            |z| lc_to_bellman::<F, CS>(&vars, a, z),
            |z| lc_to_bellman::<F, CS>(&vars, b, z),
            |z| lc_to_bellman::<F, CS>(&vars, c, z),
        );
    }

    debug!(
        "done with synth: {} vars {} cs",
        vars.len(),
        circ_r1cs.constraints.len()
    );
    */

    let lcs: Vec<LinearCombination<F>> = Vec::new();

    return lcs;
}
