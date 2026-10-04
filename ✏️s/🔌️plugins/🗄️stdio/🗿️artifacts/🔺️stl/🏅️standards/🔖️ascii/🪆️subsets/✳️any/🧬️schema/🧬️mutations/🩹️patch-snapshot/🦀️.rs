//! 🩹️ Path-scoped `StlMutation` snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot
//! sub-schema at the pointer (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::StlMutation;
use crate::StlSnapshot;
use crate::schema::diff::StlDiff;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: StlSnapshot, mutation: StlMutation, diff: StlDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/stl/ascii/any/snapshot.json" }
