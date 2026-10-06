//! binary rep for stdio.step 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::snapshot::*;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
use crate::engine::brep::{BrepFace, BrepMesh, BrepVertex};

impl store::ArtifactPack for StepSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as semio_framework_os_kernel::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { native::encode_pack(self, options) }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { native::decode_pack(bytes, options) }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::schema::snapshot::StepComplexType;
use crate::schema::snapshot::StepEntity;
use crate::schema::snapshot::StepFileDescription;
use crate::schema::snapshot::StepFileName;
use crate::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::StepValue;

/// 📸️ Full `StepSnapshot` binary codec — needed by `SetSnapshot`'s `OpBinary` (mutations file
/// imports this `pub(crate)`), never by `StepDiff` itself (no `snapshot: Option<StepSnapshot>`
/// full-replace slot exists on the diff), same split [`enc_step_snapshot`]/[`dec_step_snapshot`]
/// (the TEXT twin) uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_step_snapshot_bin(s: &StepSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_file_description_bin(&s.header.file_description, out);
    enc_file_name_bin(&s.header.file_name, out);
    enc_file_schema_bin(&s.header.file_schema, out);
    store::pack_rt::write_varint_u64(out, s.entities.len() as u64);
    for e in &s.entities {
        enc_entity_bin(e, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_step_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<StepSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let file_description = dec_file_description_bin(reader)?;
    let file_name = dec_file_name_bin(reader)?;
    let file_schema = dec_file_schema_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let entities = (0..count).map(|_| dec_entity_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(StepSnapshot { schema, header: crate::schema::snapshot::StepHeader { file_description, file_name, file_schema }, entities })
}
}
pub use diff_codec::*;
