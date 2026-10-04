//! 🩹️ Path-scoped `ObjMutation` snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot
//! sub-schema at the pointer (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use super::ObjMutation;
use crate::ObjSnapshot;
use crate::schema::diff::ObjDiff;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
#[dsl(keyword = "patch-snapshot")]
pub struct PatchSnapshot {
    #[dsl(block)]
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: ObjSnapshot, mutation: ObjMutation, diff: ObjDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/obj/3.0/geometry/snapshot.json" }
