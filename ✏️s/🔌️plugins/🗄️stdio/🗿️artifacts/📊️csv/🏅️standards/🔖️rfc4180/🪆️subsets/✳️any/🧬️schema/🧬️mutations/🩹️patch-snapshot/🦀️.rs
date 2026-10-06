//! 🩹️ Path-scoped CSV snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use crate::schema::diff::CsvDiff;
use crate::schema::mutations::CsvMutation;
use crate::CsvSnapshot;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}


semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: CsvSnapshot, mutation: CsvMutation, diff: CsvDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/csv/rfc4180/any/snapshot.json" }

#[cfg(test)]
pub(crate) fn test_case() -> CsvMutation {
    let base = CsvSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/hasHeader".into(), value: semio_framework_value::DslValue::Bool(false) };
    CsvMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare CSV patch fixture") })
}

#[cfg(test)]
#[path = "🧪️tests/🧪️compact-patch/🦀️.rs"]
mod tests;
