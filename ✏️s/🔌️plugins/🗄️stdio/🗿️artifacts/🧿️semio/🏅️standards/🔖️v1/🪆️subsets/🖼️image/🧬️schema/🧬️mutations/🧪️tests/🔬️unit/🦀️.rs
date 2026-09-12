use super::*;
/// 🔧️ `DiffAlgebra` lives at `protocol::command::DiffAlgebra`, not `protocol::DiffAlgebra`
/// (W2b closer fix — was an unresolved-import compile error).
use protocol::command::DiffAlgebra;
use protocol::{MutationDiff, OpBinary, OpText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(seed: u8, len: usize) -> SemioImageFrame {
    SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 4,
        height: 4,
        colorspace: SemioColorspace::Rgba,
        bit_depth: 8,
        frames: vec![frame(1, 16), frame(2, 16)],
        icc: Some(vec![9, 9]),
        metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "old".into() }],
        ..SemioImageSnapshot::default()
    }
}

/// 🌱 Reuses `demo_mutation_cases()` (single source of truth, also feeds
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`) rather
/// than an independent copy.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_mutations() -> Vec<SemioImageMutation> {
    demo_mutation_cases()
}

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).expect("apply must succeed for a well-formed fixture");

        let mut via_apply = base.clone();
        let diff_from_apply = apply_semio_image_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in sample_mutations() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_semio_image_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base) {
            apply_semio_image_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = MutationDiff::apply(diff.diff(), &base).expect("apply must succeed for a well-formed fixture");
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = MutationDiff::apply(&inverse_diff, &next).expect("apply must succeed for a well-formed fixture");
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️CodecRetentionLaw
/// 🧪️ codec_retention_law: `ArtifactPack` decode(encode(snapshot)) on a real (mutation-built,
/// not just default) snapshot.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let mut snap = fixture();
    apply_semio_image_mutation(&mut snap, &SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "x".into() }));
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️OpTextBinaryRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in sample_mutations() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioImageMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioImageMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️CatalogLaw
/// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a new variant has
/// no arm here, so the crate stops building until both this match and `KINDS` name it.
fn kind_of(mutation: &SemioImageMutation) -> &'static str {
    match mutation {
        SemioImageMutation::SetSnapshot(_) => "set-snapshot",
        SemioImageMutation::SetDimensions(_) => "set-dimensions",
        SemioImageMutation::SetColorspace(_) => "set-colorspace",
        SemioImageMutation::SetBitDepth(_) => "set-bit-depth",
        SemioImageMutation::SetIcc(_) => "set-icc",
        SemioImageMutation::InsertFrame(_) => "insert-frame",
        SemioImageMutation::RemoveFrame(_) => "remove-frame",
        SemioImageMutation::MoveFrame(_) => "move-frame",
        SemioImageMutation::SetFrameDelay(_) => "set-frame-delay",
        SemioImageMutation::SetFramePixels(_) => "set-frame-pixels",
        SemioImageMutation::SetMetadataEntry(_) => "set-metadata-entry",
        SemioImageMutation::RemoveMetadataEntry(_) => "remove-metadata-entry",
    }
}

/// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
/// the committed `semio-v1-image` catalog carries — the framework never parses Rust, so this is
/// the only thing that keeps the catalog honest against the enum.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let one_per_variant = [
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioImageSnapshot::default() }),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: 4, height: 2 }),
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: SemioColorspace::Rgba }),
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: 16 }),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc: None }),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: 0, frame: SemioImageFrame::default() }),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: 0 }),
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: 1, to: 0 }),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_ms: 40 }),
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, rgba8: Vec::new() }),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "semio".into() }),
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: "Author".into() }),
    ];
    assert_eq!(KINDS.len(), one_per_variant.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, mutation) in KINDS.iter().zip(one_per_variant.iter()) {
        assert_eq!(*kind, kind_of(mutation), "KINDS must follow the enum's own declaration order and kebab-case spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🔖️CatalogLaw
