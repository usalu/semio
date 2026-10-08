//! binary rep for stdio.step 📸️snapshot
use crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native;
use crate::standards::v_ap214::subsets::base::io::binary::diff::{write_str_bin,read_str_bin,enc_file_description_bin,dec_file_description_bin,enc_file_name_bin,dec_file_name_bin,enc_file_schema_bin,dec_file_schema_bin,enc_entity_bin,dec_entity_bin};

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

}
pub use diff_codec::*;
