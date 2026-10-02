//! 🩹️ Compact typed PNG snapshot patch with an exact inverse captured from the publication base.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::PngMutation;
use crate::PngSnapshot;
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

impl protocol::MutationKind<PngSnapshot, PngMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<<PngMutation as Mutation<PngSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<PngDiff as DiffAlgebra<PngSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![PngMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    let base = PngSnapshot::default();
    let event = editing::SnapshotEditEvent::InsertValue { path: "/gama".into(), value: dsl::DslValue::Number(dsl::Number::UInt(45_455)) };
    PngMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare PNG patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_pixels() {
        assert!(PngMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = PngSnapshot::default();
        base.pixels = vec![7; 2 * 1_024 * 1_024];
        let event = editing::SnapshotEditEvent::InsertValue { path: "/gama".into(), value: dsl::DslValue::Number(dsl::Number::UInt(45_455)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare gamma patch");
        let mutation = PngMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply gamma patch");
        assert_eq!(next.gama, Some(45_455));
        assert_eq!(next.pixels, base.pixels);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(PngMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(PngMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
