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
use generic_array::typenum;
use gmp_mpfr_sys::gmp::limb_t;
use log::debug;
use neptune::{
    circuit::poseidon_hash,
    poseidon::{Arity, HashMode, Poseidon, PoseidonConstants},
    Strength,
};
use nova_snark::{
    traits::{circuit::StepCircuit, Group},
    StepCounterType,
};
use rug::integer::{IsPrime, Order};
use rug::Integer;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn string_lc(pd: &ProverData, lc: &Lc) -> String {
    let mut s = vec![];
    if !lc.is_zero() {
        s.push(format!("{:#?}", lc.constant.i()));
        for (idx, coef) in &lc.monomials {
            s.push(format!(
                "({:#?} * {:#?})",
                coef.i(),
                pd.r1cs.idxs_signals.get(&idx).unwrap()
            ));
        }
    }
    s.join(" + ")
}

pub fn print_r1cs(pd: &ProverData) {
    for (a, b, c) in pd.r1cs.constraints() {
        println!(
            "[ {:#?} * {:#?} = {:#?} ]",
            string_lc(pd, a),
            string_lc(pd, b),
            string_lc(pd, c)
        );
    }
}

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
    prev_hash: F,
    next_hash: F,
    round_num: F,
    next_round_num: u64,
    pc: PoseidonConstants<F, typenum::U2>,
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
        hash_i: F,
        hash_i_plus_1: F,
        round_i: F,
        round_i_plus_1: u64,
        pcs: PoseidonConstants<F, typenum::U2>,
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
            prev_hash: hash_i,
            next_hash: hash_i_plus_1,
            round_num: round_i,
            next_round_num: round_i_plus_1,
            pc: pcs,
        };

        return circuit;
    }
}

impl<F> StepCircuit<F> for DFAStepCircuit<F>
where
    F: PrimeField,
{
    fn arity(&self) -> usize {
        5 // TODO CHECK
    }

    fn output(&self, z: &[F]) -> Vec<F> {
        // sanity check
        assert_eq!(z[0], self.current_state);
        assert_eq!(z[1], self.current_char);
        assert_eq!(z[2], self.prev_hash);
        assert_eq!(z[3], self.round_num);

        // expected hash value (witness)
        //let pc = PoseidonConstants::<F, typenum::U2>::new_with_strength(Strength::Standard);
        let mut fr_data = vec![self.prev_hash, self.current_char];
        let mut p = Poseidon::<F, typenum::U2>::new_with_preimage(&fr_data, &self.pc);
        let expected_hash: F = p.hash();

        vec![
            self.next_state,
            self.next_char,
            expected_hash,
            self.round_num + F::from(1),
        ]
    }

    fn get_counter_type(&self) -> StepCounterType {
        StepCounterType::External
    }

    // nova wants this to return the "output" of each step
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
        println!("SYN");
        // inputs
        let current_char = z[0].clone();
        let current_state = z[1].clone();
        let prev_hash = z[2].clone();
        let round_num = z[3].clone();

        // ouputs
        let mut next_state = None;
        //let mut next_hash = None;
        //let mut next_round_num = None;
        let mut bool_out = None;

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
                            //println!("value : {} -> {:?} ({})", s, ff_val, i_val);
                            ff_val
                        })
                    };
                    println!("var: {}, public: {}", s, public);
                    //let v = if public {
                    // cs.alloc_input(name_f, val_f)?
                    //let v = cs.alloc(name_f, val_f)?; // we don't care what circ thinks is public,
                    // as we are lying to circ anyway
                    //} else {
                    //cs.alloc(name_f, val_f)?
                    //};

                    // inputs
                    if s.starts_with("char") {
                        let alloc_v = current_char.clone(); //AllocatedNum::alloc(cs.namespace(name_f), val_f)?;
                                                            //assert_eq!(val_f, current_char); //current_char = Some(alloc_v); //.get_variable();
                        vars.insert(i, alloc_v.get_variable());
                    } else if s.starts_with("current_state") {
                        let alloc_v = current_state.clone(); //AllocatedNum::alloc(cs.namespace(name_f), val_f)?;
                                                             //assert_eq!(val_f, current_state); //current_state = alloc_v.get_variable();
                        vars.insert(i, alloc_v.get_variable());
                    } else if s.starts_with("round_num") {
                        let alloc_v = round_num.clone(); //AllocatedNum::inputize(cs.namespace(name_f), val_f)?;
                                                         //assert_eq!(val_f, round_num); //round_num = alloc_v.get_variable();
                        vars.insert(i, alloc_v.get_variable());
                    // outputs
                    } else if s.starts_with("next_state") {
                        let alloc_v = AllocatedNum::alloc(cs.namespace(name_f), val_f)?;
                        next_state = Some(alloc_v); //.get_variable();
                        vars.insert(i, next_state.clone().unwrap().get_variable());
                    } else if s.starts_with("bool_out") {
                        let alloc_v = AllocatedNum::alloc(cs.namespace(name_f), val_f)?;

                        alloc_v.inputize(cs.namespace(|| "output bool"))?;
                        bool_out = Some(alloc_v); //.get_variable();
                        vars.insert(i, bool_out.clone().unwrap().get_variable());

                    // intermediate (in circ) wits
                    } else {
                        let alloc_v = AllocatedNum::alloc(cs.namespace(name_f), val_f)?;
                        let v = alloc_v.get_variable();
                        vars.insert(i, v);
                    }
                } else {
                    println!("drop dead var: {}", s);
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

            /*            let z = LinearCombination::zero();
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
            */
        }

        // https://github.com/zkcrypto/bellman/blob/2759d930622a7f18b83a905c9f054d52a0bbe748/src/gadgets/num.rs,
        // line 31 ish

        // for nova passing (new inputs from prover)
        let next_char = AllocatedNum::alloc(
            cs.namespace(|| format!("char_{}", self.next_round_num)),
            || Ok(self.next_char),
        )?;
        let next_round_num = AllocatedNum::alloc(
            cs.namespace(|| format!("round_num_{}", self.next_round_num)),
            || Ok(self.round_num + F::from(1)),
        )?;
        next_round_num
            .inputize(cs.namespace(|| format!("output_round_num_{}", self.next_round_num)));

        // circuit poseidon
        let data: Vec<AllocatedNum<F>> = vec![
            prev_hash, //.unwrap(), //AllocatedNum::alloc(cs.namespace(|| "prev_hash"), || Ok(self.current_char)).unwrap(),
            current_char,
        ];

        let next_hash = poseidon_hash(cs, data, &self.pc); //.expect("poseidon hashing failed");

        /*
        println!(
            "expected {:#?}, out {:#?}",
            expected,
            next_hash?.clone().get_value(), //.unwrap()
        );
        */

        // assert_eq!(expected, out.get_value().unwrap()); //get_value().unwrap());

        debug!(
            "done with synth: {} vars {} cs",
            vars.len(),
            self.constraints.len()
        );

        Ok(vec![
            next_state.unwrap(),
            next_char,
            next_hash?,
            next_round_num,
            bool_out.unwrap(),
        ])
    }
}
