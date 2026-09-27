//! 🩹️ Compact typed MP4 snapshot patch with an exact inverse captured from the publication base.

use super::*;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "patch-snapshot")]
pub struct PatchSnapshot {
    #[dsl(block)]
    pub patch: editing::SnapshotPatch,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<Mp4Diff as protocol::command::DiffAlgebra<Mp4Snapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::error(error.code, error.message, [error.path]),
        }
    }

    fn inverse(&self, base: &Mp4Snapshot) -> Vec<Mp4Mutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![Mp4Mutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
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
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_sample_data() {
        assert!(Mp4Mutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = Mp4Snapshot::default();
        let mut track = Mp4Track::default();
        track.samples.push(Mp4Sample { data: vec![7; 2 * 1_024 * 1_024], duration: 1_000, cts_offset: 0, sync: true });
        base.tracks.push(track);
        let event = editing::SnapshotEditEvent::SetValue { path: "/movie/title".into(), value: dsl::DslValue::String("Edited title".into()) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare title patch");
        let mutation = Mp4Mutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply title patch");
        assert_eq!(next.movie.title.as_deref(), Some("Edited title"));
        assert_eq!(next.tracks[0].samples[0].data, base.tracks[0].samples[0].data);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(Mp4Mutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(Mp4Mutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
