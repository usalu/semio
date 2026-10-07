//! 🔺️ CsvDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (RFC 4180 rows have no stable identity beyond position);
//! each modified record carries its own positional per-field patch list (there is no
//! insert-field/remove-field mutation — `SetField` only ever patches an EXISTING position —
//! so a record's field vector never structurally resizes except via a whole-record
//! add/remove at the `records` collection level).

use crate::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
// 🔗 `DiffAlgebra` (spine S-1) isn't in the `protocol` facade's curated re-export list yet
// (`.🧬semio/🦑️repo/🎫️tickets/…/ARTIFACT-SYSTEM-OVERHAUL…/f1-csv-report.md` `## Deviations`); reach it
// via the same crate's directly-mounted `command` module instead of editing the shared facade.
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️FieldDiff
/// 🔺️ Sparse diff for a single [`CsvField`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvFieldDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quoted: Option<bool>,
}

impl CsvFieldDiff {
    /// 🕳️ Whether this patch changes nothing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.quoted.is_none()
    }
    /// ▶️ Applies this patch to a field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &CsvField) -> CsvField {
        CsvField { value: self.value.clone().unwrap_or_else(|| base.value.clone()), quoted: self.quoted.unwrap_or(base.quoted) }
    }
    /// 🧭️ State delta between two fields.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &CsvField, other: &CsvField) -> Self {
        Self { value: (base.value != other.value).then(|| other.value.clone()), quoted: (base.quoted != other.quoted).then_some(other.quoted) }
    }
    /// ➕️ LWW field-level absorb: `other`'s populated sub-fields win.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.value.is_some() {
            self.value = other.value;
        }
        if other.quoted.is_some() {
            self.quoted = other.quoted;
        }
    }
}
//#endregion 🔖️FieldDiff

//#region 🔖️RecordDiff
/// 🔺️ Sparse diff for a single [`CsvRecord`] — positional per-field patch list, `None` at a
/// position means that field is unchanged. Length only needs to cover the highest patched
/// index; positions beyond `base.fields.len()` are graceful no-ops on apply.
///
/// 🧪️ F6: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::)]` CANNOT be used anywhere in
/// `CsvDiff`'s tree because of THIS struct's `fields: Option<Vec<Option<CsvFieldDiff>>>` —
/// confirmed via real `cargo check` error: `the trait bound
/// std::option::Option<v_rfc4180::…::CsvFieldDiff>: DslField is not satisfied`
/// (`dsl_derive::classify_field` peels exactly one `Option<..>` layer before checking anything
/// else, so the field's remaining type after the derive's own unwrap is `Vec<Option<CsvFieldDiff>>`
/// — its blanket `impl<T: DslField> DslField for Vec<T>` then requires `Option<CsvFieldDiff>:
/// DslField`, and no `impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl`
/// crate). Same root cause as the recon report's §3b tri-state finding (`Option<Option<T>>`), one
/// `Vec` layer removed. `DiffBinary,DiffCodec,DiffText` for `CsvDiff` is hand-rolled below instead; see
/// `f6-recon-report.md` §3b and this ticket's `f6-csv-report.md` for the full citation.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvRecordDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Option<CsvFieldDiff>>>,
}

impl CsvRecordDiff {
    /// 🕳️ Whether this patch changes nothing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        match &self.fields {
            None => true,
            Some(v) => v.iter().all(|f| f.is_none()),
        }
    }
    /// ▶️ Applies this patch to a record.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &CsvRecord) -> CsvRecord {
        match &self.fields {
            None => base.clone(),
            Some(patches) => {
                let mut fields = base.fields.clone();
                for (i, patch) in patches.iter().enumerate() {
                    if let Some(p) = patch {
                        if let Some(f) = fields.get_mut(i) {
                            *f = p.apply(f);
                        }
                    }
                }
                CsvRecord { fields }
            }
        }
    }
    /// 🧭️ State delta between two records with the SAME field count (positional patch).
    /// Callers with differing field counts must instead express the change as a
    /// remove-then-add pair at the `records` collection level (see `CsvDiff::between`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &CsvRecord, other: &CsvRecord) -> Self {
        debug_assert_eq!(base.fields.len(), other.fields.len());
        let mut any = false;
        let patches: Vec<Option<CsvFieldDiff>> = base
            .fields
            .iter()
            .zip(other.fields.iter())
            .map(|(b, o)| {
                let d = CsvFieldDiff::between(b, o);
                if d.is_empty() {
                    None
                } else {
                    any = true;
                    Some(d)
                }
            })
            .collect();
        Self { fields: if any { Some(patches) } else { None } }
    }
    /// ➕️ Structural per-position absorb: `other`'s populated positions win; the patch
    /// vector grows to cover whichever side patches further out.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        match (&mut self.fields, other.fields) {
            (_, None) => {}
            (slot @ None, Some(f2)) => *slot = Some(f2),
            (Some(f1), Some(f2)) => {
                if f2.len() > f1.len() {
                    f1.resize(f2.len(), None);
                }
                for (i, patch2) in f2.into_iter().enumerate() {
                    if let Some(p2) = patch2 {
                        match &mut f1[i] {
                            Some(p1) => p1.absorb(p2),
                            slot @ None => *slot = Some(p2),
                        }
                    }
                }
            }
        }
    }
}
//#endregion 🔖️RecordDiff

