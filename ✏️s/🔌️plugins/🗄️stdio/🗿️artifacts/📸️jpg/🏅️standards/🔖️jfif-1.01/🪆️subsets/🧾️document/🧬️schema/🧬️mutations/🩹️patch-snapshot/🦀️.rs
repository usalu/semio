//! 🩹️ Path-scoped JPEG snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::JpgMutation;
use crate::schema::diff::JpgDiff;
use crate::JpgSnapshot;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}


semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: JpgSnapshot, mutation: JpgMutation, diff: JpgDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/jpg/jfif-1.01/document/snapshot.json" }

