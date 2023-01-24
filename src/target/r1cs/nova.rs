#![allow(missing_docs)]
use structopt::StructOpt;
type G1 = pasta_curves::pallas::Point;
type G2 = pasta_curves::vesta::Point;
use crate::target::r1cs::bellman::*;
use crate::target::r1cs::R1cs;
use ::bellperson::{gadgets::num::AllocatedNum, SynthesisError};
use ff::PrimeField;
use nova_snark::{
    traits::{
        circuit::{StepCircuit, TrivialTestCircuit},
        Group,
    },
    CompressedSNARK, PublicParams, RecursiveSNARK,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct DFAStepWitness<F: PrimeField> {
    x_i: F,
    x_i_plus_1: F,
}

impl<F: PrimeField> DFAStepWitness<F> {
    // sample witness
    fn new(x_0: &F) -> (Vec<F>, Self) {
        //Vec<Self>) {
        //let mut vars = Vec::new();

        //let mut hash_i = *hash_0;
        let mut x_i = *x_0;

        // note in final version, we will likely do many iters per step
        let x_i_plus_1 = x_i * x_i;

        //vars.push(
        let vars = Self { x_i, x_i_plus_1 };

        //x_i = x_i_plus_1;

        let z_0 = vec![*x_0];

        (z_0, vars)
    }
}

#[derive(Clone, Debug)]
struct DFAStepCircuit<F: PrimeField, S> {
    wit: DFAStepWitness<F>,
    r1cs: R1cs<S>,
}

impl<F, S> DFAStepCircuit<F>
where
    F: PrimeField,
{
    fn new(num_steps: usize, x0: F, circ_r1cs: R1cs<S>) -> (Vec<F>, Vec<Self>) {
        let z0 = vec![x0];
        let mut circuits = Vec::new();
        let (mut zi, mut dfa_witness) = DFAStepWitness::new(&x0);
        let circuit = DFAStepCircuit {
            wit: dfa_witness.clone(),
            r1cs: circ_r1cs.clone(),
        };
        // println!("{:#?}", circuit);
        circuits.push(circuit);

        for i in 1..num_steps {
            (zi, dfa_witness) = DFAStepWitness::new(&dfa_witness.x_i_plus_1);

            let circuit = DFAStepCircuit {
                wit: dfa_witness.clone(),
                r1cs: circ_r1cs.clone(),
            };
            // println!("{:#?}", circuit);
            circuits.push(circuit);
        }

        (z0, circuits)
    }

    // helper methods here (?)
}

// sample F: x_{i+1} = x_i * x_i
impl<F> StepCircuit<F> for DFAStepCircuit<F>
where
    F: PrimeField,
{
    // return # inputs or outputs of each step
    // synthesize() and output() take as input and output a vec of len = arity()
    fn arity(&self) -> usize {
        1
    }

    // make circuit for a computation step
    // return variable corresponding to output of step z_{i+1}
    fn synthesize<CS: bellperson::ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<F>],
    ) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
        // TODO check modulus

        let mut uses = HashMap::with_capacity(self.r1cs.next_idx);
        for (a, b, c) in self.r1cs.constraints.iter() {
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

        let mut z_out: Result<Vec<AllocatedNum<F>>, SynthesisError> =
            Err(SynthesisError::AssignmentMissing);

        // let mut hash_i = hash_0;
        // let mut x_i = z[0].clone();;

        // non deterministic advice
        let x_i = AllocatedNum::alloc(cs.namespace(|| format!("x_i")), || Ok(self.wit.x_i))?;
        let x_i_plus_1 = AllocatedNum::alloc(cs.namespace(|| format!("x_i_plus_1")), || {
            Ok(self.wit.x_i_plus_1)
        })?;

        // check conditions hold:
        // x_i_plus_1 = x_i^2

        cs.enforce(
            || format!("x_i * x_i = x_i_plus_1"),
            |lc| lc + CS::one() + CS::one(),
            |lc| lc + CS::one() + CS::one(),
            |lc| lc + CS::one() + CS::one() + CS::one() + CS::one(),
        );

        //println!("TESTING {:#?}", cs);

        // return hash(x_i_plus_1, ...TODO) since Nova circuits expect a single output
        z_out = Ok(vec![x_i_plus_1.clone()]); // # outputs??

        // update x_i and hash_i for the next iteration
        // x_i = x_i_plus_1;

        z_out
    }

    // return output of step when provided with step's input
    fn output(&self, z: &[F]) -> Vec<F> {
        // sanity check
        debug_assert_eq!(z[0], self.wit.x_i);

        // compute output using advice
        vec![self.wit.x_i_plus_1]
    }
}
/*
pub fn ark_mat_to_nova<F>(mat: ark_relations::r1cs::Matrix<F>) {
    println!("SCALAR {:#?}", type_of(&<G1 as Group>::Scalar::one()));
    for cs in mat {
        // rows = constraints
        for col in cs {
            let num = col.0;
            let slot = col.1;
        }
    }

    /*println!("mat len {:#?}", mat.len());
    for row in mat {
        println!("len row {:#?}", row.len());
        for col in row {
            println!("{:#?}", col.1);
        }
        println!("\n");
    }
    */
}
*/