//#region 🔖️RecordsDiff
/// 🧩 One record patched-in-place at a BASE index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvRecordModified {
    pub index: usize,
    pub diff: CsvRecordDiff,
}

/// 🧩 One record inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvRecordAdded {
    pub index: usize,
    pub record: CsvRecord,
}

/// 🔺️ Index-keyed removed/modified/added triple over `CsvSnapshot::records`
/// (`.claude/plans/the-current-schemas-are-scalable-journal.md` `## Diff`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvRecordsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<CsvRecordModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<CsvRecordAdded>,
}

impl CsvRecordsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️RecordsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — simulates the SAME removed(descending)/added
// (ascending, clamped) sequence `CsvDiff::apply` performs, over a virtual index universe
// bounded tightly by what a diff's own removed/modified keys actually reference (matches
// the recipe's "structural, total, base-free" absorb contract).

/// 🎰 One slot of a simulated post-removal/insertion array.
#[derive(Clone, Copy, Debug)]
enum Slot {
    /// A surviving item that was at this BASE index.
    Base(usize),
    /// An item inserted by this diff, identified by its position in the diff's own `added` vec.
    Added(usize),
}

/// 🧪 Simulates `removed`(descending)/`added`(ascending, clamped) against a virtual array of
/// `[0, len)` `Slot::Base(i)` markers, mirroring `CsvDiff::apply`'s own ordering exactly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 📏 Tight virtual-array bound: one past the highest base index this diff's own
/// removed/modified/added keys reference (0 if it references none). `added` indices are
/// mid-coordinate, not base-coordinate, but a target index of `k` is still real evidence
/// that at least `k` survivor positions exist before it — without this, a diff with ONLY
/// an `added` entry (no removed/modified at all, e.g. a lone `InsertRecord`) would simulate
/// against a zero-length virtual base and lose every earlier survivor entirely.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.csv`. No `snapshot: Option<CsvSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `CsvDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv.diff")]
pub struct CsvDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub has_header: Option<bool>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<CsvRecordsDiff>,
}

impl MutationDiff<CsvSnapshot> for CsvDiff {
    fn apply(&self, base: &CsvSnapshot) -> MutationApplyResult<CsvSnapshot> {
        validate_csv_diff(self, base)?;
        Ok(apply_csv_diff_unchecked(self, base))
    }

    fn absorb(&mut self, other: Self) {
        if other.has_header.is_some() {
            self.has_header = other.has_header;
        }
        let d2 = match other.records {
            None => return,
            Some(d2) => d2,
        };
        let d1 = match self.records.take() {
            None => {
                self.records = Some(d2);
                return;
            }
            Some(d1) => d1,
        };
        self.records = Some(absorb_records(d1, d2));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_csv_diff(diff: &CsvDiff, base: &CsvSnapshot) -> MutationApplyResult<()> {
    let Some(records) = &diff.records else { return Ok(()) };
    let mut removed = std::collections::HashSet::new();
    for &index in &records.removed {
        if index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "record removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &records.modified {
        if entry.index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "record modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "record modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record modification target is repeated"));
        }
        if let Some(fields) = &entry.diff.fields {
            if fields.len() > base.records[entry.index].fields.len() {
                return Err(MutationApplyError::new("mutation.apply.invalid-index", "record field patch exceeds the base record"));
            }
        }
    }
    let final_len = base.records.len() - removed.len() + records.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &records.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "record addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "record addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_csv_diff_unchecked(diff: &CsvDiff, base: &CsvSnapshot) -> CsvSnapshot {
    let mut next = base.clone();
    if let Some(has_header) = diff.has_header {
        next.has_header = has_header;
    }
    if let Some(rdiff) = &diff.records {
        // 🥇 modified refers to BASE indices — apply before any removal shifts them.
        for m in &rdiff.modified {
            if let Some(rec) = next.records.get_mut(m.index) {
                *rec = m.diff.apply(rec);
            }
        }
        // 🥈 removed refers to BASE indices — process descending so earlier removals
        // never shift the position of a later (larger) one still to be removed.
        let mut removed_desc = rdiff.removed.clone();
        removed_desc.sort_unstable_by(|a, b| b.cmp(a));
        removed_desc.dedup();
        for idx in removed_desc {
            if idx < next.records.len() {
                next.records.remove(idx);
            }
        }
        // 🥉 added refers to FINAL indices — process ascending, clamped.
        let mut added_asc = rdiff.added.clone();
        added_asc.sort_by_key(|a| a.index);
        for a in added_asc {
            let at = a.index.min(next.records.len());
            next.records.insert(at, a.record);
        }
    }
    next
}

/// ➕️ Structural, total, base-free absorb of two `records` triples
/// (`.claude/plans/the-current-schemas-are-scalable-journal.md` `## Absorb`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_records(d1: CsvRecordsDiff, d2: CsvRecordsDiff) -> CsvRecordsDiff {
    //#region 🔖️PhiBaseToMid
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    // 📏 The tight bound from d1's OWN references isn't always enough: d2's removed/modified
    // may query a mid position d1 never itself touched (e.g. d1 = a single `InsertRecord`
    // with no removed/modified at all, d2 = `RemoveRecord` at a position past it) — widen
    // `base_len` so the simulated mid array is long enough to answer those queries too.
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);
    //#endregion 🔖️PhiBaseToMid

