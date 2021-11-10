use super::{Private, Public};
use crate::Vec;
use ark_crypto_primitives::{crh::CRHGadget, CRH};
use ark_ff::fields::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{Namespace, SynthesisError};
use ark_std::marker::PhantomData;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct PrivateVar<F: PrimeField> {
	r: FpVar<F>,
	nullifier: FpVar<F>,
}

impl<F: PrimeField> PrivateVar<F> {
	pub fn new(r: FpVar<F>, nullifier: FpVar<F>) -> Self {
		Self { r, nullifier }
	}
}

#[derive(Clone)]
pub struct PublicVar<F: PrimeField> {
	chain_id: FpVar<F>,
}

impl<F: PrimeField> PublicVar<F> {
	pub fn new(chain_id: FpVar<F>) -> Self {
		Self { chain_id }
	}
}

pub struct BridgeLeafGadget<F: PrimeField, H: CRH, HG: CRHGadget<H, F>> {
	private: PrivateVar<F>,
	public: PublicVar<F>,
	hasher: PhantomData<H>,
	hasher_gadget: PhantomData<HG>,
}

impl<F: PrimeField, H: CRH, HG: CRHGadget<H, F>> BridgeLeafGadget<F, H, HG> {
	pub fn new(private: PrivateVar<F>, public: PublicVar<F>) -> Self {
		Self {
			private,
			public,
			hasher: PhantomData,
			hasher_gadget: PhantomData,
		}
	}

	pub fn create_leaf(&self, h: &HG::ParametersVar) -> Result<HG::OutputVar, SynthesisError> {
		// leaf
		let mut leaf_bytes = Vec::new();
		leaf_bytes.extend(self.private.r.to_bytes()?);
		leaf_bytes.extend(self.private.nullifier.to_bytes()?);
		leaf_bytes.extend(self.public.chain_id.to_bytes()?);
		HG::evaluate(h, &leaf_bytes)
	}

	pub fn create_nullifier(&self, h: &HG::ParametersVar) -> Result<HG::OutputVar, SynthesisError> {
		let mut nullifier_hash_bytes = Vec::new();
		nullifier_hash_bytes.extend(self.private.nullifier.to_bytes()?);
		nullifier_hash_bytes.extend(self.private.nullifier.to_bytes()?);
		HG::evaluate(h, &nullifier_hash_bytes)
	}
}

impl<F: PrimeField> AllocVar<Private<F>, F> for PrivateVar<F> {
	fn new_variable<T: Borrow<Private<F>>>(
		into_ns: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let private = f()?.borrow().clone();
		let ns = into_ns.into();
		let cs = ns.cs();

		let secret = private.secret;
		let nullifier = private.nullifier;

		let secret_var = FpVar::new_variable(cs.clone(), || Ok(secret), mode)?;
		let nullifier_var = FpVar::new_variable(cs, || Ok(nullifier), mode)?;

		Ok(PrivateVar::new(secret_var, nullifier_var))
	}
}

impl<F: PrimeField> AllocVar<Public<F>, F> for PublicVar<F> {
	fn new_variable<T: Borrow<Public<F>>>(
		cs: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let public = f()?.borrow().clone();
		let chain_id = FpVar::new_variable(cs, || Ok(public.chain_id), mode)?;
		Ok(PublicVar::new(chain_id))
	}
}

#[cfg(feature = "default_poseidon")]
#[cfg(test)]
mod test {
	use super::*;
	use crate::{
		leaf::bridge::BridgeLeaf,
		poseidon::{
			constraints::{CRHGadget, PoseidonParametersVar},
			sbox::PoseidonSbox,
			PoseidonParameters, Rounds, CRH,
		},
		utils::{get_mds_poseidon_bls381_x5_5, get_rounds_poseidon_bls381_x5_5},
	};
	use ark_bls12_381::Fq;
	use ark_ff::One;
	use ark_r1cs_std::R1CSVar;
	use ark_relations::r1cs::ConstraintSystem;
	use ark_std::test_rng;

	#[derive(Default, Clone)]
	struct PoseidonRounds5;

	impl Rounds for PoseidonRounds5 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 60;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 5;
	}

	type PoseidonCRH5 = CRH<Fq, PoseidonRounds5>;
	type PoseidonCRH5Gadget = CRHGadget<Fq, PoseidonRounds5>;

	type Leaf = BridgeLeaf<Fq, PoseidonCRH5>;
	type LeafGadget = BridgeLeafGadget<Fq, PoseidonCRH5, PoseidonCRH5Gadget>;
	#[test]
	fn should_create_bridge_leaf_constraints() {
		let rng = &mut test_rng();

		let cs = ConstraintSystem::<Fq>::new_ref();

		// Native version
		let rounds = get_rounds_poseidon_bls381_x5_5::<Fq>();
		let mds = get_mds_poseidon_bls381_x5_5::<Fq>();
		let params = PoseidonParameters::<Fq>::new(rounds, mds);
		let chain_id = Fq::one();

		let public = Public::new(chain_id);
		let secrets = Private::generate(rng);
		let leaf = Leaf::new(secrets.clone(), public.clone());
		let leaf_hash = leaf.create_leaf(&params).unwrap();
		let nullifier = leaf.create_nullifier(&params).unwrap();

		// Constraints version
		let params_var = PoseidonParametersVar::new_variable(
			cs.clone(),
			|| Ok(&params),
			AllocationMode::Constant,
		)
		.unwrap();

		let public_var = PublicVar::new_input(cs.clone(), || Ok(&public)).unwrap();
		let secrets_var = PrivateVar::new_witness(cs.clone(), || Ok(&secrets)).unwrap();
		let leaf_gadget = LeafGadget::new(secrets_var, public_var);
		let leaf_hash_var = leaf_gadget.create_leaf(&params_var).unwrap();
		let nullifier_var = leaf_gadget.create_nullifier(&params_var).unwrap();

		// Checking equality
		let leaf_new_var = FpVar::<Fq>::new_witness(cs.clone(), || Ok(&leaf_hash)).unwrap();
		let nullifier_new_var = FpVar::<Fq>::new_witness(cs, || Ok(&nullifier)).unwrap();
		let leaf_res = leaf_hash_var.is_eq(&leaf_new_var).unwrap();
		let nullifier_res = nullifier_var.is_eq(&nullifier_new_var).unwrap();
		assert!(leaf_res.value().unwrap());
		assert!(leaf_res.cs().is_satisfied().unwrap());
		assert!(nullifier_res.value().unwrap());
		assert!(nullifier_res.cs().is_satisfied().unwrap());
	}
}
