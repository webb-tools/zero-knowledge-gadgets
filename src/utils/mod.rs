#[cfg(feature = "poseidon_bn254_x5_3")]
pub mod bn254_x5_3;
#[cfg(feature = "poseidon_bn254_x5_3")]
pub mod bn254_x5_3_result;
#[cfg(feature = "poseidon_bn254_x5_5")]
pub mod bn254_x5_5;
#[cfg(feature = "poseidon_bn254_x5_5")]
pub mod bn254_x5_5_result;

#[cfg(feature = "poseidon_bls381_x5_3")]
pub mod bls381_x5_3;
#[cfg(feature = "poseidon_bls381_x5_5")]
pub mod bls381_x5_5;

use crate::Vec;
use ark_crypto_primitives::Error;
use ark_ff::{fields::PrimeField, BigInteger};
use ark_r1cs_std::{fields::fp::FpVar, prelude::*, uint8::UInt8};
use ark_relations::r1cs::SynthesisError;

pub fn to_field_elements<F: PrimeField>(bytes: &[u8]) -> Result<Vec<F>, Error> {
	let max_size_bytes = F::BigInt::NUM_LIMBS * 8;
	let res = bytes
		.chunks(max_size_bytes)
		.map(|chunk| F::read(chunk))
		.collect::<Result<Vec<_>, _>>()?;

	Ok(res)
}

pub fn to_field_var_elements<F: PrimeField>(
	bytes: &[UInt8<F>],
) -> Result<Vec<FpVar<F>>, SynthesisError> {
	let max_size = F::BigInt::NUM_LIMBS * 8;
	let res = bytes
		.chunks(max_size)
		.map(|chunk| Boolean::le_bits_to_fp_var(chunk.to_bits_le()?.as_slice()))
		.collect::<Result<Vec<_>, SynthesisError>>()?;

	Ok(res)
}

pub fn decode_hex(s: &str) -> Vec<u8> {
	let s = &s[2..];
	let vec: Vec<u8> = (0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
		.collect();

	vec
}

pub fn get_bytes_array_from_hex(hex_str: &str) -> [u8; 32] {
	let bytes = decode_hex(hex_str);
	let mut result: [u8; 32] = [0; 32];
	result.copy_from_slice(&bytes);
	result
}

pub fn parse_vec<F: PrimeField>(arr: Vec<&str>) -> Vec<F> {
	let mut res = Vec::new();
	for r in arr.iter() {
		let c = F::from_be_bytes_mod_order(&get_bytes_array_from_hex(r));
		res.push(c);
	}
	res
}

pub fn parse_matrix<F: PrimeField>(mds_entries: Vec<Vec<&str>>) -> Vec<Vec<F>> {
	let width = mds_entries.len();
	let mut mds: Vec<Vec<F>> = vec![vec![F::zero(); width]; width];
	for i in 0..width {
		for j in 0..width {
			// TODO: Remove unwrap, handle error
			mds[i][j] = F::from_be_bytes_mod_order(&get_bytes_array_from_hex(mds_entries[i][j]));
		}
	}
	mds
}

#[cfg(feature = "poseidon_bn254_x5_5")]
pub fn get_results_poseidon_bn254_x5_5<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_5_result::RESULT.to_vec())
}

#[cfg(feature = "poseidon_bn254_x5_3")]
pub fn get_results_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_3_result::RESULT.to_vec())
}

#[cfg(feature = "poseidon_bn254_x5_5")]
pub fn get_rounds_poseidon_bn254_x5_5<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_5::ROUND_CONSTS.to_vec())
}

#[cfg(feature = "poseidon_bn254_x5_3")]
pub fn get_rounds_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_3::ROUND_CONSTS.to_vec())
}

#[cfg(feature = "poseidon_bn254_x5_5")]
pub fn get_mds_poseidon_bn254_x5_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(
		bn254_x5_5::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}

#[cfg(feature = "poseidon_bn254_x5_3")]
pub fn get_mds_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(
		bn254_x5_3::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}

#[cfg(feature = "poseidon_bls381_x5_5")]
pub fn get_rounds_poseidon_bls381_x5_5<F: PrimeField>() -> Vec<F> {
	parse_vec(bls381_x5_5::ROUND_CONSTS.to_vec())
}

#[cfg(feature = "poseidon_bls381_x5_3")]
pub fn get_rounds_poseidon_bls381_x5_3<F: PrimeField>() -> Vec<F> {
	parse_vec(bls381_x5_3::ROUND_CONSTS.to_vec())
}

#[cfg(feature = "poseidon_bls381_x5_5")]
pub fn get_mds_poseidon_bls381_x5_5<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(
		bls381_x5_5::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}

#[cfg(feature = "poseidon_bls381_x5_3")]
pub fn get_mds_poseidon_bls381_x5_3<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(
		bls381_x5_3::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}
