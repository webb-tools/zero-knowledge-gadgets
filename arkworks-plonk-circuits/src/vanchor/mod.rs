// VAnchor is the variable deposit/withdraw/transfer shielded pool
// It supports join split transactions, meaning you can take unspent deposits
// in the pool and join them together, split them, and any combination
// of the two.

// The inputs to the VAnchor are unspent outputs we want to spend (we are spending the inputs),
// and we create outputs which are new, unspent UTXOs. We create commitments
// for each output and these are inserted into merkle trees.

// The VAnchor is also a bridged system. It takes as a public input
// a set of merkle roots that it will use to verify the membership
// of unspent deposits within. The VAnchor prevents double-spending
// through the use of a public input chain identifier `chain_id`.

// We will take inputs and do a merkle tree reconstruction for each input.
// Then we will verify that the reconstructed root from each input's
// membership path is within a set of public merkle roots.

use std::ops::Add;

use crate::{
	merkle_tree::{PatHadget, PathGadget}, vanchor::add_public_input_variable,
	poseidon::poseidon::FieldHasherGadget, set_membership::check_set_membership, mixer::add_public_input_variable,
};
use ark_ec::{models::TEModelParameters, PairingEngine};
use ark_ff::PrimeField;
use ark_std::{One, Zero};
use arkworks_gadgets::merkle_tree::simple_merkle::Path;
use plonk_core::{
	circuit::Circuit, constraint_system::StandardComposer, error::Error, prelude::Variable,
};
use arkworks_gadgets::poseidon::field_hasher::FieldHasher;

pub struct VariableAnchorCircuit<
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
	HG: FieldHasherGadget<F, P>,
    // Tree height
	const N: usize,
    // Size of the root set (bridge length)
	const M: usize,
    // Number of inputs
    const INS: usize,
    // Number of outputs
    const OUTS: usize,
> {
	// chain_id: F,
	// secret: F,
	// nullifier: F,
	// nullifier_hash: F,
	// path: Path<F, H::Native, N>,
	// roots: [F; M],
	// arbitrary_data: F,
	// hasher: H::Native,

    // INS = number of input transactions
    // OUTS = number of output transactions
    // N = Tree height
    // M = Size of the root set

    // sum of input amounts + public amount == sum of output amounts
    public_amount: F, // Public
    public_chain_id: F, // Public

    // Input transactions
    in_amounts: [F; INS],
    in_blindings: [F; INS],
    in_nullifier_hashes: [F; INS], // Public
    in_private_keys: [F; INS],
    in_paths: [Path<F, HG::Native, N>; INS],
    in_indices: [F; INS],
    in_root_set: [F; M],

    // Output transactions 
    out_amounts: [F; OUTS],
    out_blindings: [F; OUTS],
    out_chain_ids: [F; OUTS],
    out_public_keys: [F; OUTS],
    out_commitments: [F; OUTS], // Public

    // Arbitrary data to be added to the transcript
    arbitrary_data: F, // Public

    // All the hashers used in this circuit
    // Used for hashing private_key -- width 2
    public_key_hasher: HG::Native,
    // Used for hashing nodes in the tree -- width 3
    tree_hasher: HG::Native,
    // Used for creating leaf signature and the nullifier hash -- width 4
    signature_hasher: HG::Native,
    // Used for creating leaf -- width 5
    leaf_hasher: HG::Native
}

