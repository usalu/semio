//! 🩹️ Compact typed JSON snapshot patch with an exact inverse captured from the publication base.

use crate::schema::diff::JsonDiff;
use crate::schema::mutations::JsonMutation;
use crate::JsonSnapshot;
use protocol::{DiffAlgebra, Mutation};
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<JsonSnapshot, JsonMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonMutation as Mutation<JsonSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<JsonDiff as DiffAlgebra<JsonSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![JsonMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> JsonMutation {
    let base = JsonSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/value".into(), value: dsl::DslValue::Object(vec![("kind".into(), dsl::DslValue::String("null".into()))]) };
    JsonMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare JSON patch fixture") })
}

#[cfg(test)]
#[path = "🧪️tests/🧪️compact-patch/🦀️.rs"]
mod tests;
