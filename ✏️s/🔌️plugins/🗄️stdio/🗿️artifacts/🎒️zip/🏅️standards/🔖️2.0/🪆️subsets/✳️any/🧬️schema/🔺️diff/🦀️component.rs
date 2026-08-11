//! 🔺️ ZipDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `ZipDiff{snapshot: Option<ZipSnapshot>}` full-replace template with a real per-field patch —
//! archive `comment` plus a name-keyed `entries` triple (`removed`/`modified`/`added`), every
//! `ZipEntry` field individually patchable (including a `name` field on `ZipEntryDiff` for
//! renames and a tri-state `unixMtime` for clearing the Info-ZIP timestamp).
//!
//! 🧪️ F6 FINDING: `#[derive(dsl::DslDiff)]` CANNOT be used on `ZipDiff` — confirmed by real
//! `cargo check` (not guessed): `ZipEntryDiff::unix_mtime: Option<Option<i64>>` gives `error[E0277]:
//! the trait bound std::option::Option<i64>: DslField is not satisfied`. Root cause (per the
//! ticket's `f6-recon-report.md` §3b): `dsl_derive::classify_field` peels exactly ONE `Option<..>`
//! layer before binding, so a tri-state field's REMAINING type after that peel is `Option<i64>`
//! itself, and no `impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl` crate.
//! `unix_mtime` is zip's only tri-state field and there is zero data-carrying enum anywhere in
//! `ZipDiff`'s tree (`ZipCompressionMethod` is unit-variant-only, `DslScalar`-eligible — see
//! `🧬️mutations/component.rs`, whose `ZipMutation` DOES derive cleanly via `dsl::DslOps`, the
//! same "diff hand-rolled, mutation derived" split `f6-recon-report.md` documents for gif 89a).
//! `DiffCodec` for `ZipDiff` is hand-rolled below instead, following the report's §5 grammar
//! template (hex for strings/bytes, positional `[f1,f2,...]` tuples for structs, single-letter
//! `tag:value` pairs for `ZipEntryDiff`'s own sparse fields, `name{[removed];[modified];[added]}`
//! for the `entries` collection triple — adapted here for a NAME-keyed, not index-keyed,
//! collection: `removed`/`modified` keys are hex-encoded entry names, `added` keys are the
//! final-position `usize` index, matching `ZipEntriesDiff`'s own field types above).

use std::collections::{HashMap, HashSet};

use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry, ZipExtraField};
use crate::artifacts::zip::ZipSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️EntryDiff
/// 🎒️ Sparse per-field patch for one `ZipEntry`. `name` present = rename (the entry keeps its
/// identity in `ZipEntriesDiff` via the pre-rename name in `ZipEntryModified::name` — renames are
/// tracked through absorb's key-transport map, never by re-keying mid-merge). `unix_mtime` is
/// tri-state: `None` = unchanged, `Some(None)` = the Info-ZIP `UT` timestamp was cleared,
/// `Some(Some(t))` = set to `t`. `local_extra`/`central_extra` are weak value-lists — whole-vec
/// replaced, never sub-diffed (matches the recipe's weak-entity rule for `ZipExtraField`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntryDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<ZipCompressionMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dos_date: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dos_time: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_mtime: Option<Option<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_made_by: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_needed: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_attrs: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_attrs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_extra: Option<Vec<ZipExtraField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub central_extra: Option<Vec<ZipExtraField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// ▶️ Applies a per-field entry patch in place.
fn apply_entry_diff(entry: &mut ZipEntry, diff: &ZipEntryDiff) {
    if let Some(v) = &diff.name { entry.name = v.clone(); }
    if let Some(v) = &diff.data { entry.data = v.clone(); }
    if let Some(v) = diff.method { entry.method = v; }
    if let Some(v) = diff.dos_date { entry.dos_date = v; }
    if let Some(v) = diff.dos_time { entry.dos_time = v; }
    if let Some(v) = &diff.unix_mtime { entry.unix_mtime = *v; }
    if let Some(v) = diff.flags { entry.flags = v; }
    if let Some(v) = diff.version_made_by { entry.version_made_by = v; }
    if let Some(v) = diff.version_needed { entry.version_needed = v; }
    if let Some(v) = diff.internal_attrs { entry.internal_attrs = v; }
    if let Some(v) = diff.external_attrs { entry.external_attrs = v; }
    if let Some(v) = &diff.local_extra { entry.local_extra = v.clone(); }
    if let Some(v) = &diff.central_extra { entry.central_extra = v.clone(); }
    if let Some(v) = &diff.comment { entry.comment = v.clone(); }
}

