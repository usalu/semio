//! 🔺️ TiffDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `TiffDiff{snapshot: Option<TiffSnapshot>}` full-replace template. `ifds` is an INDEX-keyed
//! `removed`/`modified`/`added` triple (TIFF's own IFD chain is positional); within each IFD,
//! `entries` is a TAG-ID-keyed triple (`tag: u16`, not array index — tag SETS can differ in
//! size between two IFDs, and tags are TIFF's own natural stable identity, unlike an ordinal
//! position). A `TiffTag` is a weak value (`kind`/`values` move together atomically), so a
//! tag-triple's `modified`/`added` payload carries the whole new tag, never a nested diff.

use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
use crate::artifacts::tiff::TiffSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️TagsTriple
/// 🏷️ One `entries.modified[]`/`.added[]` entity — `TiffTag` is a weak value, so both carry
/// the entry's NEW `kind`/`values` directly (never a nested per-field diff).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagModified {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagAdded {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

/// 🔺️ Tag-id-keyed `entries` triple for one IFD.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffTagModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TiffTagAdded>,
}

impl TiffTagsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a tag-id-keyed triple to one IFD's entries. TIFF6 §2 requires ascending-tag-
/// order within an IFD — `apply` re-sorts on every call, keeping that invariant regardless of
/// the triple's own insertion order.
fn apply_tags(base: &[TiffTag], d: &TiffTagsDiff) -> Vec<TiffTag> {
    let mut items: Vec<TiffTag> = base.iter().filter(|t| !d.removed.contains(&t.tag)).cloned().collect();
    for m in &d.modified {
        if let Some(it) = items.iter_mut().find(|t| t.tag == m.tag) {
            it.kind = m.kind;
            it.values = m.values.clone();
        }
    }
    for a in &d.added {
        if let Some(it) = items.iter_mut().find(|t| t.tag == a.tag) {
            it.kind = a.kind;
            it.values = a.values.clone();
        } else {
            items.push(TiffTag { tag: a.tag, kind: a.kind, values: a.values.clone() });
        }
    }
    items.sort_by_key(|t| t.tag);
    items
}

