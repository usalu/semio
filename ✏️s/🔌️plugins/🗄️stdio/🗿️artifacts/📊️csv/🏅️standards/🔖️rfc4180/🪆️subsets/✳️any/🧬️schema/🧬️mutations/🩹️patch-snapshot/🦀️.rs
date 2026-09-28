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
            Err(error) => protocol::MutationOutcome::error(error.code, error.message, [error.path]),
        }
    }

    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![CsvMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
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
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use semio_s_artifact_stdio_contract::pack;

    #[test]
    fn compact_snapshot_patch_matches_neutral_large_field_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
        let row = &fixture["nativePilots"]["csv"];
        let mut original = row["before"].clone();
        *original.pointer_mut(row["largeValuePath"].as_str().unwrap()).unwrap() = "x".repeat(row["largeValueBytes"].as_u64().unwrap() as usize).into();
        let base: CsvSnapshot = pack::json::from_json_str(&original.to_string()).unwrap();
        let event: editing::SnapshotEditEvent = pack::json::from_json_str(&row["event"].to_string()).unwrap();
        let patch = editing::prepare_snapshot_patch(&base, &event).unwrap();
        let mutation = CsvMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).unwrap();
        let mut expected = original.clone();
        *expected.pointer_mut(row["event"]["path"].as_str().unwrap()).unwrap() = row["event"]["value"].clone();
        let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).unwrap(), base);
        let maximum = row["maximumPatchBytes"].as_u64().unwrap() as usize;
        assert!(mutation.encode_op().unwrap().len() < maximum);
        assert!(inverse[0].encode_op().unwrap().len() < maximum);
        assert_eq!(CsvMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(CsvMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        println!("[DEBUG] csv compact patch preserves unrelated {} byte field through forward/inverse codecs", row["largeValueBytes"]);
    }
}
