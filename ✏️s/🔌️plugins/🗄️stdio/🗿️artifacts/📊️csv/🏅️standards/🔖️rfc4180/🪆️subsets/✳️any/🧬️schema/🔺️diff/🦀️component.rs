//! 🔺️ CsvDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (RFC 4180 rows have no stable identity beyond position);
//! each modified record carries its own positional per-field patch list (there is no
//! insert-field/remove-field mutation — `SetField` only ever patches an EXISTING position —
//! so a record's field vector never structurally resizes except via a whole-record
//! add/remove at the `records` collection level).

use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
// 🔗 `DiffAlgebra` (spine S-1) isn't in the `protocol` facade's curated re-export list yet
// (`.🦑️repo/🎫️tickets/…/ARTIFACT-SYSTEM-OVERHAUL…/f1-csv-report.md` `## Deviations`); reach it
// via the same crate's directly-mounted `command` module instead of editing the shared facade.
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️FieldDiff
/// 🔺️ Sparse diff for a single [`CsvField`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvFieldDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted: Option<bool>,
}

impl CsvFieldDiff {
    /// 🕳️ Whether this patch changes nothing.
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.quoted.is_none()
    }
    /// ▶️ Applies this patch to a field.
    pub fn apply(&self, base: &CsvField) -> CsvField {
        CsvField { value: self.value.clone().unwrap_or_else(|| base.value.clone()), quoted: self.quoted.unwrap_or(base.quoted) }
    }
    /// 🧭️ State delta between two fields.
    pub fn between(base: &CsvField, other: &CsvField) -> Self {
        Self { value: (base.value != other.value).then(|| other.value.clone()), quoted: (base.quoted != other.quoted).then_some(other.quoted) }
    }
    /// ➕️ LWW field-level absorb: `other`'s populated sub-fields win.
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
/// 🧪️ F6: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` CANNOT be used anywhere in
/// `CsvDiff`'s tree because of THIS struct's `fields: Option<Vec<Option<CsvFieldDiff>>>` —
/// confirmed via real `cargo check` error: `the trait bound
/// std::option::Option<v_rfc4180::…::CsvFieldDiff>: DslField is not satisfied`
/// (`dsl_derive::classify_field` peels exactly one `Option<..>` layer before checking anything
/// else, so the field's remaining type after the derive's own unwrap is `Vec<Option<CsvFieldDiff>>`
/// — its blanket `impl<T: DslField> DslField for Vec<T>` then requires `Option<CsvFieldDiff>:
/// DslField`, and no `impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl`
/// crate). Same root cause as the recon report's §3b tri-state finding (`Option<Option<T>>`), one
/// `Vec` layer removed. `DiffCodec` for `CsvDiff` is hand-rolled below instead; see
/// `f6-recon-report.md` §3b and this ticket's `f6-csv-report.md` for the full citation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecordDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Option<CsvFieldDiff>>>,
}

impl CsvRecordDiff {
    /// 🕳️ Whether this patch changes nothing.
    pub fn is_empty(&self) -> bool {
        match &self.fields {
            None => true,
            Some(v) => v.iter().all(|f| f.is_none()),
        }
    }
    /// ▶️ Applies this patch to a record.
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecordModified {
    pub index: usize,
    pub diff: CsvRecordDiff,
}

/// 🧩 One record inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecordAdded {
    pub index: usize,
    pub record: CsvRecord,
}

/// 🔺️ Index-keyed removed/modified/added triple over `CsvSnapshot::records`
/// (`.claude/plans/the-current-schemas-are-scalable-journal.md` `## Diff`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecordsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<CsvRecordModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<CsvRecordAdded>,
}

