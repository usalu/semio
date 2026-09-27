//! 🩹️ Compact typed WAV snapshot patch with an exact inverse captured from the publication base.

use super::*;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(diff_set_snapshot(base, &next)),
            Err(error) => protocol::MutationOutcome::error(error.code, error.message, [error.path]),
        }
    }

    fn inverse(&self, base: &WavSnapshot) -> Vec<WavMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![WavMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_samples() {
        assert!(WavMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = WavSnapshot::default();
        base.data = WavData::Pcm8(vec![7; 2 * 1_024 * 1_024]);
        let event = editing::SnapshotEditEvent::SetValue { path: "/data/value/1048577".into(), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare sample patch");
        let mutation = WavMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply sample patch");
        let WavData::Pcm8(samples) = &next.data else { panic!("expected pcm8") };
        assert_eq!(samples[1_048_577], 9);
        let WavData::Pcm8(base_samples) = &base.data else { unreachable!() };
        assert_eq!(&samples[..1_048_577], &base_samples[..1_048_577]);
        assert_eq!(&samples[1_048_578..], &base_samples[1_048_578..]);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(WavMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(WavMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
