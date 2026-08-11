//! 🔺️ DwgDiff — handcrafted sparse diff for `ac1024`. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION F5: replaces the old
//! `DwgDiff{snapshot: Option<DwgSnapshot>}` full-replace template with a real per-field patch —
//! `version`/`maintenance_version`/`codepage` header scalars, raw `bytes`, and a name-keyed
//! triple (`removed`/`modified`/`added`) over `sections`. Section identity (`name`) is immutable
//! (DWG section names are the format's own fixed labels, never user-renamed), so — unlike zip's
//! entries — no rename-transport map is needed in `absorb`, simplifying it relative to the
//! zip precedent this file otherwise mirrors closely.
//!
//! `section_names`/`decode_status` are DERIVED from `sections` (see `schema::snapshot`'s
//! `derive_section_names`/`derive_decode_status`) and deliberately do NOT appear as their own
//! diff fields — `apply` always recomputes them from the post-apply `sections` list, so they can
//! never drift out of sync with the collection that determines them.

use std::collections::HashSet;

use crate::artifacts::dwg::schema::snapshot::{derive_decode_status, derive_section_names};
use crate::artifacts::dwg::schema::snapshot::{DwgSection, DwgSectionPage};
use crate::artifacts::dwg::DwgSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️SectionDiff
/// 🎒️ Sparse per-field patch for one `DwgSection`. `pages` is a weak value-list (per the recipe's
/// weak-entity rule) — whole-vec replaced, never sub-diffed byte-range-at-a-time: DWG sections
/// aren't edited that way by any real use case here (mirrors the ticket's own guidance for this
/// artifact).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<DwgSectionPage>>,
}

fn apply_section_diff(section: &mut DwgSection, diff: &DwgSectionDiff) {
    if let Some(v) = diff.compressed { section.compressed = v; }
    if let Some(v) = diff.declared_size { section.declared_size = v; }
    if let Some(v) = &diff.pages { section.pages = v.clone(); }
}

fn section_between(a: &DwgSection, b: &DwgSection) -> DwgSectionDiff {
    DwgSectionDiff {
        compressed: (a.compressed != b.compressed).then_some(b.compressed),
        declared_size: (a.declared_size != b.declared_size).then_some(b.declared_size),
        pages: (a.pages != b.pages).then(|| b.pages.clone()),
    }
}

fn section_diff_is_empty(d: &DwgSectionDiff) -> bool {
    d == &DwgSectionDiff::default()
}

fn absorb_section_diff(base: &mut DwgSectionDiff, other: DwgSectionDiff) {
    if other.compressed.is_some() { base.compressed = other.compressed; }
    if other.declared_size.is_some() { base.declared_size = other.declared_size; }
    if other.pages.is_some() { base.pages = other.pages; }
}
//#endregion 🔖️SectionDiff

//#region 🔖️SectionsTriple
/// 📦️ One `sections.modified[]` entity — `name` is the section's identity (immutable).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionModified {
    pub name: String,
    pub diff: DwgSectionDiff,
}

/// 📦️ One `sections.added[]` entity — `index` is the section's position in the FINAL sequence
/// (apply semantics: `added` indices refer to final state, inserted ascending at `min(index,
/// len)`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionAdded {
    pub index: usize,
    pub section: DwgSection,
}

/// 📦️ Sparse name-keyed `sections` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSectionsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DwgSectionModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DwgSectionAdded>,
}

impl DwgSectionsDiff {
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️SectionsTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dwg` (ac1024). `schema` is an identity field and never appears here;
/// `section_names`/`decode_status` are derived (see module doc) and never appear here either.
/// 🧪️ F6: `dsl::DslDiff` derive added — emits `protocol::DiffCodec` (print_diff/parse_diff/
/// encode_diff/decode_diff) from the same `RecordSpec` machinery `DslRecord` uses. Verified
/// DERIVE-eligible for real (per f6-recon-report.md §3's decision rule): zero `Option<Option<_>>`
/// fields anywhere in this diff's field tree (every nullable field here is a single-layer
/// `Option<T>` — "the new value", never tri-state "removed vs unchanged"), and zero data-carrying
/// enums reachable from it (`DwgDecodeStatus` is unit-variant-only and DOESN'T even appear here —
/// it's derived, see module doc — so it's moot either way). `bytes: Option<Vec<u8>>` does NOT get
/// the compact base64 grammar (`#[dsl(base64)]` is a documented no-op through one `Option` layer
/// per the recon report's derive-quirk note) — falls back to a verbose bracketed decimal list,
/// harmless for this ticket's small test fixtures.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.diff")]
pub struct DwgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_version: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codepage: Option<u16>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<DwgSectionsDiff>,
}

