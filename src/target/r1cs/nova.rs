#![allow(missing_docs)]
type G1 = pasta_curves::pallas::Point;
type G2 = pasta_curves::vesta::Point;
use super::*;
use crate::target::r1cs::R1cs;
use ::bellperson::{
    gadgets::num::AllocatedNum, ConstraintSystem, LinearCombination, SynthesisError, Variable,
};
use circ_fields::FieldT;
use ff::{Field, PrimeField};
use fxhash::FxHashMap;
use fxhash::FxHasher;
use gmp_mpfr_sys::gmp::limb_t;
use log::debug;
//use nova_snark::poseidon::*;
use nova_snark::traits::{circuit::StepCircuit, Group};
use rug::integer::{IsPrime, Order};
use rug::Integer;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

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

#[derive(Clone, Debug)]
pub struct DFAStepCircuit<F: PrimeField> {
    modulus: FieldT,
    idxs_signals: HashMap<usize, String, BuildHasherDefault<FxHasher>>,
    next_idx: usize,
    public_idxs: HashSet<usize>,
    constraints: Vec<(Lc, Lc, Lc)>,
    vals: Option<FxHashMap<String, Value>>,
    current_state: F,
    current_char: F,
    next_state: F,
    next_char: F,
    //    round: usize,
}

// note that this will generate a single round, and no witnesses, unlike nova example code
// witness and loops will happen at higher level as to put as little as possible deep in circ
impl<F: PrimeField> DFAStepCircuit<F> {
    pub fn new(
        r1cs: &R1cs<String>,
        wits: Option<FxHashMap<String, Value>>,
        state_i: F,
        char_i: F,
        state_i_plus_1: F,
        char_i_plus_1: F,
    ) -> Self {
        // todo check wits line up with the non det advice

        let circuit = DFAStepCircuit {
            modulus: r1cs.modulus.clone(),
            idxs_signals: r1cs.idxs_signals.clone(),
            next_idx: r1cs.next_idx,
            public_idxs: r1cs.public_idxs.clone(),
            constraints: r1cs.constraints.clone(),
            vals: wits,
            current_state: state_i,
            current_char: char_i,
            next_state: state_i_plus_1,
            next_char: char_i_plus_1,
        };

        return circuit;
    }
}

impl<F: PrimeField> StepCircuit<F> for DFAStepCircuit<F> {
    fn arity(&self) -> usize {
        2
    }

    fn output(&self, z: &[F]) -> Vec<F> {
        // sanity check
        assert_eq!(z[0], self.current_state);
        assert_eq!(z[1], self.current_char);

        vec![self.next_state, self.next_char]
    }

    // nova wants this to return the "output" of each step, meaning alloc'ed `next_state` and
    // `next_char`
    fn synthesize<CS>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<F>],
    ) -> Result<Vec<AllocatedNum<F>>, SynthesisError>
    where
        CS: ConstraintSystem<F>,
        G1: Group<Base = <G2 as Group>::Scalar>,
        G2: Group<Base = <G1 as Group>::Scalar>,
    {
        //let state_i = z[0].clone();
        //let char_i = z[1].clone();
        // incorp into circuit?

        let f_mod = get_modulus::<F>(); // TODO

        assert_eq!(
            self.modulus.modulus(),
            &f_mod,
            "\nR1CS has modulus \n{},\n but Nova CS expects \n{}",
            self.modulus,
            f_mod
        );

        let mut uses = HashMap::with_capacity(self.next_idx);
        for (a, b, c) in self.constraints.iter() {
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
        let mut vars = HashMap::with_capacity(self.next_idx);
        for i in 0..self.next_idx {
            if let Some(s) = self.idxs_signals.get(&i) {
                //for (_i, s) in self.0.idxs_signals.get() {
                let public = self.public_idxs.contains(&i);
                if uses.get(&i).is_some() || public {
                    let name_f = || s.to_string();
                    let val_f = || {
                        Ok({
                            let i_val = self.vals.as_ref().expect("missing values").get(s).unwrap();
                            let ff_val = int_to_ff(i_val.as_pf().into());
                            debug!("value : {} -> {:?} ({})", s, ff_val, i_val);
                            ff_val
                        })
                    };
                    debug!("var: {}, public: {}", s, public);
                    let v = if public {
                        cs.alloc_input(name_f, val_f)?
                    } else {
                        cs.alloc(name_f, val_f)?
                    };
                    vars.insert(i, v);
                } else {
                    debug!("drop dead var: {}", s);
                }
            }
        }
        for (i, (a, b, c)) in self.constraints.iter().enumerate() {
            cs.enforce(
                || format!("con{}", i),
                |z| lc_to_bellman::<F, CS>(&vars, a, z),
                |z| lc_to_bellman::<F, CS>(&vars, b, z),
                |z| lc_to_bellman::<F, CS>(&vars, c, z),
            );

            let z = LinearCombination::zero();
            println!(
                "i= {:#?}, a= {:#?} -> {:#?}, b= {:#?} -> {:#?}, c= {:#?} -> {:#?}",
                i,
                a,
                lc_to_bellman::<F, CS>(&vars, a, z.clone()),
                b,
                lc_to_bellman::<F, CS>(&vars, b, z.clone()),
                c,
                lc_to_bellman::<F, CS>(&vars, c, z.clone()),
            );
        }

        //let pc_constants = ROConstants::<G1>::new();
        //let num_absorbs = 2;

        // let mut poseidon: ROConstantsCircuit<G2> = ROConstantsCircuit::<G2>::new();

        // let mut test_cs: SatisfyingAssignment<G> = SatisfyingAssignment::new();
        /*
        let num = F::random(&mut OsRng);
        let i = 0;
        let num_gadget =
            AllocatedNum::alloc(cs.namespace(|| format!("data {}", i)), || Ok(num)).unwrap();
        num_gadget
            .inputize(&mut cs.namespace(|| format!("input {}", i)))
            .unwrap();
        poseidon.absorb(num_gadget);

        let num_chal_bits = 128;
        let out = poseidon.squeeze(&mut cs, 128).unwrap();
        */
        println!("CIRC CS {:#?}", self.constraints);
        debug!(
            "done with synth: {} vars {} cs",
            vars.len(),
            self.constraints.len()
        );

        let next_state = AllocatedNum::alloc(cs.namespace(|| format!("next_state")), || {
            Ok(self.next_state)
        })?; // idk if we should pull this from cs
        let next_char =
            AllocatedNum::alloc(cs.namespace(|| format!("next_char")), || Ok(self.next_char))?;
        Ok(vec![next_state, next_char])
    }
}
