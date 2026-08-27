//! 🧪️ Preserved raster sparse-diff and codec regression laws.
use crate::artifacts::bmp::schema::diff::{diff_set_snapshot, BmpDiff, BmpPaletteAdded, BmpPaletteDiff, BmpPaletteModified};
use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::BmpSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

use crate::artifacts::bmp::schema::mutations::*;
//#region 🔖️DemoFixtures
/// 🧬️ 4x3 8-bit indexed base with a small, non-trivial palette — enough to exercise
/// insert/remove/set-entry mutations meaningfully. Module-level (not nested in `mod tests`,
/// mirroring `stdio.png`'s own `demo_base_snapshot()` placement) so `regression_mutation_cases()`
/// below AND `⚙️engine/🦀️component.rs`'s `conformance_laws` module can both reach it.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entry(b: u8, g: u8, r: u8, reserved: u8) -> BmpPaletteEntry {
    BmpPaletteEntry { b, g, r, reserved }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_snapshot() -> BmpSnapshot {
    BmpSnapshot {
        schema: "stdio.bmp".into(),
        header_size: 40,
        width: 4,
        height: 3,
        row_order: BmpRowOrder::BottomUp,
        planes: 1,
        bits_per_pixel: 8,
        compression: 0,
        image_size: 48,
        x_pixels_per_meter: 2835,
        y_pixels_per_meter: 2835,
        colors_used: 3,
        colors_important: 0,
        palette: vec![entry(0, 0, 255, 0), entry(0, 255, 0, 0), entry(255, 0, 0, 0)],
        pixels: vec![0u8; 4 * 3 * 4],
    }
}

/// 🧬️ Canonical "differs in every mutable field" snapshot A: every scalar header field set
/// to one value, a 2-entry palette (index 0 stable, index 1 will be modified in every
/// field), 4x3 pixels.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_a() -> BmpSnapshot {
    BmpSnapshot {
        schema: "stdio.bmp".into(),
        header_size: 40,
        width: 4,
        height: 3,
        row_order: BmpRowOrder::BottomUp,
        planes: 1,
        bits_per_pixel: 8,
        compression: 0,
        image_size: 100,
        x_pixels_per_meter: 1000,
        y_pixels_per_meter: 2000,
        colors_used: 2,
        colors_important: 1,
        palette: vec![entry(10, 20, 30, 0), entry(1, 2, 3, 0)],
        pixels: (0..(4 * 3 * 4)).map(|i| (i % 256) as u8).collect(),
    }
}
/// 🧬️ Sweep B: every scalar header field flips to a DIFFERENT value, the palette grows to
/// 3 entries (index 0 stable, index 1 modified in every field vs. `sweep_a`, index 2
/// brand-new — asymmetric length on purpose, see `~/.claude/plans/…journal.md`'s F1-txt
/// trap note: a single same-length `between()` call cannot show both `removed` AND `added`
/// from one direction, so this fixture is deliberately asymmetric and the field_sweep test
/// below splits its collection assertions across BOTH `between()` directions), and pixels
/// change size (8x6) + content.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_b() -> BmpSnapshot {
    BmpSnapshot {
        schema: "stdio.bmp".into(),
        header_size: 56,
        width: 8,
        height: 6,
        row_order: BmpRowOrder::TopDown,
        planes: 2,
        bits_per_pixel: 24,
        compression: 3,
        image_size: 200,
        x_pixels_per_meter: 3000,
        y_pixels_per_meter: 4000,
        colors_used: 5,
        colors_important: 2,
        palette: vec![entry(10, 20, 30, 0), entry(99, 88, 77, 1), entry(200, 201, 202, 0)],
        pixels: (0..(8 * 6 * 4)).map(|i| ((i * 7 + 3) % 256) as u8).collect(),
    }
}

