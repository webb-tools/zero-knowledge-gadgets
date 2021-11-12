use ark_crypto_primitives::crh::{constraints::CRHGadget, CRH};
use ark_ff::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::SynthesisError;

use ark_ff::Field;
use core::fmt::Debug;

use crate::leaf::{LeafCreation, VanchorLeafCreation};

pub trait LeafCreationGadget<F: Field, H: CRH, HG: CRHGadget<H, F>, L: LeafCreation<H>>:
	Sized
{
	type LeafVar: EqGadget<F>
		+ ToBytesGadget<F>
		+ CondSelectGadget<F>
		+ AllocVar<L::Leaf, F>
		+ R1CSVar<F>
		+ Debug
		+ Clone
		+ Sized;

	type NullifierVar: EqGadget<F>
		+ ToBytesGadget<F>
		+ CondSelectGadget<F>
		+ AllocVar<L::Nullifier, F>
		+ R1CSVar<F>
		+ Debug
		+ Clone
		+ Sized;

	type PrivateVar: AllocVar<L::Private, F> + Clone;
	type PublicVar: AllocVar<L::Public, F> + Clone;

	fn create_leaf(
		s: &Self::PrivateVar,
		p: &Self::PublicVar,
		h: &HG::ParametersVar,
	) -> Result<Self::LeafVar, SynthesisError>;

	fn create_nullifier(
		s: &Self::PrivateVar,
		h: &HG::ParametersVar,
	) -> Result<Self::NullifierVar, SynthesisError>;
}

pub trait VanchorLeafCreationGadget<
	F: PrimeField,
	H2: CRH,
	HG2: CRHGadget<H2, F>,
	H4: CRH,
	HG4: CRHGadget<H4, F>,
	H5: CRH,
	HG5: CRHGadget<H5, F>,
	L: VanchorLeafCreation<F, H2, H4, H5>,
>: Sized
{
	type LeafVar: EqGadget<F>
		+ ToBytesGadget<F>
		+ CondSelectGadget<F>
		+ AllocVar<L::Leaf, F>
		+ R1CSVar<F>
		+ Debug
		+ Clone
		+ Sized;

	type NullifierVar: EqGadget<F>
		+ ToBytesGadget<F>
		+ CondSelectGadget<F>
		+ AllocVar<L::Nullifier, F>
		+ R1CSVar<F>
		+ Debug
		+ Clone
		+ Sized;

	type PrivateVar: AllocVar<L::Private, F> + Clone;
	type PublicVar: AllocVar<L::Public, F> + Clone;

	fn create_leaf(
		s: &Self::PrivateVar,
		p: &Self::PublicVar,
		h_w2: &HG2::ParametersVar,
		h_w5: &HG5::ParametersVar,
	) -> Result<Self::LeafVar, SynthesisError>;

	fn create_nullifier(
		s: &Self::PrivateVar,
		c: &Self::LeafVar,
		h_w4: &HG4::ParametersVar,
		indices: &FpVar<F>,
	) -> Result<Self::NullifierVar, SynthesisError>;

	fn get_private_key(s: &Self::PrivateVar) -> Result<FpVar<F>, SynthesisError>;
	fn get_amount(s: &Self::PrivateVar) -> Result<FpVar<F>, SynthesisError>;
	fn get_blinding(s: &Self::PrivateVar) -> Result<FpVar<F>, SynthesisError>;

	fn gen_public_key(
		s: &Self::PrivateVar,
		h_w2: &HG2::ParametersVar,
	) -> Result<HG2::OutputVar, SynthesisError>;

	fn get_chain_id(p: &Self::PublicVar) -> Result<FpVar<F>, SynthesisError>;
}