impl<F, P, HG, const N: usize, const M: usize, const INS: usize, const OUTS: usize> VariableAnchorCircuit<F, P, HG, N, M, INS, OUTS>
where
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
	HG: FieldHasherGadget<F, P>,
{
	pub fn new(
        public_amount: F,
        public_chain_id: F,
        in_amounts: [F; INS],
        in_blindings: [F; INS],
        in_nullifier_hashes: [F; INS],
        in_private_keys: [F; INS],
        in_paths: [Path<F, HG::Native, N>; INS],
        in_indices: [F; INS],
        in_root_set: [F; M],
        out_amounts: [F; OUTS],
        out_blindings: [F; OUTS],
        out_chain_ids: [F; OUTS],
        out_public_keys: [F; OUTS],
        out_commitments: [F; OUTS],
        arbitrary_data: F,
        public_key_hasher: HG::Native,
        tree_hasher: HG::Native,
        signature_hasher: HG::Native,
        leaf_hasher: HG::Native
	) -> Self {
		Self {
            public_amount,
            public_chain_id,
            in_amounts,
            in_blindings,
            in_nullifier_hashes,
            in_private_keys,
            in_paths,
            in_indices,
            in_root_set,
            out_amounts,
            out_blindings,
            out_chain_ids,
            out_public_keys,
            out_commitments,
            arbitrary_data,
            public_key_hasher,
            tree_hasher,
            signature_hasher,
            leaf_hasher,
		}
	}
}

