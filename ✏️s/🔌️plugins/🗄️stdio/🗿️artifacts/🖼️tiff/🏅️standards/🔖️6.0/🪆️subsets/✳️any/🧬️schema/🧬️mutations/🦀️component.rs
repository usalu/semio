//! 🧬️ TiffMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TiffDiff` directly via the `schema::diff` builders — apply-and-
//! capture is banned); `inverse()` is handcrafted per variant, index/tag-aware, reading the
//! pre-state it needs from `base`. `apply_tiff_mutation` follows csv/png's proven
//! single-source-of-truth shape: `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`.

use crate::artifacts::tiff::schema::diff::{self, TiffDiff};
use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffValues};
use crate::artifacts::tiff::TiffSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.tiff`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TiffMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: TiffSnapshot,
    },
    /// 🧭️ Replaces the II/MM byte-order mark.
    SetByteOrder {
        byte_order: TiffByteOrder,
    },
    /// ➕️ Inserts a whole IFD at `index` (final position, clamped to `len`).
    InsertIfd {
        index: usize,
        ifd: TiffIfd,
    },
    /// ➖️ Removes the IFD at `index` (no-op if out of range).
    RemoveIfd {
        index: usize,
    },
    /// ✏️ Creates-or-updates one tag entry in `ifds[ifd_index]` (no-op if `ifd_index` is out
    /// of range).
    SetTag {
        ifd_index: usize,
        tag: u16,
        kind: TiffFieldType,
        values: TiffValues,
    },
    /// ➖️ Removes one tag entry from `ifds[ifd_index]` (no-op if out of range or absent).
    RemoveTag {
        ifd_index: usize,
        tag: u16,
    },
    /// 🖼️ Replaces the decoded canonical RGBA8 raster wholesale.
    SetPixels {
        pixels: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (csv/png precedent).
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> TiffDiff {
    let d = <TiffMutation as Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    *snapshot = <TiffDiff as MutationDiff<TiffSnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TiffSnapshot> for TiffMutation {
    type Diff = TiffDiff;

    fn diff(&self, base: &TiffSnapshot) -> Self::Diff {
        match self {
            TiffMutation::NoMutation => TiffDiff::default(),
            TiffMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            TiffMutation::SetByteOrder { byte_order } => diff::diff_set_byte_order(base, *byte_order),
            TiffMutation::InsertIfd { index, ifd } => diff::diff_insert_ifd(base, *index, ifd.clone()),
            TiffMutation::RemoveIfd { index } => diff::diff_remove_ifd(base, *index),
            TiffMutation::SetTag { ifd_index, tag, kind, values } => diff::diff_set_tag(base, *ifd_index, *tag, *kind, values.clone()),
            TiffMutation::RemoveTag { ifd_index, tag } => diff::diff_remove_tag(base, *ifd_index, *tag),
            TiffMutation::SetPixels { pixels } => diff::diff_set_pixels(base, pixels.clone()),
        }
    }

    /// ↩️ Handcrafted, index/tag-aware mutation-level inverses. Out-of-range targets invert to
    /// `NoMutation` (nothing to undo).
    fn inverse(&self, base: &TiffSnapshot) -> Vec<Self> {
        match self {
            TiffMutation::NoMutation => vec![TiffMutation::NoMutation],
            TiffMutation::SetSnapshot { .. } => vec![TiffMutation::SetSnapshot { snapshot: base.clone() }],
            TiffMutation::SetByteOrder { .. } => vec![TiffMutation::SetByteOrder { byte_order: base.byte_order }],
            TiffMutation::InsertIfd { index, .. } => vec![TiffMutation::RemoveIfd { index: (*index).min(base.ifds.len()) }],
            TiffMutation::RemoveIfd { index } => match base.ifds.get(*index) {
                Some(ifd) => vec![TiffMutation::InsertIfd { index: *index, ifd: ifd.clone() }],
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::SetTag { ifd_index, tag, .. } => match base.ifds.get(*ifd_index) {
                Some(ifd) => match ifd.entries.iter().find(|t| t.tag == *tag) {
                    Some(existing) => vec![TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() }],
                    None => vec![TiffMutation::RemoveTag { ifd_index: *ifd_index, tag: *tag }],
                },
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::RemoveTag { ifd_index, tag } => match base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|t| t.tag == *tag)) {
                Some(existing) => vec![TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() }],
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::SetPixels { .. } => vec![TiffMutation::SetPixels { pixels: base.pixels.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for TiffMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for TiffMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::tiff::schema::snapshot::TiffTag;
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    fn short_tag(tag: u16, v: u16) -> TiffTag {
        TiffTag { tag, kind: TiffFieldType::Short, values: TiffValues::Short(vec![v]) }
    }

    fn base_snapshot() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd {
                entries: vec![
                    TiffTag { tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageWidth
                    TiffTag { tag: 257, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageLength
                    short_tag(296, 2), // ResolutionUnit
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
    fn sweep_a() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![
                TiffIfd { entries: vec![short_tag(300, 1), short_tag(301, 9)] }, // tag 300 survives+changes, 301 removed
                TiffIfd { entries: vec![TiffTag { tag: 302, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("gone".into()) }] }, // whole IFD removed in b
            ],
            pixels: vec![0u8, 0, 0, 255, 1, 1, 1, 255],
        }
    }

    fn sweep_b() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::BigEndian,
            ifds: vec![TiffIfd { entries: vec![short_tag(300, 2), TiffTag { tag: 303, kind: TiffFieldType::Long, values: TiffValues::Long(vec![42]) }] }], // 300 changed, 303 added
            pixels: vec![9u8, 9, 9, 255],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &TiffSnapshot, mutation: TiffMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_tiff_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_tiff_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    fn all_variants(base: &TiffSnapshot) -> Vec<TiffMutation> {
        vec![
            TiffMutation::NoMutation,
            TiffMutation::SetSnapshot { snapshot: { let mut s = base.clone(); s.byte_order = TiffByteOrder::BigEndian; s } },
            TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian },
            TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(270, 1)] } },
            TiffMutation::RemoveIfd { index: 0 },
            TiffMutation::SetTag { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) }, // modify existing
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) }, // create new
            TiffMutation::RemoveTag { ifd_index: 0, tag: 296 },
            TiffMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
            // Out-of-range targets: graceful no-ops, still law-compliant.
            TiffMutation::RemoveIfd { index: 99 },
            TiffMutation::RemoveTag { ifd_index: 99, tag: 1 },
            TiffMutation::RemoveTag { ifd_index: 0, tag: 9999 },
            TiffMutation::SetTag { ifd_index: 99, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) },
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
            apply_tiff_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_tiff_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.apply(&base);
            let inv_d = d.inverse(&base);
            assert_eq!(inv_d.apply(&mutated), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &TiffSnapshot, m1: TiffMutation, m2: TiffMutation) {
        let d1 = m1.diff(base);
        let mid = d1.apply(base);
        let d2 = m2.diff(&mid);
        let sequential = d2.apply(&mid);

        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        assert_eq!(merged.apply(base), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // IFD-level (index-keyed), Insert+Remove-before: insert a new IFD at 1 -> [ifd0,new],
        // then remove index 0 -> [new] lands at final index 0 (the recipe's own canonical
        // shift case).
        assert_absorb_law(&base, TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(1, 1)] } }, TiffMutation::RemoveIfd { index: 0 });

        // IFD-level, Insert+Insert-same-index: both survive.
        assert_absorb_law(
            &base,
            TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(2, 2)] } },
            TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(3, 3)] } },
        );

        // Tag-level (id-keyed), Add+SetField: the second mutation patches directly into the
        // still-pending added tag.
        assert_absorb_law(
            &base,
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("orig".into()) },
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("patched".into()) },
        );

        // Tag-level, Modify+Remove: a pending field patch on a since-removed base tag vanishes.
        assert_absorb_law(
            &base,
            TiffMutation::SetTag { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![7]) },
            TiffMutation::RemoveTag { ifd_index: 0, tag: 296 },
        );

        // Tag-level, Add then annihilate the very same add.
        assert_absorb_law(
            &base,
            TiffMutation::SetTag { ifd_index: 0, tag: 317, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) },
            TiffMutation::RemoveTag { ifd_index: 0, tag: 317 },
        );

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian }, TiffMutation::SetByteOrder { byte_order: TiffByteOrder::LittleEndian });
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a".into()) }.diff(&base);
        let s1 = d1.apply(&base);
        let d2 = TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a2".into()) }.diff(&s1);
        let s2 = d2.apply(&s1);
        let d3 = TiffMutation::RemoveTag { ifd_index: 0, tag: 296 }.diff(&s2);
        let s3 = d3.apply(&s2);

        // (d1∘d2)∘d3
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), s3);
        assert_eq!(right.apply(&base), s3);
        assert_eq!(left.apply(&base), right.apply(&base), "absorb must associate");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.byte_order = TiffByteOrder::BigEndian;
        b.ifds[0].entries.push(TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("Extra".into()) });
        b.pixels = vec![5u8; a.pixels.len()];

        let d = TiffDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = TiffDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(TiffDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = crate::artifacts::tiff::engine::encode_tiff(&base_snapshot()).expect("encode synthetic fixture");
        let decoded = crate::artifacts::tiff::engine::decode_tiff(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::tiff::engine::encode_tiff(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::tiff::engine::decode_tiff(&reencoded).expect("re-decode fixture");
        // Engine's EncodeScopeNote: encode always canonicalizes to a single IFD/single strip —
        // pixel CONTENT + carried non-core tags are the retained invariant.
        assert_eq!(decoded.width(), redecoded.width());
        assert_eq!(decoded.height(), redecoded.height());
        assert_eq!(decoded.pixels, redecoded.pixels);
        assert_eq!(decoded.tag(296), redecoded.tag(296), "carried non-core tag must survive a second round trip");
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = TiffDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = TiffDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
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
        let fwd_entries = &fwd_ifds.modified[0].diff;
        assert_eq!(fwd_entries.removed, vec![301]);
        assert_eq!(fwd_entries.modified.len(), 1);
        assert_eq!(fwd_entries.modified[0].tag, 300);
        assert_eq!(fwd_entries.added.len(), 1);
        assert_eq!(fwd_entries.added[0].tag, 303);

        let bwd_entries = &bwd_ifds.modified[0].diff;
        assert_eq!(bwd_entries.removed, vec![303]);
        assert_eq!(bwd_entries.modified.len(), 1);
        assert_eq!(bwd_entries.modified[0].tag, 300);
        assert_eq!(bwd_entries.added.len(), 1);
        assert_eq!(bwd_entries.added[0].tag, 301);
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveIfd { index: 42 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag { ifd_index: 42, tag: 1 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag { ifd_index: 0, tag: 9999 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::SetTag { ifd_index: 42, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) });
        assert_eq!(snap, base);
    }
}
//#endregion Tests
