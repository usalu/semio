//! binary rep for stdio.dxf 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::snapshot::*;
use crate::STDIO_DXF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::standards::v_r12::subsets::any::io::text::snapshot as snapshot_text;

impl store::ArtifactPack for DxfSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let raw = store::pack_rt::encode_document(&snapshot_text::spec(), &snapshot_text::to_record(self), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, report) = store::pack_rt::decode_document(&inner, &snapshot_text::spec(), options)?;
        if report.schema_drift || !report.unknown_field_ids.is_empty() { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DXF snapshot record schema mismatch"))); }
        snapshot_text::from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(snapshot_text::spec()) }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_block_bin, dec_dxf_entities_bin, dec_header_var_bin, enc_block_bin, enc_dxf_entities_bin, enc_header_var_bin, read_str_lp, write_str_lp};
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_other_table_bin, dec_other_table_bin, enc_dxf_tables_bin, dec_dxf_tables_bin};
}
pub use diff_codec::*;
