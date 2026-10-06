//! 💾️ Binary representation codec surface for `stdio.semio.value` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::value::io::text::snapshot::{dec_semio_value_snapshot};
use crate::standards::v1::subsets::value::io::text::snapshot::{enc_semio_value_snapshot};
impl store::ArtifactPack for SemioValueSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = enc_semio_value_snapshot(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        let text = std::str::from_utf8(&inner).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
        dec_semio_value_snapshot(text).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::value::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioValueSnapshot`] as a semio pack envelope — the binary twin of the DSL text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_value_pack(snapshot: &SemioValueSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioValueSnapshot`] — the inverse of
/// [`encode_semio_value_pack`], reading
/// `../../../../✉️base/📚️examples/🕸️graph/🖼️assets/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_value_pack(bytes: &[u8]) -> Result<SemioValueSnapshot, String> {
    <SemioValueSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
