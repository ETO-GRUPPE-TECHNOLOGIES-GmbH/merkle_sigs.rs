#![deny(
    missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications
)]

//! `merkle_sigs` implements Merkle signatures in Rust.

#![deny(
    // missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications
)]

#[doc(no_inline)]
extern crate lamport_sigs;
extern crate merkle;
extern crate ring;

mod signatures;

pub use merkle::Proof;

pub use lamport_sigs::PublicKey;
pub use signatures::{
    MerklePublicKey,
    MerkleSignature,
    MerkleSignedData,
    verify_data_vec_signature,
    sign_data_vec
};

#[cfg(test)]
mod tests;
