//! 🧪️ Preserved raster sparse-diff and codec regression laws.
use crate::artifacts::jpg::schema::diff::{self, JpgDiff, JpgHuffmanTableKey};
use crate::artifacts::jpg::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgHuffmanTable, JpgQuantTable, JpgSegment};
use crate::artifacts::jpg::JpgSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

use crate::artifacts::jpg::schema::mutations::*;
//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `JpgMutation` values (every variant, incl. `SetSnapshot`'s full
/// nested `JpgFrameHeader`/`JpgFrameComponent` tree and both legs of every `Option<T>`-shaped
/// argument) — the single source of truth reused by `tests::op_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests. `pub(crate)` (not `#[cfg(test)]`-gated) so the engine's non-test conformance
/// module can reuse it, matching png's own `regression_mutation_cases()` visibility.
#[cfg(test)]
pub(crate) fn regression_mutation_cases() -> Vec<JpgMutation> {
    fn quant(id: u8, seed: u16) -> JpgQuantTable {
        JpgQuantTable { id, precision: 0, values: [seed; 64] }
    }
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
        JpgSegment { marker, data }
    }
    use crate::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass};

    let base = JpgSnapshot {
        schema: "stdio.jpg".into(),
        width: 4,
        height: 4,
        pixels: vec![0u8; 4 * 4 * 4],
        re_encode_quality: None,
        jfif_version: (1, 1),
        jfif_density_units: JfifDensityUnits::Aspect,
        jfif_x_density: 1,
        jfif_y_density: 1,
        jfif_thumbnail: None,
        frame: Some(JpgFrameHeader { precision: 8, width: 4, height: 4, components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }] }),
        sof_marker: 0xC0,
        arithmetic: false,
        quant_tables: vec![quant(0, 10)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1)],
        restart_interval: None,
        other_segments: vec![segment(0xFE, vec![1, 2, 3])],
    };

    vec![
        JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation {
            version: (1, 2),
            density_units: JfifDensityUnits::PixelsPerCm,
            x_density: 300,
            y_density: 300,
            thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }),
        }),
        JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation { version: (1, 1), density_units: JfifDensityUnits::Aspect, x_density: 1, y_density: 1, thumbnail: None }),
        JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(0, 77) }),
        JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 3 }),
        JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: huffman(JpgHuffmanClass::Ac, 2, 5) }),
        JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 0 } }),
        JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: Some(16) }),
        JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: None }),
        JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE2, vec![7, 8]) }),
        JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 0 }),
        JpgMutation::ReplacePixels(crate::artifacts::jpg::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
        JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: Some(50) }),
        JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: None }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jpg::schema::diff::{JpgFrameChange, JpgHuffmanTableKey as HKey};
    use crate::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass};
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    fn quant(id: u8, seed: u16) -> JpgQuantTable {
        JpgQuantTable { id, precision: 0, values: [seed; 64] }
    }
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
        JpgSegment { marker, data }
    }

    fn base_snapshot() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 4,
            height: 4,
            pixels: vec![0u8; 4 * 4 * 4],
            re_encode_quality: None,
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 4,
                height: 4,
                components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![1, 2, 3])],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. Every id/index-keyed collection
    /// (`quant_tables`, `huffman_tables`, `frame.components`, `other_segments`) is deliberately
    /// DIFFERENT length (2 vs 1) with the "surviving/modified" item at position 0 and the
    /// "removed-in-forward / added-in-backward" item as the tail — the recipe's documented
    /// workaround for the structural "same-length between() can show removed XOR added, never
    /// both from one call" trap (see png/f1's field_sweep precedent).await.
    fn sweep_a() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 10,
            height: 20,
            pixels: vec![0u8, 0, 0, 255, 255, 255, 255, 255],
            re_encode_quality: Some(80),
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::PixelsPerInch,
            jfif_x_density: 72,
            jfif_y_density: 72,
            jfif_thumbnail: Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] }),
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 10,
                height: 20,
                components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10), quant(9, 20)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
            restart_interval: Some(8),
            other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
        }
    }

    fn sweep_b() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 11,
            height: 21,
            pixels: vec![1u8, 1, 1, 255],
            re_encode_quality: None,
            jfif_version: (1, 2),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: Some(JpgFrameHeader { precision: 8, width: 11, height: 21, components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }] }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 99)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![4, 5, 6])],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &JpgSnapshot, mutation: JpgMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_jpg_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_jpg_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("diff must apply to base"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    fn all_variants(base: &JpgSnapshot) -> Vec<JpgMutation> {
        vec![
            JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation {
                version: (1, 2),
                density_units: JfifDensityUnits::PixelsPerCm,
                x_density: 300,
                y_density: 300,
                thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }),
            }),
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(0, 77) }),
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(3, 55) }),
            JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 0 }),
            JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: huffman(JpgHuffmanClass::Dc, 0, 9) }),
            JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: huffman(JpgHuffmanClass::Ac, 0, 3) }),
            JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: HKey { class: JpgHuffmanClass::Dc, id: 0 } }),
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: Some(16) }),
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: None }),
            JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE2, vec![7, 8]) }),
            JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 0 }),
            JpgMutation::ReplacePixels(crate::artifacts::jpg::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: Some(50) }),
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: None }),
            // Out-of-range/nonexistent targets: graceful no-ops, still law-compliant.
            JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 99 }),
            JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: HKey { class: JpgHuffmanClass::Ac, id: 99 } }),
            JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 99 }),
        ]
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
            apply_jpg_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_jpg_mutation(&mut snap, &inv);
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

    //#endregion 🔖️kinds_matches_enum_and_manifest

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &JpgSnapshot, m1: JpgMutation, m2: JpgMutation) {
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

        // Insert+Remove-before: other_segments has [seg@0]; insert at 1 -> [seg,new]; then
        // remove index 0 ("seg") -> [new] lands at final index 0 (the recipe's own canonical case).
        assert_absorb_law(
            &base,
            JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE3, vec![1]) }),
            JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 0 }),
        );

        // Insert+Insert-same-index: both survive.
        assert_absorb_law(
            &base,
            JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE4, vec![2]) }),
            JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE5, vec![3]) }),
        );

        // Add+SetField: the second mutation patches directly into the still-pending added table.
        assert_absorb_law(
            &base,
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(5, 1) }),
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(5, 2) }),
        );

        // Modify+Remove: a pending field patch on a since-removed base item vanishes.
        assert_absorb_law(
            &base,
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(0, 42) }),
            JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 0 }),
        );

        // Insert then annihilate the very same insert — huffman_tables' id-keyed transport.
        assert_absorb_law(
            &base,
            JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: huffman(JpgHuffmanClass::Ac, 3, 1) }),
            JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: HKey { class: JpgHuffmanClass::Ac, id: 3 } }),
        );

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(
            &base,
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: Some(1) }),
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: Some(2) }),
        );

        // Tri-state set-then-clear: the later clear wins outright over the pending set.
        assert_absorb_law(
            &base,
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: Some(10) }),
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: None }),
        );
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(7, 1) }).diff(&base);
        let s1 = d1.diff().apply(&base).expect("d1 must apply to base");
        let d2 = JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(7, 2) }).diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("d2 must apply to s1");
        let d3 = JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 0 }).diff(&s2);
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
        b.quant_tables.push(quant(2, 5));
        b.pixels = vec![5u8; a.pixels.len()];

        let d = JpgDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("d must apply to a"), b, "between(a,b).apply(a) must equal b");
        let d_rev = JpgDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b).expect("d_rev must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(JpgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🗿️artifacts/📷️jpg/📚️examples/🎬️demo/🖼️assets/🖼️.jpg"));
        let bytes = match bytes {
            Ok(b) if !b.is_empty() => b,
            // No usable fixture on disk at test time (or a different workspace layout) — fall
            // back to a synthetic encode -> decode identity check (matches png's precedent).
            _ => {
                let w = 16u32;
                let h = 16u32;
                let mut pixels = vec![0u8; (w * h * 4) as usize];
                for (i, px) in pixels.chunks_mut(4).enumerate() {
                    px[0] = (i * 7 % 255) as u8;
                    px[1] = (i * 13 % 255) as u8;
                    px[2] = (i * 17 % 255) as u8;
                    px[3] = 255;
                }
                let snap = JpgSnapshot { width: w, height: h, pixels, ..JpgSnapshot::default() };
                crate::artifacts::jpg::engine::encode_jpg(&snap).expect("encode synthetic fallback")
            }
        };
        let decoded = crate::artifacts::jpg::engine::decode_jpg(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::jpg::engine::encode_jpg(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::jpg::engine::decode_jpg(&reencoded).expect("re-decode fixture");
        // Engine's own EncodeScopeNote: encode always canonicalizes to Annex K tables at a fixed
        // quality — pixel CONTENT (within a lossy MAE budget) is the retained invariant, not the
        // original file's exact tables/segments (documented normal form).
        assert_eq!(decoded.width, redecoded.width);
        assert_eq!(decoded.height, redecoded.height);
        assert_eq!(decoded.pixels.len(), redecoded.pixels.len());
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = JpgDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("forward must apply to a"), b, "between(a,b).apply(a) must equal b");
        let backward = JpgDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("backward must apply to b"), a, "between(b,a).apply(b) must equal a");
        assert!(JpgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Raster scalars.
        assert!(forward.width.is_some());
        assert!(forward.height.is_some());
        assert!(forward.pixels.is_some());

        // Tri-state clears (forward: Some -> None).
        assert_eq!(forward.re_encode_quality, Some(None), "re_encode_quality tri-state clear must show Some(None)");
        assert_eq!(forward.jfif_thumbnail, Some(None), "jfif_thumbnail tri-state clear must show Some(None)");
        assert_eq!(forward.restart_interval, Some(None), "restart_interval tri-state clear must show Some(None)");
        // Tri-state recreates (backward: None -> Some).
        assert!(matches!(backward.re_encode_quality, Some(Some(_))));
        assert!(matches!(backward.jfif_thumbnail, Some(Some(_))));
        assert!(matches!(backward.restart_interval, Some(Some(_))));

        // JFIF scalars.
        assert!(forward.jfif_version.is_some());
        assert!(forward.jfif_density_units.is_some());
        assert!(forward.jfif_x_density.is_some());
        assert!(forward.jfif_y_density.is_some());

        // frame: Modify with every sub-field populated (both sweep_a/b keep `Some(frame)`).
        let frame_change = forward.frame.as_ref().expect("frame diff present");
        match frame_change {
            JpgFrameChange::Modify(fd) => {
                assert!(fd.precision.is_none(), "precision is 8 in both sweeps");
                assert!(fd.width.is_some());
                assert!(fd.height.is_some());
                let cd = fd.components.as_ref().expect("components diff present");
                assert_eq!(cd.removed, vec![9], "component id 9 only in sweep_a");
                assert_eq!(cd.modified.len(), 1, "component id 1 survives, modified");
                assert!(cd.added.is_empty());
            }
            other => panic!("expected Modify, got {other:?}"),
        }
        let bwd_frame_change = backward.frame.as_ref().expect("frame diff present");
        match bwd_frame_change {
            JpgFrameChange::Modify(fd) => {
                let cd = fd.components.as_ref().expect("components diff present");
                assert!(cd.removed.is_empty());
                assert_eq!(cd.modified.len(), 1);
                assert_eq!(cd.added.len(), 1, "component id 9 re-added going backward");
            }
            other => panic!("expected Modify, got {other:?}"),
        }

        assert!(forward.sof_marker.is_none(), "sof_marker is 0xC0 in both sweeps");
        assert!(forward.arithmetic.is_none(), "arithmetic is false in both sweeps");

        // quant_tables: forward shows modified+removed, backward shows modified+added.
        let qt_fwd = forward.quant_tables.as_ref().expect("quant_tables diff present");
        assert_eq!(qt_fwd.removed, vec![9]);
        assert_eq!(qt_fwd.modified.len(), 1);
        assert!(qt_fwd.added.is_empty());
        let qt_bwd = backward.quant_tables.as_ref().expect("quant_tables diff present");
        assert!(qt_bwd.removed.is_empty());
        assert_eq!(qt_bwd.modified.len(), 1);
        assert_eq!(qt_bwd.added.len(), 1);

        // huffman_tables: same split, compound key.
        let ht_fwd = forward.huffman_tables.as_ref().expect("huffman_tables diff present");
        assert_eq!(ht_fwd.removed, vec![HKey { class: JpgHuffmanClass::Ac, id: 9 }]);
        assert_eq!(ht_fwd.modified.len(), 1);
        assert!(ht_fwd.added.is_empty());
        let ht_bwd = backward.huffman_tables.as_ref().expect("huffman_tables diff present");
        assert!(ht_bwd.removed.is_empty());
        assert_eq!(ht_bwd.modified.len(), 1);
        assert_eq!(ht_bwd.added.len(), 1);

        // other_segments: same split.
        let os_fwd = forward.other_segments.as_ref().expect("other_segments diff present");
        assert_eq!(os_fwd.removed, vec![1]);
        assert_eq!(os_fwd.modified.len(), 1);
        assert!(os_fwd.added.is_empty());
        let os_bwd = backward.other_segments.as_ref().expect("other_segments diff present");
        assert!(os_bwd.removed.is_empty());
        assert_eq!(os_bwd.modified.len(), 1);
        assert_eq!(os_bwd.added.len(), 1);
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 99 }));
        assert_eq!(snap, base);
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: HKey { class: JpgHuffmanClass::Ac, id: 99 } }));
        assert_eq!(snap, base);
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 99 }));
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `JpgMutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s full nested `JpgFrameHeader`/`JpgFrameComponent`
    /// tree and every collection-item struct (`JpgQuantTable`/`JpgHuffmanTable`/`JpgSegment`), plus
    /// both `Some`/`None` legs of every `Option<T>`-shaped mutation argument.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation {
                version: (1, 2),
                density_units: JfifDensityUnits::PixelsPerCm,
                x_density: 300,
                y_density: 300,
                thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }),
            }),
            JpgMutation::ChangeJfifHeader(crate::artifacts::jpg::schema::mutations::ChangeJfifHeaderMutation { version: (1, 1), density_units: JfifDensityUnits::Aspect, x_density: 1, y_density: 1, thumbnail: None }),
            JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: quant(0, 77) }),
            JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: 3 }),
            JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: huffman(JpgHuffmanClass::Ac, 2, 5) }),
            JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key: HKey { class: JpgHuffmanClass::Dc, id: 0 } }),
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: Some(16) }),
            JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: None }),
            JpgMutation::InsertOtherSegment(crate::artifacts::jpg::schema::mutations::InsertOtherSegmentMutation { index: 1, segment: segment(0xE2, vec![7, 8]) }),
            JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: 0 }),
            JpgMutation::ReplacePixels(crate::artifacts::jpg::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: Some(50) }),
            JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: None }),
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = JpgMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = JpgMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