/// 🧭️ Field-by-field state delta between two entries sharing the same identity slot.
fn entry_between(a: &ZipEntry, b: &ZipEntry) -> ZipEntryDiff {
    ZipEntryDiff {
        name: (a.name != b.name).then(|| b.name.clone()),
        data: (a.data != b.data).then(|| b.data.clone()),
        method: (a.method != b.method).then_some(b.method),
        dos_date: (a.dos_date != b.dos_date).then_some(b.dos_date),
        dos_time: (a.dos_time != b.dos_time).then_some(b.dos_time),
        unix_mtime: (a.unix_mtime != b.unix_mtime).then_some(b.unix_mtime),
        flags: (a.flags != b.flags).then_some(b.flags),
        version_made_by: (a.version_made_by != b.version_made_by).then_some(b.version_made_by),
        version_needed: (a.version_needed != b.version_needed).then_some(b.version_needed),
        internal_attrs: (a.internal_attrs != b.internal_attrs).then_some(b.internal_attrs),
        external_attrs: (a.external_attrs != b.external_attrs).then_some(b.external_attrs),
        local_extra: (a.local_extra != b.local_extra).then(|| b.local_extra.clone()),
        central_extra: (a.central_extra != b.central_extra).then(|| b.central_extra.clone()),
        comment: (a.comment != b.comment).then(|| b.comment.clone()),
    }
}

fn entry_diff_is_empty(d: &ZipEntryDiff) -> bool {
    d == &ZipEntryDiff::default()
}

/// ➕️ LWW field-by-field absorb of one entry patch into another (`other` was authored after
/// `self` against the state `self` already produced — later field values win).
fn absorb_entry_diff(base: &mut ZipEntryDiff, other: ZipEntryDiff) {
    if other.name.is_some() { base.name = other.name; }
    if other.data.is_some() { base.data = other.data; }
    if other.method.is_some() { base.method = other.method; }
    if other.dos_date.is_some() { base.dos_date = other.dos_date; }
    if other.dos_time.is_some() { base.dos_time = other.dos_time; }
    if other.unix_mtime.is_some() { base.unix_mtime = other.unix_mtime; }
    if other.flags.is_some() { base.flags = other.flags; }
    if other.version_made_by.is_some() { base.version_made_by = other.version_made_by; }
    if other.version_needed.is_some() { base.version_needed = other.version_needed; }
    if other.internal_attrs.is_some() { base.internal_attrs = other.internal_attrs; }
    if other.external_attrs.is_some() { base.external_attrs = other.external_attrs; }
    if other.local_extra.is_some() { base.local_extra = other.local_extra; }
    if other.central_extra.is_some() { base.central_extra = other.central_extra; }
    if other.comment.is_some() { base.comment = other.comment; }
}
//#endregion 🔖️EntryDiff

//#region 🔖️EntriesTriple
/// 📦️ One `entries.modified[]` entity — `name` is the entry's identity **in BASE** (pre-rename;
/// see `ZipEntryDiff::name` for the rename payload itself).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntryModified {
    pub name: String,
    pub diff: ZipEntryDiff,
}

/// 📦️ One `entries.added[]` entity — `index` is the entry's position in the FINAL sequence
/// (apply semantics: `added` indices refer to final state, inserted ascending at `min(index,
/// len)`; see engine module docs / recipe `## Absorb` for the full apply/absorb contract).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntryAdded {
    pub index: usize,
    pub entry: ZipEntry,
}

/// 📦️ Sparse name-keyed `entries` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntriesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ZipEntryModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ZipEntryAdded>,
}

