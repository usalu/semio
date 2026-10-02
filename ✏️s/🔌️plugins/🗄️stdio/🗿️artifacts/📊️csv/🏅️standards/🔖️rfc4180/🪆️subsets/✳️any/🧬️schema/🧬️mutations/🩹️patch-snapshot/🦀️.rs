//! 🩹️ Compact typed CSV snapshot patch with an exact inverse captured from the publication base.

use crate::schema::diff::CsvDiff;
use crate::schema::mutations::CsvMutation;
use crate::CsvSnapshot;
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

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as Mutation<CsvSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<CsvDiff as DiffAlgebra<CsvSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]),
        }
    }

    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![CsvMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> CsvMutation {
    let base = CsvSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/hasHeader".into(), value: dsl::DslValue::Bool(false) };
    CsvMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare CSV patch fixture") })
}

#[cfg(test)]
#[path = "🧪️tests/🧪️compact-patch/🦀️.rs"]
mod tests;