fn between_tags(a: &[TiffTag], b: &[TiffTag]) -> Option<TiffTagsDiff> {
    let a_map: BTreeMap<u16, &TiffTag> = a.iter().map(|t| (t.tag, t)).collect();
    let b_map: BTreeMap<u16, &TiffTag> = b.iter().map(|t| (t.tag, t)).collect();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (tag, at) in &a_map {
        match b_map.get(tag) {
            None => removed.push(*tag),
            Some(bt) => {
                if at.kind != bt.kind || at.values != bt.values {
                    modified.push(TiffTagModified { tag: *tag, kind: bt.kind, values: bt.values.clone() });
                }
            }
        }
    }
    for (tag, bt) in &b_map {
        if !a_map.contains_key(tag) {
            added.push(TiffTagAdded { tag: *tag, kind: bt.kind, values: bt.values.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(TiffTagsDiff { removed, modified, added })
    }
}

/// ➕️ Structural, total, base-free absorb for a TAG-ID-keyed triple. Simpler than an
/// index-keyed collection's transport: tag ids are stable identity, never renumbered by
/// insert/remove, so no position-simulation is needed — a plain keyed union/override algebra.
fn absorb_tags(d1: TiffTagsDiff, d2: TiffTagsDiff) -> TiffTagsDiff {
    let mut removed: BTreeSet<u16> = d1.removed.into_iter().collect();
    let mut modified: BTreeMap<u16, TiffTagModified> = d1.modified.into_iter().map(|m| (m.tag, m)).collect();
    let mut added: BTreeMap<u16, TiffTagAdded> = d1.added.into_iter().map(|a| (a.tag, a)).collect();

    for r in d2.removed {
        if added.remove(&r).is_some() {
            // A d2-removal of a d1-added tag annihilates the add (recipe's canonical case).
        } else {
            modified.remove(&r);
            removed.insert(r);
        }
    }
    for m in d2.modified {
        if let Some(a) = added.get_mut(&m.tag) {
            // d2 patch on a d1-added tag patches INTO the still-pending added payload.
            a.kind = m.kind;
            a.values = m.values;
        } else if !removed.contains(&m.tag) {
            modified.insert(m.tag, m);
        }
        // modified-of-removed (by d1) is illegal per the apply contract — ignored here too.
    }
    for a in d2.added {
        removed.remove(&a.tag);
        added.insert(a.tag, a);
    }

    TiffTagsDiff { removed: removed.into_iter().collect(), modified: modified.into_values().collect(), added: added.into_values().collect() }
}
//#endregion 🔖️TagsTriple

//#region 🔖️IfdsTriple
/// 🗂️ One `ifds.modified[]` entity — the recursive per-IFD tag-triple.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdModified {
    pub index: usize,
    pub diff: TiffTagsDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdAdded {
    pub index: usize,
    pub ifd: TiffIfd,
}

/// 🔺️ Index-keyed `ifds` triple (TIFF's IFD chain is positional).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffIfdModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TiffIfdAdded>,
}

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for `ifds`' absorb — the same position-simulation shape as
// PNG's `text_chunks`/csv's `records` (`simulate_slots`/`base_len_hint`), since `ifds`'
// `modified` payload IS a nested diff (needs field-aware absorb, not plain LWW).
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}

fn absorb_ifds(d1: TiffIfdsDiff, d2: TiffIfdsDiff) -> TiffIfdsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied())
        .max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed;
    let mut modified_map: BTreeMap<usize, TiffTagsDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<TiffIfdAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                let entry = modified_map.entry(*b).or_default();
                *entry = absorb_tags(entry.clone(), m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.ifd.entries = apply_tags(&a.ifd.entries, &m2.diff);
                }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<TiffIfdModified> =
        modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| TiffIfdModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2
        .removed
        .iter()
        .copied()
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(alive_mid_positions.iter().copied())
        .chain(d2_added_indices.iter().copied())
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<TiffIfdAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(TiffIfdAdded { index: *after_pos, ifd: added.ifd });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    TiffIfdsDiff { removed: final_removed, modified: final_modified, added: final_added }
}
//#endregion 🔖️IndexTransport

fn apply_ifds(base: &[TiffIfd], d: &TiffIfdsDiff) -> Vec<TiffIfd> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            it.entries = apply_tags(&it.entries, &m.diff);
        }
    }
    let mut removed_desc = d.removed.clone();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for idx in removed_desc {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.ifd);
    }
    items
}

fn between_ifds(a: &[TiffIfd], b: &[TiffIfd]) -> Option<TiffIfdsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(d) = between_tags(&a[i].entries, &b[i].entries) {
            modified.push(TiffIfdModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<TiffIfdAdded> = (min..b.len()).map(|i| TiffIfdAdded { index: i, ifd: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(TiffIfdsDiff { removed, modified, added }) }
}

fn absorb_ifds_opt(base: &mut Option<TiffIfdsDiff>, other: Option<TiffIfdsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_ifds(b, o)),
    }
}
//#endregion 🔖️IfdsTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tiff`. No `snapshot: Option<TiffSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `TiffDiff::between(base, next)`.
/// 🧪️ F6 CONFIRMED (real `cargo check`, ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION): adding
/// `#[derive(dsl::DslDiff)]` here fails — `TiffValues` (12 non-unit variants: `Byte(Vec<u8>)`,
/// `Ascii(String)`, `Short(Vec<u16>)`, … `Double(Vec<f64>)`) is a genuine data-carrying enum
/// reachable through `ifds: Option<TiffIfdsDiff>` -> `TiffIfdModified.diff.modified[].values` /
/// `.added[].values`, and `DslField` has no impl for it (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement `DslField` — recon report §3a): `error[E0277]:
/// the trait bound v6_0::…::TiffValues: DslField is not satisfied`. Same root cause independently
/// blocks the Mutation side (`TiffMutation::SetTag.values`/`SetSnapshot.snapshot` recursively
/// reach the same `TiffValues`). `DiffCodec` hand-rolled below (see `HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff.diff")]
pub struct TiffDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_order: Option<TiffByteOrder>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ifds: Option<TiffIfdsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
}

