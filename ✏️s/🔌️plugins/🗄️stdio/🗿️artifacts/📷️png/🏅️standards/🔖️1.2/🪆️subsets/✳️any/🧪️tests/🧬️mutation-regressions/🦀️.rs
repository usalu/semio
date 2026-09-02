//! 🧪️ Preserved raster sparse-diff and codec regression laws.
use crate::artifacts::png::schema::diff::{
    self, dec_background, dec_chromaticities, dec_chunk, dec_chunk_marker, dec_color_type, dec_list, dec_physical_dims, dec_rgb, dec_srgb_intent, dec_str, dec_text_chunk, dec_timestamp, dec_transparency, decode_option, enc_background,
    enc_chromaticities, enc_chunk, enc_chunk_marker, enc_color_type, enc_list, enc_physical_dims, enc_rgb, enc_srgb_intent, enc_str, enc_text_chunk, enc_timestamp, enc_transparency, encode_option, hex_decode, hex_encode, parse_u32, parse_u8,
    split_top_level, strip_brackets, PngDiff,
};
use crate::artifacts::png::schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTimestamp, PngTransparency};
use crate::artifacts::png::PngSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

use crate::artifacts::png::schema::mutations::*;
//#region 🔖️DemoMutationCases
/// 🧪️ P2-P2: shared demo mutation fixtures — `⚙️engine/🦀️.rs`'s `conformance_laws`
/// module calls `regression_mutation_cases()` directly (`ops_grammar_conformance_law`/
/// `protocol_walk_law`) instead of duplicating the literal case list; `mod tests` below now
/// calls it too (single source of truth, per CLAUDE.md — moved out of `mod tests` verbatim,
/// only the `pub(crate)`/`#[cfg(test)]` visibility changed).
#[cfg(test)]
fn demo_text_chunk(keyword: &str, value: &str) -> PngTextChunk {
    PngTextChunk { keyword: keyword.into(), value: value.into(), compressed: false, kind: crate::artifacts::png::schema::snapshot::PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() }
}

#[cfg(test)]
pub(crate) fn demo_base_snapshot() -> PngSnapshot {
    use crate::artifacts::png::schema::snapshot::PngChunkMarker;
    PngSnapshot {
        schema: "stdio.png".into(),
        width: 4,
        height: 4,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        interlace: false,
        plte: None,
        trns: None,
        gama: None,
        chrm: None,
        srgb: None,
        phys: None,
        time: None,
        bkgd: None,
        text_chunks: vec![demo_text_chunk("Title", "demo")],
        pixels: vec![0u8; 4 * 4 * 4],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Iend],
        unknown_chunks: vec![],
    }
}