impl<F, P, HG, const N: usize, const M: usize, const INS: usize, const OUTS: usize> Circuit<F, P> for VariableAnchorCircuit<F, P, HG, N, M, INS, OUTS>
where
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
	HG: FieldHasherGadget<F, P>,
{
	const CIRCUIT_ID: [u8; 32] = [0xff; 32];

	fn gadget(&mut self, composer: &mut StandardComposer<F, P>) -> Result<(), Error> {
        // Initialize public inputs
        let public_amount = add_public_input_variable(composer, self.public_amount);
        let public_chain_id = add_public_input_variable(composer, self.public_chain_id);
        let roots = self
            .in_root_set
            .iter()
            .map(|root| add_public_input_variable(composer, *root))
            .collect::<Vec<Variable>>();

        // Initialize hashers
        let pk_hasher_gadget: HG = FieldHasherGadget::<F, P>::from_native(composer, self.public_key_hasher.clone());
        let tree_hasher_gadget: HG = FieldHasherGadget::<F, P>::from_native(composer, self.tree_hasher.clone());
        let sig_hasher_gadget: HG = FieldHasherGadget::<F, P>::from_native(composer, self.signature_hasher.clone());
        let leaf_hasher_gadget: HG = FieldHasherGadget::<F, P>::from_native(composer, self.leaf_hasher.clone());

        // Sum of input amounts + public amount must equal output amounts at the end
        let mut input_sum = public_amount;
        let mut output_sum = composer.zero_var();

        // Storage for the nullifier hash variables as we allocate them
        let nullifier_hashes: Vec<Variable> = Vec::new();

        // General strategy
        // 1. Reconstruct the commitments (along the way reconstruct other values)
        // 2. Reconstruct the target merkle root with the input's merkle path
        // 3. Verify that the target merkle root is within the root set
        // 4. Sum the input amounts
        for i in 0..INS {
            // Private inputs for each input UTXO being spent
            let in_private_key_i = composer.add_input(self.in_private_keys[i]);
            let in_amount_i = composer.add_input(self.in_amounts[i]);
            let in_blinding_i = composer.add_input(self.in_blindings[i]);
            let in_index_i = composer.add_input(self.in_indices[i]);
            let path_gadget = PathGadget::<F, P, HG, N>::from_native(
                composer,
                self.in_paths[i].clone()
            );

            // Add public inputs for each input UTXO being spent
            let in_nullifier_hash_i = add_public_input_variable(composer, self.in_nullifier_hashes[i]);
            nullifier_hashes.push(in_nullifier_hash_i);

            // Computing the public key, which is done, just by hashing the private key
            let calc_public_key = pk_hasher_gadget.hash(composer, &[in_private_key_i])?;

            // Computing the leaf
            let calc_leaf = leaf_hasher_gadget.hash(composer, &[
                public_chain_id,
                in_amount_i,
                calc_public_key,
                in_blinding_i,
            ])?;

            // Computing the signature: sign(private_key, leaf, input_index)
            let calc_signature = sig_hasher_gadget.hash(composer, &[
                in_private_key_i,
                calc_leaf,
                in_index_i,
            ])?;

            // Computing the nullifier hash. This is used to prevent spending
            // already spent UTXOs.
            let calc_nullifier = sig_hasher_gadget.hash(composer, &[
                calc_leaf,
                in_index_i,
                calc_signature
            ])?;

            // Checking if the passed nullifier hash is the same as the calculated one
            // Optimized version of allocating public nullifier input and constraining
            // to the calculate one.
            let _ = composer.arithmetic_gate( |gate|
                gate.witness( 
                    calc_nullifier,
                    calc_nullifier,
                    Some(composer.zero_var())
                ).add(
                    - F::one(),
                    F::zero()
                ).pi(self.in_nullifier_hashes[i])
            );

            // Calculate the root hash
            let calc_root_hash = path_gadget.calculate_root(composer, &calc_leaf, &tree_hasher_gadget)?;

            // Check if calculated root hash is in the set
            // TODO: Check membership enabled is needed here.
            let is_member = check_set_membership(composer, roots, calc_root_hash);
            composer.constrain_to_constant(is_member, F::one(), None);
            
            // Finally add the amount to the sum
            // TODO: Investigate improvements to accumulating sums
            input_sum = composer.arithmetic_gate( |gate|
                gate.witness( 
                    input_sum,
                    in_amount_i,
                    None
                ).add(
                    F::one(),
                    F::one()
                )
            );
        }

        // Check all the nullifiers are unique to prevent double-spending
        // TODO: Investigate checking nullifier uniqueness this check to the application side
        for i in 0..INS {
            for j in (i + 1)..INS {
                let result = composer.is_eq_with_output(nullifier_hashes[i], nullifier_hashes[j]);
                composer.assert_equal(result, composer.zero_var());
            }
        }

        for i in 0..OUTS {
            let out_chain_id_i = composer.add_input(self.out_chain_ids[i]);
            let out_amount_i = composer.add_input(self.out_amounts[i]);
            let out_public_key_i = composer.add_input(self.out_public_keys[i]);
            let out_blinding_i = composer.add_input(self.out_blindings[i]);
            // Calculate the leaf commitment
            let calc_leaf = leaf_hasher_gadget.hash(composer, &[
                out_chain_id_i,
                out_amount_i,
                out_public_key_i,
                out_blinding_i,
            ])?;
            
            // Check if calculated leaf is the same as the passed one
            let _ = composer.arithmetic_gate( |gate|
                gate.witness( 
                    calc_leaf,
                    calc_leaf,
                    Some(composer.zero_var())
                ).add(
                    - F::one(),
                    F::zero()
                ).pi(self.out_commitments[i])
            );
            
            
            // Each amount should not be greater than the limit constant
            // TODO: Investigate if we can get the field size bits
            composer.range_gate(out_amount_i, 254);
            // assert_less_than(out_amounts[i], limit);
            
            // Add in to the sum
            output_sum = composer.arithmetic_gate( |gate|
                gate.witness( 
                    output_sum,
                    out_amount_i,
                    None
                ).add(
                    F::one(),
                    F::one()
                )
            );
        }

        composer.assert_equal(input_sum, output_sum);

		Ok(())
	}

	fn padded_circuit_size(&self) -> usize {
		1 << 21
	}
}

#[cfg(test)]
mod test {
	use super::VariableAnchorCircuit;
	use crate::{poseidon::poseidon::PoseidonGadget, utils::gadget_tester};
	use ark_bn254::{Bn254, Fr as Bn254Fr};
	use ark_ed_on_bn254::{EdwardsParameters as JubjubParameters, Fq};
	use ark_ff::Field;
	use ark_poly::polynomial::univariate::DensePolynomial;
	use ark_poly_commit::{
		kzg10::{self, UniversalParams},
		sonic_pc::{self, SonicKZG10},
		PolynomialCommitment,
	};
	use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
	use ark_std::test_rng;
	use arkworks_gadgets::{
		ark_std::UniformRand,
		merkle_tree::simple_merkle::SparseMerkleTree,
		poseidon::field_hasher::{FieldHasher, Poseidon},
	};
	use arkworks_utils::utils::common::{setup_params_x5_3, Curve};
	use plonk_core::{
		prelude::*,
		proof_system::{Prover, Verifier},
	};