/// ✅️ P2-FG2: every `BmpMutation` variant (incl. two out-of-range no-op cases) built off
/// `base_snapshot()` — the single case list `mutation_diff_law`/`inverse_law`/
/// `op_text_binary_roundtrip_law` (`mod tests` below) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️component.rs`'s `conformance_laws` module) all exercise —
/// same consolidation `stdio.png`'s own `regression_mutation_cases()` already made (single source of
/// truth, per this repo's own CLAUDE.md).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn regression_mutation_cases() -> Vec<BmpMutation> {
    let base = base_snapshot();
    vec![
        BmpMutation::ChangeHeaderFields(crate::artifacts::bmp::schema::mutations::ChangeHeaderFieldsMutation {
            header_size: Some(56),
            width: Some(9),
            height: None,
            row_order: Some(BmpRowOrder::TopDown),
            planes: None,
            bits_per_pixel: None,
            compression: None,
            image_size: None,
            x_pixels_per_meter: Some(5000),
            y_pixels_per_meter: None,
            colors_used: None,
            colors_important: None,
        }),
        BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 1, entry: entry(9, 9, 9, 0) }),
        BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: 0 }),
        BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: 2, entry: entry(1, 1, 1, 1) }),
        BmpMutation::ReplacePixelData(crate::artifacts::bmp::schema::mutations::ReplacePixelDataMutation { pixels: vec![7u8; base.pixels.len()] }),
        // Out-of-range targets: graceful no-ops, still law-compliant.
        BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: 99 }),
        BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: 99, entry: entry(0, 0, 0, 0) }),
    ]
}
//#endregion 🔖️DemoFixtures

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for m in regression_mutation_cases() {
            let diff = m.diff(&base);
            let expected = diff.diff().apply(&base).expect("diff must apply to base");

            let mut via_apply = base.clone();
            let returned_diff = apply_bmp_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_bmp_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        for m in regression_mutation_cases() {
            // 🔁️ mutation-level round trip
            let mut forward = base.clone();
            apply_bmp_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_bmp_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            // 🔁️ diff-level round trip
            let d = m.diff(&base);
            let mid = d.diff().apply(&base).unwrap();
            let back = d.diff().inverse(&base).apply(&mid).unwrap();
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
        let d1 = BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 2, entry: entry(1, 2, 3, 0) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: 0 }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f) + Insert(2,g): both must survive (fixes the old op-slot LWW bug).
        let d1 = BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 2, entry: entry(9, 0, 0, 0) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 2, entry: entry(0, 9, 0, 0) }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.palette.len(), base.palette.len() + 2, "both inserts must survive");

        // 🧩 Add + SetField (patch into the added payload).
        let d1 = BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 1, entry: entry(1, 1, 1, 1) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: 1, entry: entry(2, 2, 2, 2) }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetPaletteEntry absorb mismatch");
        assert_eq!(after.palette[1], entry(2, 2, 2, 2));

        // 🧩 Modify + Remove: modifying then removing the same entry collapses to a removal.
        let d1 = BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: 1, entry: entry(5, 5, 5, 5) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: 1 }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: 0, entry: entry(1, 0, 0, 0) }).diff(&base);
        let s1 = d1.diff().apply(&base).unwrap();
        let d2 = BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: 0, entry: entry(2, 0, 0, 0) }).diff(&s1);
        let s2 = d2.diff().apply(&s1).unwrap();
        let d3 = BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: 2 }).diff(&s2);
        let s3 = d3.diff().apply(&s2).unwrap();

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).unwrap(), s3);
        assert_eq!(right.apply(&base).unwrap(), s3);
        assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap(), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let b = sweep_b();
        assert_eq!(BmpDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(BmpDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(BmpDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let ab = BmpDiff::between(&a, &b);
        assert_eq!(ab.apply(&a).unwrap(), b, "between(a,b).apply(a) == b");
        let ba = BmpDiff::between(&b, &a);
        assert_eq!(ba.apply(&b).unwrap(), a, "between(b,a).apply(b) == a");

        // 🔍 Hand-written per-field assertion: every scalar field of `BmpDiff` is populated —
        // scalar compares have no positional-collision issue, so a single direction suffices.
        assert!(ab.header_size.is_some(), "header_size must be populated");
        assert!(ab.width.is_some(), "width must be populated");
        assert!(ab.height.is_some(), "height must be populated");
        assert!(ab.row_order.is_some(), "row_order must be populated");
        assert!(ab.planes.is_some(), "planes must be populated");
        assert!(ab.bits_per_pixel.is_some(), "bits_per_pixel must be populated");
        assert!(ab.compression.is_some(), "compression must be populated");
        assert!(ab.image_size.is_some(), "image_size must be populated");
        assert!(ab.x_pixels_per_meter.is_some(), "x_pixels_per_meter must be populated");
        assert!(ab.y_pixels_per_meter.is_some(), "y_pixels_per_meter must be populated");
        assert!(ab.colors_used.is_some(), "colors_used must be populated");
        assert!(ab.colors_important.is_some(), "colors_important must be populated");
        assert!(ab.pixels.is_some(), "pixels must be populated");

        // 🧮 `palette` is a genuinely index-keyed collection: on a SAME-length pair, one
        // `between()` call can only ever show `removed` XOR `added`, never both (the F1-txt
        // structural trap) — `sweep_a`/`sweep_b` are deliberately asymmetric (2 vs 3 entries)
        // so each direction proves a different tail kind, exactly as `between_roundtrip_law`'s
        // own two-directions-checked shape already implies is the right level of rigor.
        let pd_ab = ab.palette.as_ref().expect("palette diff must be populated (a->b)");
        assert!(pd_ab.removed.is_empty(), "a->b must not need a removal (palette grows)");
        assert!(!pd_ab.modified.is_empty(), "a->b must show the modified entry");
        assert!(!pd_ab.added.is_empty(), "a->b must show the added entry");
        let modified = &pd_ab.modified[0];
        let old_entry = &a.palette[modified.index];
        assert_ne!(modified.entry.b, old_entry.b, "modified entry must change b");
        assert_ne!(modified.entry.g, old_entry.g, "modified entry must change g");
        assert_ne!(modified.entry.r, old_entry.r, "modified entry must change r");
        assert_ne!(modified.entry.reserved, old_entry.reserved, "modified entry must change reserved");

        let pd_ba = ba.palette.as_ref().expect("palette diff must be populated (b->a)");
        assert!(!pd_ba.removed.is_empty(), "b->a must show the removed entry");
        assert!(!pd_ba.modified.is_empty(), "b->a must show the modified entry");
        assert!(pd_ba.added.is_empty(), "b->a must not need an addition (palette shrinks)");

        assert!(BmpDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `op_text_binary_roundtrip_law` — every variant, incl. the struct-payload
    /// (`SetSnapshot`/`InsertPaletteEntry`/`SetPaletteEntry`) and bare-`Vec<u8>`
    /// (`SetPixelData`) cases (`f6-recon-report.md` §9 STEP-3's mandated shape).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};

        for m in regression_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = BmpMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = BmpMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#endregion 🔖️KindsLaw
}
//#endregion 🧪️Tests