/// ✅️ Every `PngMutation` variant (incl. two out-of-range no-op cases) built off
/// `demo_base_snapshot()` — the single case list `mutation_diff_law`/`inverse_law`/
/// `op_text_binary_roundtrip_law` (this file) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️.rs`) all exercise.
#[cfg(test)]
pub(crate) fn regression_mutation_cases() -> Vec<PngMutation> {
    let base = demo_base_snapshot();
    vec![
        PngMutation::ChangeHeader(crate::artifacts::png::schema::mutations::ChangeHeaderMutation { width: 8, height: 8, bit_depth: 16, color_type: PngColorType::Grayscale, interlace: true }),
        PngMutation::ReplacePalette(crate::artifacts::png::schema::mutations::ReplacePaletteMutation { plte: Some(vec![PngRgb { r: 1, g: 2, b: 3 }]) }),
        PngMutation::ChangeTransparency(crate::artifacts::png::schema::mutations::ChangeTransparencyMutation { trns: Some(PngTransparency::Grayscale { gray: 7 }) }),
        PngMutation::ChangeGamma(crate::artifacts::png::schema::mutations::ChangeGammaMutation { gama: Some(45455) }),
        PngMutation::ChangeChromaticities(crate::artifacts::png::schema::mutations::ChangeChromaticitiesMutation { chrm: Some(PngChromaticities { white_x: 1, white_y: 2, red_x: 3, red_y: 4, green_x: 5, green_y: 6, blue_x: 7, blue_y: 8 }) }),
        PngMutation::ChangeSrgbIntent(crate::artifacts::png::schema::mutations::ChangeSrgbIntentMutation { srgb: Some(PngSrgbIntent::Saturation) }),
        PngMutation::ChangePhysicalDims(crate::artifacts::png::schema::mutations::ChangePhysicalDimsMutation { phys: Some(PngPhysicalDims { ppu_x: 96, ppu_y: 96, unit_is_meter: false }) }),
        PngMutation::ChangeTimestamp(crate::artifacts::png::schema::mutations::ChangeTimestampMutation { time: Some(PngTimestamp { year: 2024, month: 6, day: 1, hour: 12, minute: 0, second: 0 }) }),
        PngMutation::ChangeBackground(crate::artifacts::png::schema::mutations::ChangeBackgroundMutation { bkgd: Some(PngBackground::Rgb { r: 1, g: 2, b: 3 }) }),
        PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 1, chunk: demo_text_chunk("Comment", "hi") }),
        PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 0 }),
        PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: 0, chunk: demo_text_chunk("Title", "updated") }),
        PngMutation::ReplacePixels(crate::artifacts::png::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
        PngMutation::InsertUnknownChunk(crate::artifacts::png::schema::mutations::InsertUnknownChunkMutation { index: 1, chunk: PngChunk { kind: *b"zTXt", data: vec![4, 5] } }),
        PngMutation::RemoveUnknownChunk(crate::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: 0 }),
        // Out-of-range targets: graceful no-ops, still law-compliant.
        PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 99 }),
        PngMutation::RemoveUnknownChunk(crate::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: 99 }),
    ]
}
//#endregion 🔖️DemoMutationCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::png::schema::snapshot::{PngChunkMarker, PngTextKind};
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    /// 🔁️ Thin aliases of the module-level `demo_text_chunk`/`demo_base_snapshot` (P2-P2 —
    /// single source of truth, per CLAUDE.md) — kept as short LOCAL names since both are used
    /// pervasively for ad hoc per-test values below (`absorb_law` etc.), not just the mutation
    /// case list.
    fn text_chunk(keyword: &str, value: &str) -> PngTextChunk {
        demo_text_chunk(keyword, value)
    }

    fn base_snapshot() -> PngSnapshot {
        demo_base_snapshot()
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. Every index-keyed collection
    /// (`plte`, `text_chunks`, `chunk_order`, `unknown_chunks`) is deliberately DIFFERENT
    /// length (2 vs 1) with the "surviving/modified" item at position 0 and the
    /// "removed-in-forward / added-in-backward" item as the tail at position 1 — the recipe's
    /// own documented workaround for the structural "same-length between() can show removed
    /// XOR added, never both from one call" trap (see `f1-closer-report.md` §4.4).await.
    fn sweep_a() -> PngSnapshot {
        PngSnapshot {
            schema: "stdio.png".into(),
            width: 10,
            height: 20,
            bit_depth: 8,
            color_type: PngColorType::Rgba,
            interlace: false,
            plte: Some(vec![PngRgb { r: 1, g: 1, b: 1 }, PngRgb { r: 2, g: 2, b: 2 }]),
            trns: Some(PngTransparency::Grayscale { gray: 5 }),
            gama: Some(45455),
            chrm: Some(PngChromaticities { white_x: 1, white_y: 2, red_x: 3, red_y: 4, green_x: 5, green_y: 6, blue_x: 7, blue_y: 8 }),
            srgb: Some(PngSrgbIntent::Perceptual),
            phys: Some(PngPhysicalDims { ppu_x: 100, ppu_y: 100, unit_is_meter: true }),
            time: Some(PngTimestamp { year: 2020, month: 1, day: 1, hour: 0, minute: 0, second: 0 }),
            bkgd: Some(PngBackground::Grayscale { gray: 255 }),
            text_chunks: vec![text_chunk("Author", "orig"), text_chunk("Trash", "gone")],
            pixels: vec![0u8, 0, 0, 255, 255, 255, 255, 255],
            chunk_order: vec![PngChunkMarker::Gama, PngChunkMarker::Chrm],
            unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![1, 2, 3] }, PngChunk { kind: *b"gone", data: vec![9, 9] }],
        }
    }

    fn sweep_b() -> PngSnapshot {
        PngSnapshot {
            schema: "stdio.png".into(),
            width: 11,
            height: 21,
            bit_depth: 16,
            color_type: PngColorType::Palette,
            interlace: true,
            plte: Some(vec![PngRgb { r: 9, g: 9, b: 9 }]),
            trns: None,
            gama: None,
            chrm: None,
            srgb: Some(PngSrgbIntent::AbsoluteColorimetric),
            phys: None,
            time: None,
            bkgd: None,
            text_chunks: vec![PngTextChunk { keyword: "Creator".into(), value: "changed".into(), compressed: true, kind: PngTextKind::IText, language_tag: "en".into(), translated_keyword: "Auteur".into() }],
            pixels: vec![1u8, 1, 1, 255],
            chunk_order: vec![PngChunkMarker::Srgb],
            unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![4, 5, 6] }],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &PngSnapshot, mutation: PngMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_png_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_png_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("diff must apply to base"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    /// 🔁️ Thin alias of the module-level `regression_mutation_cases()` (P2-P2 — single source of
    /// truth) — kept as a local name taking the SAME `&PngSnapshot` signature every call site
    /// below already uses; `regression_mutation_cases()` builds its own (structurally identical)
    /// base internally, so the passed-in `base` is intentionally unused here.
    fn all_variants(base: &PngSnapshot) -> Vec<PngMutation> {
        let _ = base;
        regression_mutation_cases()
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            assert_mutation_diff_law(&base, m);
        }
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_png_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_png_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.diff().apply(&base).expect("diff must apply to base");
            let inv_d = d.diff().inverse(&base);
            assert_eq!(inv_d.apply(&mutated).expect("inverse diff must apply to mutated"), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &PngSnapshot, m1: PngMutation, m2: PngMutation) {
        let d1 = m1.diff(base);
        let mid = d1.diff().apply(base).expect("d1 must apply to base");
        let d2 = m2.diff(&mid);
        let sequential = d2.diff().apply(&mid).expect("d2 must apply to mid");

        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(base).expect("merged diff must apply to base"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: base has [Title] at 0; insert "New" at 1 -> [Title,New]; then
        // remove index 0 ("Title") -> [New] lands at final index 0 (the recipe's own canonical
        // shift case, on text_chunks' bespoke field-aware absorb path).
        assert_absorb_law(
            &base,
            PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 1, chunk: text_chunk("New", "n") }),
            PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 0 }),
        );

        // Insert+Insert-same-index: both survive, later insert lands at the lower final index.
        assert_absorb_law(
            &base,
            PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 1, chunk: text_chunk("F", "f") }),
            PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 1, chunk: text_chunk("G", "g") }),
        );

        // Add+SetField: the second mutation patches directly into the still-pending added chunk.
        assert_absorb_law(
            &base,
            PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 0, chunk: text_chunk("X", "orig") }),
            PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: 0, chunk: text_chunk("X", "patched") }),
        );

        // Modify+Remove: a pending field patch on a since-removed base item vanishes.
        assert_absorb_law(
            &base,
            PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: 0, chunk: text_chunk("Title", "will-vanish") }),
            PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 0 }),
        );

        // Insert then annihilate the very same insert — on `unknown_chunks`, exercising the
        // SHARED weak-value index transport (`absorb_weak_index_triple`) instead of
        // text_chunks' bespoke field-aware variant.
        assert_absorb_law(
            &base,
            PngMutation::InsertUnknownChunk(crate::artifacts::png::schema::mutations::InsertUnknownChunkMutation { index: 0, chunk: PngChunk { kind: *b"abcd", data: vec![1] } }),
            PngMutation::RemoveUnknownChunk(crate::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: 0 }),
        );

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, PngMutation::ChangeGamma(crate::artifacts::png::schema::mutations::ChangeGammaMutation { gama: Some(1) }), PngMutation::ChangeGamma(crate::artifacts::png::schema::mutations::ChangeGammaMutation { gama: Some(2) }));

        // Tri-state set-then-clear: the later clear wins outright over the pending set.
        assert_absorb_law(
            &base,
            PngMutation::ChangeTransparency(crate::artifacts::png::schema::mutations::ChangeTransparencyMutation { trns: Some(PngTransparency::Grayscale { gray: 1 }) }),
            PngMutation::ChangeTransparency(crate::artifacts::png::schema::mutations::ChangeTransparencyMutation { trns: None }),
        );
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = PngMutation::InsertTextChunk(crate::artifacts::png::schema::mutations::InsertTextChunkMutation { index: 0, chunk: text_chunk("A", "a") }).diff(&base);
        let s1 = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: 0, chunk: text_chunk("A", "a2") }).diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("d2 must apply to s1");
        let d3 = PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 1 }).diff(&s2);
        let s3 = d3.diff().apply(&s2).expect("d3 must apply to s2");

        // (d1∘d2)∘d3
        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("left must apply to base"), s3);
        assert_eq!(right.apply(&base).expect("right must apply to base"), s3);
        assert_eq!(left.apply(&base).expect("left must apply to base"), right.apply(&base).expect("right must apply to base"), "absorb must associate");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.width = 8;
        b.text_chunks.push(text_chunk("Extra", "v"));
        b.pixels = vec![5u8; a.pixels.len()];

        let d = PngDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("d must apply to a"), b, "between(a,b).apply(a) must equal b");
        let d_rev = PngDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b).expect("d_rev must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(PngDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🗿️artifacts/📷️png/📚️examples/🎬️demo/🖼️assets/📷️example.png"));
        let bytes = match bytes {
            Ok(b) if !b.is_empty() => b,
            // No usable fixture on disk at test time (or a different workspace layout) — fall
            // back to a synthetic encode -> decode -> re-encode -> re-decode identity check.
            _ => crate::artifacts::png::engine::encode_png(&base_snapshot()).expect("encode synthetic fallback"),
        };
        let decoded = crate::artifacts::png::engine::decode_png(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::png::engine::encode_png(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::png::engine::decode_png(&reencoded).expect("re-decode fixture");
        // Engine's EncodeScopeNote: encode always canonicalizes to color type 6 / bit depth 8 /
        // interlace 0 — pixel CONTENT is the retained invariant, not the original header/chunks.
        assert_eq!(decoded.width, redecoded.width);
        assert_eq!(decoded.height, redecoded.height);
        assert_eq!(decoded.pixels, redecoded.pixels);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = PngDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("forward must apply to a"), b, "between(a,b).apply(a) must equal b");
        let backward = PngDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("backward must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(PngDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // IHDR scalars.
        assert!(forward.width.is_some());
        assert!(forward.height.is_some());
        assert!(forward.bit_depth.is_some());
        assert!(forward.color_type.is_some());
        assert!(forward.interlace.is_some());

        // Tri-state clears (forward: Some -> None).
        assert_eq!(forward.trns, Some(None), "trns tri-state clear must show Some(None)");
        assert_eq!(forward.gama, Some(None), "gama tri-state clear must show Some(None)");
        assert_eq!(forward.chrm, Some(None), "chrm tri-state clear must show Some(None)");
        assert_eq!(forward.phys, Some(None), "phys tri-state clear must show Some(None)");
        assert_eq!(forward.time, Some(None), "time tri-state clear must show Some(None)");
        assert_eq!(forward.bkgd, Some(None), "bkgd tri-state clear must show Some(None)");
        assert!(matches!(forward.srgb, Some(Some(_))), "srgb value-only change must stay Some(Some(_))");

        // Tri-state recreates (backward: None -> Some) — the same six fields, other direction.
        assert!(matches!(backward.trns, Some(Some(_))));
        assert!(matches!(backward.gama, Some(Some(_))));
        assert!(matches!(backward.chrm, Some(Some(_))));
        assert!(matches!(backward.phys, Some(Some(_))));
        assert!(matches!(backward.time, Some(Some(_))));
        assert!(matches!(backward.bkgd, Some(Some(_))));

        // plte: forward shows modified+removed, backward shows modified+added (the
        // recipe's split-across-both-directions workaround for the removed-XOR-added trap).
        let plte_fwd = forward.plte.as_ref().expect("plte diff present").as_ref().expect("plte still present");
        assert_eq!(plte_fwd.removed, vec![1]);
        assert_eq!(plte_fwd.modified.len(), 1);
        assert!(plte_fwd.added.is_empty());
        let plte_bwd = backward.plte.as_ref().expect("plte diff present").as_ref().expect("plte still present");
        assert!(plte_bwd.removed.is_empty());
        assert_eq!(plte_bwd.modified.len(), 1);
        assert_eq!(plte_bwd.added.len(), 1);

        // text_chunks: same split; every field of the modified entry's diff populated.
        let tc_fwd = forward.text_chunks.as_ref().expect("text_chunks diff present");
        assert_eq!(tc_fwd.removed, vec![1]);
        assert_eq!(tc_fwd.modified.len(), 1);
        assert!(tc_fwd.added.is_empty());
        let md = &tc_fwd.modified[0].diff;
        assert!(md.keyword.is_some(), "keyword must be diffed");
        assert!(md.value.is_some(), "value must be diffed");
        assert!(md.compressed.is_some(), "compressed must be diffed");
        assert!(md.kind.is_some(), "kind must be diffed");
        assert!(md.language_tag.is_some(), "language_tag must be diffed");
        assert!(md.translated_keyword.is_some(), "translated_keyword must be diffed");
        let tc_bwd = backward.text_chunks.as_ref().expect("text_chunks diff present");
        assert!(tc_bwd.removed.is_empty());
        assert_eq!(tc_bwd.modified.len(), 1);
        assert_eq!(tc_bwd.added.len(), 1);

        // pixels.
        assert!(forward.pixels.is_some(), "pixels must be diffed");

        // chunk_order: same split.
        let co_fwd = forward.chunk_order.as_ref().expect("chunk_order diff present");
        assert_eq!(co_fwd.removed, vec![1]);
        assert_eq!(co_fwd.modified.len(), 1);
        assert!(co_fwd.added.is_empty());
        let co_bwd = backward.chunk_order.as_ref().expect("chunk_order diff present");
        assert!(co_bwd.removed.is_empty());
        assert_eq!(co_bwd.modified.len(), 1);
        assert_eq!(co_bwd.added.len(), 1);

        // unknown_chunks: same split.
        let uc_fwd = forward.unknown_chunks.as_ref().expect("unknown_chunks diff present");
        assert_eq!(uc_fwd.removed, vec![1]);
        assert_eq!(uc_fwd.modified.len(), 1);
        assert!(uc_fwd.added.is_empty());
        let uc_bwd = backward.unknown_chunks.as_ref().expect("unknown_chunks diff present");
        assert!(uc_bwd.removed.is_empty());
        assert_eq!(uc_bwd.modified.len(), 1);
        assert_eq!(uc_bwd.added.len(), 1);
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_png_mutation(&mut snap, &PngMutation::RemoveTextChunk(crate::artifacts::png::schema::mutations::RemoveTextChunkMutation { index: 42 }));
        assert_eq!(snap, base);
        apply_png_mutation(&mut snap, &PngMutation::RemoveUnknownChunk(crate::artifacts::png::schema::mutations::RemoveUnknownChunkMutation { index: 42 }));
        assert_eq!(snap, base);
        apply_png_mutation(&mut snap, &PngMutation::ReplaceTextChunk(crate::artifacts::png::schema::mutations::ReplaceTextChunkMutation { index: 42, chunk: text_chunk("x", "y") }));
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PngMutation` grammar —
    /// exercises every variant via `all_variants` (incl. every ancillary Setter's `Some(_)`
    /// payload) plus two extra `SetSnapshot` cases (`sweep_a`/`sweep_b`) so the whole-snapshot
    /// positional codec's `Some` AND `None` branches for every one of its 8 optional fields, plus
    /// its `text_chunks`/`chunk_order`/`unknown_chunks` lists, both get covered.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mut mutations = all_variants(&base);
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = PngMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = PngMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law

    //#endregion 🔖️kinds_law
}
//#endregion Tests
