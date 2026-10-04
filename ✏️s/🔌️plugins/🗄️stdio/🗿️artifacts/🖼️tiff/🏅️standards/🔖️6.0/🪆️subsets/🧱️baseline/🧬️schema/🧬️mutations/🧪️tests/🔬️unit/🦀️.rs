use super::*;
use crate::standards::v6_0::subsets::baseline::schema::{check_tiff_baseline_conformance, CODE_MISSING_STRIP_OFFSETS, CODE_TILED_NOT_BASELINE, CODE_UNSUPPORTED_BITS_PER_SAMPLE, CODE_UNSUPPORTED_COMPRESSION, CODE_UNSUPPORTED_PHOTOMETRIC};
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffByteOrder, TiffIfd, TiffStorage, TiffStorageKind};

fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
    let _ = kind;
    TiffTag { tag: id, values }
}

/// 🧫️ A conforming 4x2 Baseline document: RGB, uncompressed, 8 bits per sample, strip-organized.
fn conforming() -> TiffSnapshot {
    TiffSnapshot {
        schema: "stdio.tiff".into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd {
            storage: TiffStorage { kind: TiffStorageKind::Strips, chunks: vec![vec![0u8; 4 * 2 * 3]], ..TiffStorage::default() },
            entries: vec![
                tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                tag(257, TiffFieldType::Long, TiffValues::Long(vec![2])),
                tag(TAG_BITS_PER_SAMPLE, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                tag(TAG_COMPRESSION, TiffFieldType::Short, TiffValues::Short(vec![1])),
                tag(TAG_PHOTOMETRIC, TiffFieldType::Short, TiffValues::Short(vec![2])),
                tag(TAG_STRIP_OFFSETS, TiffFieldType::Long, TiffValues::Long(vec![8])),
            ],
        }],
    }
}

fn codes(snapshot: &TiffSnapshot) -> Vec<String> {
    check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
}

/// 🏷️ [`KINDS`] against the committed catalog. The framework never parses Rust, so without this
/// the manifest could keep measuring `🧱️mutate-tiff-6-0-baseline` against a vocabulary this subset
/// no longer has — which is exactly the gap that left this vocabulary with no catalog at all
/// until the completeness gate learned to see an unregistered one.
#[test]
fn kinds_match_the_committed_catalog() {
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
    assert!(manifest.contains("tiff-6-0-baseline-mutate"), "the manifest must declare this subset's OWN capability, not the ✳️any subset's");
}

#[test]
fn kinds_match_enum_variants_in_declaration_order() {
    let variants = [
        TiffBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: TiffSnapshot::default() }),
        TiffBaselineMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        TiffBaselineMutation::SetCompression(set_compression::SetCompression { compression: 1 }),
        TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 2 }),
        TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![8] }),
        TiffBaselineMutation::InsertTileTags(insert_tile_tags::InsertTileTags { tile_width: 16, tile_length: 16 }),
        TiffBaselineMutation::RemoveTileTags(remove_tile_tags::RemoveTileTags {}),
        TiffBaselineMutation::SetStripOffsets(set_strip_offsets::SetStripOffsets { offsets: vec![8] }),
        TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {}),
    ];
    assert_eq!(variants.len(), KINDS.len(), "every variant needs exactly one KINDS entry");
    for (variant, kind) in variants.iter().zip(KINDS) {
        let tag = match serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(variant)).expect("serialize") {
            serde_json::Value::Object(members) => members.get("mutation").and_then(|value| value.as_str()).expect("tagged enum carries its own discriminant").to_string(),
            other => panic!("a tagged enum must serialize as an object, got {other:?}"),
        };
        assert_eq!(&tag.as_str(), kind, "declaration order must match KINDS");
    }
}

/// 🛡️ The point of the whole vocabulary: every kind moves the document across the axis its own
/// diagnostic reports, and only that axis.
#[test]
fn each_kind_moves_exactly_the_axis_its_diagnostic_reports() {
    assert!(codes(&conforming()).is_empty(), "the fixture must start conforming, got {:?}", codes(&conforming()));

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetCompression(set_compression::SetCompression { compression: 7 }));
    assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_COMPRESSION.to_string()]);

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 6 }));
    assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_PHOTOMETRIC.to_string()]);

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![16, 16, 16] }));
    assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_BITS_PER_SAMPLE.to_string()]);

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::InsertTileTags(insert_tile_tags::InsertTileTags { tile_width: 16, tile_length: 16 }));
    assert_eq!(codes(&snapshot), vec![CODE_TILED_NOT_BASELINE.to_string()]);

    let mut snapshot = conforming();
    apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {}));
    assert!(codes(&snapshot).is_empty(), "wire offsets are derived from canonical strip storage");
}

