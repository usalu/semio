//! 🩹️ Path-scoped MP4 snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::*;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
#[dsl(keyword = "patch-snapshot")]
pub struct PatchSnapshot {
    #[dsl(block)]
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: Mp4Snapshot, mutation: Mp4Mutation, diff: Mp4Diff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/mp4/isobmff/any/snapshot.json" }

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
        let event = editing::SnapshotEditEvent::SetValue { path: "/movie/title".into(), value: semio_framework_value::DslValue::String("Edited title".into()) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare title patch");
        let mutation = Mp4Mutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply title patch");
        assert_eq!(next.movie.title.as_deref(), Some("Edited title"));
        assert_eq!(next.tracks[0].samples[0].data, base.tracks[0].samples[0].data);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(Mp4Mutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(Mp4Mutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
