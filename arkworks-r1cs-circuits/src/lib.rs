#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
pub extern crate ark_std;

use ark_std::boxed::Box;

pub mod anchor;
pub mod basic;
pub mod mixer;
pub mod poseidon;
pub mod vanchor;

pub type Error = Box<dyn ark_std::error::Error>;

pub mod prelude {
	pub use ark_crypto_primitives;
	pub use ark_ff;
	pub use ark_std;
}
