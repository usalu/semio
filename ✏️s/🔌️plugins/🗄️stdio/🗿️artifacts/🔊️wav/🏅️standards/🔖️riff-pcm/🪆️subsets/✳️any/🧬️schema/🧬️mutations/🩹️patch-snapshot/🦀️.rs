//! 🩹️ Path-scoped WAV snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::*;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: WavSnapshot, mutation: WavMutation, diff: WavDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/wav/riff-pcm/any/snapshot.json" }

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_samples() {
        assert!(WavMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = WavSnapshot::default();
        base.data = WavData::Pcm8(vec![7; 2 * 1_024 * 1_024]);
        let event = editing::SnapshotEditEvent::SetValue { path: "/data/value/1048577".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare sample patch");
        let mutation = WavMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply sample patch");
        let WavData::Pcm8(samples) = &next.data else { panic!("expected pcm8") };
        assert_eq!(samples[1_048_577], 9);
        let WavData::Pcm8(base_samples) = &base.data else { unreachable!() };
        assert_eq!(&samples[..1_048_577], &base_samples[..1_048_577]);
        assert_eq!(&samples[1_048_578..], &base_samples[1_048_578..]);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(WavMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(WavMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
