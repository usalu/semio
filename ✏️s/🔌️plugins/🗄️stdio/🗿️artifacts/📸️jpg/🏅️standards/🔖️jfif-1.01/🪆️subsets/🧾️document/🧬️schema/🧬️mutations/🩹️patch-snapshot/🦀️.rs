//! 🩹️ Path-scoped JPEG snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::JpgMutation;
use crate::schema::diff::JpgDiff;
use crate::JpgSnapshot;
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: JpgSnapshot, mutation: JpgMutation, diff: JpgDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/jpg/jfif-1.01/document/snapshot.json" }

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    let base = JpgSnapshot::default();
    let event = editing::SnapshotEditEvent::InsertValue { path: "/reEncodeQuality".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(73)) };
    JpgMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare JPEG patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_pixels() {
        assert!(JpgMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = JpgSnapshot::default();
        base.pixels = vec![7; 2 * 1_024 * 1_024];
        let event = editing::SnapshotEditEvent::InsertValue { path: "/reEncodeQuality".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(73)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare quality patch");
        let mutation = JpgMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply quality patch");
        assert_eq!(next.re_encode_quality, Some(73));
        assert_eq!(next.pixels, base.pixels);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(JpgMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(JpgMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
