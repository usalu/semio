//! 🧪️ Preserved raster sparse-diff and codec regression laws.
use crate::artifacts::tiff::schema::diff::{
    self, dec_byte_order, dec_field_type, dec_ifd, dec_ifd_bin, dec_list, dec_str, dec_values, dec_values_bin, enc_byte_order, enc_field_type, enc_ifd, enc_ifd_bin, enc_list, enc_str, enc_values, enc_values_bin, hex_decode, hex_encode, parse_num,
    read_bytes_lp, read_str_lp, split_top_level, strip_brackets, write_bytes_lp, write_str_lp, TiffDiff,
};
#[cfg(test)]
use crate::artifacts::tiff::schema::snapshot::TiffTag;
use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffValues};
use crate::artifacts::tiff::TiffSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

use crate::artifacts::tiff::schema::mutations::*;
//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `TiffMutation` values (every variant, incl. every `TiffValues`
/// field-type family the recursive `SetTag` payload can carry) — the single source of truth
/// reused by `ops_grammar_conformance_law`/`protocol_walk_law` below (`⚙️engine/🦀️component.rs`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn regression_mutation_cases() -> Vec<TiffMutation> {
    vec![
        TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: TiffByteOrder::BigEndian }),
        TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Short, values: TiffValues::Short(vec![1]) }] } }),
        TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 0 }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 282, kind: TiffFieldType::Rational, values: TiffValues::Rational(vec![(72, 1)]) }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 700, kind: TiffFieldType::Undefined, values: TiffValues::Undefined(vec![0xde, 0xad]) }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 33421, kind: TiffFieldType::SRational, values: TiffValues::SRational(vec![(-3, 10)]) }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 65001, kind: TiffFieldType::Float, values: TiffValues::Float(vec![1.5, -2.25]) }),
        TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 65002, kind: TiffFieldType::Double, values: TiffValues::Double(vec![3.14159265358979]) }),
        TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 296 }),
        TiffMutation::ReplacePixels(crate::artifacts::tiff::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; 16] }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn short_tag(tag: u16, v: u16) -> TiffTag {
        TiffTag { tag, kind: TiffFieldType::Short, values: TiffValues::Short(vec![v]) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd {
                pixels: Vec::new(),
                entries: vec![
                    TiffTag { tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageWidth
                    TiffTag { tag: 257, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageLength
                    short_tag(296, 2),                                                                  // ResolutionUnit
                ],
            }],
            pixels: vec![0u8; 4 * 4 * 4],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. `ifds` (index-keyed) and, within
    /// the surviving `ifds[0]`, `entries` (tag-id-keyed) are both deliberately DIFFERENT
    /// length/membership — the recipe's own documented workaround for the structural
    /// "same-length `between()` can show removed XOR added, never both from one call" trap
    /// (see F1's `f1-closer-report.md` §4.4): the IFD-level triple needs the split-across-
    /// directions workaround (positional pairwise matching), while the TAG-level triple is
    /// id-keyed via a `BTreeMap` union, so it genuinely shows removed+modified+added from a
    /// SINGLE `between()` call — no split needed there.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![
                TiffIfd { pixels: Vec::new(), entries: vec![short_tag(300, 1), short_tag(301, 9)] }, // tag 300 survives+changes, 301 removed
                TiffIfd { pixels: Vec::new(), entries: vec![TiffTag { tag: 302, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("gone".into()) }] }, // whole IFD removed in b
            ],
            pixels: vec![0u8, 0, 0, 255, 1, 1, 1, 255],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::BigEndian,
            ifds: vec![TiffIfd { pixels: Vec::new(), entries: vec![short_tag(300, 2), TiffTag { tag: 303, kind: TiffFieldType::Long, values: TiffValues::Long(vec![42]) }] }], // 300 changed, 303 added
            pixels: vec![9u8, 9, 9, 255],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_mutation_diff_law(base: &TiffSnapshot, mutation: TiffMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_tiff_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_tiff_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("diff must apply to base"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn all_variants(base: &TiffSnapshot) -> Vec<TiffMutation> {
        vec![
            TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: TiffByteOrder::BigEndian }),
            TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![short_tag(270, 1)] } }),
            TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 0 }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) }), // modify existing
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) }), // create new
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 296 }),
            TiffMutation::ReplacePixels(crate::artifacts::tiff::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
            // Out-of-range targets: graceful no-ops, still law-compliant.
            TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 99 }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 99, tag: 1 }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 9999 }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 99, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            assert_mutation_diff_law(&base, m);
        }
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_tiff_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_tiff_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.diff().apply(&base).unwrap();
            let inv_d = d.diff().inverse(&base);
            assert_eq!(inv_d.apply(&mutated).unwrap(), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_law(base: &TiffSnapshot, m1: TiffMutation, m2: TiffMutation) {
        let d1 = m1.diff(base);
        let mid = d1.diff().apply(base).unwrap();
        let d2 = m2.diff(&mid);
        let sequential = d2.diff().apply(&mid).unwrap();

        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(base).unwrap(), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // IFD-level (index-keyed), Insert+Remove-before: insert a new IFD at 1 -> [ifd0,new],
        // then remove index 0 -> [new] lands at final index 0 (the recipe's own canonical
        // shift case).
        assert_absorb_law(
            &base,
            TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![short_tag(1, 1)] } }),
            TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 0 }),
        );

        // IFD-level, Insert+Insert-same-index: both survive.
        assert_absorb_law(
            &base,
            TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![short_tag(2, 2)] } }),
            TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![short_tag(3, 3)] } }),
        );

        // Tag-level (id-keyed), Add+SetField: the second mutation patches directly into the
        // still-pending added tag.
        assert_absorb_law(
            &base,
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("orig".into()) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("patched".into()) }),
        );

        // Tag-level, Modify+Remove: a pending field patch on a since-removed base tag vanishes.
        assert_absorb_law(
            &base,
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![7]) }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 296 }),
        );

        // Tag-level, Add then annihilate the very same add.
        assert_absorb_law(
            &base,
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 317, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 317 }),
        );

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(
            &base,
            TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: TiffByteOrder::BigEndian }),
            TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: TiffByteOrder::LittleEndian }),
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a".into()) }).diff(&base);
        let s1 = d1.diff().apply(&base).unwrap();
        let d2 = TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a2".into()) }).diff(&s1);
        let s2 = d2.diff().apply(&s1).unwrap();
        let d3 = TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 296 }).diff(&s2);
        let s3 = d3.diff().apply(&s2).unwrap();

        // (d1∘d2)∘d3
        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).unwrap(), s3);
        assert_eq!(right.apply(&base).unwrap(), s3);
        assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap(), "absorb must associate");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.byte_order = TiffByteOrder::BigEndian;
        b.ifds[0].entries.push(TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("Extra".into()) });
        b.pixels = vec![5u8; a.pixels.len()];

        let d = TiffDiff::between(&a, &b);
        assert_eq!(d.apply(&a).unwrap(), b, "between(a,b).apply(a) must equal b");
        let d_rev = TiffDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b).unwrap(), a, "between(b,a).apply(b) must equal a");
        assert!(TiffDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let bytes = crate::artifacts::tiff::engine::encode_tiff(&base_snapshot()).expect("encode synthetic fixture");
        let decoded = crate::artifacts::tiff::engine::decode_tiff(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::tiff::engine::encode_tiff(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::tiff::engine::decode_tiff(&reencoded).expect("re-decode fixture");
        // `base_snapshot()` is single-IFD, so this only exercises IFD 0's own canonicalization
        // invariant (see `../../🚪️io/🦀️component.rs`'s `MultiIfdEncodeScopeNote`: IFD 0's
        // strip/geometry tags are always recomputed fresh from `pixels`) — pixel CONTENT + carried
        // non-core tags are the retained invariant. Real multi-IFD chain preservation is exercised
        // by `../../🚪️io/🦀️component.rs`'s own `multi_ifd_round_trip_preserves_every_ifd` and
        // `insert_ifd_and_remove_ifd_are_observable_through_the_codec` tests.
        assert_eq!(decoded.width(), redecoded.width());
        assert_eq!(decoded.height(), redecoded.height());
        assert_eq!(decoded.pixels, redecoded.pixels);
        assert_eq!(decoded.tag(296), redecoded.tag(296), "carried non-core tag must survive a second round trip");
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = TiffDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).unwrap(), b, "between(a,b).apply(a) must equal b");
        let backward = TiffDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).unwrap(), a, "between(b,a).apply(&b) must equal a");
        assert!(TiffDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        assert_eq!(forward.byte_order, Some(TiffByteOrder::BigEndian));
        assert_eq!(backward.byte_order, Some(TiffByteOrder::LittleEndian));
        assert!(forward.pixels.is_some(), "pixels must be diffed");
        assert!(backward.pixels.is_some());

        // ifds (index-keyed): forward shows removed(IFD1)+modified(IFD0); backward shows
        // added(IFD1)+modified(IFD0) — the split-across-both-directions workaround.
        let fwd_ifds = forward.ifds.as_ref().expect("ifds diff present (forward)");
        assert_eq!(fwd_ifds.removed, vec![1]);
        assert_eq!(fwd_ifds.modified.len(), 1);
        assert!(fwd_ifds.added.is_empty());
        let bwd_ifds = backward.ifds.as_ref().expect("ifds diff present (backward)");
        assert!(bwd_ifds.removed.is_empty());
        assert_eq!(bwd_ifds.modified.len(), 1);
        assert_eq!(bwd_ifds.added.len(), 1);

        // entries within ifds[0] (tag-id-keyed): a SINGLE between() call genuinely shows
        // removed+modified+added together (id-keyed union, no positional-pairing trap).
        let fwd_entries = &fwd_ifds.modified[0].diff.entries;
        assert_eq!(fwd_entries.removed, vec![301]);
        assert_eq!(fwd_entries.modified.len(), 1);
        assert_eq!(fwd_entries.modified[0].tag, 300);
        assert_eq!(fwd_entries.added.len(), 1);
        assert_eq!(fwd_entries.added[0].tag, 303);

        let bwd_entries = &bwd_ifds.modified[0].diff.entries;
        assert_eq!(bwd_entries.removed, vec![303]);
        assert_eq!(bwd_entries.modified.len(), 1);
        assert_eq!(bwd_entries.modified[0].tag, 300);
        assert_eq!(bwd_entries.added.len(), 1);
        assert_eq!(bwd_entries.added[0].tag, 301);
    }
    //#endregion 🔖️field_sweep

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 42 }));
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 42, tag: 1 }));
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 9999 }));
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 42, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) }));
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `TiffMutation` grammar —
    /// exercises every variant incl. `SetTag`/`SetSnapshot`'s bare `TiffValues` payload across
    /// every one of the 12 field-type variants (`Rational`/`SRational` pair lists, `Ascii`/`Byte`/
    /// `Undefined` hex, signed and unsigned numeric lists, `Float`/`Double`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            TiffMutation::ChangeByteOrder(crate::artifacts::tiff::schema::mutations::ChangeByteOrderMutation { byte_order: TiffByteOrder::BigEndian }),
            TiffMutation::InsertIfd(crate::artifacts::tiff::schema::mutations::InsertIfdMutation { index: 1, ifd: TiffIfd { pixels: Vec::new(), entries: vec![short_tag(270, 1)] } }),
            TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 0 }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 258, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 282, kind: TiffFieldType::Rational, values: TiffValues::Rational(vec![(72, 1), (0, 1)]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 700, kind: TiffFieldType::Undefined, values: TiffValues::Undefined(vec![0xde, 0xad, 0xbe, 0xef]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1, 2, 3]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 2, kind: TiffFieldType::SByte, values: TiffValues::SByte(vec![-1, -2, 3]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 3, kind: TiffFieldType::SShort, values: TiffValues::SShort(vec![-100, 200]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 4, kind: TiffFieldType::SLong, values: TiffValues::SLong(vec![-100000]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 5, kind: TiffFieldType::SRational, values: TiffValues::SRational(vec![(-3, 10)]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 6, kind: TiffFieldType::Float, values: TiffValues::Float(vec![1.5, -2.25]) }),
            TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: 0, tag: 7, kind: TiffFieldType::Double, values: TiffValues::Double(vec![3.14159265358979]) }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 0, tag: 296 }),
            TiffMutation::ReplacePixels(crate::artifacts::tiff::schema::mutations::ReplacePixelsMutation { pixels: vec![9u8; base.pixels.len()] }),
            // Out-of-range targets: still valid grammar, no special-casing needed.
            TiffMutation::RemoveIfd(crate::artifacts::tiff::schema::mutations::RemoveIfdMutation { index: 99 }),
            TiffMutation::RemoveTag(crate::artifacts::tiff::schema::mutations::RemoveTagMutation { ifd_index: 99, tag: 1 }),
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = TiffMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = TiffMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
