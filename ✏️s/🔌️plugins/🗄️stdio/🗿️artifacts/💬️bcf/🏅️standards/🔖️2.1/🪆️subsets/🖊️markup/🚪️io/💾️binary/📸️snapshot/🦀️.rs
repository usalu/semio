//! bcf rep for stdio.bcf 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v2_1::subsets::any::schema::snapshot::*;
use crate::STDIO_BCF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_framework_value::{ValueError, ValueRefusalKind};

impl store::ArtifactPack for BcfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let raw = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_1::subsets::any::schema::diff::*;
use crate::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::BcfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

/// 🌱 Full (non-diff) `BcfSnapshot` binary codec -- only `SetSnapshot`'s whole-payload encoding
/// needs this, mirroring `enc_bcf_snapshot`'s text form above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bcf_snapshot_bin(s: &BcfSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    write_str_lp(out, &s.version);
    store::pack_rt::write_varint_u64(out, s.topics.len() as u64);
    for t in &s.topics {
        enc_topic_bin(t, out);
    }
    store::pack_rt::write_varint_u64(out, s.parts.len() as u64);
    for p in &s.parts {
        enc_part_bin(p, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bcf_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let version = read_str_lp(reader)?;
    let topic_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut topics = Vec::with_capacity(topic_count as usize);
    for _ in 0..topic_count {
        topics.push(dec_topic_bin(reader)?);
    }
    let part_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut parts = Vec::with_capacity(part_count as usize);
    for _ in 0..part_count {
        parts.push(dec_part_bin(reader)?);
    }
    Ok(BcfSnapshot { schema, version, topics, parts })
}
}
pub use diff_codec::*;
