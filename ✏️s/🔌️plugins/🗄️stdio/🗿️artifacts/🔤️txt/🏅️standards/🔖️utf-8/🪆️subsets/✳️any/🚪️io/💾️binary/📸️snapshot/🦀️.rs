//! binary rep for stdio.txt 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::snapshot::*;
use crate::STDIO_TXT_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactPack for TxtSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;

        let raw = self.to_body();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error|store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw.as_bytes()))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        let body = String::from_utf8(inner).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())))?;
        Ok(Self::from_body(&body))
    }
    /// 🧬️ The structural fingerprint `ArtifactCodec::pack_schema_hash` and the hub's trusted
    /// catalog pin this kind by. The `dsl::DslRecord` derive above already generates the spec —
    /// the same one `register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec)` publishes to the
    /// DSL registry — but a HAND-ROLLED `ArtifactPack` never picks up the derive's `record_spec`
    /// override the way `DslArtifact` does, so this kind silently answered the trait's `None`
    /// opt-out. That opt-out is what `codec.pack-schema-hash(stdio.txt)` reported as
    /// `artifact codec schema has no structural record specification`, and what the catalog
    /// builder refuses as the zero fingerprint (ticket 26/09/18 slice TC4). The body's raw
    /// line-joined pack encoding is unaffected: this hash fingerprints the SNAPSHOT RECORD's
    /// fields, not the pack container.
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
}
pub use snapshot_codec::*;