/// ↩️ `apply(inverse(m), apply(m, base))` must land back on `base` for every kind, including the
/// two whose inverse is a REMOVAL of a tag the base never carried.
///
/// The comparison is a whole-snapshot equality, entry ORDER included, and that is sound rather
/// than lucky: `apply_tags` sorts an IFD's entries by tag number after every application, which
/// is TIFF 6.0 §2's own requirement ("entries must be sorted in ascending order by Tag"). A
/// removal followed by its re-insertion therefore lands in the same place it left, and no kind
/// here needs to carry a position the way the JPEG baseline vocabulary's insertions do.
#[test]
fn every_kind_is_inverted_by_its_own_inverse() {
    let cases = [
        TiffBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: TiffSnapshot::default() }),
        TiffBaselineMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        TiffBaselineMutation::SetCompression(set_compression::SetCompression { compression: 32773 }),
        TiffBaselineMutation::SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation { photometric: 0 }),
        TiffBaselineMutation::SetBitsPerSample(set_bits_per_sample::SetBitsPerSample { bits: vec![4] }),
        TiffBaselineMutation::InsertTileTags(insert_tile_tags::InsertTileTags { tile_width: 16, tile_length: 16 }),
        TiffBaselineMutation::RemoveTileTags(remove_tile_tags::RemoveTileTags {}),
        TiffBaselineMutation::SetStripOffsets(set_strip_offsets::SetStripOffsets { offsets: vec![64, 128] }),
        TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {}),
    ];
    for mutation in cases {
        let base = conforming();
        let mut snapshot = base.clone();
        apply_tiff_baseline_mutation(&mut snapshot, &mutation);
        for undo in crate::mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            apply_tiff_baseline_mutation(&mut snapshot, &undo);
        }
        assert_eq!(snapshot, base, "inverse of {mutation:?} did not restore the base");
    }
}

#[test]
fn removing_strip_offsets_restores_the_neutral_fixture() {
    let before_json = include_str!("../../../../🧫️fixtures/✂️remove-strip-offsets/⬅️before.json");
    let after_json = include_str!("../../../../🧫️fixtures/✂️remove-strip-offsets/➡️after.json");
    let before: TiffSnapshot = semio_framework_pack_json::from_json_str(before_json,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before fixture");
    let after: TiffSnapshot = semio_framework_pack_json::from_json_str(after_json,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after fixture");
    let mutation = TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {});
    let mut actual = before.clone();
    apply_tiff_baseline_mutation(&mut actual, &mutation);
    assert_eq!(actual, after);
    let inverse = crate::mutation_inverse(&mutation, &before).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1);
    for undo in inverse {
        apply_tiff_baseline_mutation(&mut actual, &undo);
    }
    assert_eq!(actual, before);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&actual)).unwrap(), serde_json::from_str::<serde_json::Value>(before_json).unwrap());
    assert!(crate::mutation_inverse(&mutation, &after).expect("valid retained mutation inverse fixture").is_empty());
}

/// 🧭️ An IFD 0 that never carried the tag inverts to its ABSENCE, not to a fabricated value —
/// the case a `restore_or_remove` that always wrote a default would get silently wrong.
#[test]
fn setting_an_absent_strip_offsets_inverts_to_removing_it_again() {
    let mut base = conforming();
    base.ifds[0].entries.retain(|entry| entry.tag != TAG_STRIP_OFFSETS);
    let mutation = TiffBaselineMutation::SetStripOffsets(set_strip_offsets::SetStripOffsets { offsets: vec![8] });
    assert_eq!(crate::mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture"), vec![TiffBaselineMutation::RemoveStripOffsets(remove_strip_offsets::RemoveStripOffsets {})]);

    let mut snapshot = base.clone();
    apply_tiff_baseline_mutation(&mut snapshot, &mutation);
    assert!(codes(&snapshot).is_empty(), "adding StripOffsets makes the IFD strip-organized again");
    for undo in crate::mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
        apply_tiff_baseline_mutation(&mut snapshot, &undo);
    }
    assert_eq!(snapshot, base);
}

/// 🚫️ A kind that sets an axis to the value it already holds must produce the EMPTY diff — a
/// mutation that changes nothing may not report a change.
#[test]
fn setting_an_axis_to_its_current_value_produces_an_empty_diff() {
    let base = conforming();
    let outcome = <TiffBaselineMutation as Mutation<TiffSnapshot>>::diff(&TiffBaselineMutation::SetCompression(set_compression::SetCompression { compression: 1 }), &base);
    assert_eq!(outcome.diff(), &TiffDiff::default());
}
