//! 🩹️ Path-scoped `PlyMutation` snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot
//! sub-schema at the pointer (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::PlyMutation;
use crate::PlySnapshot;
use crate::schema::diff::PlyDiff;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: PlySnapshot, mutation: PlyMutation, diff: PlyDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/ply/1.0/any/snapshot.json" }
