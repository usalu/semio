//! 🔺️ TsvDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (TSV rows have no stable identity beyond position, same as
//! csv's own records); each modified row's own columns get a sparse positional patch via
//! [`TsvRowDiff`] (there is no insert-column/remove-column mutation — a patched position only
//! ever replaces an EXISTING cell — so a row's column count never resizes except via a whole-row
//! add/remove at the `records` collection level, matching csv's own `CsvRecordDiff` convention).

use crate::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️RowDiff
/// 🔺️ Sparse diff for a single TSV row (`Vec<String>`) — positional per-column patch list,
/// `None` at a position means that column is unchanged.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TsvRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Option<String>>>,
}

impl TsvRowDiff {
    /// 🕳️ Whether this patch changes nothing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        match &self.fields {
            None => true,
            Some(v) => v.iter().all(|f| f.is_none()),
        }
    }
    /// ▶️ Applies this patch to a row.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[String]) -> Vec<String> {
        match &self.fields {
            None => base.to_vec(),
            Some(patches) => {
                let mut row = base.to_vec();
                for (i, patch) in patches.iter().enumerate() {
                    if let Some(v) = patch {
                        if let Some(cell) = row.get_mut(i) {
                            *cell = v.clone();
                        }
                    }
                }
                row
            }
        }
    }
    /// ↩️ The positional patch restoring exactly the columns this patch sets back to their `base` values.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &[String]) -> Self {
        Self { fields: self.fields.as_ref().map(|patches| patches.iter().enumerate().map(|(index, patch)| patch.as_ref().and_then(|_| base.get(index).cloned())).collect()) }
    }
    /// 🧭️ State delta between two rows with the SAME column count (positional patch). Callers
    /// with differing column counts must instead express the change as a remove-then-add pair
    /// at the `records` collection level (see `TsvDiff::between`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[String], other: &[String]) -> Self {
        debug_assert_eq!(base.len(), other.len());
        let mut any = false;
        let patches: Vec<Option<String>> = base
            .iter()
            .zip(other.iter())
            .map(|(b, o)| {
                if b == o {
                    None
                } else {
                    any = true;
                    Some(o.clone())
                }
            })
            .collect();
        Self { fields: if any { Some(patches) } else { None } }
    }
    /// ➕️ Structural per-position absorb: `other`'s populated positions win.
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
                    if patch2.is_some() {
                        f1[i] = patch2;
                    }
                }
            }
        }
    }
}
//#endregion 🔖️RowDiff

//#region 🔖️RecordsDiff
/// 🧩 One row patched-in-place at a BASE index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TsvRowModified {
    pub index: usize,
    pub diff: TsvRowDiff,
}

/// 🧩 One row inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TsvRowAdded {
    pub index: usize,
    pub row: Vec<String>,
}

/// 🔺️ Index-keyed removed/modified/added triple over `TsvSnapshot::records`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TsvRowsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TsvRowModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TsvRowAdded>,
}

