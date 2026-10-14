//! binary rep for stdio.step 📸️snapshot
use crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::snapshot::*;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::standards::v_ap214::subsets::base::schema::snapshot::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
use crate::standards::v_ap214::subsets::base::io::brep::{BrepFace, BrepMesh, BrepVertex};

impl pack::snapshot::ArtifactPack for StepSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { native::encode_pack(self, options) }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { native::decode_pack(bytes, options) }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for StepSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as semio_framework_os_kernel::os_store::serialization::ArtifactSqliteSnapshotHost>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepComplexType;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepEntity;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileDescription;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileName;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepValue;

}
