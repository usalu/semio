//! 🩹️ Path-scoped `TsvMutation` snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot
//! sub-schema at the pointer (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::TsvMutation;
use crate::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use crate::standards::iana::subsets::any::schema::diff::TsvDiff;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: TsvSnapshot, mutation: TsvMutation, diff: TsvDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/tsv/iana/any/snapshot.json" }
