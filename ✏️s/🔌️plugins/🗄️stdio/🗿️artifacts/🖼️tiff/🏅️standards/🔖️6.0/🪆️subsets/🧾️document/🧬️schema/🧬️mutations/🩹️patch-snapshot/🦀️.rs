//! 🩹️ Compact typed TIFF snapshot patch with an exact inverse captured from the publication base.

use super::TiffMutation;
use crate::schema::diff::TiffDiff;
use crate::TiffSnapshot;
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

impl protocol::MutationKind<TiffSnapshot, TiffMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffMutation as Mutation<TiffSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<TiffDiff as DiffAlgebra<TiffSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::refuse(error.outcome_code(), format!("{}: {}", error.code, error.message), [error.path]),
        }
    }

    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![TiffMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    let mut base = TiffSnapshot::default();
    base.pixels.push(7);
    let event = editing::SnapshotEditEvent::SetValue { path: "/pixels/0".into(), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
    TiffMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare TIFF patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_pixels() {
        assert!(TiffMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = TiffSnapshot::default();
        base.pixels = vec![7; 2 * 1_024 * 1_024];
        let event = editing::SnapshotEditEvent::SetValue { path: "/pixels/0".into(), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare pixel patch");
        let mutation = TiffMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply pixel patch");
        assert_eq!(next.pixels[0], 9);
        assert_eq!(&next.pixels[1..], &base.pixels[1..]);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(TiffMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(TiffMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