impl MutationDiff<TiffSnapshot> for TiffDiff {
    fn apply(&self, base: &TiffSnapshot) -> TiffSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.byte_order {
            next.byte_order = v;
        }
        if let Some(d) = &self.ifds {
            next.ifds = apply_ifds(&next.ifds, d);
        }
        if let Some(v) = &self.pixels {
            next.pixels = v.clone();
        }
        next
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). `byte_order`/
    /// `pixels`: LWW. `ifds`: index-transported merge with the nested tag-id-keyed merge for
    /// `modified` entries.
    fn absorb(&mut self, other: Self) {
        if other.byte_order.is_some() {
            self.byte_order = other.byte_order;
        }
        absorb_ifds_opt(&mut self.ifds, other.ifds);
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
    }
}

impl DiffAlgebra<TiffSnapshot> for TiffDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta
    /// from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &TiffSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): index-keyed pairwise `0..min(len)` matching for
    /// `ifds`, recursive tag-id-keyed matching within each surviving IFD pair.
    fn between(base: &TiffSnapshot, other: &TiffSnapshot) -> Self {
        Self {
            byte_order: (base.byte_order != other.byte_order).then_some(other.byte_order),
            ifds: between_ifds(&base.ifds, &other.ifds),
            pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.byte_order.is_none() && self.ifds.is_none() && self.pixels.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
pub fn diff_set_snapshot(base: &TiffSnapshot, next: &TiffSnapshot) -> TiffDiff {
    TiffDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧩 One handcrafted builder per `schema::mutations::TiffMutation` variant (excluding
// `NoMutation`/`SetSnapshot`, covered above).

pub fn diff_set_byte_order(base: &TiffSnapshot, byte_order: TiffByteOrder) -> TiffDiff {
    TiffDiff { byte_order: (base.byte_order != byte_order).then_some(byte_order), ..Default::default() }
}

pub fn diff_insert_ifd(base: &TiffSnapshot, index: usize, ifd: TiffIfd) -> TiffDiff {
    let at = index.min(base.ifds.len());
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![], added: vec![TiffIfdAdded { index: at, ifd }] }), ..Default::default() }
}

pub fn diff_remove_ifd(base: &TiffSnapshot, index: usize) -> TiffDiff {
    if index >= base.ifds.len() {
        return TiffDiff::default();
    }
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_tag(base: &TiffSnapshot, ifd_index: usize, tag: u16, kind: TiffFieldType, values: TiffValues) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    let already = ifd.entries.iter().find(|t| t.tag == tag);
    if let Some(existing) = already {
        if existing.kind == kind && existing.values == values {
            return TiffDiff::default();
        }
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![], modified: vec![TiffTagModified { tag, kind, values }], added: vec![] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![], modified: vec![], added: vec![TiffTagAdded { tag, kind, values }] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    }
}

pub fn diff_remove_tag(base: &TiffSnapshot, ifd_index: usize, tag: u16) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    if !ifd.entries.iter().any(|t| t.tag == tag) {
        return TiffDiff::default();
    }
    TiffDiff {
        ifds: Some(TiffIfdsDiff {
            removed: vec![],
            modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![tag], modified: vec![], added: vec![] } }],
            added: vec![],
        }),
        ..Default::default()
    }
}

