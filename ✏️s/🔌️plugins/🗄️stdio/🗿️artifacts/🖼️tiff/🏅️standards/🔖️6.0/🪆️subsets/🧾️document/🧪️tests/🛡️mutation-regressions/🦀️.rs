use crate::schema::diff::TiffDiff;
use crate::schema::mutations::*;
use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues};
use crate::TiffSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};

fn tag(tag: u16, value: u16) -> TiffTag { TiffTag { tag, values: TiffValues::Short(vec![value]) } }

fn snapshot() -> TiffSnapshot {
    TiffSnapshot {
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd {
            entries: vec![tag(256, 2), tag(257, 1)],
            storage: TiffStorage { kind: TiffStorageKind::Strips, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks: vec![vec![1, 2, 3, 4, 5, 6]] },
        }],
        ..TiffSnapshot::default()
    }
}

fn variants() -> Vec<TiffMutation> {
    vec![
        TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order: TiffByteOrder::BigEndian }),
        TiffMutation::InsertIfd(InsertIfdMutation { index: 1, ifd: TiffIfd { entries: vec![tag(256, 1)], storage: TiffStorage::default() } }),
        TiffMutation::RemoveIfd(RemoveIfdMutation { index: 0 }),
        TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: 0, tag: 256, values: TiffValues::Long(vec![4]) }),
        TiffMutation::RemoveTag(RemoveTagMutation { ifd_index: 0, tag: 257 }),
    ]
}

#[test]
fn every_mounted_mutation_diff_matches_imperative_apply_and_inverse() {
    let base = snapshot();
    for mutation in variants() {
        let expected = mutation.diff(&base);
        let mut imperative = base.clone();
        assert_eq!(apply_tiff_mutation(&mut imperative, &mutation), expected);
        let applied = expected.diff().apply(&base).expect("diff applies");
        assert_eq!(applied, imperative);
        assert_eq!(expected.diff().inverse(&base).apply(&applied).expect("inverse applies"), base);
    }
}

#[test]
fn between_tracks_storage_without_a_second_raster_authority() {
    let a = snapshot();
    let mut b = a.clone();
    b.ifds[0].storage.chunks[0][2] = 99;
    assert_eq!(TiffDiff::between(&a, &b).apply(&a).expect("forward"), b);
    assert_eq!(TiffDiff::between(&b, &a).apply(&b).expect("reverse"), a);
}

#[test]
fn absorbed_tag_changes_equal_sequential_application() {
    let base = snapshot();
    let first = TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: 0, tag: 256, values: TiffValues::Long(vec![4]) }).diff(&base);
    let middle = first.diff().apply(&base).expect("first");
    let second = TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: 0, tag: 256, values: TiffValues::Long(vec![8]) }).diff(&middle);
    let expected = second.diff().apply(&middle).expect("second");
    let mut absorbed = first.diff().clone();
    absorbed.absorb(second.diff().clone());
    assert_eq!(absorbed.apply(&base).expect("absorbed"), expected);
}
