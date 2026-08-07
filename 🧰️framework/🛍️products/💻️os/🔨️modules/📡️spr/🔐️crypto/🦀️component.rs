//! 🔐 Crypto trait seam for signed protocol frames.

use crate::os_spr::wire::ProtocolError;
//#region 🔖️Crypto
// Trait-only — no algorithm ships in protocol_core (repo rule: external libs behind an
// interface). protocol_format provides a Blake3Hasher impl of RecordHasher (it already owns the
// blake3 dep); Signer/SignatureVerifier have zero impls in this family — supplied by the
// integration layer or protocol_cli's optional feature-gated tooling.

/// @emoji 🔗️ Content-hashes raw bytes into a 32-byte digest (the commit chain's hash primitive).
pub trait RecordHasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32];
}

/// @emoji ✍️ Produces a detached signature over a 32-byte message (a commit's `chain_hash`).
pub trait Signer {
    fn scheme(&self) -> &str;
    fn key_id(&self) -> &str;
    fn sign(&self, message: &[u8; 32]) -> Result<Vec<u8>, ProtocolError>;
}

/// @emoji ✅️ Verifies a detached signature produced by some `Signer`.
pub trait SignatureVerifier {
    fn verify(&self, scheme: &str, key_id: &str, message: &[u8; 32], signature: &[u8]) -> Result<bool, ProtocolError>;
}
//#endregion 🔖️Crypto

