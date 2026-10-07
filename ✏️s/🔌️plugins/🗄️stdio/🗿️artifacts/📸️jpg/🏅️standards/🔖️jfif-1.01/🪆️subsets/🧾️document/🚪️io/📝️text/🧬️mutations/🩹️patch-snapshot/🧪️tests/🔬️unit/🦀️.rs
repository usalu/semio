//! 🧪️ Native JPEG snapshot patch codec and semantic replay laws.
use semio_s_artifact_stdio_contract::editing;

    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_pixels() {
        assert!(JpgMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = JpgSnapshot::default();
        base.pixels = vec![7; 2 * 1_024 * 1_024];
        let event = editing::SnapshotEditEvent::SetValue { path: "/jfifXDensity".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(73)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare quality patch");
        let mutation = JpgMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply quality patch");
        assert_eq!(next.jfif_x_density, 73);
        assert_eq!(next.pixels, base.pixels);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(JpgMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(JpgMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
