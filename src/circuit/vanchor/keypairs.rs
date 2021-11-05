use crate::leaf::{newleaf};
use ark_crypto_primitives::CRH;
use ark_ff::{fields::PrimeField, to_bytes};
use ark_std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct Keypairs<H: CRH, F: PrimeField> {
	pubkey: <H as CRH>::Output,
	data: PhantomData<F>,
}
impl<H: CRH, F: PrimeField> Keypairs<H, F> {
	pub fn public_key(h: &H::Parameters, secrets: &newleaf::Private<F>) -> Self {
		let bytes = to_bytes![secrets.priv_key].unwrap();
		let pubkey = H::evaluate(&h, &bytes).unwrap();
		Keypairs {
			pubkey,
			data: PhantomData,
		}
	}
}


#[cfg(feature = "default_poseidon")]
#[cfg(test)]
mod test {
	use crate::{circuit::vanchor::keypairs::Keypairs, leaf::{newleaf::NewLeaf,NewLeafCreation}, poseidon::{
			
			sbox::PoseidonSbox,
			PoseidonParameters, Rounds, CRH,
		}, utils::{get_mds_poseidon_bls381_x5_5, get_rounds_poseidon_bls381_x5_5}};
	use ark_bls12_381::Fq;
	use ark_crypto_primitives::crh::{CRH as CRHTrait};
	use ark_ff::to_bytes;

	use ark_relations::r1cs::ConstraintSystem;
	use ark_std::test_rng;
	#[derive(Default, Clone)]
	struct PoseidonRounds3;

	impl Rounds for PoseidonRounds3 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 57;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 4;
	}

	type PoseidonCRH3 = CRH<Fq, PoseidonRounds3>;

	type Leaf = NewLeaf<Fq, PoseidonCRH3>;
	#[test]
	fn should_crate_new_public_key() {
		let rng = &mut test_rng();

		let rounds = get_rounds_poseidon_bls381_x5_5::<Fq>();
		let mds = get_mds_poseidon_bls381_x5_5::<Fq>();
		let params = PoseidonParameters::<Fq>::new(rounds, mds);

		let secrets = Leaf::generate_secrets(rng).unwrap();
		let privkey = to_bytes![secrets.priv_key].unwrap();
		let pubkey = PoseidonCRH3::evaluate(&params, &privkey).unwrap();

		let keypairs = Keypairs::<PoseidonCRH3,Fq>::public_key(&params, &secrets);
		let new_pubkey = keypairs.pubkey;
		
		assert_eq!(new_pubkey,pubkey)
	}
}