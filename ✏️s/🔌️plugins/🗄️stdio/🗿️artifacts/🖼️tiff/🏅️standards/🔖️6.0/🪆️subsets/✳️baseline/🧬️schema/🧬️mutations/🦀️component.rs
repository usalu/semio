//! 🧬️ `TiffBaselineMutation` — the Adobe TIFF 6.0 Part 1 "Baseline TIFF" CONFORMANCE-CLASS
//! vocabulary of `stdio.tiff`. Every variant's `diff()` is handcrafted (it constructs the sparse
//! `TiffDiff` directly — apply-and-capture is banned) and every variant's `inverse()` is
//! handcrafted, reading whatever pre-state it needs out of the base.
//!
//! # Why this subset needs a vocabulary of its own
//!
//! `✳️any` owns the DOCUMENT vocabulary — `set-byte-order`, `insert-ifd`, `remove-ifd`, `set-tag`,
//! `remove-tag`, `set-pixels`. Those address the IFD chain generically: `set-tag` can write ANY of
//! the 65 536 tag numbers with any field type, which is the right vocabulary for editing a TIFF and
//! the wrong one for moving a document between conformance classes. A Baseline class is not a
//! property of an arbitrary tag; it is a property of five specific fields of IFD 0, and
//! `check_tiff_baseline_conformance` (`../🦀️component.rs`) reads exactly those:
//!
//! | Axis | Diagnostic | Restriction |
//! |---|---|---|
//! | `Compression` (259) | `CODE_UNSUPPORTED_COMPRESSION` | one of {1 none, 2 CCITT G3 1D, 32773 PackBits} |
//! | `PhotometricInterpretation` (262) | `CODE_UNSUPPORTED_PHOTOMETRIC` | 0..=3 |
//! | `BitsPerSample` (258) | `CODE_UNSUPPORTED_BITS_PER_SAMPLE` | every value one of {1, 4, 8} |
//! | `TileWidth`/`TileLength` (322/323) | `CODE_TILED_NOT_BASELINE` | absent — Baseline is strip-organized |
//! | `StripOffsets` (273) | `CODE_MISSING_STRIP_OFFSETS` | present when the IFD is not tiled |
//!
//! One variant per axis, plus the two baseline variants every vocabulary carries, plus the
//! insert/remove pairing the two structural axes need to be reachable in both directions — the same
//! one-kind-per-axis derivation the OOXML conformance-class subsets make from their own
//! `check_strict_conformance`, and for the same reason: a vocabulary derived from the checker is a
//! vocabulary a reader can hold against the checker.
//!
//! The two vocabularies are disjoint in intent, not in reach: `✳️any`'s `SetTag` could write tag 259
//! as well, exactly as an assembly instruction could implement any statement. What this enum adds is
//! that each kind NAMES the axis it moves, carries only the values that axis can take, and inverts
//! by restoring that axis alone.
//!
//! `Diff` is `TiffDiff`, the SAME diff type `✳️any` uses — the two subsets share one snapshot type,
//! so they share its diff. What differs is the vocabulary that produces it, which is what a subset
//! is.
//!
//! # Where this vocabulary is observable, and where it is not
//!
//! `encode_tiff` (`../../✳️any/🚪️io/🦀️component.rs`) REGENERATES every one of `CORE_STRIP_TAGS` on
//! IFD 0 from the raster it is about to write — `BitsPerSample` 8, `Compression` 1 (or 32773 for the
//! PackBits entry point), `PhotometricInterpretation` 2, `SamplesPerPixel` 3, `RowsPerStrip`,
//! `StripByteCounts`, `StripOffsets` — because those fields DESCRIBE the strip it emits and any
//! other value would describe bytes it did not write. That is correct, and it is the same
//! constraint PNG's IHDR and JPEG's SOF0 are under.
//!
//! The consequence for testing is exact. Four of the six conformance kinds below cannot survive a
//! re-serialization by this repository's own encoder, so a BYTE-level exhaustive case built on this
//! catalog would report `mutate-set-compression`, `mutate-set-photometric-interpretation`,
//! `mutate-set-bits-per-sample` and `mutate-set-strip-offsets` as green while the mutation never
//! reached a byte — the precise shape of shallow green ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR exists to remove. Only the two tile kinds are
//! byte-observable, because `TileWidth`/`TileLength` are outside `CORE_STRIP_TAGS` and are carried
//! verbatim.
//!
//! The catalog `tiff-6-0-baseline` (`../../🧪️oracle/🔣️.json`) is therefore declared and
//! claimed by `mutate-tiff-6-0-baseline`, and that case measures this vocabulary where its axes
//! actually live: on the DECODED SNAPSHOT, against [`check_tiff_baseline_conformance`]'s verdict.
//! Each kind must move its own axis and raise its own diagnostic; each inverse must restore the
//! snapshot exactly. The case states in as many words that it makes no byte-level claim for those
//! four kinds, and its `identity-round-trip` is the one scenario that does touch bytes — decode,
//! re-encode, and read both through the INDEPENDENT `image`-backed IFD reader the sibling `✳️any`
//! subset registers.
//!
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.
//! @see ../../✳️any/🧬️schema/🧬️mutations/🦀️component.rs — the DOCUMENT vocabulary this one is disjoint from.