impl MutationDiff<DwgSnapshot> for DwgDiff {
    fn apply(&self, base: &DwgSnapshot) -> DwgSnapshot {
        let mut sections = base.sections.clone();
        if let Some(sd) = &self.sections {
            // 1. removed — by BASE name.
            if !sd.removed.is_empty() {
                let removed: HashSet<&str> = sd.removed.iter().map(String::as_str).collect();
                sections.retain(|s| !removed.contains(s.name.as_str()));
            }
            // 2. modified — found by BASE name; modified-of-already-removed is a graceful no-op.
            for m in &sd.modified {
                if let Some(s) = sections.iter_mut().find(|s| s.name == m.name) {
                    apply_section_diff(s, &m.diff);
                }
            }
            // 3. added — stable-sorted ascending by final index, sequential `insert(min(index,
            //    len))` (matches zip's own documented "two inserts at the same index" stability).
            let mut adds: Vec<&DwgSectionAdded> = sd.added.iter().collect();
            adds.sort_by_key(|a| a.index);
            for a in adds {
                let at = a.index.min(sections.len());
                sections.insert(at, a.section.clone());
            }
        }
        let section_names = derive_section_names(&sections);
        let decode_status = derive_decode_status(&sections);
        DwgSnapshot {
            schema: base.schema.clone(),
            version: self.version.clone().unwrap_or_else(|| base.version.clone()),
            maintenance_version: self.maintenance_version.unwrap_or(base.maintenance_version),
            codepage: self.codepage.unwrap_or(base.codepage),
            bytes: self.bytes.clone().unwrap_or_else(|| base.bytes.clone()),
            section_names,
            sections,
            decode_status,
        }
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `sections`: name-keyed transport, simplified relative to zip's own `absorb_entries` since
    /// section names are immutable (no rename map needed) — `other`'s removal of a `self`-added
    /// section annihilates the add; `other`'s modification of a `self`-added section patches
    /// directly into the carried added payload; a surviving `self`-added item's final index is
    /// decremented by the count of genuine (non-annihilated) `other.removed` names, exact when
    /// those removals sit before the add (same documented best-effort caveat as zip's).
    fn absorb(&mut self, other: Self) {
        if other.version.is_some() { self.version = other.version; }
        if other.maintenance_version.is_some() { self.maintenance_version = other.maintenance_version; }
        if other.codepage.is_some() { self.codepage = other.codepage; }
        if other.bytes.is_some() { self.bytes = other.bytes; }
        self.sections = absorb_sections(self.sections.take(), other.sections);
    }
}

/// ➕️ Free-function core of `DwgDiff::absorb`'s `sections` merge (standalone so it composes
/// cleanly and stays unit-testable without a full `DwgDiff`).
fn absorb_sections(d1: Option<DwgSectionsDiff>, d2: Option<DwgSectionsDiff>) -> Option<DwgSectionsDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    let added_names: HashSet<String> = d1.added.iter().map(|a| a.section.name.clone()).collect();

    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    let mut removed_shift_count = 0usize;

    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else {
            removed_shift_count += 1;
            if !merged_removed.contains(name) {
                merged_removed.push(name.clone());
            }
            d1.modified.retain(|m| &m.name != name);
        }
    }

    let mut merged_modified: Vec<DwgSectionModified> = d1.modified;
    let mut merged_added: Vec<DwgSectionAdded> = d1
        .added
        .into_iter()
        .filter(|a| !annihilated.contains(&a.section.name))
        .map(|mut a| { a.index = a.index.saturating_sub(removed_shift_count); a })
        .collect();

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue; // modified-of-annihilated-add: moot.
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.section.name == dm.name) {
                apply_section_diff(&mut a.section, &dm.diff);
            }
        } else {
            if merged_removed.contains(&dm.name) {
                continue; // modified-of-removed: illegal, ignored (matches apply()'s no-op rule).
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == dm.name) {
                absorb_section_diff(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(DwgSectionModified { name: dm.name.clone(), diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);

    let merged = DwgSectionsDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() { None } else { Some(merged) }
}

impl DiffAlgebra<DwgSnapshot> for DwgDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base`.
    fn inverse(&self, base: &DwgSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): name-keyed matching over `sections`, scalars compare
    /// field-by-field. `section_names`/`decode_status` are never compared directly (derived).
    fn between(base: &DwgSnapshot, other: &DwgSnapshot) -> Self {
        let version = (base.version != other.version).then(|| other.version.clone());
        let maintenance_version = (base.maintenance_version != other.maintenance_version).then_some(other.maintenance_version);
        let codepage = (base.codepage != other.codepage).then_some(other.codepage);
        let bytes = (base.bytes != other.bytes).then(|| other.bytes.clone());
        let sections = if base.sections == other.sections {
            None
        } else {
            let base_names: HashSet<&str> = base.sections.iter().map(|s| s.name.as_str()).collect();
            let other_names: HashSet<&str> = other.sections.iter().map(|s| s.name.as_str()).collect();

            let removed: Vec<String> = base.sections.iter()
                .filter(|s| !other_names.contains(s.name.as_str()))
                .map(|s| s.name.clone())
                .collect();

            let mut modified = Vec::new();
            for bs in &base.sections {
                if let Some(os) = other.sections.iter().find(|o| o.name == bs.name) {
                    let d = section_between(bs, os);
                    if !section_diff_is_empty(&d) {
                        modified.push(DwgSectionModified { name: bs.name.clone(), diff: d });
                    }
                }
            }

            let added: Vec<DwgSectionAdded> = other.sections.iter().enumerate()
                .filter(|(_, s)| !base_names.contains(s.name.as_str()))
                .map(|(index, s)| DwgSectionAdded { index, section: s.clone() })
                .collect();

            let d = DwgSectionsDiff { removed, modified, added };
            if d.is_empty() { None } else { Some(d) }
        };
        DwgDiff { version, maintenance_version, codepage, bytes, sections }
    }

    fn is_empty(&self) -> bool {
        self.version.is_none()
            && self.maintenance_version.is_none()
            && self.codepage.is_none()
            && self.bytes.is_none()
            && self.sections.as_ref().map_or(true, DwgSectionsDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `DwgDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &DwgSnapshot, next: &DwgSnapshot) -> DwgDiff {
    DwgDiff::between(base, next)
}

/// 🧩 Patches `bytes` at the plain-preamble offsets (`0..6` for `version` when it is exactly 6
/// bytes, `0x12` for `maintenance_version`, `0x13..0x15` LE for `codepage`), growing `bytes` with
/// zero padding first if it's too short to reach `0x15`. Keeps the scalar mirror fields and the
/// byte-level ground truth in sync, the same invariant `encode_dwg` already enforces for
/// `version`.
pub fn patch_version_info_bytes(bytes: &[u8], version: &str, maintenance_version: u8, codepage: u16) -> Vec<u8> {
    let mut out = bytes.to_vec();
    if out.len() < 0x15 {
        out.resize(0x15, 0);
    }
    if version.as_bytes().len() == 6 {
        out[0..6].copy_from_slice(version.as_bytes());
    }
    out[0x12] = maintenance_version;
    out[0x13..0x15].copy_from_slice(&codepage.to_le_bytes());
    out
}

pub fn diff_set_version_info(base: &DwgSnapshot, version: &str, maintenance_version: u8, codepage: u16) -> DwgDiff {
    let new_bytes = patch_version_info_bytes(&base.bytes, version, maintenance_version, codepage);
    DwgDiff {
        version: (base.version != version).then(|| version.to_string()),
        maintenance_version: (base.maintenance_version != maintenance_version).then_some(maintenance_version),
        codepage: (base.codepage != codepage).then_some(codepage),
        bytes: (base.bytes != new_bytes).then_some(new_bytes),
        sections: None,
    }
}

pub fn diff_insert_section(index: usize, section: DwgSection) -> DwgDiff {
    DwgDiff { sections: Some(DwgSectionsDiff { removed: vec![], modified: vec![], added: vec![DwgSectionAdded { index, section }] }), ..Default::default() }
}

pub fn diff_remove_section(name: &str) -> DwgDiff {
    DwgDiff { sections: Some(DwgSectionsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_section_data(name: &str, compressed: bool, declared_size: u64, pages: Vec<DwgSectionPage>) -> DwgDiff {
    let diff = DwgSectionDiff { compressed: Some(compressed), declared_size: Some(declared_size), pages: Some(pages) };
    DwgDiff { sections: Some(DwgSectionsDiff { removed: vec![], modified: vec![DwgSectionModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn page(n: i32, addr: u64, size: u32, decoded: &[u8]) -> DwgSectionPage {
        DwgSectionPage { page_number: n, file_address: addr, compressed_size: size, decoded: decoded.to_vec(), error: None }
    }

    fn section(name: &str, compressed: bool, declared_size: u64, pages: Vec<DwgSectionPage>) -> DwgSection {
        DwgSection { name: name.into(), compressed, declared_size, pages }
    }

    /// 🧪️ F6: `DiffCodec` round-trip law (derived via `dsl::DslDiff`) — exercises every scalar
    /// field plus every arm of the `sections` triple (`removed`/`modified`/`added`) at once,
    /// including a `DwgSectionPage.error: Some(String)` payload so the derive's `Option<String>`
    /// (single-layer, NOT tri-state) path is covered too.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let cases = vec![
            DwgDiff::default(),
            DwgDiff {
                version: Some("AC1032".into()),
                maintenance_version: Some(9),
                codepage: Some(65001),
                bytes: Some(vec![0xAA, 0xBB, 0xCC]),
                sections: Some(DwgSectionsDiff {
                    removed: vec!["gone".into()],
                    modified: vec![DwgSectionModified {
                        name: "stay".into(),
                        diff: DwgSectionDiff {
                            compressed: Some(false),
                            declared_size: Some(999),
                            pages: Some(vec![
                                page(0, 0x900, 50, b"after"),
                                {
                                    let mut p = page(1, 0x901, 10, b"");
                                    p.error = Some("truncated page".into());
                                    p
                                },
                            ]),
                        },
                    }],
                    added: vec![DwgSectionAdded { index: 2, section: section("new", true, 5, vec![page(2, 0x30, 5, b"brand new")]) }],
                }),
            },
            diff_set_version_info(&DwgSnapshot::default(), "AC1024", 2, 30),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = DwgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = DwgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }
}
//#endregion 🧪️Tests
