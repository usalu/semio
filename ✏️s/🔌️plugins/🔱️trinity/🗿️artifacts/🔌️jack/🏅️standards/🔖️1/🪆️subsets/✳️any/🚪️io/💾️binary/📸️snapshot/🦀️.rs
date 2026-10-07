//! 📦️ `trinity.graph` artifact — binary document surface + laws (constitutional: pack).
//!
//! 📌️ The `ArtifactPack` impl itself lives in `🗣️dsl/🦀️.rs`, next to the private
//! `JackSnapshotDsl` mirror it delegates through (same reason the DSL impl lives there too) — this
//! file only holds the public encode/decode entry points, matching the old bundle crate's shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::JackSnapshot;
use store::{ArtifactPack, PackError};

/// 📦️ Encodes a `JackSnapshot` to its binary pack form.
pub fn encode(document: &JackSnapshot) -> Vec<u8> {
    ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `JackSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<JackSnapshot, PackError> {
    <JackSnapshot as ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::JackSnapshot;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use store::ArtifactDsl;
use store::PackDecodeOptions;
use store::PackEncodeOptions;
use store::PackError;
pub(crate) use records::JackPackRecord;

impl store::ArtifactPack for JackSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&JackPackRecord::__dsl_spec(), &JackPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &JackPackRecord::__dsl_spec(), options)?;
        let mut snapshot = JackPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(PackError::from)?;
        crate::attach_bundled_content(&mut snapshot);
        Ok(snapshot)
    }

    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(JackPackRecord::__dsl_spec())
    }
}
}