	type PoseidonBn254 = Poseidon<Fq>;

	const BRIDGE_SIZE: usize = 2;
    const INS: usize = 2;
    const OUTS: usize = 2;

	#[test]
	fn should_verify_correct_anchor_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();

		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();

		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				nullifier,
				nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		let res = gadget_tester::<Bn254, JubjubParameters, _>(&mut anchor, 1 << 17);
		assert!(res.is_ok(), "{:?}", res.err().unwrap());
	}

	#[test]
	fn should_fail_with_invalid_root_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut root = tree.root();
		let bad_root = root.double();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = bad_root;

		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				nullifier,
				nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// Verify proof
		let res = verifier
			.verify(&proof, &vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}

	#[test]
	fn should_fail_with_wrong_secret_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();
		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// An incorrect secret value to use below
		let bad_secret = secret.double();

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				bad_secret,
				nullifier,
				nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254Fr> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// Verify proof
		let mut vk_bytes = Vec::new();
		sonic_pc::VerifierKey::<Bn254Fr>::serialize(&vk, &mut vk_bytes).unwrap();
		let kzg_vk = kzg10::VerifierKey::<Bn254Fr>::deserialize(&vk_bytes[..]).unwrap();
		let res = verifier
			.verify(&proof, &kzg_vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}

	#[test]
	fn should_fail_with_wrong_nullifier_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();

		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// An incorrect secret value to use below
		let bad_nullifier = nullifier.double();

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				bad_nullifier,
				nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254Fr> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// Verify proof
		let mut vk_bytes = Vec::new();
		sonic_pc::VerifierKey::<Bn254Fr>::serialize(&vk, &mut vk_bytes).unwrap();
		let kzg_vk = kzg10::VerifierKey::<Bn254Fr>::deserialize(&vk_bytes[..]).unwrap();
		let res = verifier
			.verify(&proof, &kzg_vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}

	#[test]
	fn should_fail_with_wrong_path_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();

		// An incorrect path to use below
		let bad_path = tree.generate_membership_proof((last_index as u64) - 1);

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				nullifier,
				nullifier_hash,
				bad_path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254Fr> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// Verify proof
		let mut vk_bytes = Vec::new();
		sonic_pc::VerifierKey::<Bn254Fr>::serialize(&vk, &mut vk_bytes).unwrap();
		let kzg_vk = kzg10::VerifierKey::<Bn254Fr>::deserialize(&vk_bytes[..]).unwrap();
		let res = verifier
			.verify(&proof, &kzg_vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}

	#[test]
	fn should_fail_with_wrong_nullifier_hash_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();

		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// Incorrect nullifier hash to use below
		let bad_nullifier_hash = nullifier_hash.double();

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				nullifier,
				bad_nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254Fr> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// Verify proof
		let mut vk_bytes = Vec::new();
		sonic_pc::VerifierKey::<Bn254Fr>::serialize(&vk, &mut vk_bytes).unwrap();
		let kzg_vk = kzg10::VerifierKey::<Bn254Fr>::deserialize(&vk_bytes[..]).unwrap();
		let res = verifier
			.verify(&proof, &kzg_vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}

	#[test]
	fn should_fail_with_wrong_arbitrary_data_plonk() {
		let rng = &mut test_rng();
		let curve = Curve::Bn254;

		let params = setup_params_x5_3(curve);
		let poseidon_native = PoseidonBn254 { params };

		// Randomly generated secrets
		let secret = Fq::rand(rng);
		let nullifier = Fq::rand(rng);

		// Public data
		let chain_id = Fq::from(1u32);
		let arbitrary_data = Fq::rand(rng);
		let nullifier_hash = poseidon_native.hash_two(&nullifier, &nullifier).unwrap();
		let leaf_hash = poseidon_native.hash_two(&secret, &nullifier).unwrap();

		// Create a tree whose leaves are already populated with 2^HEIGHT - 1 random
		// scalars, then add leaf_hash as the final leaf
		const HEIGHT: usize = 6usize;
		let last_index = 1 << (HEIGHT - 1) - 1;
		let mut leaves = [Fq::from(0u8); 1 << (HEIGHT - 1)];
		for i in 0..last_index {
			leaves[i] = Fq::rand(rng);
		}
		leaves[last_index] = leaf_hash;
		let tree = SparseMerkleTree::<Fq, PoseidonBn254, HEIGHT>::new_sequential(
			&leaves,
			&poseidon_native,
			&[0u8; 32],
		)
		.unwrap();
		let mut roots = [Fq::from(0u8); BRIDGE_SIZE];
		roots[0] = tree.root();

		// Path
		let path = tree.generate_membership_proof(last_index as u64);

		// Create VariableAnchorCircuit
		let mut anchor =
			VariableAnchorCircuit::<Bn254Fr, JubjubParameters, PoseidonGadget, HEIGHT, BRIDGE_SIZE, INS, OUTS>::new(
				chain_id,
				secret,
				nullifier,
				nullifier_hash,
				path,
				roots,
				arbitrary_data,
				poseidon_native,
			);

		// Fill a composer to extract the public_inputs
		let mut composer = StandardComposer::<Bn254Fr, JubjubParameters>::new();
		let _ = anchor.gadget(&mut composer);
		let mut public_inputs = composer.construct_dense_pi_vec();

		// Go through proof generation/verification
		let u_params: UniversalParams<Bn254Fr> =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 18, None, rng).unwrap();
		let proof = {
			// Create a prover struct
			let mut prover = Prover::<Bn254, JubjubParameters, SonicKZG10<Bn254, DensePolynomial<Bn254Fr>>>::new(b"vanchor");
			prover.key_transcript(b"key", b"additional seed information");
			// Add gadgets
			let _ = anchor.gadget(prover.mut_cs());
			// Commit Key (being lazy with error)
			let (ck, _) =
				SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
					.unwrap();
			// Preprocess circuit
			let _ = prover.preprocess(&ck.powers());
			// Compute Proof
			prover.prove(&ck.powers()).unwrap()
		};

		// Verifier's view

		// Create a Verifier object
		let mut verifier = Verifier::new(b"vanchor");
		verifier.key_transcript(b"key", b"additional seed information");
		// Add gadgets
		let _ = anchor.gadget(verifier.mut_cs());
		// Compute Commit and Verifier key
		let (ck, vk) =
			SonicKZG10::<Bn254, DensePolynomial<Bn254Fr>>::trim(&u_params, 1 << 18, 0, None)
				.unwrap();
		// Preprocess circuit
		verifier.preprocess(&ck.powers()).unwrap();

		// The arbitrary data is stored at index 5 of the public input vector:
		assert_eq!(arbitrary_data, public_inputs[7]);
		// Modify the arbitrary data so that prover/verifier disagree
		public_inputs[5].double_in_place();

		// Verify proof
		let mut vk_bytes = Vec::new();
		sonic_pc::VerifierKey::<Bn254Fr>::serialize(&vk, &mut vk_bytes).unwrap();
		let kzg_vk = kzg10::VerifierKey::<Bn254Fr>::deserialize(&vk_bytes[..]).unwrap();
		let res = verifier
			.verify(&proof, &kzg_vk, &public_inputs)
			.unwrap_err();
		match res {
			Error::ProofVerificationError => (),
			err => panic!("Unexpected error: {:?}", err),
		};
	}
}
