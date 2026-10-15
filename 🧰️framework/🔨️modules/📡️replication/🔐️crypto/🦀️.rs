//! 🔐️ Crypto ports for signed protocol frames below wire encoding.
#![allow(async_fn_in_trait)]

use semio_framework_replication_error::ProtocolError;

/// 🔗️ Content-hashes raw bytes into a 32-byte digest (the commit chain's hash primitive).
pub trait RecordHasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32];
}

/// ✍️ Produces a detached signature over a 32-byte message (a commit's `chain_hash`).
pub trait Signer {
    async fn scheme(&self) -> &str;
    async fn key_id(&self) -> &str;
    async fn sign(&self, message: &[u8; 32]) -> Result<Vec<u8>, ProtocolError>;
}

/// ✅️ Verifies a detached signature produced by some `Signer`.
pub trait SignatureVerifier {
    async fn verify(&self, scheme: &str, key_id: &str, message: &[u8; 32], signature: &[u8]) -> Result<bool, ProtocolError>;
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
