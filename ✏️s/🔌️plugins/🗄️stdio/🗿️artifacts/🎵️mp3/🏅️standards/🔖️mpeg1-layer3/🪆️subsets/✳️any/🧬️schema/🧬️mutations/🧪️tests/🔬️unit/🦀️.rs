use crate::{standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3v1Tag, standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3v2Tag, standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Frame, standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot};
use crate::apply_mutation;
use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Mp3FrameHeader};
use protocol::command::{ColdDiffAlgebra, DiffAlgebra};
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
        Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: Some(Id3v2Tag { frames: vec![Id3Frame { id: "TIT2".into(), content: crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content::Text { values: vec!["x".into()] } }] }) }),
        Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: None }),
        Mp3Mutation::SetFrames(set_frames::SetFrames { frames: vec![frame(), frame(), frame()] }),
        Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: Some(Id3v1Tag::default()) }),
        Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: None }),
        Mp3Mutation::PatchFrames(patch_frames::PatchFrames { index: 0, remove_count: 1, frames: vec![frame(), frame()] }),
        Mp3Mutation::SetId3Frame(set_id3_frame::SetId3Frame { frame: Id3Frame { id: "TIT2".into(), content: crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content::Text { values: vec!["one frame".into()] } } }),
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
    assert_eq!(KINDS.len(), 6, "KINDS must list exactly the declared 6 kinds");
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
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
        let returned = apply_mutation(&mut via_apply, &m);
        let direct = m.diff(&base);
        assert_eq!(direct, returned, "diff mismatch for {m:?}");
        assert_eq!(protocol::apply_diff_cold(direct.diff(), &base).unwrap(), via_apply, "apply mismatch for {m:?}");
    }
}
//#endregion mutation_diff_law

//#region inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_and_diff_level() {
    let base = base_snapshot();
    for m in variants(&base) {
        let mut round = base.clone();
        apply_mutation(&mut round, &m);
        for inv in m.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
            apply_mutation(&mut round, &inv);
        }
        assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

        let d = m.diff(&base);
        let applied = protocol::apply_diff_cold(d.diff(), &base).unwrap();
        let undone = protocol::apply_diff_cold(&d.diff().inverse(&base), &applied).unwrap();
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

/// ⚖️ `mp3_mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn mp3_mutation_inverse_sum_law_holds_for_every_leaf() {
    let plain = base_snapshot();
    let tagged = Mp3Snapshot {
        id3v2: Some(Id3v2Tag { frames: vec![Id3Frame { id: "TPE1".into(), content: crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content::Text { values: vec!["x".into()] } }] }),
        id3v1: Some(Id3v1Tag::default()),
        ..base_snapshot()
    };
    for base in [&plain, &tagged] {
        for mutation in variants(base) {
            protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, base).await;
        }
    }
}

/// ✂️ A cut at frame boundaries removes exactly the addressed whole frames, is one exactly reversible edit and refuses a range outside the lane.
#[semio_framework_async_macros::async_test]
async fn patch_frames_cuts_whole_frames_reversibly_and_refuses_ranges_outside_the_lane() {
    let numbered = |value: u8| Mp3Frame { payload: vec![value], ..frame() };
    let base = Mp3Snapshot { frames: (0..5).map(numbered).collect(), ..Mp3Snapshot::default() };
    let cut = Mp3Mutation::PatchFrames(patch_frames::PatchFrames { index: 1, remove_count: 3, frames: vec![] });
    let mut edited = base.clone();
    apply_mutation(&mut edited, &cut);
    assert_eq!(edited.frames, vec![numbered(0), numbered(4)]);
    for inverse in cut.inverse(&base).expect("valid cut inverse").into_iter().rev() {
        apply_mutation(&mut edited, &inverse);
    }
    assert_eq!(edited, base);
    let outside = Mp3Mutation::PatchFrames(patch_frames::PatchFrames { index: 4, remove_count: 2, frames: vec![] });
    assert!(outside.inverse(&base).expect("refusal has no inverse").is_empty());
    let mut untouched = base.clone();
    apply_mutation(&mut untouched, &outside);
    assert_eq!(untouched, base);
}

/// 🏷️ One-frame ID3 edits replace in place, append when the id is new, remove only what exists, and undo exactly.
#[semio_framework_async_macros::async_test]
async fn id3_frame_edits_replace_append_remove_and_undo_exactly() {
    use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Id3Content;
    let text = |id: &str, value: &str| Id3Frame { id: id.into(), content: Id3Content::Text { values: vec![value.into()] } };
    let base = Mp3Snapshot { id3v2: Some(Id3v2Tag { frames: vec![text("TIT2", "a"), text("TPE1", "b")] }), ..base_snapshot() };
    let replace = Mp3Mutation::SetId3Frame(set_id3_frame::SetId3Frame { frame: text("TIT2", "z") });
    let append = Mp3Mutation::SetId3Frame(set_id3_frame::SetId3Frame { frame: text("TALB", "c") });
    let remove = Mp3Mutation::RemoveId3Frame(remove_id3_frame::RemoveId3Frame { id: "TPE1".into() });
    let missing = Mp3Mutation::RemoveId3Frame(remove_id3_frame::RemoveId3Frame { id: "TCON".into() });
    for (mutation, expected) in [(&replace, vec![text("TIT2", "z"), text("TPE1", "b")]), (&append, vec![text("TIT2", "a"), text("TPE1", "b"), text("TALB", "c")]), (&remove, vec![text("TIT2", "a")])] {
        let mut edited = base.clone();
        apply_mutation(&mut edited, mutation);
        assert_eq!(edited.id3v2.as_ref().expect("tag").frames, expected, "{mutation:?}");
        for inverse in mutation.inverse(&base).expect("valid inverse").into_iter().rev() {
            apply_mutation(&mut edited, &inverse);
        }
        assert_eq!(edited, base, "{mutation:?}");
    }
    let mut untouched = base.clone();
    apply_mutation(&mut untouched, &missing);
    assert_eq!(untouched, base);
    assert!(missing.inverse(&base).expect("refusal has no inverse").is_empty());
    let mut created = base_snapshot();
    apply_mutation(&mut created, &append);
    assert_eq!(created.id3v2.expect("created tag").frames, vec![text("TALB", "c")]);
}
