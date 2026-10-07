//! 🩹️ Path-scoped PNG snapshot patch.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::PngMutation;
use crate::PngSnapshot;
use semio_s_artifact_stdio_contract::editing;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}


semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: PngSnapshot, mutation: PngMutation, diff: PngDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/png/1.2/any/snapshot.json" }
