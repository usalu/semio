//! 🩹️ Path-scoped TIFF snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot sub-schema at the pointer.

use super::TiffMutation;
use crate::schema::diff::TiffDiff;
use crate::schema::snapshot::{TiffIfd, TiffStorage, TiffStorageKind};
use crate::TiffSnapshot;
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

semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf: PatchSnapshot, snapshot: TiffSnapshot, mutation: TiffMutation, diff: TiffDiff, snapshot_schema: "https://json.schemas.assets.semio-tech.com/s/stdio/tiff/6.0/document/snapshot.json" }

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    let mut base = TiffSnapshot::default();
    base.ifds.push(TiffIfd { storage: TiffStorage { kind: TiffStorageKind::Strips, chunks: vec![vec![7]], ..TiffStorage::default() }, ..TiffIfd::default() });
    let event = editing::SnapshotEditEvent::SetValue { path: "/ifds/0/storage/chunks/0/0".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
    TiffMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare TIFF patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_unrelated_storage_bytes() {
        assert!(TiffMutation::parse_op("patch-snapshot patch=€0").is_err());
        let mut base = TiffSnapshot::default();
        base.ifds.push(TiffIfd { storage: TiffStorage { kind: TiffStorageKind::Strips, chunks: vec![vec![7; 2 * 1_024 * 1_024]], ..TiffStorage::default() }, ..TiffIfd::default() });
        let event = editing::SnapshotEditEvent::SetValue { path: "/ifds/0/storage/chunks/0/0".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare storage byte patch");
        let mutation = TiffMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply pixel patch");
        assert_eq!(next.ifds[0].storage.chunks[0][0], 9);
        assert_eq!(&next.ifds[0].storage.chunks[0][1..], &base.ifds[0].storage.chunks[0][1..]);
        let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert!(mutation.encode_op().expect("encode forward patch").len() < 1_048_576);
        assert!(inverse[0].encode_op().expect("encode inverse patch").len() < 1_048_576);
        assert_eq!(TiffMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(TiffMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