impl ZipEntriesDiff {
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️EntriesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.zip`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.diff")]
pub struct ZipDiff {
    /// 💬️ Archive-level (EOCD) comment.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<ZipEntriesDiff>,
}

impl MutationDiff<ZipSnapshot> for ZipDiff {
    fn apply(&self, base: &ZipSnapshot) -> ZipSnapshot {
        let mut entries = base.entries.clone();
        if let Some(ed) = &self.entries {
            // 1. removed — by BASE name, descending order doesn't matter for a name-keyed retain.
            if !ed.removed.is_empty() {
                let removed: HashSet<&str> = ed.removed.iter().map(String::as_str).collect();
                entries.retain(|e| !removed.contains(e.name.as_str()));
            }
            // 2. modified — found by BASE name; a modified-of-already-removed name is a graceful
            //    no-op (the `find` below simply won't match).
            for m in &ed.modified {
                if let Some(e) = entries.iter_mut().find(|e| e.name == m.name) {
                    apply_entry_diff(e, &m.diff);
                }
            }
            // 3. added — stable-sorted ascending by final index, sequential `insert(min(index,
            //    len))`; stability is load-bearing for the "two inserts at the same index" case
            //    (the later-listed insert lands at the lower final position, matching sequential
            //    mutation application — see `absorb`'s doc comment).
            let mut adds: Vec<&ZipEntryAdded> = ed.added.iter().collect();
            adds.sort_by_key(|a| a.index);
            for a in adds {
                let at = a.index.min(entries.len());
                entries.insert(at, a.entry.clone());
            }
        }
        ZipSnapshot {
            schema: base.schema.clone(),
            entries,
            comment: self.comment.clone().unwrap_or_else(|| base.comment.clone()),
        }
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `entries`: name/key-transport φ built from `self`'s own renames (`modified[].diff.name`)
    /// and additions — `other`'s removed/modified are resolved back to BASE identity through that
    /// map before merging (`removed: r1 ∪ φ⁻¹(r2 ∩ Base)`, a `other`-removal of a `self`-added
    /// name annihilates the add instead of surfacing as a base removal, a `other`-modification of
    /// a `self`-added name patches directly into the carried added payload). `added` index
    /// bookkeeping (`ψ`) is derived only from what the two diffs themselves make observable: a
    /// surviving `self`-added item's final index is decremented by the count of `other.removed`
    /// names that resolve to a genuine mid-state removal (not an annihilation) — this is exact
    /// when those removed names sit *before* the add (the recipe's own `Insert+Remove` canonical
    /// case) and is a documented best-effort approximation when an untouched, unrenamed base
    /// survivor removed by `other` in fact sat *after* the add (position information this
    /// key-kind's diffs do not carry — see the ticket's `deviations` note).
    fn absorb(&mut self, other: Self) {
        if other.comment.is_some() {
            self.comment = other.comment;
        }
        self.entries = absorb_entries(self.entries.take(), other.entries);
    }
}

/// ➕️ Free-function core of `ZipDiff::absorb`'s `entries` merge (kept standalone so it composes
/// cleanly and stays unit-testable without a full `ZipDiff`).
fn absorb_entries(d1: Option<ZipEntriesDiff>, d2: Option<ZipEntriesDiff>) -> Option<ZipEntriesDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    // φ: base name -> mid name, from d1's own renames.
    let rename_map: HashMap<String, String> = d1.modified.iter()
        .filter_map(|m| m.diff.name.as_ref().map(|n| (m.name.clone(), n.clone())))
        .collect();
    let reverse_rename: HashMap<&str, &str> = rename_map.iter().map(|(k, v)| (v.as_str(), k.as_str())).collect();
    let added_names: HashSet<String> = d1.added.iter().map(|a| a.entry.name.clone()).collect();

    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    let mut removed_shift_count = 0usize;

    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else {
            let base_name = reverse_rename.get(name.as_str()).map(|s| s.to_string()).unwrap_or_else(|| name.clone());
            removed_shift_count += 1;
            if !merged_removed.contains(&base_name) {
                merged_removed.push(base_name.clone());
            }
            d1.modified.retain(|m| m.name != base_name);
        }
    }

    let mut merged_modified: Vec<ZipEntryModified> = d1.modified;
    let mut merged_added: Vec<ZipEntryAdded> = d1.added.into_iter()
        .filter(|a| !annihilated.contains(&a.entry.name))
        .map(|mut a| { a.index = a.index.saturating_sub(removed_shift_count); a })
        .collect();

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue; // modified-of-annihilated-add: moot.
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.entry.name == dm.name) {
                apply_entry_diff(&mut a.entry, &dm.diff);
            }
        } else {
            let base_name = reverse_rename.get(dm.name.as_str()).map(|s| s.to_string()).unwrap_or_else(|| dm.name.clone());
            if merged_removed.contains(&base_name) {
                continue; // modified-of-removed: illegal, ignored (matches apply()'s no-op rule).
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == base_name) {
                absorb_entry_diff(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(ZipEntryModified { name: base_name, diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);

    let merged = ZipEntriesDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() { None } else { Some(merged) }
}

impl DiffAlgebra<ZipSnapshot> for ZipDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base` — `between` is the single source of truth for turning a
    /// state pair into a diff, so `inverse` doesn't duplicate its per-field logic.
    fn inverse(&self, base: &ZipSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): name-keyed matching — a rename shows as a
    /// remove+add pair (documented; `between` never infers renames, only mutation-level
    /// `RenameEntry` diffs do), everything else compares field-by-field.
    fn between(base: &ZipSnapshot, other: &ZipSnapshot) -> Self {
        let comment = (base.comment != other.comment).then(|| other.comment.clone());
        let entries = if base.entries == other.entries {
            None
        } else {
            let base_names: HashSet<&str> = base.entries.iter().map(|e| e.name.as_str()).collect();
            let other_names: HashSet<&str> = other.entries.iter().map(|e| e.name.as_str()).collect();

            let removed: Vec<String> = base.entries.iter()
                .filter(|e| !other_names.contains(e.name.as_str()))
                .map(|e| e.name.clone())
                .collect();

            let mut modified = Vec::new();
            for be in &base.entries {
                if let Some(oe) = other.entries.iter().find(|o| o.name == be.name) {
                    let d = entry_between(be, oe);
                    if !entry_diff_is_empty(&d) {
                        modified.push(ZipEntryModified { name: be.name.clone(), diff: d });
                    }
                }
            }

            let added: Vec<ZipEntryAdded> = other.entries.iter().enumerate()
                .filter(|(_, e)| !base_names.contains(e.name.as_str()))
                .map(|(index, e)| ZipEntryAdded { index, entry: e.clone() })
                .collect();

            let d = ZipEntriesDiff { removed, modified, added };
            if d.is_empty() { None } else { Some(d) }
        };
        ZipDiff { comment, entries }
    }

    fn is_empty(&self) -> bool {
        self.comment.is_none() && self.entries.as_ref().map_or(true, ZipEntriesDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `ZipDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &ZipSnapshot, next: &ZipSnapshot) -> ZipDiff {
    ZipDiff::between(base, next)
}
pub fn diff_set_archive_comment(comment: &str) -> ZipDiff {
    ZipDiff { comment: Some(comment.to_string()), entries: None }
}
pub fn diff_add_entry(index: usize, entry: ZipEntry) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { removed: vec![], modified: vec![], added: vec![ZipEntryAdded { index, entry }] }) }
}
pub fn diff_remove_entry(name: &str) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }) }
}
fn diff_entry_field(name: &str, field: ZipEntryDiff) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { removed: vec![], modified: vec![ZipEntryModified { name: name.to_string(), diff: field }], added: vec![] }) }
}
pub fn diff_rename_entry(name: &str, new_name: &str) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { name: Some(new_name.to_string()), ..Default::default() })
}
pub fn diff_set_entry_data(name: &str, data: Vec<u8>) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { data: Some(data), ..Default::default() })
}
pub fn diff_set_entry_method(name: &str, method: ZipCompressionMethod) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { method: Some(method), ..Default::default() })
}
pub fn diff_set_entry_timestamps(name: &str, dos_date: u16, dos_time: u16, unix_mtime: Option<i64>) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { dos_date: Some(dos_date), dos_time: Some(dos_time), unix_mtime: Some(unix_mtime), ..Default::default() })
}
pub fn diff_set_entry_flags(name: &str, flags: u16) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { flags: Some(flags), ..Default::default() })
}
pub fn diff_set_entry_versions(name: &str, version_made_by: u16, version_needed: u16) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { version_made_by: Some(version_made_by), version_needed: Some(version_needed), ..Default::default() })
}
pub fn diff_set_entry_attributes(name: &str, internal_attrs: u16, external_attrs: u32) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { internal_attrs: Some(internal_attrs), external_attrs: Some(external_attrs), ..Default::default() })
}
pub fn diff_set_entry_extra(name: &str, local_extra: Vec<ZipExtraField>, central_extra: Vec<ZipExtraField>) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { local_extra: Some(local_extra), central_extra: Some(central_extra), ..Default::default() })
}
pub fn diff_set_entry_comment(name: &str, comment: &str) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { comment: Some(comment.to_string()), ..Default::default() })
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `ZipDiff` (see the module doc comment for the
/// confirmed derive-blocking compile error). Grammar matches `f6-recon-report.md` §5 and gif 89a's
/// precedent exactly: space-separated `name=value` top-level tokens (absent token = unchanged),
/// hex for strings/bytes, positional `[f1,f2,...]` tuples for plain structs, a uniform
/// `[0]`=None / `[1,<T>]`=Some(T) tag for every `Option<T>` (real optional fields AND diff
/// tri-states alike), single-letter `tag:value` pairs for `ZipEntryDiff`'s sparse fields, and
/// `entries{[removed];[modified];[added]}` for the one collection triple — `removed`/`modified`
/// keyed by hex-encoded entry NAME (matching `ZipEntriesDiff`'s own `String` key), `added` keyed
/// by the final-position `usize` index (matching `ZipEntryAdded::index`).
//#region 🔖️Primitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn hex_decode_string(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_u16(s: &str) -> Result<u16, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_i64(s: &str) -> Result<i64, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
fn enc_method(m: ZipCompressionMethod) -> char {
    match m {
        ZipCompressionMethod::Stored => 's',
        ZipCompressionMethod::Deflate => 'd',
    }
}
fn dec_method(s: &str) -> Result<ZipCompressionMethod, String> {
    match s {
        "s" => Ok(ZipCompressionMethod::Stored),
        "d" => Ok(ZipCompressionMethod::Deflate),
        other => Err(format!("bad compression method {other:?}")),
    }
}
fn enc_extra_field(e: &ZipExtraField) -> String {
    format!("[{},{}]", e.id, hex_encode(&e.payload))
}
fn dec_extra_field(s: &str) -> Result<ZipExtraField, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, payload] = parts.as_slice() else { return Err(format!("extra field: expected 2 fields, got {}", parts.len())) };
    Ok(ZipExtraField { id: parse_u16(id)?, payload: hex_decode(payload)? })
}
fn enc_extra_list(v: &[ZipExtraField]) -> String {
    format!("[{}]", v.iter().map(enc_extra_field).collect::<Vec<_>>().join(","))
}
fn dec_extra_list(s: &str) -> Result<Vec<ZipExtraField>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_extra_field).collect()
}
/// 🧭️ Whole `ZipEntry` — positional tuple, field order matches the struct declaration exactly
/// (📸️snapshot/component.rs).
fn enc_entry(e: &ZipEntry) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        hex_encode(e.name.as_bytes()),
        hex_encode(&e.data),
        enc_method(e.method),
        e.dos_date,
        e.dos_time,
        encode_option(&e.unix_mtime, |v| v.to_string()),
        e.flags,
        e.version_made_by,
        e.version_needed,
        e.internal_attrs,
        e.external_attrs,
        enc_extra_list(&e.local_extra),
        enc_extra_list(&e.central_extra),
        hex_encode(e.comment.as_bytes()),
    )
}
fn dec_entry(s: &str) -> Result<ZipEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, data, method, dos_date, dos_time, unix_mtime, flags, version_made_by, version_needed, internal_attrs, external_attrs, local_extra, central_extra, comment] = parts.as_slice() else {
        return Err(format!("entry: expected 14 fields, got {}", parts.len()));
    };
    Ok(ZipEntry {
        name: hex_decode_string(name)?,
        data: hex_decode(data)?,
        method: dec_method(method)?,
        dos_date: parse_u16(dos_date)?,
        dos_time: parse_u16(dos_time)?,
        unix_mtime: decode_option(unix_mtime, parse_i64)?,
        flags: parse_u16(flags)?,
        version_made_by: parse_u16(version_made_by)?,
        version_needed: parse_u16(version_needed)?,
        internal_attrs: parse_u16(internal_attrs)?,
        external_attrs: external_attrs.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        local_extra: dec_extra_list(local_extra)?,
        central_extra: dec_extra_list(central_extra)?,
        comment: hex_decode_string(comment)?,
    })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
