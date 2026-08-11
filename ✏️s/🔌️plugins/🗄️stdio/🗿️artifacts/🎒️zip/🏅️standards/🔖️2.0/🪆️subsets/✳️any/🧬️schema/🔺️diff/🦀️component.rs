//! 🔺️ ZipDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `ZipDiff{snapshot: Option<ZipSnapshot>}` full-replace template with a real per-field patch —
//! archive `comment` plus a name-keyed `entries` triple (`removed`/`modified`/`added`), every
//! `ZipEntry` field individually patchable (including a `name` field on `ZipEntryDiff` for
//! renames and a tri-state `unixMtime` for clearing the Info-ZIP timestamp).

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
