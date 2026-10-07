//! 🌱️ Immutable decoded artifact genesis and its exact authoritative Pack bytes.

use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
use std::sync::Arc;

/// 📦️ Lower-layer binary genesis capability, independent of Store registration and transports.
pub trait ArtifactGenesisCodec: Sized {
    fn encode_genesis_pack(&self) -> Result<Vec<u8>, ValueError>;
    fn decode_genesis_pack(pack: &[u8]) -> Result<Self, ValueError>;
}

#[derive(Debug, PartialEq)]
pub struct ArtifactGenesis<P> {
    snapshot: Arc<P>,
    pack: Arc<Vec<u8>>,
    digest: [u8; 32],
}

impl<P> Clone for ArtifactGenesis<P> {
    fn clone(&self) -> Self { Self { snapshot: Arc::clone(&self.snapshot), pack: Arc::clone(&self.pack), digest: self.digest } }
}

impl<P> ArtifactGenesis<P> {
    pub fn snapshot(&self) -> &P { &self.snapshot }
    pub fn share_snapshot(&self) -> Arc<P> { Arc::clone(&self.snapshot) }
    pub fn pack(&self) -> &[u8] { &self.pack }
    pub fn share_pack(&self) -> Arc<Vec<u8>> { Arc::clone(&self.pack) }
    pub fn digest(&self) -> [u8; 32] { self.digest }
    pub(crate) fn from_decoded_pack(snapshot: P, pack: Vec<u8>) -> Self {
        let digest = *semio_framework_hash::hash(&pack).as_bytes();
        Self::from_verified_pack(snapshot, pack, digest)
    }
    /// 🪪️ Transfers a decoded snapshot and its exact Pack from a generation-fenced admission that already verified the supplied digest.
    pub fn from_verified_pack(snapshot: P, pack: Vec<u8>, digest: [u8; 32]) -> Self {
        Self { snapshot: Arc::new(snapshot), pack: Arc::new(pack), digest }
    }
    pub(crate) fn into_owners(self) -> (Arc<P>, Arc<Vec<u8>>) { (self.snapshot, self.pack) }
}

impl<P: ArtifactGenesisCodec> ArtifactGenesis<P> {
    pub fn born(snapshot: P) -> Result<Self, ValueError> {
        let pack = snapshot.encode_genesis_pack()?;
        Ok(Self::from_decoded_pack(snapshot, pack))
    }
    pub fn from_stored_pack(pack: Vec<u8>) -> Result<Self, ValueError> {
        let snapshot = P::decode_genesis_pack(&pack)?;
        Ok(Self::from_decoded_pack(snapshot, pack))
    }
}

impl<P> ToValue for ArtifactGenesis<P> {
    fn to_value(&self) -> DslValue {
        use std::fmt::Write;
        let mut hex = String::with_capacity(self.pack.len().saturating_mul(2));
        for byte in self.pack.iter() { write!(&mut hex, "{byte:02x}").expect("writing bytes into a String is infallible"); }
        DslValue::String(hex)
    }
}

impl<P: ArtifactGenesisCodec> FromValue for ArtifactGenesis<P> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let hex = String::from_value(value)?;
        if hex.is_empty() || hex.len() & 1 != 0 || hex.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "initial Pack requires an even nonempty hex string")); }
        let pack = hex.as_bytes().chunks_exact(2).map(|pair| {
            let text = std::str::from_utf8(pair).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "initial Pack has invalid hex"))?;
            u8::from_str_radix(text, 16).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "initial Pack has invalid hex"))
        }).collect::<Result<Vec<_>, _>>()?;
        Self::from_stored_pack(pack)
    }
}
