//! binary rep for stdio.binary 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_raw::subsets::any::schema::snapshot::*;
use crate::STDIO_BINARY_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 🧬️ CARRIER LAW (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3):
/// `s.stdio.binary@raw/*` is `CARRIER_BINARY` — its native `Binary` `IoPayload` IS the raw
/// external file content, byte-for-byte. The previous impl wrapped `self.bytes` in a
/// `SemioEnvelope` (`BINARY_MAGIC` header + token), which made every exported `.bin` file an
/// unopenable `.semio` pack container instead of the honest raw bytes — exactly the
/// `registry_export_media` class of bug the ticket exists to remove. Fixed here (the codec, not
/// the test): `encode_pack_with`/`decode_pack_with` are now the identity function on `bytes`.
/// Proven by `carrier_native_is_raw` in `🚪️io/🦀️.rs`.
impl store::ArtifactPack for BinarySnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        Ok(self.bytes.clone())
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let _ = options;
        Ok(Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: bytes.to_vec() })
    }
}
}
pub use snapshot_codec::*;
