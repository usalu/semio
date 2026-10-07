//! 🩹️ Path-scoped TIFF snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::TiffMutation;
use crate::schema::diff::TiffDiff;

use crate::TiffSnapshot;
#[cfg(test)]
use crate::schema::snapshot::{TiffWord64,TiffSampleBlock,TiffValues,TiffTag,TiffIfd};
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}


semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: TiffSnapshot, mutation: TiffMutation, diff: TiffDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/tiff/6.0/document/snapshot.json" }

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    let mut base = TiffSnapshot::default();
    base.ifds.push(sample_page(1));
    let event = editing::SnapshotEditEvent::SetValue { path: "/ifds/0/blocks/0/samples/0/lo".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
    TiffMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare TIFF patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_owned_samples() {
        assert!(TiffMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = TiffSnapshot::default();
        base.ifds.push(sample_page(2 * 1_024 * 1_024));
        let event = editing::SnapshotEditEvent::SetValue { path: "/ifds/0/blocks/0/samples/0/lo".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare storage byte patch");
        let mutation = TiffMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply pixel patch");
        assert_eq!(next.ifds[0].blocks[0].samples[0].lo, 9);
        assert_eq!(&next.ifds[0].blocks[0].samples[1..], &base.ifds[0].blocks[0].samples[1..]);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(TiffMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(TiffMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}

#[cfg(test)]
fn sample_page(width:u32)->TiffIfd{TiffIfd{entries:vec![TiffTag{tag:256,values:TiffValues::Long(vec![width])},TiffTag{tag:257,values:TiffValues::Long(vec![1])},TiffTag{tag:258,values:TiffValues::Short(vec![8])},TiffTag{tag:262,values:TiffValues::Short(vec![1])}],blocks:vec![TiffSampleBlock{x:0,y:0,width,height:1,channels:1,samples:vec![TiffWord64::from_word(7);width as usize]}]}}