impl CsvRecordsDiff {
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
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.csv`. No `snapshot: Option<CsvSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `CsvDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv.diff")]
pub struct CsvDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_header: Option<bool>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
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
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
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
        self.has_header.is_none() && self.records.as_ref().map_or(true, CsvRecordsDiff::is_empty)
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
pub fn diff_set_snapshot(base: &CsvSnapshot, next: &CsvSnapshot) -> CsvDiff {
    CsvDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `CsvDiff` — the derive path
/// (`#[derive(dsl::DslDiff)]`) is NOT usable: `CsvRecordDiff::fields: Option<Vec<Option<CsvFieldDiff>>>`
/// (see the doc comment on `CsvRecordDiff` above for the confirmed compile error and the exact
/// root-cause mechanism — a `Vec`-wrapped sibling of the recon report's §3b tri-state finding).
///
/// **Grammar** (real, not `serde_json`), following `f6-recon-report.md` §5's template exactly:
/// one space-separated `name=value` token per changed top-level field (a field absent from the
/// line = unchanged); `records` prints as `records{[removed];[modified];[added]}`. Strings are
/// lowercase hex (this artifact's own `ArtifactDsl`/`⚙️engine` codec doesn't use hex — RFC 4180 is
/// already its own text grammar — but hex is still the right choice HERE since a `CsvField.value`
/// may itself legally contain any byte incl. `,`/`[`/`]`/space, which this diff grammar's own
/// separators are built from; hex sidesteps escaping entirely). `Option<T>` values use the
/// uniform `[0]`=None / `[1,<T>]` = Some(T) tag. Structs are positional `[f1,f2,...]` tuples.
/// `CsvRecordDiff`'s own sparse per-position field-patch list prints as a bracketed list of
/// `encode_option`-tagged `CsvFieldDiff` entries (`[[0],[1,[V:...,Q:1]]]`); `CsvFieldDiff` itself
/// uses single-letter `tag:value` pairs (`V`/`Q`), same convention as gif89a's `GifFrameDiff`.
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
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
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn enc_field(f: &CsvField) -> String {
    format!("[{},{}]", enc_str(&f.value), if f.quoted { 1 } else { 0 })
}
pub(crate) fn dec_field(s: &str) -> Result<CsvField, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [value, quoted] = parts.as_slice() else { return Err(format!("field: expected 2 fields, got {}", parts.len())) };
    Ok(CsvField { value: dec_str(value)?, quoted: *quoted == "1" })
}
pub(crate) fn enc_record(r: &CsvRecord) -> String {
    format!("[{}]", r.fields.iter().map(enc_field).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_record(s: &str) -> Result<CsvRecord, String> {
    let fields = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_field).collect::<Result<Vec<_>, String>>()?;
    Ok(CsvRecord { fields })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_field_diff(d: &CsvFieldDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.value {
        parts.push(format!("V:{}", enc_str(v)));
    }
    if let Some(v) = d.quoted {
        parts.push(format!("Q:{}", if v { 1 } else { 0 }));
    }
    format!("[{}]", parts.join(","))
}
fn dec_field_diff(s: &str) -> Result<CsvFieldDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = CsvFieldDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("field diff: bad entry {entry:?}"))?;
        match tag {
            "V" => d.value = Some(dec_str(val)?),
            "Q" => d.quoted = Some(val == "1"),
            other => return Err(format!("field diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
fn enc_record_diff(d: &CsvRecordDiff) -> String {
    encode_option(&d.fields, |fields| format!("[{}]", fields.iter().map(|f| encode_option(f, enc_field_diff)).collect::<Vec<_>>().join(",")))
}
fn dec_record_diff(s: &str) -> Result<CsvRecordDiff, String> {
    let fields = decode_option(s, |inner| split_top_level(strip_brackets(inner)?, ',').into_iter().filter(|s| !s.is_empty()).map(|p| decode_option(p, dec_field_diff)).collect::<Result<Vec<_>, String>>())?;
    Ok(CsvRecordDiff { fields })
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (mirrors gif89a's `enc_collection_triple`/`dec_collection_triple`, hand-instantiated here for
/// `records` since only one collection needs it in this artifact).
fn enc_records_diff(d: &CsvRecordsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_record_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_record(&a.record))).collect::<Vec<_>>().join(",");
    format!("records{{[{removed}];[{modified}];[{added}]}}")
}
fn dec_records_diff(body: &str) -> Result<CsvRecordsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("records: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records modified: bad entry {entry:?}"))?;
            Ok(CsvRecordModified { index: parse_usize(idx)?, diff: dec_record_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records added: bad entry {entry:?}"))?;
            Ok(CsvRecordAdded { index: parse_usize(idx)?, record: dec_record(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(CsvRecordsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_csv_diff(d: &CsvDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.has_header {
        tokens.push(format!("has-header={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.records {
        tokens.push(enc_records_diff(v));
    }
    tokens.join(" ")
}
fn parse_csv_diff(line: &str) -> Result<CsvDiff, String> {
    let mut d = CsvDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("has-header=") {
            d.has_header = Some(rest == "1");
        } else if let Some(rest) = token.strip_prefix("records{") {
            d.records = Some(dec_records_diff(rest.strip_suffix('}').ok_or_else(|| "records: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("csv diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

//#region 🔖️RealBinaryDiffFrame
/// 🧪️ P2-P1: **real binary diff-frame** for `CsvDiff` — upgraded from the F6-era
/// `print_diff().into_bytes()` text-as-binary shortcut. `CsvDiff` is a STRUCT
/// (`has_header: Option<bool>`, `records: Option<CsvRecordsDiff>`), so the frame is one
/// presence-flag byte PER field (not an ordinal dispatch — that's the mutations enum's own
/// shape) directly modeling `../💾️binary/📡️component.protocol.semio`'s real
/// `field X u8` / `field Y u8 if X eq 1` conditional-presence layout (P2-M2 item 4). The
/// `records` triple's own recursive removed/modified/added contents are hand-rolled via
/// `dsl::ByteWriter`/`dsl::ByteReader` and placed LAST so they can honestly consume "rest of
/// buffer" with no length prefix — see the protocol file's own doc comment for why (no
/// `Ref`-to-struct / no heterogeneous `Array` in this dialect yet).
fn write_bin_field_diff(w: &mut dsl::ByteWriter, d: &CsvFieldDiff) {
    match &d.value {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            let bytes = v.as_bytes();
            w.write_varint_u64(bytes.len() as u64);
            w.write_bytes(bytes);
        }
    }
    match d.quoted {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            w.write_u8(if v { 1 } else { 0 });
        }
    }
}
fn read_bin_field_diff(r: &mut dsl::ByteReader) -> Result<CsvFieldDiff, dsl::PackError> {
    let mut d = CsvFieldDiff::default();
    if r.read_u8()? == 1 {
        let len = r.read_varint_u64()? as usize;
        let bytes = r.read_bytes(len)?;
        d.value = Some(String::from_utf8(bytes.to_vec()).map_err(|e| dsl::PackError::Malformed { what: "csv diff field value utf8", offset: 0, detail: e.to_string() })?);
    }
    if r.read_u8()? == 1 {
        d.quoted = Some(r.read_u8()? != 0);
    }
    Ok(d)
}
fn write_bin_record_diff(w: &mut dsl::ByteWriter, d: &CsvRecordDiff) {
    match &d.fields {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            w.write_varint_u64(v.len() as u64);
            for item in v {
                match item {
                    None => w.write_u8(0),
                    Some(fd) => {
                        w.write_u8(1);
                        write_bin_field_diff(w, fd);
                    }
                }
            }
        }
    }
}
fn read_bin_record_diff(r: &mut dsl::ByteReader) -> Result<CsvRecordDiff, dsl::PackError> {
    let fields = if r.read_u8()? == 1 {
        let n = r.read_varint_u64()? as usize;
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(if r.read_u8()? == 1 { Some(read_bin_field_diff(r)?) } else { None });
        }
        Some(items)
    } else {
        None
    };
    Ok(CsvRecordDiff { fields })
}
fn write_bin_records_diff(w: &mut dsl::ByteWriter, d: &CsvRecordsDiff) {
    w.write_varint_u64(d.removed.len() as u64);
    for idx in &d.removed {
        w.write_varint_u64(*idx as u64);
    }
    w.write_varint_u64(d.modified.len() as u64);
    for m in &d.modified {
        w.write_varint_u64(m.index as u64);
        write_bin_record_diff(w, &m.diff);
    }
    w.write_varint_u64(d.added.len() as u64);
    for a in &d.added {
        w.write_varint_u64(a.index as u64);
        crate::artifacts::csv::schema::mutations::write_bin_record(w, &a.record);
    }
}
fn read_bin_records_diff(r: &mut dsl::ByteReader) -> Result<CsvRecordsDiff, dsl::PackError> {
    let removed_n = r.read_varint_u64()? as usize;
    let mut removed = Vec::with_capacity(removed_n);
    for _ in 0..removed_n {
        removed.push(r.read_varint_u64()? as usize);
    }
    let modified_n = r.read_varint_u64()? as usize;
    let mut modified = Vec::with_capacity(modified_n);
    for _ in 0..modified_n {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_record_diff(r)?;
        modified.push(CsvRecordModified { index, diff });
    }
    let added_n = r.read_varint_u64()? as usize;
    let mut added = Vec::with_capacity(added_n);
    for _ in 0..added_n {
        let index = r.read_varint_u64()? as usize;
        let record = crate::artifacts::csv::schema::mutations::read_bin_record(r)?;
        added.push(CsvRecordAdded { index, record });
    }
    Ok(CsvRecordsDiff { removed, modified, added })
}
fn diff_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "csv diff binary", offset: 0, detail: e.to_string() }
}

impl DiffCodec for CsvDiff {
    fn print_diff(&self) -> String {
        print_csv_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_csv_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        match self.has_header {
            Some(v) => {
                w.write_u8(1);
                w.write_u8(if v { 1 } else { 0 });
            }
            None => w.write_u8(0),
        }
        match &self.records {
            Some(r) => {
                w.write_u8(1);
                write_bin_records_diff(&mut w, r);
            }
            None => w.write_u8(0),
        }
        Ok(w.into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let hh_flag = r.read_u8().map_err(diff_pack_err)?;
        let has_header = if hh_flag == 1 { Some(r.read_u8().map_err(diff_pack_err)? != 0) } else { None };
        let rec_flag = r.read_u8().map_err(diff_pack_err)?;
        let records = if rec_flag == 1 { Some(read_bin_records_diff(&mut r).map_err(diff_pack_err)?) } else { None };
        Ok(CsvDiff { has_header, records })
    }
}
//#endregion 🔖️RealBinaryDiffFrame
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::CsvField;

    fn field(value: &str, quoted: bool) -> CsvField {
        CsvField { value: value.into(), quoted }
    }
    fn record(fields: &[(&str, bool)]) -> CsvRecord {
        CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() }
    }

    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note, with comma", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] };
        let b = CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("x", false), ("y", false)]), record(&[("brand [new]", true)])] };
        let cases = vec![CsvDiff::default(), CsvDiff::between(&a, &b), CsvDiff::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = CsvDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = CsvDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    //#region 🔖️DiffGrammarConformanceLaw
    /// 🧪️ P2-P1 item 6: `dsl::parse_grammar` + `dsl::Recognizer` recognize REAL `print_diff`
    /// output for several real diffs, including the `records` COLLECTION-TRIPLE production
    /// (removed/modified/added) — the first real collection-triple grammar in this program.
    #[test]
    fn diff_grammar_conformance_law() {
        let grammar_text = crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);

        let a = CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note, with comma", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] };
        let b = CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("x", false), ("y", false)]), record(&[("brand [new]", true)])] };
        let diffs = vec![CsvDiff::default(), CsvDiff::between(&a, &b), CsvDiff::between(&b, &a)];
        for d in diffs {
            let printed = d.print_diff();
            let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
            assert!(ok, "diff grammar must recognize real print_diff output {printed:?}");
        }
    }
    //#endregion 🔖️DiffGrammarConformanceLaw
}
//#endregion 🧪️Tests
