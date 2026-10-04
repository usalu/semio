//! 🩹️ Path-scoped JSON snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use crate::schema::diff::JsonDiff;
use crate::schema::mutations::JsonMutation;
use crate::JsonSnapshot;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: JsonSnapshot, mutation: JsonMutation, diff: JsonDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/json/rfc8259/base/snapshot.json" }

#[cfg(test)]
pub(crate) fn test_case() -> JsonMutation {
    let base = JsonSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/value".into(), value: semio_framework_value::DslValue::Object(vec![("kind".into(), semio_framework_value::DslValue::String("null".into()))]) };
    JsonMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare JSON patch fixture") })
}

#[cfg(test)]
#[path = "🧪️tests/🧪️compact-patch/🦀️.rs"]
mod tests;