    //#region 🔖️Seed
    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, CsvRecordDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    // Parallel to `d1.added`, indexed by the SAME `i` used in `Slot::Added(i)`; `None` once
    // annihilated by a `d2` removal of that mid slot.
    let mut added_alive: Vec<Option<CsvRecordAdded>> = d1.added.into_iter().map(Some).collect();
    //#endregion 🔖️Seed

    //#region 🔖️ApplyD2
    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {} // 🕳️ out-of-range: graceful no-op
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    added.record = m2.diff.apply(&added.record);
                }
            }
            None => {} // 🕳️ out-of-range: graceful no-op
        }
    }
    //#endregion 🔖️ApplyD2

    //#region 🔖️FinalizeRemovedModified
    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<CsvRecordModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| CsvRecordModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);
    //#endregion 🔖️FinalizeRemovedModified

    //#region 🔖️PsiMidToAfter
    // 🧭️ Surviving d1-added items must be remapped mid→after through d2's own removed/added
    // (ψ); d2's own added entries are already after-coordinates and pass through verbatim.
    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }
    //#endregion 🔖️PsiMidToAfter

    //#region 🔖️FinalizeAdded
    let mut final_added: Vec<CsvRecordAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(CsvRecordAdded { index: *after_pos, record: added.record });
            }
            // 🕳️ else: this mid slot was itself removed by d2 — but that always routes
            // through `added_alive[ai] = None` above, so this branch is unreachable.
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);
    //#endregion 🔖️FinalizeAdded

    CsvRecordsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

impl DiffAlgebra<CsvSnapshot> for CsvDiff {
    fn inverse(&self, base: &CsvSnapshot) -> Self {
        let applied = apply_csv_diff_unchecked(self, base);
        Self::between(&applied, base)
    }

    fn between(base: &CsvSnapshot, other: &CsvSnapshot) -> Self {
        let has_header = (base.has_header != other.has_header).then_some(other.has_header);

        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let min_len = base.records.len().min(other.records.len());
        for i in 0..min_len {
            let b = &base.records[i];
            let o = &other.records[i];
            if b == o {
                continue;
            }
            if b.fields.len() == o.fields.len() {
                let d = CsvRecordDiff::between(b, o);
                if !d.is_empty() {
                    modified.push(CsvRecordModified { index: i, diff: d });
                }
            } else {
                // 🔀 Field count changed: not expressible as a positional patch — replace
                // the whole record via a same-index remove+add pair instead.
                removed.push(i);
                added.push(CsvRecordAdded { index: i, record: o.clone() });
            }
        }
        for i in min_len..base.records.len() {
            removed.push(i);
        }
        for i in min_len..other.records.len() {
            added.push(CsvRecordAdded { index: i, record: other.records[i].clone() });
        }

        let records = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(CsvRecordsDiff { removed, modified, added }) };
        Self { has_header, records }
    }

    fn is_empty(&self) -> bool {
        self.has_header.is_none() && self.records.as_ref().is_none_or(CsvRecordsDiff::is_empty)
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &CsvSnapshot, next: &CsvSnapshot) -> CsvDiff {
    CsvDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
