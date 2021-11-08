use ark_crypto_primitives::{crh::CRH, Error};
use ark_ff::{bytes::ToBytes, PrimeField};
use ark_std::{hash::Hash, rand::Rng};

pub mod basic;
pub mod bridge;
pub mod mixer;
pub mod vanchor;

#[cfg(feature = "r1cs")]
pub mod constraints;
#[cfg(feature = "r1cs")]
pub use constraints::*;

pub trait LeafCreation<H: CRH>: Sized {
	type Leaf: ToBytes + Clone + Eq + core::fmt::Debug + Hash + Default;
	type Nullifier: ToBytes + Clone + Eq + core::fmt::Debug + Hash + Default;
	type Private: Clone + Default;
	type Public: Clone + Default;

	fn generate_secrets<R: Rng>(r: &mut R) -> Result<Self::Private, Error>;
	fn create_leaf(
		s: &Self::Private,
		p: &Self::Public,
		h: &H::Parameters,
	) -> Result<Self::Leaf, Error>;
	fn create_nullifier_hash(
		s: &Self::Private,
		h: &H::Parameters,
	) -> Result<Self::Nullifier, Error>;
}

pub trait VanchorLeafCreation<H: CRH, F: PrimeField>: Sized {
	type Leaf: ToBytes + Clone + Eq + core::fmt::Debug + Hash + Default;
	type Private: Clone;
	type Public: Clone + Default;
	type Nullifier: ToBytes + Clone + Eq + core::fmt::Debug + Hash + Default;

	fn generate_secrets<R: Rng>(r: &mut R) -> Result<Self::Private, Error>;
	fn create_leaf(
		s: &Self::Private,
		p: &Self::Public,
		h: &H::Parameters,
	) -> Result<Self::Leaf, Error>;

	fn create_nullifier(
		s: &Self::Private,
		c: &Self::Leaf,
		h: &H::Parameters,
		f: &F,
	) -> Result<Self::Nullifier, Error>;

	fn get_private_key(s: &Self::Private) -> Result<F, Error>;

	fn gen_public_key(s: &Self::Private, h: &H::Parameters) -> Result<H::Output, Error>;
}