pub fn diff_set_pixels(base: &TiffSnapshot, pixels: Vec<u8>) -> TiffDiff {
    TiffDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `TiffDiff` — `TiffValues` (a genuine
/// data-carrying enum, real compile error captured on the `TiffDiff` doc comment above) rules out
/// `#[derive(dsl::DslDiff)]`. Same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs use
/// (bracket-depth-aware split, hex for strings/bytes, single-letter tag prefix for enums,
/// `[removed];[modified];[added]` for collection triples) — see `f6-recon-report.md` §5 for the
/// primitive rationale; this file re-derives its own copies of the small helper functions (no
/// shared "hand-roll helpers" module exists yet). No `Option<T>`/tri-state wrapping is needed
/// here — every `TiffDiff`/`TiffMutation` field is a required value, so `encode_option`/
/// `decode_option` (present in `GifDiff`/`SvgDiff`) are deliberately omitted as dead code.
//#region 🔖️Primitives
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
/// 🔢️ Generic numeric-token parser (`u8`/`u16`/`u32`/`i8`/`i16`/`i32`/`f32`/`f64`/`usize`, every
/// scalar this grammar carries) — `f32`/`f64`'s `Display`/`FromStr` round-trip exactly for every
/// finite value this codec ever produces (same assumption `svg`'s `ViewBox` float fields make).
pub(crate) fn parse_num<T: std::str::FromStr>(s: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    s.parse::<T>().map_err(|e| e.to_string())
}
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
/// 📃️ Generic bracketed comma list (`[e1,e2,...]`) — every `Vec<T>` in this grammar (IFD entries,
/// an IFD list, a numeric value list) uses this same shape.
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|x| enc(x)).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_byte_order(b: TiffByteOrder) -> String {
    match b {
        TiffByteOrder::LittleEndian => "0".to_string(),
        TiffByteOrder::BigEndian => "1".to_string(),
    }
}
pub(crate) fn dec_byte_order(s: &str) -> Result<TiffByteOrder, String> {
    match s {
        "0" => Ok(TiffByteOrder::LittleEndian),
        "1" => Ok(TiffByteOrder::BigEndian),
        other => Err(format!("byte order: unknown code {other:?}")),
    }
}
pub(crate) fn enc_field_type(k: TiffFieldType) -> String {
    k.to_u16().to_string()
}
pub(crate) fn dec_field_type(s: &str) -> Result<TiffFieldType, String> {
    TiffFieldType::from_u16(parse_num::<u16>(s)?)
}
/// 📦️ `TiffValues` — single-uppercase-letter tag prefix immediately followed by the bracketed
/// positional payload (same convention `svg`'s `enc_xml_node`/gif's enum codecs use): `B`=Byte,
/// `A`=Ascii, `S`=Short, `L`=Long, `R`=Rational, `E`=SByte, `U`=Undefined, `H`=SShort, `G`=SLong,
/// `Q`=SRational, `F`=Float, `D`=Double. `Byte`/`Undefined` (raw octets) and `Ascii` (text) are hex;
/// every numeric list is decimal comma-separated; `Rational`/`SRational` pairs nest as `[n,d]`.
pub(crate) fn enc_values(v: &TiffValues) -> String {
    match v {
        TiffValues::Byte(b) => format!("B[{}]", hex_encode(b)),
        TiffValues::Ascii(s) => format!("A[{}]", enc_str(s)),
        TiffValues::Short(v) => format!("S{}", enc_list(v, |x| x.to_string())),
        TiffValues::Long(v) => format!("L{}", enc_list(v, |x| x.to_string())),
        TiffValues::Rational(v) => format!("R{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::SByte(v) => format!("E{}", enc_list(v, |x| x.to_string())),
        TiffValues::Undefined(b) => format!("U[{}]", hex_encode(b)),
        TiffValues::SShort(v) => format!("H{}", enc_list(v, |x| x.to_string())),
        TiffValues::SLong(v) => format!("G{}", enc_list(v, |x| x.to_string())),
        TiffValues::SRational(v) => format!("Q{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::Float(v) => format!("F{}", enc_list(v, |x| x.to_string())),
        TiffValues::Double(v) => format!("D{}", enc_list(v, |x| x.to_string())),
    }
}
pub(crate) fn dec_values(s: &str) -> Result<TiffValues, String> {
    let (tag, rest) = s.split_at(1);
    let pair = |s: &str| -> Result<(u32, u32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<u32>(n)?, parse_num::<u32>(d)?))
    };
    let spair = |s: &str| -> Result<(i32, i32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("srational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<i32>(n)?, parse_num::<i32>(d)?))
    };
    match tag {
        "B" => Ok(TiffValues::Byte(hex_decode(strip_brackets(rest)?)?)),
        "A" => Ok(TiffValues::Ascii(dec_str(strip_brackets(rest)?)?)),
        "S" => Ok(TiffValues::Short(dec_list(rest, parse_num::<u16>)?)),
        "L" => Ok(TiffValues::Long(dec_list(rest, parse_num::<u32>)?)),
        "R" => Ok(TiffValues::Rational(dec_list(rest, pair)?)),
        "E" => Ok(TiffValues::SByte(dec_list(rest, parse_num::<i8>)?)),
        "U" => Ok(TiffValues::Undefined(hex_decode(strip_brackets(rest)?)?)),
        "H" => Ok(TiffValues::SShort(dec_list(rest, parse_num::<i16>)?)),
        "G" => Ok(TiffValues::SLong(dec_list(rest, parse_num::<i32>)?)),
        "Q" => Ok(TiffValues::SRational(dec_list(rest, spair)?)),
        "F" => Ok(TiffValues::Float(dec_list(rest, parse_num::<f32>)?)),
        "D" => Ok(TiffValues::Double(dec_list(rest, parse_num::<f64>)?)),
        other => Err(format!("tiff values: unknown tag {other:?}")),
    }
}
/// 🏷️ One IFD entry: `[tag,kind,values]` positional triple.
pub(crate) fn enc_tag(t: &TiffTag) -> String {
    format!("[{},{},{}]", t.tag, enc_field_type(t.kind), enc_values(&t.values))
}
pub(crate) fn dec_tag(s: &str) -> Result<TiffTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [tag, kind, values] = parts.as_slice() else { return Err(format!("tag: expected 3 fields, got {}", parts.len())) };
    Ok(TiffTag { tag: parse_num::<u16>(tag)?, kind: dec_field_type(kind)?, values: dec_values(values)? })
}
/// 🗂️ One IFD: bracketed list of `enc_tag` entries.
pub(crate) fn enc_ifd(ifd: &TiffIfd) -> String {
    enc_list(&ifd.entries, enc_tag)
}
pub(crate) fn dec_ifd(s: &str) -> Result<TiffIfd, String> {
    Ok(TiffIfd { entries: dec_list(s, dec_tag)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
/// 🔺️ Tag-id-keyed `entries` triple: `[removed];[modified];[added]`, `modified`/`added` entries
/// are `tag:kind:values` (colon-separated — safe since `kind` is bare decimal and `values` never
/// contains a literal `:`).
fn enc_tags_diff(d: &TiffTagsDiff) -> String {
    let removed = d.removed.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}:{}", m.tag, enc_field_type(m.kind), enc_values(&m.values))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.tag, enc_field_type(a.kind), enc_values(&a.values))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_tags_diff(body: &str) -> Result<TiffTagsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("tags diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<u16>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (tag_s, rest) = entry.split_once(':').ok_or_else(|| format!("tag modified: bad entry {entry:?}"))?;
        let (kind_s, values_s) = rest.split_once(':').ok_or_else(|| format!("tag modified: bad entry {entry:?}"))?;
        Ok(TiffTagModified { tag: parse_num::<u16>(tag_s)?, kind: dec_field_type(kind_s)?, values: dec_values(values_s)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (tag_s, rest) = entry.split_once(':').ok_or_else(|| format!("tag added: bad entry {entry:?}"))?;
        let (kind_s, values_s) = rest.split_once(':').ok_or_else(|| format!("tag added: bad entry {entry:?}"))?;
        Ok(TiffTagAdded { tag: parse_num::<u16>(tag_s)?, kind: dec_field_type(kind_s)?, values: dec_values(values_s)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(TiffTagsDiff { removed, modified, added })
}

/// 🗂️ Index-keyed `ifds` triple: `[removed];[modified];[added]`, `modified` entries are
/// `index:<tags-triple>` (recursive), `added` entries are `index:<ifd>`.
pub(crate) fn enc_ifds_diff(d: &TiffIfdsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_tags_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_ifd(&a.ifd))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
pub(crate) fn dec_ifds_diff(body: &str) -> Result<TiffIfdsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("ifds diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<usize>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd modified: bad entry {entry:?}"))?;
        Ok(TiffIfdModified { index: parse_num::<usize>(idx)?, diff: dec_tags_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd added: bad entry {entry:?}"))?;
        Ok(TiffIfdAdded { index: parse_num::<usize>(idx)?, ifd: dec_ifd(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(TiffIfdsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_tiff_diff(d: &TiffDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.byte_order {
        tokens.push(format!("byte-order={}", enc_byte_order(v)));
    }
    if let Some(v) = &d.ifds {
        tokens.push(format!("ifds={}", enc_ifds_diff(v)));
    }
    if let Some(v) = &d.pixels {
        tokens.push(format!("pixels={}", hex_encode(v)));
    }
    tokens.join(" ")
}
fn parse_tiff_diff(line: &str) -> Result<TiffDiff, String> {
    let mut d = TiffDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("byte-order=") {
            d.byte_order = Some(dec_byte_order(rest)?);
        } else if let Some(rest) = token.strip_prefix("ifds=") {
            d.ifds = Some(dec_ifds_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("pixels=") {
            d.pixels = Some(hex_decode(rest)?);
        } else {
            return Err(format!("tiff diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for TiffDiff {
    fn print_diff(&self) -> String {
        print_tiff_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_tiff_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`'s hand-rolled
    /// codecs use — satisfies every `DiffCodec` law without inventing a second wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `TiffDiff` grammar — exercises every
    /// `TiffValues` variant (incl. `Rational`/`SRational` pair lists and `Ascii`/`Byte` hex),
    /// both IFD-level (index-keyed) and tag-level (id-keyed) removed/modified/added, and the
    /// scalar `byte_order`/`pixels` tokens.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![
                TiffIfd {
                    entries: vec![
                        tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                        tag(258, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                        tag(315, TiffFieldType::Ascii, TiffValues::Ascii("An Author".into())),
                        tag(282, TiffFieldType::Rational, TiffValues::Rational(vec![(72, 1)])),
                        tag(700, TiffFieldType::Undefined, TiffValues::Undefined(vec![0xde, 0xad])),
                    ],
                },
                TiffIfd { entries: vec![tag(1, TiffFieldType::Byte, TiffValues::Byte(vec![1, 2, 3]))] },
            ],
            pixels: vec![0u8; 16],
        };
        let mut b = a.clone();
        b.byte_order = TiffByteOrder::BigEndian;
        b.ifds[0].entries.retain(|t| t.tag != 258); // remove
        b.ifds[0].entries.iter_mut().find(|t| t.tag == 315).unwrap().values = TiffValues::Ascii("New Author".into()); // modify
        b.ifds[0].entries.push(tag(37380, TiffFieldType::SRational, TiffValues::SRational(vec![(-3, 10), (0, 1)]))); // add
        b.ifds[0].entries.push(tag(50000, TiffFieldType::SByte, TiffValues::SByte(vec![-1, -2])));
        b.ifds[0].entries.push(tag(50001, TiffFieldType::SShort, TiffValues::SShort(vec![-100])));
        b.ifds[0].entries.push(tag(50002, TiffFieldType::SLong, TiffValues::SLong(vec![-100000])));
        b.ifds[0].entries.push(tag(50003, TiffFieldType::Float, TiffValues::Float(vec![1.5, -2.25])));
        b.ifds[0].entries.push(tag(50004, TiffFieldType::Double, TiffValues::Double(vec![3.14159265358979])));
        b.ifds.push(TiffIfd { entries: vec![tag(2, TiffFieldType::Long, TiffValues::Long(vec![9]))] }); // whole IFD added
        b.pixels = vec![9u8; 16];
        let c = TiffSnapshot { schema: "stdio.tiff".into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![], pixels: vec![] };

        let cases = vec![
            TiffDiff::default(),
            TiffDiff::between(&a, &b),
            TiffDiff::between(&b, &a),
            TiffDiff::between(&a, &c),
            TiffDiff::between(&c, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = TiffDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = TiffDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
