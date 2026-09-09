use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Mp3FrameHeader};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame() -> Mp3Frame {
    Mp3Frame {
        header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
        payload: vec![0u8; 4],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> Mp3Snapshot {
    Mp3Snapshot { frames: vec![frame()], ..Mp3Snapshot::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn variants(base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
    vec![
        Mp3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Mp3Snapshot { frames: vec![frame(), frame()], ..base.clone() } }),
        Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0] }] }) }),
        Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: None }),
        Mp3Mutation::SetFrames(set_frames::SetFrames { frames: vec![frame(), frame(), frame()] }),
        Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: Some(Id3v1Tag { raw: vec![b'T', b'A', b'G'] }) }),
        Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: None }),
    ]
}

/// 🧪️ Keeps the declaration honest, which nothing else can: the framework never parses Rust, so
/// the CATALOG is what the contract gate counts against, and this is the only check that ties it
/// to the enum. `variants()` already carries every declared variant, `kind()` is an exhaustive
/// match, and the manifest is read as committed text — so a kind added to one of the three and
/// not the others fails here. The sibling `KINDS` in `../../🦀️oracle.rs` mirrors
/// this one from the oracle crate, which must never link this crate; it can only compare
/// strings, whereas this test compares against real values.
#[test]
fn kinds_matches_every_variant_and_the_catalog() {
    let from_variants: std::collections::BTreeSet<&str> = variants(&base_snapshot()).iter().map(Mp3Mutation::kind).collect();
    let from_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(from_variants, from_kinds, "KINDS must equal every Mp3Mutation variant's kind()");
    assert_eq!(KINDS.len(), 4, "KINDS must list exactly the declared 4 kinds");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle catalog manifest must declare kind {kind:?}");
    }
}

//#region mutation_diff_law
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_every_variant() {
    let base = base_snapshot();
    for m in variants(&base) {
        let mut via_apply = base.clone();
        let returned = apply_mp3_mutation(&mut via_apply, &m);
        let direct = m.diff(&base);
        assert_eq!(direct, returned, "diff mismatch for {m:?}");
        assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
    }
}
//#endregion mutation_diff_law

//#region inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_and_diff_level() {
    let base = base_snapshot();
    for m in variants(&base) {
        let mut round = base.clone();
        apply_mp3_mutation(&mut round, &m);
        for inv in m.inverse(&base) {
            apply_mp3_mutation(&mut round, &inv);
        }
        assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

        let d = m.diff(&base);
        let applied = d.diff().apply(&base).unwrap();
        let undone = d.diff().inverse(&base).apply(&applied).unwrap();
        assert_eq!(undone, base, "diff-level inverse failed for {m:?}");
    }
}
//#endregion inverse_law

//#region op_text_binary_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    for m in variants(&base) {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = Mp3Mutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = Mp3Mutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion op_text_binary_roundtrip_law