/// 🧭️ `ZipEntryDiff`'s sparse fields as single-letter `tag:value` pairs (only present tags are
/// emitted — an absent tag = unchanged). `U` (`unix_mtime`) is the artifact's one tri-state field:
/// its `Option<Option<i64>>` value's OUTER layer gates emission (like every other field here), its
/// INNER `Option<i64>` uses the shared `encode_option`/`decode_option` primitive.
fn enc_entry_diff(d: &ZipEntryDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.name { parts.push(format!("N:{}", hex_encode(v.as_bytes()))); }
    if let Some(v) = &d.data { parts.push(format!("D:{}", hex_encode(v))); }
    if let Some(v) = d.method { parts.push(format!("M:{}", enc_method(v))); }
    if let Some(v) = d.dos_date { parts.push(format!("A:{v}")); }
    if let Some(v) = d.dos_time { parts.push(format!("T:{v}")); }
    if let Some(v) = d.unix_mtime { parts.push(format!("U:{}", encode_option(&v, |x| x.to_string()))); }
    if let Some(v) = d.flags { parts.push(format!("F:{v}")); }
    if let Some(v) = d.version_made_by { parts.push(format!("B:{v}")); }
    if let Some(v) = d.version_needed { parts.push(format!("V:{v}")); }
    if let Some(v) = d.internal_attrs { parts.push(format!("I:{v}")); }
    if let Some(v) = d.external_attrs { parts.push(format!("E:{v}")); }
    if let Some(v) = &d.local_extra { parts.push(format!("L:{}", enc_extra_list(v))); }
    if let Some(v) = &d.central_extra { parts.push(format!("C:{}", enc_extra_list(v))); }
    if let Some(v) = &d.comment { parts.push(format!("O:{}", hex_encode(v.as_bytes()))); }
    format!("[{}]", parts.join(","))
}
fn dec_entry_diff(s: &str) -> Result<ZipEntryDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ZipEntryDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() { continue; }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("entry diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.name = Some(hex_decode_string(val)?),
            "D" => d.data = Some(hex_decode(val)?),
            "M" => d.method = Some(dec_method(val)?),
            "A" => d.dos_date = Some(parse_u16(val)?),
            "T" => d.dos_time = Some(parse_u16(val)?),
            "U" => d.unix_mtime = Some(decode_option(val, parse_i64)?),
            "F" => d.flags = Some(parse_u16(val)?),
            "B" => d.version_made_by = Some(parse_u16(val)?),
            "V" => d.version_needed = Some(parse_u16(val)?),
            "I" => d.internal_attrs = Some(parse_u16(val)?),
            "E" => d.external_attrs = Some(val.parse().map_err(|e: std::num::ParseIntError| e.to_string())?),
            "L" => d.local_extra = Some(dec_extra_list(val)?),
            "C" => d.central_extra = Some(dec_extra_list(val)?),
            "O" => d.comment = Some(hex_decode_string(val)?),
            other => return Err(format!("entry diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ The one, NAME-keyed collection triple (`f6-recon-report.md` §5's `name{[removed];[modified];[added]}`
/// shape, adapted per this module's doc comment: `removed`/`modified` keys are hex-encoded entry
/// names, `added` keys are the final-position `usize` index).
fn enc_entries_diff(d: &ZipEntriesDiff) -> String {
    let removed = d.removed.iter().map(|n| hex_encode(n.as_bytes())).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", hex_encode(m.name.as_bytes()), enc_entry_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_entry(&a.entry))).collect::<Vec<_>>().join(",");
    format!("entries{{[{removed}];[{modified}];[{added}]}}")
}
fn dec_entries_diff(body: &str) -> Result<ZipEntriesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("entries: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(hex_decode_string).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (name_hex, diff_s) = entry.split_once(':').ok_or_else(|| format!("entries modified: bad entry {entry:?}"))?;
        Ok(ZipEntryModified { name: hex_decode_string(name_hex)?, diff: dec_entry_diff(diff_s)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx_s, entry_s) = entry.split_once(':').ok_or_else(|| format!("entries added: bad entry {entry:?}"))?;
        Ok(ZipEntryAdded { index: parse_usize(idx_s)?, entry: dec_entry(entry_s)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(ZipEntriesDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_zip_diff(d: &ZipDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.comment { tokens.push(format!("comment={}", hex_encode(v.as_bytes()))); }
    if let Some(v) = &d.entries { tokens.push(enc_entries_diff(v)); }
    tokens.join(" ")
}
fn parse_zip_diff(line: &str) -> Result<ZipDiff, String> {
    let mut d = ZipDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("comment=") { d.comment = Some(hex_decode_string(rest)?); }
        else if let Some(rest) = token.strip_prefix("entries{") { d.entries = Some(dec_entries_diff(rest.strip_suffix('}').ok_or_else(|| "entries: missing closing brace".to_string())?)?); }
        else { return Err(format!("zip diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for ZipDiff {
    fn print_diff(&self) -> String {
        print_zip_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_zip_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-P2: REAL binary frame (`format u8 | has_comment u8 | has_entries u8 | opaque
    /// payload`), matching `../💾️binary/📡️component.protocol.semio`'s header shape exactly —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (per the P2-W0
    /// census, 100% of stdio's `DiffCodec` impls were still on that shortcut). Delegates the
    /// variable-length body to `enc_entries_diff_bin`/`dec_entries_diff_bin` below (real
    /// LEB128-varint-framed binary, genuinely structured — `ZipDiff` is flat, no self-recursive
    /// value type, unlike json's `JsonValue`, so this is real record-shaped binary all the way
    /// down, not text-as-bytes at any layer).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, if self.comment.is_some() { 1 } else { 0 }, if self.entries.is_some() { 1 } else { 0 }];
        if let Some(comment) = &self.comment {
            write_str_lp(&mut out, comment);
        }
        if let Some(entries) = &self.entries {
            enc_entries_diff_bin(entries, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
        let has_comment = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_comment", offset: 1, detail: e.to_string() })?;
        let has_entries = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_entries", offset: 2, detail: e.to_string() })?;
        let comment = if has_comment != 0 {
            Some(read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff comment", offset: reader.position() as u64, detail: e })?)
        } else {
            None
        };
        let entries = if has_entries != 0 {
            Some(dec_entries_diff_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff entries", offset: reader.position() as u64, detail: e })?)
        } else {
            None
        };
        Ok(ZipDiff { comment, entries })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️BinaryDiffCodec
/// 🧪️ P2-P2: real LEB128-varint-framed binary twins of the hex-text codecs above, backing the
/// upgraded `DiffCodec::encode_diff`/`decode_diff` (`#region 🔖️TopLevel`) — reuses
/// `store::pack_rt::write_varint_u64` / `store::ByteReader` rather than reinventing varint
/// encode/decode (same convention P2-P1's json pilot established).
//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
/// ➡️ `store::pack_rt` re-exports `write_varint_u64` but not its zigzag-signed sibling (only
/// `ByteReader::read_varint_i64` is public) — this is the write-side counterpart, same zigzag
/// formula (`(v << 1) ^ (v >> 63)`) `crate::os_pack`'s own `write_varint_i64` uses.
fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️EntryBinaryCodec
fn method_tag(m: ZipCompressionMethod) -> u8 {
    match m {
        ZipCompressionMethod::Stored => 0,
        ZipCompressionMethod::Deflate => 1,
    }
}
fn method_from_tag(tag: u8) -> Result<ZipCompressionMethod, String> {
    match tag {
        0 => Ok(ZipCompressionMethod::Stored),
        1 => Ok(ZipCompressionMethod::Deflate),
        other => Err(format!("bad compression method tag {other}")),
    }
}
fn enc_extra_field_bin(f: &ZipExtraField, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, f.id as u64);
    write_bytes_lp(out, &f.payload);
}
fn dec_extra_field_bin(reader: &mut store::ByteReader<'_>) -> Result<ZipExtraField, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let payload = read_bytes_lp(reader)?;
    Ok(ZipExtraField { id, payload })
}
fn enc_extra_list_bin(v: &[ZipExtraField], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for f in v {
        enc_extra_field_bin(f, out);
    }
}
fn dec_extra_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<ZipExtraField>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_extra_field_bin(reader)).collect()
}

/// 🧭️ A whole `ZipEntry`, positional, field order matching the struct declaration exactly (same
/// order the sibling text codec's `enc_entry`/`dec_entry` use) — used by `added[]` entries, which
/// carry a full entry payload, not a sparse patch.
fn enc_entry_bin(e: &ZipEntry, out: &mut Vec<u8>) {
    write_str_lp(out, &e.name);
    write_bytes_lp(out, &e.data);
    out.push(method_tag(e.method));
    store::pack_rt::write_varint_u64(out, e.dos_date as u64);
    store::pack_rt::write_varint_u64(out, e.dos_time as u64);
    match e.unix_mtime {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            write_varint_i64(out, v);
        }
    }
    store::pack_rt::write_varint_u64(out, e.flags as u64);
    store::pack_rt::write_varint_u64(out, e.version_made_by as u64);
    store::pack_rt::write_varint_u64(out, e.version_needed as u64);
    store::pack_rt::write_varint_u64(out, e.internal_attrs as u64);
    store::pack_rt::write_varint_u64(out, e.external_attrs as u64);
    enc_extra_list_bin(&e.local_extra, out);
    enc_extra_list_bin(&e.central_extra, out);
    write_str_lp(out, &e.comment);
}
fn dec_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<ZipEntry, String> {
    let name = read_str_lp(reader)?;
    let data = read_bytes_lp(reader)?;
    let method = method_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let dos_date = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let dos_time = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let unix_mtime = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(reader.read_varint_i64().map_err(|e| e.to_string())?),
        other => return Err(format!("bad option tag {other}")),
    };
    let flags = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let version_made_by = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let version_needed = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let internal_attrs = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let external_attrs = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let local_extra = dec_extra_list_bin(reader)?;
    let central_extra = dec_extra_list_bin(reader)?;
    let comment = read_str_lp(reader)?;
    Ok(ZipEntry { name, data, method, dos_date, dos_time, unix_mtime, flags, version_made_by, version_needed, internal_attrs, external_attrs, local_extra, central_extra, comment })
}
//#endregion 🔖️EntryBinaryCodec

//#region 🔖️EntryDiffBinaryCodec
/// 🎚️ `ZipEntryDiff` has 14 sparse fields — a `u16` bitmask (one bit per field, declaration
/// order) says which are present, followed by only the present fields' payloads, in bitmask order.
const EDF_NAME: u16 = 1 << 0;
const EDF_DATA: u16 = 1 << 1;
const EDF_METHOD: u16 = 1 << 2;
const EDF_DOS_DATE: u16 = 1 << 3;
const EDF_DOS_TIME: u16 = 1 << 4;
const EDF_UNIX_MTIME: u16 = 1 << 5;
const EDF_FLAGS: u16 = 1 << 6;
const EDF_VERSION_MADE_BY: u16 = 1 << 7;
const EDF_VERSION_NEEDED: u16 = 1 << 8;
const EDF_INTERNAL_ATTRS: u16 = 1 << 9;
const EDF_EXTERNAL_ATTRS: u16 = 1 << 10;
const EDF_LOCAL_EXTRA: u16 = 1 << 11;
const EDF_CENTRAL_EXTRA: u16 = 1 << 12;
const EDF_COMMENT: u16 = 1 << 13;

fn enc_entry_diff_bin(d: &ZipEntryDiff, out: &mut Vec<u8>) {
    let mut mask = 0u16;
    if d.name.is_some() { mask |= EDF_NAME; }
    if d.data.is_some() { mask |= EDF_DATA; }
    if d.method.is_some() { mask |= EDF_METHOD; }
    if d.dos_date.is_some() { mask |= EDF_DOS_DATE; }
    if d.dos_time.is_some() { mask |= EDF_DOS_TIME; }
    if d.unix_mtime.is_some() { mask |= EDF_UNIX_MTIME; }
    if d.flags.is_some() { mask |= EDF_FLAGS; }
    if d.version_made_by.is_some() { mask |= EDF_VERSION_MADE_BY; }
    if d.version_needed.is_some() { mask |= EDF_VERSION_NEEDED; }
    if d.internal_attrs.is_some() { mask |= EDF_INTERNAL_ATTRS; }
    if d.external_attrs.is_some() { mask |= EDF_EXTERNAL_ATTRS; }
    if d.local_extra.is_some() { mask |= EDF_LOCAL_EXTRA; }
    if d.central_extra.is_some() { mask |= EDF_CENTRAL_EXTRA; }
    if d.comment.is_some() { mask |= EDF_COMMENT; }
    out.extend_from_slice(&mask.to_le_bytes());

    if let Some(v) = &d.name { write_str_lp(out, v); }
    if let Some(v) = &d.data { write_bytes_lp(out, v); }
    if let Some(v) = d.method { out.push(method_tag(v)); }
    if let Some(v) = d.dos_date { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.dos_time { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.unix_mtime {
        match v {
            None => out.push(0),
            Some(x) => {
                out.push(1);
                write_varint_i64(out, x);
            }
        }
    }
    if let Some(v) = d.flags { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.version_made_by { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.version_needed { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.internal_attrs { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = d.external_attrs { store::pack_rt::write_varint_u64(out, v as u64); }
    if let Some(v) = &d.local_extra { enc_extra_list_bin(v, out); }
    if let Some(v) = &d.central_extra { enc_extra_list_bin(v, out); }
    if let Some(v) = &d.comment { write_str_lp(out, v); }
}
fn dec_entry_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ZipEntryDiff, String> {
    let mask = reader.read_u16_le().map_err(|e| e.to_string())?;
    let mut d = ZipEntryDiff::default();
    if mask & EDF_NAME != 0 { d.name = Some(read_str_lp(reader)?); }
    if mask & EDF_DATA != 0 { d.data = Some(read_bytes_lp(reader)?); }
    if mask & EDF_METHOD != 0 { d.method = Some(method_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?); }
    if mask & EDF_DOS_DATE != 0 { d.dos_date = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_DOS_TIME != 0 { d.dos_time = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_UNIX_MTIME != 0 {
        let inner = match reader.read_u8().map_err(|e| e.to_string())? {
            0 => None,
            1 => Some(reader.read_varint_i64().map_err(|e| e.to_string())?),
            other => return Err(format!("bad option tag {other}")),
        };
        d.unix_mtime = Some(inner);
    }
    if mask & EDF_FLAGS != 0 { d.flags = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_VERSION_MADE_BY != 0 { d.version_made_by = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_VERSION_NEEDED != 0 { d.version_needed = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_INTERNAL_ATTRS != 0 { d.internal_attrs = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u16); }
    if mask & EDF_EXTERNAL_ATTRS != 0 { d.external_attrs = Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32); }
    if mask & EDF_LOCAL_EXTRA != 0 { d.local_extra = Some(dec_extra_list_bin(reader)?); }
    if mask & EDF_CENTRAL_EXTRA != 0 { d.central_extra = Some(dec_extra_list_bin(reader)?); }
    if mask & EDF_COMMENT != 0 { d.comment = Some(read_str_lp(reader)?); }
    Ok(d)
}
//#endregion 🔖️EntryDiffBinaryCodec

//#region 🔖️EntriesDiffBinaryCodec
/// 📦️ The one name-keyed collection triple, three runtime-counted lists — mirrors
/// `enc_entries_diff`'s three text-codec sections exactly, in the same order.
fn enc_entries_diff_bin(d: &ZipEntriesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for name in &d.removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        write_str_lp(out, &m.name);
        enc_entry_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_entry_bin(&a.entry, out);
    }
}
fn dec_entries_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ZipEntriesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let removed = (0..removed_count).map(|_| read_str_lp(reader)).collect::<Result<Vec<_>, String>>()?;
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let modified = (0..modified_count)
        .map(|_| -> Result<ZipEntryModified, String> {
            let name = read_str_lp(reader)?;
            let diff = dec_entry_diff_bin(reader)?;
            Ok(ZipEntryModified { name, diff })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let added = (0..added_count)
        .map(|_| -> Result<ZipEntryAdded, String> {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let entry = dec_entry_bin(reader)?;
            Ok(ZipEntryAdded { index, entry })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(ZipEntriesDiff { removed, modified, added })
}
//#endregion 🔖️EntriesDiffBinaryCodec
//#endregion 🔖️BinaryDiffCodec
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-P2: representative `ZipDiff` values (empty, a forward `between`, and its reverse) —
/// exercises the archive `comment` scalar, the tri-state `unix_mtime` (both `Some(None)`-clear and
/// `Some(Some(_))`-set), and all three sections of the `entries` collection triple simultaneously.
/// Single source of truth reused by `diff_codec_text_binary_roundtrip_law`
/// (`../🧬️mutations/🦀️component.rs`) AND by `../../⚙️engine/🦀️component.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same convention P2-P1's
/// json pilot established (`diff::demo_diff_cases()`).
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<ZipDiff> {
    fn entry(name: &str, data: &[u8]) -> ZipEntry {
        ZipEntry {
            name: name.into(),
            data: data.to_vec(),
            method: ZipCompressionMethod::Stored,
            dos_date: 0x1111,
            dos_time: 0x2222,
            unix_mtime: Some(1_600_000_000),
            flags: 0,
            version_made_by: 20,
            version_needed: 20,
            internal_attrs: 0,
            external_attrs: 0o100644 << 16,
            local_extra: vec![ZipExtraField { id: 1, payload: vec![1] }],
            central_extra: vec![ZipExtraField { id: 2, payload: vec![2] }],
            comment: "before comment".into(),
        }
    }

    let a = ZipSnapshot {
        schema: "stdio.zip".into(),
        entries: vec![entry("gone.txt", b"will be removed"), entry("stay.txt", b"before")],
        comment: "archive before".into(),
    };
    let b = ZipSnapshot {
        schema: "stdio.zip".into(),
        entries: vec![
            ZipEntry {
                data: b"after".to_vec(),
                method: ZipCompressionMethod::Deflate,
                dos_date: 0x3333,
                dos_time: 0x4444,
                unix_mtime: None, // tri-state: was Some, now cleared -> Some(None) in the diff.
                flags: 0x0800,
                version_made_by: 63,
                version_needed: 45,
                internal_attrs: 1,
                external_attrs: 0o100755 << 16,
                local_extra: vec![ZipExtraField { id: 9, payload: vec![9, 9] }],
                central_extra: vec![ZipExtraField { id: 10, payload: vec![10] }],
                comment: "after comment".into(),
                ..entry("stay.txt", b"after")
            },
            entry("new.bin", b"brand new"),
        ],
        comment: "archive after".into(),
    };

    vec![ZipDiff::default(), ZipDiff::between(&a, &b), ZipDiff::between(&b, &a)]
}
//#endregion 🔖️DemoCases
