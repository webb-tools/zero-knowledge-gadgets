use super::{NewLeaf, Private, Public};
use crate::{
	leaf::{NewLeafCreation, NewLeafCreationGadget},
	Vec,
};
use ark_crypto_primitives::{crh::CRHGadget, CRH};
use ark_ff::fields::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{Namespace, SynthesisError};
use ark_std::marker::PhantomData;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct PrivateVar<F: PrimeField> {
	amount: FpVar<F>,
	blinding: FpVar<F>,
	priv_key: FpVar<F>,
	indices: Vec<FpVar<F>>,
}

#[derive(Clone, Default)]
pub struct PublicVar<F: PrimeField> {
	f: PhantomData<F>,
}

impl<F: PrimeField> PrivateVar<F> {
	pub fn new( amount: FpVar<F>, blinding: FpVar<F>,
		priv_key: FpVar<F>, indices: Vec<FpVar<F>>) -> Self {
		Self {amount, blinding, priv_key, indices }
	}
}

pub struct NewLeafGadget<F: PrimeField, H1: CRH, HG1: CRHGadget<H1, F>, L: NewLeafCreation<H1>> {
	field: PhantomData<F>,
	hasher1: PhantomData<H1>,
	hasher_gadget1: PhantomData<HG1>,
	leaf_creation: PhantomData<L>,
}

impl<F: PrimeField, H1: CRH, HG1: CRHGadget<H1, F>> NewLeafCreationGadget<F, H1, HG1 , NewLeaf<F, H1>>// TODO: Change
	for NewLeafGadget<F, H1, HG1, NewLeaf<F, H1>>// TODO: Change
{
	type LeafVar = HG1::OutputVar;
	type PrivateVar = PrivateVar<F>;
	type PublicVar = PublicVar<F>;

	fn create_leaf(
		s: &Self::PrivateVar,
		_: &Self::PublicVar,
		h: &HG1::ParametersVar,
	) -> Result<Self::LeafVar, SynthesisError> {
		let mut bytes = Vec::new();
		bytes.extend(s.amount.to_bytes()?);
		bytes.extend(s.blinding.to_bytes()?);
		bytes.extend(s.indices[0].to_bytes()?); // TODO: extend to all indices
		HG1::evaluate(h, &bytes)
	}
}

impl<F: PrimeField> AllocVar<Private<F>, F> for PrivateVar<F> { // Todo: change to accept more than one 
	fn new_variable<T: Borrow<Private<F>>>(
		into_ns: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let secrets = f()?.borrow().clone();
		let ns = into_ns.into();
		let cs = ns.cs();

		let amount = secrets.amount;
		let blinding = secrets.blinding;
		let priv_key = secrets.priv_key;
		let indices = secrets.indices;
		
		let amount_var=FpVar::new_variable(cs.clone(), || Ok(amount), mode)?;
		let blinding_var=FpVar::new_variable(cs.clone(), || Ok(blinding), mode)?;
		let priv_key_var=FpVar::new_variable(cs.clone(), || Ok(priv_key), mode)?;
		let indice_var=FpVar::new_witness(cs.clone(), || {	Ok(indices[0])
		})?;
		Ok(PrivateVar::new(amount_var,blinding_var,priv_key_var,vec![indice_var]))
	}

fn new_constant(
        cs: impl Into<Namespace<F>>,
        t: impl Borrow<Private<F>>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, || Ok(t), AllocationMode::Constant)
    }

fn new_input<T: Borrow<Private<F>>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, f, AllocationMode::Input)
    }

fn new_witness<T: Borrow<Private<F>>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
    ) -> Result<Self, SynthesisError> {
        Self::new_variable(cs, f, AllocationMode::Witness)
    }
}

impl<F: PrimeField> AllocVar<Public<F>, F> for PublicVar<F> {
	fn new_variable<T: Borrow<Public<F>>>(
		_: impl Into<Namespace<F>>,
		_: impl FnOnce() -> Result<T, SynthesisError>,
		_: AllocationMode,
	) -> Result<Self, SynthesisError> {
		Ok(PublicVar::default())
	}
}

#[cfg(feature = "default_poseidon")]
#[cfg(test)]
mod test {
	use super::*;
	use crate::{
		poseidon::{
			constraints::{CRHGadget, PoseidonParametersVar},
			sbox::PoseidonSbox,
			PoseidonParameters, Rounds, CRH,
		},
		utils::{get_mds_poseidon_bls381_x5_3, get_rounds_poseidon_bls381_x5_3},
	};
	use ark_bls12_381::Fq;
	use ark_relations::r1cs::ConstraintSystem;
	use ark_std::test_rng;

	#[derive(Default, Clone)]
	struct PoseidonRounds3;

	impl Rounds for PoseidonRounds3 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 57;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 3;
	}

	#[derive(Default, Clone)]
	struct PoseidonRounds3_1;

	impl Rounds for PoseidonRounds3_1 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 57;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 4;
	}

	type PoseidonCRH3 = CRH<Fq, PoseidonRounds3>;
	type PoseidonCRH3Gadget = CRHGadget<Fq, PoseidonRounds3>;

	type Leaf = NewLeaf<Fq, PoseidonCRH3,>; // TODO: Change
	type LeafGadget = NewLeafGadget<Fq, PoseidonCRH3, PoseidonCRH3Gadget, Leaf>;
	#[test]
	fn should_crate_basic_leaf_constraints() {
		let rng = &mut test_rng();

		let cs = ConstraintSystem::<Fq>::new_ref();

		let rounds = get_rounds_poseidon_bls381_x5_3::<Fq>();
		let mds = get_mds_poseidon_bls381_x5_3::<Fq>();

		// Native version
		let public = Public::default();
		let secrets = Leaf::generate_secrets(rng).unwrap();
		let params = PoseidonParameters::<Fq>::new(rounds, mds);
		let leaf = Leaf::create_leaf(&secrets, &public, &params).unwrap();

		// Constraints version
		let public_var = PublicVar::default();
		let secrets_var = PrivateVar::new_witness(cs.clone(), || Ok(&secrets)).unwrap();
		let params_var =
			PoseidonParametersVar::new_variable(cs, || Ok(&params), AllocationMode::Constant)
				.unwrap();

		let leaf_var = LeafGadget::create_leaf(&secrets_var, &public_var, &params_var).unwrap();

		// Check equality
		let leaf_new_var = FpVar::<Fq>::new_witness(leaf_var.cs(), || Ok(leaf)).unwrap();
		let res = leaf_var.is_eq(&leaf_new_var).unwrap();
		assert!(res.value().unwrap());
		assert!(res.cs().is_satisfied().unwrap());
	}
}