use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::diff::{TiffDiff, TiffIfdDiff, TiffIfdModified, TiffIfdsDiff, TiffTagAdded, TiffTagModified, TiffTagsDiff};
use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffFieldType, TiffSnapshot, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.tiff` under Adobe TIFF 6.0 Part 1 Baseline.
/// Every variant addresses ONE axis of the class; none addresses arbitrary document content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TiffBaselineMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. A conformance class is a whole-document property, so this is
    /// the class stamp in its total form — every axis at once.
    SetSnapshot {
        snapshot: TiffSnapshot,
    },
    /// 🗜️ Sets IFD 0's `Compression` (tag 259). Baseline admits 1 (none), 2 (CCITT Group 3 1-D) and
    /// 32773 (PackBits); any other value is what `CODE_UNSUPPORTED_COMPRESSION` reports.
    SetCompression {
        compression: u16,
    },
    /// 🎨️ Sets IFD 0's `PhotometricInterpretation` (tag 262). Baseline admits 0 (WhiteIsZero),
    /// 1 (BlackIsZero), 2 (RGB) and 3 (Palette); `CODE_UNSUPPORTED_PHOTOMETRIC` reports the rest.
    SetPhotometricInterpretation {
        photometric: u16,
    },
    /// 📏️ Sets IFD 0's `BitsPerSample` (tag 258), one value per sample. Baseline admits 1, 4 and 8;
    /// `CODE_UNSUPPORTED_BITS_PER_SAMPLE` reports any other.
    SetBitsPerSample {
        bits: Vec<u16>,
    },
    /// ➕️ Adds IFD 0's `TileWidth`/`TileLength` (tags 322/323) together, which is what makes an IFD
    /// tile-organized. Baseline TIFF is strip-organized, so this is the kind that leaves the class;
    /// `CODE_TILED_NOT_BASELINE` is what reports it.
    InsertTileTags {
        tile_width: u32,
        tile_length: u32,
    },
    /// ➖️ Removes IFD 0's `TileWidth`/`TileLength` together, restoring strip organization.
    RemoveTileTags,
    /// 📍️ Sets IFD 0's `StripOffsets` (tag 273), the pointer list a strip-organized IFD must carry.
    SetStripOffsets {
        offsets: Vec<u32>,
    },
    /// ➖️ Removes IFD 0's `StripOffsets`, leaving an IFD with neither strip nor tile organization —
    /// what `CODE_MISSING_STRIP_OFFSETS` reports.
    RemoveStripOffsets,
}

/// 🏷️ Kebab-case spelling of every `TiffBaselineMutation` variant, in declaration order — the
/// vocabulary the `tiff-6-0-baseline` mutation catalog (`../../🧪️oracle/🔣️.json`) declares
/// and `mutate-tiff-6-0-baseline` measures itself against.
/// `kinds_match_enum_variants_in_declaration_order` below keeps the two honest against the enum,
/// and `kinds_match_the_committed_catalog` against the manifest.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-compression", "set-photometric-interpretation", "set-bits-per-sample", "insert-tile-tags", "remove-tile-tags", "set-strip-offsets", "remove-strip-offsets"];

crate::impl_serde_op_codec!(TiffBaselineMutation, "tiff-baseline-mutation");

//#region 🌉️ConformanceProjection
/// 👁️ The comparison surface `mutate-tiff-6-0-baseline` measures this vocabulary through: the five
/// Baseline TIFF axes as they stand on IFD 0 of the DECODED snapshot, plus
/// [`check_tiff_baseline_conformance`]'s verdict over them. It carries no pixels and no other tag on
/// purpose — this is a conformance-class vocabulary, and a Baseline class is a property of five
/// specific fields of IFD 0, not of the raster or of the other 65 530 tag numbers `✳️any`'s
/// `set-tag` can reach.
///
/// Rendered by hand rather than through `serde` because it is a PROJECTION, not the snapshot: the
/// snapshot's own serialization carries a multi-megabyte raster no comparison here should have to
/// walk.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_tiff_baseline_projection_json(snapshot: &TiffSnapshot) -> String {
    let list = |tag: u16| match snapshot.ifds.first().and_then(|ifd| ifd.entries.iter().find(|entry| entry.tag == tag)) {
        Some(entry) => match &entry.values {
            TiffValues::Short(values) => values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "),
            TiffValues::Long(values) => values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "),
            other => format!("{other:?}"),
        },
        None => "absent".to_string(),
    };
    let verdict = crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance(snapshot)
        .into_iter()
        .map(|finding| format!("\"{}\"", finding.code.0))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"format\":\"tiff-baseline\",\"ifdCount\":{},\"compression\":\"{}\",\"photometric\":\"{}\",\"bitsPerSample\":\"{}\",\"tileWidth\":\"{}\",\"tileLength\":\"{}\",\"stripOffsets\":\"{}\",\"conformance\":[{verdict}]}}",
        snapshot.ifds.len(),
        list(TAG_COMPRESSION),
        list(TAG_PHOTOMETRIC),
        list(TAG_BITS_PER_SAMPLE),
        list(TAG_TILE_WIDTH),
        list(TAG_TILE_LENGTH),
        list(TAG_STRIP_OFFSETS)
    )
}

/// 🛡️ [`check_tiff_baseline_conformance`]'s verdict as bare diagnostic codes — what a
/// `mutate-<kind>` scenario names when it claims a kind leaves the class by its own axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_baseline_conformance_codes(snapshot: &TiffSnapshot) -> Vec<String> {
    crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
}
//#endregion 🌉️ConformanceProjection
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_tiff_baseline_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffBaselineMutation) -> protocol::MutationOutcome<TiffDiff> {
    let outcome = <TiffBaselineMutation as Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate reaches it without naming the `protocol::Mutation` trait.
pub fn inverse_tiff_baseline_mutation(mutation: &TiffBaselineMutation, base: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️Axes
/// 🔎️ IFD 0's entry for `tag`, or `None` when there is no IFD 0 or it does not carry the tag.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifd0_tag(base: &TiffSnapshot, tag: u16) -> Option<&TiffTag> {
    base.ifds.first().and_then(|ifd| ifd.entries.iter().find(|entry| entry.tag == tag))
}

/// 🧾️ One IFD-0 diff carrying whatever tag additions, modifications and removals a single
/// conformance axis needs. Every arm of `diff` below funnels through this, so the diff a kind
/// produces is visibly the diff its axis calls for and nothing else.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifd0_diff(added: Vec<TiffTagAdded>, modified: Vec<TiffTagModified>, removed: Vec<u16>) -> TiffDiff {
    if added.is_empty() && modified.is_empty() && removed.is_empty() {
        return TiffDiff::default();
    }
    TiffDiff {
        ifds: Some(TiffIfdsDiff {
            removed: Vec::new(),
            modified: vec![TiffIfdModified { index: 0, diff: TiffIfdDiff { entries: TiffTagsDiff { removed, modified, added }, pixels: None } }],
            added: Vec::new(),
        }),
        ..Default::default()
    }
}

/// ✏️ Creates-or-updates one IFD-0 tag, and produces the empty diff when the value is already what
/// it should be — a mutation that changes nothing must produce a diff that says so.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_ifd0_tag(base: &TiffSnapshot, tag: u16, kind: TiffFieldType, values: TiffValues) -> TiffDiff {
    if base.ifds.is_empty() {
        return TiffDiff::default();
    }
    match ifd0_tag(base, tag) {
        Some(existing) if existing.kind == kind && existing.values == values => TiffDiff::default(),
        Some(_) => ifd0_diff(Vec::new(), vec![TiffTagModified { tag, kind, values }], Vec::new()),
        None => ifd0_diff(vec![TiffTagAdded { tag, kind, values }], Vec::new(), Vec::new()),
    }
}

/// ➖️ Removes one IFD-0 tag, and produces the empty diff when it was not there to begin with.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_ifd0_tags(base: &TiffSnapshot, tags: &[u16]) -> TiffDiff {
    let present: Vec<u16> = tags.iter().copied().filter(|tag| ifd0_tag(base, *tag).is_some()).collect();
    ifd0_diff(Vec::new(), Vec::new(), present)
}

/// ↩️ The mutation that restores IFD 0's current state for `tag` — the shared shape every
/// structural inverse below needs: put the tag back with the value it had, or remove it again when
/// it was absent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn restore_or_remove(base: &TiffSnapshot, tag: u16, restore: impl FnOnce(&TiffValues) -> TiffBaselineMutation, remove: TiffBaselineMutation) -> TiffBaselineMutation {
    match ifd0_tag(base, tag) {
        Some(entry) => restore(&entry.values),
        None => remove,
    }
}

/// 🔢️ A tag's values as `u16`s, whatever integer field type it was stored with — Baseline writes
/// `BitsPerSample` and `Compression` as SHORT, but a real file is allowed to widen them.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shorts(values: &TiffValues) -> Vec<u16> {
    match values {
        TiffValues::Short(entries) => entries.clone(),
        TiffValues::Long(entries) => entries.iter().map(|value| *value as u16).collect(),
        TiffValues::Byte(entries) => entries.iter().map(|value| *value as u16).collect(),
        _ => Vec::new(),
    }
}

/// 🔢️ A tag's values as `u32`s — `StripOffsets` is LONG in practice and SHORT in older writers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn longs(values: &TiffValues) -> Vec<u32> {
    match values {
        TiffValues::Long(entries) => entries.clone(),
        TiffValues::Short(entries) => entries.iter().map(|value| *value as u32).collect(),
        _ => Vec::new(),
    }
}
//#endregion 🔖️Axes

//#region 🔖️MutationTrait
impl Mutation<TiffSnapshot> for TiffBaselineMutation {
    type Diff = TiffDiff;

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            TiffBaselineMutation::NoMutation => TiffDiff::default(),
            TiffBaselineMutation::SetSnapshot { snapshot } => crate::artifacts::tiff::standards::v6_0::subsets::any::schema::diff::diff_set_snapshot(base, snapshot),
            TiffBaselineMutation::SetCompression { compression } => set_ifd0_tag(base, TAG_COMPRESSION, TiffFieldType::Short, TiffValues::Short(vec![*compression])),
            TiffBaselineMutation::SetPhotometricInterpretation { photometric } => set_ifd0_tag(base, TAG_PHOTOMETRIC, TiffFieldType::Short, TiffValues::Short(vec![*photometric])),
            TiffBaselineMutation::SetBitsPerSample { bits } => set_ifd0_tag(base, TAG_BITS_PER_SAMPLE, TiffFieldType::Short, TiffValues::Short(bits.clone())),
            TiffBaselineMutation::InsertTileTags { tile_width, tile_length } => {
                if base.ifds.is_empty() {
                    return protocol::MutationOutcome::new(TiffDiff::default());
                }
                let mut added = Vec::new();
                let mut modified = Vec::new();
                for (tag, value) in [(TAG_TILE_WIDTH, *tile_width), (TAG_TILE_LENGTH, *tile_length)] {
                    let values = TiffValues::Long(vec![value]);
                    match ifd0_tag(base, tag) {
                        Some(existing) if existing.kind == TiffFieldType::Long && existing.values == values => {}
                        Some(_) => modified.push(TiffTagModified { tag, kind: TiffFieldType::Long, values }),
                        None => added.push(TiffTagAdded { tag, kind: TiffFieldType::Long, values }),
                    }
                }
                ifd0_diff(added, modified, Vec::new())
            }
            TiffBaselineMutation::RemoveTileTags => remove_ifd0_tags(base, &[TAG_TILE_WIDTH, TAG_TILE_LENGTH]),
            TiffBaselineMutation::SetStripOffsets { offsets } => set_ifd0_tag(base, TAG_STRIP_OFFSETS, TiffFieldType::Long, TiffValues::Long(offsets.clone())),
            TiffBaselineMutation::RemoveStripOffsets => remove_ifd0_tags(base, &[TAG_STRIP_OFFSETS]),
        })
    }

    /// ↩️ Handcrafted, axis-aware inverses. A kind that SETS an axis inverts to whatever IFD 0 held
    /// for that axis — which for a document that did not carry the tag at all is its removal, not a
    /// fabricated default value. A kind that ADDS an axis inverts to its removal, and vice versa.
    fn inverse(&self, base: &TiffSnapshot) -> Vec<Self> {
        vec![match self {
            TiffBaselineMutation::NoMutation => TiffBaselineMutation::NoMutation,
            TiffBaselineMutation::SetSnapshot { .. } => TiffBaselineMutation::SetSnapshot { snapshot: base.clone() },
            TiffBaselineMutation::SetCompression { .. } => restore_or_remove(
                base,
                TAG_COMPRESSION,
                |values| TiffBaselineMutation::SetCompression { compression: shorts(values).first().copied().unwrap_or(1) },
                // 🧭️ A document with no Compression tag reads as "uncompressed" per TIFF 6.0's own
                // default, so restoring the absent state means writing that default back rather
                // than removing a tag this vocabulary has no removal kind for.
                TiffBaselineMutation::SetCompression { compression: 1 },
            ),
            TiffBaselineMutation::SetPhotometricInterpretation { .. } => restore_or_remove(
                base,
                TAG_PHOTOMETRIC,
                |values| TiffBaselineMutation::SetPhotometricInterpretation { photometric: shorts(values).first().copied().unwrap_or(1) },
                TiffBaselineMutation::SetPhotometricInterpretation { photometric: 1 },
            ),
            TiffBaselineMutation::SetBitsPerSample { .. } => restore_or_remove(
                base,
                TAG_BITS_PER_SAMPLE,
                |values| TiffBaselineMutation::SetBitsPerSample { bits: shorts(values) },
                // 🧭️ TIFF 6.0's own default for a missing BitsPerSample is a single 1-bit sample.
                TiffBaselineMutation::SetBitsPerSample { bits: vec![1] },
            ),
            TiffBaselineMutation::InsertTileTags { .. } => match (ifd0_tag(base, TAG_TILE_WIDTH), ifd0_tag(base, TAG_TILE_LENGTH)) {
                (Some(width), Some(length)) => TiffBaselineMutation::InsertTileTags { tile_width: longs(&width.values).first().copied().unwrap_or(0), tile_length: longs(&length.values).first().copied().unwrap_or(0) },
                _ => TiffBaselineMutation::RemoveTileTags,
            },
            TiffBaselineMutation::RemoveTileTags => match (ifd0_tag(base, TAG_TILE_WIDTH), ifd0_tag(base, TAG_TILE_LENGTH)) {
                (Some(width), Some(length)) => TiffBaselineMutation::InsertTileTags { tile_width: longs(&width.values).first().copied().unwrap_or(0), tile_length: longs(&length.values).first().copied().unwrap_or(0) },
                _ => TiffBaselineMutation::NoMutation,
            },
            TiffBaselineMutation::SetStripOffsets { .. } => restore_or_remove(base, TAG_STRIP_OFFSETS, |values| TiffBaselineMutation::SetStripOffsets { offsets: longs(values) }, TiffBaselineMutation::RemoveStripOffsets),
            TiffBaselineMutation::RemoveStripOffsets => restore_or_remove(base, TAG_STRIP_OFFSETS, |values| TiffBaselineMutation::SetStripOffsets { offsets: longs(values) }, TiffBaselineMutation::NoMutation),
        }]
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffIfd};
    use crate::artifacts::tiff::standards::v6_0::subsets::baseline::schema::{check_tiff_baseline_conformance, CODE_MISSING_STRIP_OFFSETS, CODE_TILED_NOT_BASELINE, CODE_UNSUPPORTED_BITS_PER_SAMPLE, CODE_UNSUPPORTED_COMPRESSION, CODE_UNSUPPORTED_PHOTOMETRIC};

    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }

    /// 🧫️ A conforming 4x2 Baseline document: RGB, uncompressed, 8 bits per sample, strip-organized.
    fn conforming() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd { pixels: Vec::new(),
                entries: vec![
                    tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                    tag(257, TiffFieldType::Long, TiffValues::Long(vec![2])),
                    tag(TAG_BITS_PER_SAMPLE, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                    tag(TAG_COMPRESSION, TiffFieldType::Short, TiffValues::Short(vec![1])),
                    tag(TAG_PHOTOMETRIC, TiffFieldType::Short, TiffValues::Short(vec![2])),
                    tag(TAG_STRIP_OFFSETS, TiffFieldType::Long, TiffValues::Long(vec![8])),
                ],
            }],
            pixels: vec![0u8; 4 * 2 * 4],
        }
    }

    fn codes(snapshot: &TiffSnapshot) -> Vec<String> {
        check_tiff_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
    }

    /// 🏷️ [`KINDS`] against the committed catalog. The framework never parses Rust, so without this
    /// the manifest could keep measuring `mutate-tiff-6-0-baseline` against a vocabulary this subset
    /// no longer has — which is exactly the gap that left this vocabulary with no catalog at all
    /// until the completeness gate learned to see an unregistered one.
    #[test]
    fn kinds_match_the_committed_catalog() {
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(manifest.contains("tiff-6-0-baseline-mutate"), "the manifest must declare this subset's OWN capability, not the ✳️any subset's");
    }

    #[test]
    fn kinds_match_enum_variants_in_declaration_order() {
        let variants = [
            TiffBaselineMutation::NoMutation,
            TiffBaselineMutation::SetSnapshot { snapshot: TiffSnapshot::default() },
            TiffBaselineMutation::SetCompression { compression: 1 },
            TiffBaselineMutation::SetPhotometricInterpretation { photometric: 2 },
            TiffBaselineMutation::SetBitsPerSample { bits: vec![8] },
            TiffBaselineMutation::InsertTileTags { tile_width: 16, tile_length: 16 },
            TiffBaselineMutation::RemoveTileTags,
            TiffBaselineMutation::SetStripOffsets { offsets: vec![8] },
            TiffBaselineMutation::RemoveStripOffsets,
        ];
        assert_eq!(variants.len(), KINDS.len(), "every variant needs exactly one KINDS entry");
        for (variant, kind) in variants.iter().zip(KINDS) {
            let tag = match serde_json::to_value(variant).expect("serialize") {
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
        apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetCompression { compression: 7 });
        assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_COMPRESSION.to_string()]);

        let mut snapshot = conforming();
        apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetPhotometricInterpretation { photometric: 6 });
        assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_PHOTOMETRIC.to_string()]);

        let mut snapshot = conforming();
        apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::SetBitsPerSample { bits: vec![16, 16, 16] });
        assert_eq!(codes(&snapshot), vec![CODE_UNSUPPORTED_BITS_PER_SAMPLE.to_string()]);

        let mut snapshot = conforming();
        apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::InsertTileTags { tile_width: 16, tile_length: 16 });
        assert_eq!(codes(&snapshot), vec![CODE_TILED_NOT_BASELINE.to_string()]);

        let mut snapshot = conforming();
        apply_tiff_baseline_mutation(&mut snapshot, &TiffBaselineMutation::RemoveStripOffsets);
        assert_eq!(codes(&snapshot), vec![CODE_MISSING_STRIP_OFFSETS.to_string()]);
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
            TiffBaselineMutation::NoMutation,
            TiffBaselineMutation::SetSnapshot { snapshot: TiffSnapshot::default() },
            TiffBaselineMutation::SetCompression { compression: 32773 },
            TiffBaselineMutation::SetPhotometricInterpretation { photometric: 0 },
            TiffBaselineMutation::SetBitsPerSample { bits: vec![4] },
            TiffBaselineMutation::InsertTileTags { tile_width: 16, tile_length: 16 },
            TiffBaselineMutation::RemoveTileTags,
            TiffBaselineMutation::SetStripOffsets { offsets: vec![64, 128] },
            TiffBaselineMutation::RemoveStripOffsets,
        ];
        for mutation in cases {
            let base = conforming();
            let mut snapshot = base.clone();
            apply_tiff_baseline_mutation(&mut snapshot, &mutation);
            for undo in inverse_tiff_baseline_mutation(&mutation, &base) {
                apply_tiff_baseline_mutation(&mut snapshot, &undo);
            }
            assert_eq!(snapshot, base, "inverse of {mutation:?} did not restore the base");
        }
    }

    /// 🧭️ An IFD 0 that never carried the tag inverts to its ABSENCE, not to a fabricated value —
    /// the case a `restore_or_remove` that always wrote a default would get silently wrong.
    #[test]
    fn setting_an_absent_strip_offsets_inverts_to_removing_it_again() {
        let mut base = conforming();
        base.ifds[0].entries.retain(|entry| entry.tag != TAG_STRIP_OFFSETS);
        let mutation = TiffBaselineMutation::SetStripOffsets { offsets: vec![8] };
        assert_eq!(inverse_tiff_baseline_mutation(&mutation, &base), vec![TiffBaselineMutation::RemoveStripOffsets]);

        let mut snapshot = base.clone();
        apply_tiff_baseline_mutation(&mut snapshot, &mutation);
        assert!(codes(&snapshot).is_empty(), "adding StripOffsets makes the IFD strip-organized again");
        for undo in inverse_tiff_baseline_mutation(&mutation, &base) {
            apply_tiff_baseline_mutation(&mut snapshot, &undo);
        }
        assert_eq!(snapshot, base);
    }

    /// 🚫️ A kind that sets an axis to the value it already holds must produce the EMPTY diff — a
    /// mutation that changes nothing may not report a change.
    #[test]
    fn setting_an_axis_to_its_current_value_produces_an_empty_diff() {
        let base = conforming();
        let outcome = <TiffBaselineMutation as Mutation<TiffSnapshot>>::diff(&TiffBaselineMutation::SetCompression { compression: 1 }, &base);
        assert_eq!(outcome.diff(), &TiffDiff::default());
    }
}
//#endregion 🧪️Tests