impl TsvRowsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️RecordsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — identical in shape to csv's own
// `simulate_slots`/`base_len_hint`/`absorb_records`.
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tsv`. No `snapshot: Option<TsvSnapshot>` full-replace slot — every diff is sparse.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv.diff")]
pub struct TsvDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<bool>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<TsvRowsDiff>,
}

impl MutationDiff<TsvSnapshot> for TsvDiff {
    fn apply(&self, base: &TsvSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<TsvSnapshot> {
        validate_tsv_diff(self, base)?;
        Ok(apply_tsv_diff_unchecked(self, base))
    }

    fn absorb(&mut self, other: Self) {
        if other.trailing_newline.is_some() {
            self.trailing_newline = other.trailing_newline;
        }
        if other.line_ending.is_some() {
            self.line_ending = other.line_ending;
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
fn validate_tsv_diff(diff: &TsvDiff, base: &TsvSnapshot) -> MutationApplyResult<()> {
    let Some(records) = &diff.records else { return Ok(()) };
    let mut removed = std::collections::HashSet::new();
    for &index in &records.removed {
        if index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "row removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &records.modified {
        if entry.index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "row modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "row modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row modification target is repeated"));
        }
        if let Some(fields) = &entry.diff.fields {
            if fields.len() > base.records[entry.index].len() {
                return Err(MutationApplyError::new("mutation.apply.invalid-index", "row field patch exceeds the base row"));
            }
        }
    }
    let final_len = base.records.len() - removed.len() + records.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &records.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "row addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_tsv_diff_unchecked(diff: &TsvDiff, base: &TsvSnapshot) -> TsvSnapshot {
    let mut next = base.clone();
    if let Some(v) = diff.trailing_newline {
        next.trailing_newline = v;
    }
    if let Some(v) = diff.line_ending {
        next.line_ending = v;
    }
    if let Some(rdiff) = &diff.records {
        for m in &rdiff.modified {
            if let Some(row) = next.records.get_mut(m.index) {
                *row = m.diff.apply(row);
            }
        }
        let mut removed_desc = rdiff.removed.clone();
        removed_desc.sort_unstable_by(|a, b| b.cmp(a));
        removed_desc.dedup();
        for idx in removed_desc {
            if idx < next.records.len() {
                next.records.remove(idx);
            }
        }
        let mut added_asc = rdiff.added.clone();
        added_asc.sort_by_key(|a| a.index);
        for a in added_asc {
            let at = a.index.min(next.records.len());
            next.records.insert(at, a.row);
        }
    }
    next
}

/// ➕️ Structural, total, base-free absorb of two `records` triples (same algorithm as csv's).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_records(d1: TsvRowsDiff, d2: TsvRowsDiff) -> TsvRowsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, TsvRowDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<TsvRowAdded>> = d1.added.into_iter().map(Some).collect();

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
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    added.row = m2.diff.apply(&added.row);
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
    let mut final_modified: Vec<TsvRowModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| TsvRowModified { index, diff }).collect();
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
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<TsvRowAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(TsvRowAdded { index: *after_pos, row: added.row });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    TsvRowsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

/// ↩️ Negative rows for the records triple against its BASE rows: added rows become removals at their final index, removed rows
/// return at their base index, and each modified row restores its base cells at the index the row has after the diff. Every list
/// comes back ascending, the normal form [`absorb_records`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rows(diff: &TsvRowsDiff, base: &[Vec<String>]) -> TsvRowsDiff {
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<TsvRowModified> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|cells| TsvRowModified { index: after_index(row.index), diff: row.diff.inverse(cells) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|row| TsvRowAdded { index: *index, row: row.clone() })).collect();
    TsvRowsDiff { removed: added_final, modified, added }
}

impl DiffAlgebra<TsvSnapshot> for TsvDiff {
    fn inverse(&self, base: &TsvSnapshot) -> Self {
        Self {
            trailing_newline: self.trailing_newline.map(|_| base.trailing_newline),
            line_ending: self.line_ending.map(|_| base.line_ending),
            records: self.records.as_ref().map(|records| inverse_rows(records, &base.records)).filter(|records| !records.is_empty()),
        }
    }

    fn between(base: &TsvSnapshot, other: &TsvSnapshot) -> Self {
        let trailing_newline = (base.trailing_newline != other.trailing_newline).then_some(other.trailing_newline);
        let line_ending = (base.line_ending != other.line_ending).then_some(other.line_ending);

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
            if b.len() == o.len() {
                let d = TsvRowDiff::between(b, o);
                if !d.is_empty() {
                    modified.push(TsvRowModified { index: i, diff: d });
                }
            } else {
                removed.push(i);
                added.push(TsvRowAdded { index: i, row: o.clone() });
            }
        }
        for i in min_len..base.records.len() {
            removed.push(i);
        }
        for i in min_len..other.records.len() {
            added.push(TsvRowAdded { index: i, row: other.records[i].clone() });
        }

        let records = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(TsvRowsDiff { removed, modified, added }) };
        Self { trailing_newline, line_ending, records }
    }

    fn is_empty(&self) -> bool {
        self.trailing_newline.is_none() && self.line_ending.is_none() && self.records.as_ref().is_none_or(TsvRowsDiff::is_empty)
    }
}

//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` — `TsvRowDiff::fields: Option<Vec<Option<String>>>`
/// hits the same dsl-derive `Vec<Option<T>>` rejection csv's `CsvRecordDiff` documents (one
/// `Vec`-wrapped tri-state layer). **Grammar**: one space-separated `name=value` token per
/// changed top-level field; `records` prints as `records{[removed];[modified];[added]}`. Strings
/// are lowercase hex (TSV cells legally contain almost anything except tab/CR/LF, which this
/// grammar's own separators are built from — hex sidesteps escaping entirely).
//#region 🔖️Primitives








//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs






//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs





//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
