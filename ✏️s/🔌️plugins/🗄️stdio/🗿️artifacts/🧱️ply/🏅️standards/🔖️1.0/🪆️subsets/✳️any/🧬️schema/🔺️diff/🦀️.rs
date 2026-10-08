//! 🔺️ PlyDiff — handcrafted sparse diff. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL: replaces the
//! old `PlyDiff{snapshot: Option<PlySnapshot>}` full-replace template with a real per-field
//! patch — `format` + `comments` (weak, whole-vec replace) + a name-keyed `elements` triple,
//! each modified element carrying its own `properties` (weak, whole-vec replace) and an
//! index-keyed `rows` triple, each modified row carrying a name-keyed sparse per-property patch.
//! Two collection levels nest (elements → rows), matching the recipe's "trees nest" rule.
//!
//! 🧪️ F6 CONFIRMED (ticket `f6-recon-report.md` §9 STEP 1, real `cargo check`, not guessed):
//! `#[derive(dsl::DslDiff)]` on `PlyDiff` fails —
//! `error[E0277]: the trait bound `PlyProperty: DslField` is not satisfied` at
//! `pub properties: Option<Vec<PlyProperty>>` (this file), because `PlyProperty` (`Scalar{..}` /
//! `List{..}`, both data-carrying variants) has no `DslField` impl and none is derivable (it is
//! not unit-variant-only, so `#[derive(dsl::DslScalar)]` does not apply either) — the classic 3a
//! "enum-in-tree" blocker (`PlyValue` is the same shape and would block equally via
//! `PlyRowFieldChange::value`). `DiffBinary,DiffCodec,DiffText` for `PlyDiff` is hand-rolled below instead, following
//! the ticket's §5 grammar template (verbatim primitives from the gif89a/svg pilots).





/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};

//#region 🔖️RowFieldDiff
/// 🔣️ One changed cell inside a row's sparse patch, keyed by the owning element's property
/// NAME (stable per-element schema — see module doc; positions can shift if `properties`
/// itself is replaced, names don't).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyRowFieldChange {
    pub name: String,
    pub value: PlyValue,
}

/// 🔺️ Sparse per-property patch for one [`PlyRow`] — only changed cells appear.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyRowDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<PlyRowFieldChange>,
}

impl PlyRowDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
    /// ➕️ LWW per-field-name upsert absorb.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        for change in other.fields {
            if let Some(existing) = self.fields.iter_mut().find(|f| f.name == change.name) {
                existing.value = change.value;
            } else {
                self.fields.push(change);
            }
        }
    }
}

/// ▶️ Applies a row patch in place, resolving each change's property name against `properties`
/// (the OWNING element's declared column order) to find the cell index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_diff(properties: &[PlyProperty], row: &mut PlyRow, diff: &PlyRowDiff) {
    for change in &diff.fields {
        for (index, property) in properties.iter().enumerate() {
            if property.name() == change.name {
                row.values[index] = change.value.clone();
            }
        }
    }
}

/// 🧭️ Field-by-field state delta between two rows of the SAME element (same `properties`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_between(properties: &[PlyProperty], a: &PlyRow, b: &PlyRow) -> PlyRowDiff {
    let mut fields = Vec::new();
    for (i, prop) in properties.iter().enumerate() {
        let av = a.values.get(i);
        let bv = b.values.get(i);
        if av != bv {
            if let Some(bv) = bv {
                fields.push(PlyRowFieldChange { name: prop.name().to_string(), value: bv.clone() });
            }
        }
    }
    PlyRowDiff { fields }
}
//#endregion 🔖️RowFieldDiff

//#region 🔖️RowsTriple
/// 📦️ One `rows.modified[]` entity — `index` is the row's position in BASE.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyRowModified {
    pub index: usize,
    pub diff: PlyRowDiff,
}

/// 📦️ One `rows.added[]` entity — `index` is the row's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyRowAdded {
    pub index: usize,
    pub row: PlyRow,
}

/// 🔺️ Index-keyed removed/modified/added triple over one element's `rows` (PLY rows have no
/// stable identity beyond position, same rationale as csv's `records` triple).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyRowsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PlyRowModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PlyRowAdded>,
}

impl PlyRowsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a rows patch in place: modified (BASE indices, applied first) → removed
/// (descending, so earlier removals never shift a later one still pending) → added (FINAL
/// indices, ascending, clamped) — apply-order contract from the recipe's `## Diff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rows_diff(properties: &[PlyProperty], rows: &mut Vec<PlyRow>, diff: &PlyRowsDiff) {
    for m in &diff.modified {
        apply_row_diff(properties, &mut rows[m.index], &m.diff);
    }
    let mut removed_desc = diff.removed.clone();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_desc {
        rows.remove(idx);
    }
    let mut adds: Vec<&PlyRowAdded> = diff.added.iter().collect();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        rows.insert(a.index, a.row.clone());
    }
}

/// 🧭️ Index-pairwise state delta between two same-element row lists: `0..min(len)` compared
/// positionally (modified), the longer side's tail supplies removed (base longer) or added
/// (other longer) — never both from one call (see `field_sweep`'s two-direction test).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rows_between(properties: &[PlyProperty], a: &[PlyRow], b: &[PlyRow]) -> Option<PlyRowsDiff> {
    let min_len = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if a[i] == b[i] {
            continue;
        }
        let d = row_between(properties, &a[i], &b[i]);
        if !d.fields.is_empty() {
            modified.push(PlyRowModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min_len..a.len()).collect();
    let added: Vec<PlyRowAdded> = (min_len..b.len()).map(|i| PlyRowAdded { index: i, row: b[i].clone() }).collect();
    let d = PlyRowsDiff { removed, modified, added };
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

//#region 🔖️RowsAbsorb
/// 🎰 One slot of a simulated post-removal/insertion row array (index-transport for absorb,
/// mirrors csv's `records` absorb — duplicated locally per-artifact, not shared, per the
/// recipe's anti-generic-code rule).
#[derive(Clone, Copy, Debug)]
enum RowSlot {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<RowSlot> {
    let mut slots: Vec<RowSlot> = (0..len).map(RowSlot::Base).collect();
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
        slots.insert(at, RowSlot::Added(i));
    }
    slots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}

/// ➕️ Structural, total, base-free absorb of two `rows` triples belonging to the SAME element
/// (`## Absorb` contract) — index-transport twin of `absorb_elements` below, one nesting level
/// deeper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rows(d1: PlyRowsDiff, d2: PlyRowsDiff) -> PlyRowsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
    let base_len = row_base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = row_simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, PlyRowDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<PlyRowAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(RowSlot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(RowSlot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(RowSlot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(RowSlot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    apply_row_field_changes_by_position_fallback(&mut added.row, &m2.diff);
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
    let mut final_modified: Vec<PlyRowModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| PlyRowModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            RowSlot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
    let after_slots = row_simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let RowSlot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<PlyRowAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            if let Some(mid_pos) = mid_slots.iter().position(|s| matches!(s, RowSlot::Added(idx) if *idx == ai)) {
                if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                    final_added.push(PlyRowAdded { index: *after_pos, row: added.row });
                }
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    PlyRowsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

/// ➕️ Scope cut (see `deviations`): patching a carried `added` ROW's cells by property NAME
/// requires the owning element's `properties` (name→position) — but row-level absorb is
/// base-free (no snapshot, no element context) per the `## Absorb` contract, so that anchor
/// isn't available here. Safe no-op fallback: a `SetRowProperty` absorbed onto a not-yet-applied
/// `InsertRow` of a DIFFERENT diff (both targeting the same still-uncommitted row, within an
/// EXISTING element) drops the patch rather than guessing a position — never corrupts data. The
/// canonical, tested "Add+SetField" case — `AddElement` (whole element, real `properties`
/// attached) followed by `SetRowProperty` on that same still-pending row — is unaffected: it
/// flows through `absorb_elements`' `apply_element_diff`-into-added path instead, which DOES
/// carry real `properties` and resolves correctly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_field_changes_by_position_fallback(row: &mut PlyRow, diff: &PlyRowDiff) {
    let _ = (row, diff);
}
//#endregion 🔖️RowsAbsorb
//#endregion 🔖️RowsTriple

//#region 🔖️ElementDiff
/// 🔺️ Sparse per-field patch for one [`PlyElement`]. `properties` is a weak value-list —
/// whole-vec replaced, never sub-diffed (recipe's weak-entity rule) — because a property-schema
/// change invalidates positional row data anyway (see `element_between`'s scope-cut note).
/// 🧪️ F6: this struct is the exact real blocker cited in the module doc comment
/// (`properties: Option<Vec<PlyProperty>>` — `PlyProperty: DslField` unsatisfied); it needs no
/// `dsl` derive at all, it's a plain leaf type consumed by the hand-rolled `print_diff`/
/// `parse_diff`/`encode_diff`/`decode_diff` below.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyElementDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PlyProperty>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<PlyRowsDiff>,
}

impl PlyElementDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.count.is_none() && self.properties.is_none() && self.rows.as_ref().is_none_or(PlyRowsDiff::is_empty)
    }
}

/// ▶️ Applies independently owned declaration metadata and occurrence changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_element_diff(element: &mut PlyElement, diff: &PlyElementDiff) {
    if let Some(count) = diff.count { element.count=count; }
    if let Some(props) = &diff.properties {
        element.properties = props.clone();
    }
    if let Some(rd) = &diff.rows {
        apply_rows_diff(&element.properties, &mut element.rows, rd);
    }
}

/// ➕️ Recursive per-field absorb of one element's patch into another.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_element_diff(base: &mut PlyElementDiff, other: PlyElementDiff) {
    if other.count.is_some() { base.count=other.count; }
    if other.properties.is_some() {
        base.properties = other.properties;
    }
    base.rows = match (base.rows.take(), other.rows) {
        (None, None) => None,
        (Some(d1), None) => Some(d1),
        (None, Some(d2)) => Some(d2),
        (Some(d1), Some(d2)) => Some(absorb_rows(d1, d2)),
    };
}

/// 🧭️ Field-by-field state delta between two elements sharing the same NAME. If `properties`
/// itself differs (a genuine schema change — there is no `ChangeElementProperties` mutation, so
/// this only arises from hand-built `between()` calls or `SetSnapshot`), row-level positional
/// diffing is meaningless across two different schemas: fall back to a whole-rows replace
/// (documented scope cut — see `deviations`), matching the recipe's "trees recursive with
/// Replace fallback on node-kind change" rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn element_between(a: &PlyElement, b: &PlyElement) -> PlyElementDiff {
    if a.properties != b.properties {
        let removed: Vec<usize> = (0..a.rows.len()).collect();
        let added: Vec<PlyRowAdded> = b.rows.iter().enumerate().map(|(i, r)| PlyRowAdded { index: i, row: r.clone() }).collect();
        let rd = PlyRowsDiff { removed, modified: vec![], added };
        return PlyElementDiff { count: (a.count != b.count).then_some(b.count), properties: Some(b.properties.clone()), rows: if rd.is_empty() { None } else { Some(rd) } };
    }
    PlyElementDiff { count: (a.count != b.count).then_some(b.count), properties: None, rows: rows_between(&a.properties, &a.rows, &b.rows) }
}
//#endregion 🔖️ElementDiff

//#region 🔖️ElementsTriple
/// 📦️ One `elements.modified[]` entity — `name` is the element's identity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyElementModified {
    pub name: String,
    pub diff: PlyElementDiff,
}

/// 📦️ One `elements.added[]` entity — `index` is the element's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyElementAdded {
    pub index: usize,
    pub element: PlyElement,
}

/// 🔺️ Sparse name-keyed `elements` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlyElementsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PlyElementModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PlyElementAdded>,
}

impl PlyElementsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ➕️ Name-keyed absorb (mirrors zip's `entries` absorb — no rename support for elements since
/// there is no `RenameElement` mutation, which simplifies the key-transport map to identity).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_elements(d1: Option<PlyElementsDiff>, d2: Option<PlyElementsDiff>) -> Option<PlyElementsDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    let added_names: HashSet<String> = d1.added.iter().map(|a| a.element.name.clone()).collect();
    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    let mut removed_shift = 0usize;
    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else {
            removed_shift += 1;
            if !merged_removed.contains(name) {
                merged_removed.push(name.clone());
            }
            d1.modified.retain(|m| &m.name != name);
        }
    }

    let mut merged_modified: Vec<PlyElementModified> = d1.modified;
    let mut merged_added: Vec<PlyElementAdded> = d1
        .added
        .into_iter()
        .filter(|a| !annihilated.contains(&a.element.name))
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift);
            a
        })
        .collect();

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue;
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.element.name == dm.name) {
                apply_element_diff(&mut a.element, &dm.diff);
            }
        } else {
            if merged_removed.contains(&dm.name) {
                continue;
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == dm.name) {
                absorb_element_diff(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(PlyElementModified { name: dm.name.clone(), diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);
    let merged = PlyElementsDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️ElementsTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ply`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.diff")]
pub struct PlyDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<PlyFormat>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<PlyElementsDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_rows_diff(properties: &[PlyProperty], rows: &[PlyRow], diff: &PlyRowsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut property_positions = BTreeMap::new();
    for (index, property) in properties.iter().enumerate() {
        if property_positions.insert(property.name(), index).is_some() {
            let mut target = prefix.to_vec();
            target.extend(["properties".to_string(), property.name().to_string()]);
            return Err(target_error("mutation.apply.duplicate-base-target", "property names must be unique", target));
        }
    }
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let mut target = prefix.to_vec();
        target.extend(["rows".to_string(), index.to_string()]);
        if index >= rows.len() || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "row removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let mut row_target = prefix.to_vec();
        row_target.extend(["rows".to_string(), entry.index.to_string()]);
        if entry.index >= rows.len() || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "row modification target must exist exactly once and remain present", row_target));
        }
        let mut fields = BTreeSet::new();
        for field in &entry.diff.fields {
            let mut target = prefix.to_vec();
            target.extend(["rows".to_string(), entry.index.to_string(), "fields".to_string(), field.name.clone()]);
            let position = property_positions.get(field.name.as_str()).copied();
            if !fields.insert(field.name.as_str()) || position.is_none() || position.is_some_and(|value| value >= rows[entry.index].values.len()) {
                return Err(target_error("invalid-field-target", "row field target must be unique and resolve to an existing cell", target));
            }
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (rows.len() - removed.len()..).zip(additions) {
        let mut target = prefix.to_vec();
        target.extend(["rows".to_string(), index.to_string()]);
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "row addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_elements_diff(base: &[PlyElement], diff: &PlyElementsDiff) -> MutationApplyResult<()> {
    let mut base_by_name = BTreeMap::new();
    for element in base {
        if base_by_name.insert(element.name.as_str(), element).is_some() {
            return Err(target_error("mutation.apply.duplicate-base-target", "base element names must be unique", vec!["elements".to_string(), element.name.clone()]));
        }
    }
    let mut removed = BTreeSet::new();
    for name in &diff.removed {
        if !base_by_name.contains_key(name.as_str()) || !removed.insert(name.as_str()) {
            return Err(target_error("mutation.apply.invalid-remove-target", "element removal target must exist exactly once", vec!["elements".to_string(), name.clone()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let base_element = base_by_name.get(entry.name.as_str()).copied();
        if base_element.is_none() || removed.contains(entry.name.as_str()) || !modified.insert(entry.name.as_str()) {
            return Err(target_error("mutation.apply.invalid-modify-target", "element modification target must exist exactly once and remain present", vec!["elements".to_string(), entry.name.clone()]));
        }
        if let (Some(rows), Some(element)) = (&entry.diff.rows, base_element) {
            let properties = entry.diff.properties.as_deref().unwrap_or(&element.properties);
            validate_rows_diff(properties, &element.rows, rows, &["elements".to_string(), entry.name.clone()])?;
        }
    }
    let mut additions: Vec<&PlyElementAdded> = diff.added.iter().collect();
    additions.sort_by_key(|entry| entry.index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (length, entry) in (base.len() - removed.len()..).zip(additions) {
        if base_by_name.contains_key(entry.element.name.as_str()) || !added_names.insert(entry.element.name.as_str()) || entry.index > length || previous == Some(entry.index) {
            return Err(target_error("mutation.apply.invalid-add-target", "element name and position must be unique and valid", vec!["elements".to_string(), entry.element.name.clone()]));
        }
        previous = Some(entry.index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ply_diff_unchecked(diff: &PlyDiff, base: &PlySnapshot) -> PlySnapshot {
    let mut next = base.clone();
    if let Some(format) = diff.format {
        next.format = format;
    }
    if let Some(comments) = &diff.comments {
        next.comments = comments.clone();
    }
    if let Some(elements) = &diff.elements {
        for modified in &elements.modified {
            for element in &mut next.elements {
                if element.name == modified.name {
                    apply_element_diff(element, &modified.diff);
                }
            }
        }
        let removed: HashSet<&str> = elements.removed.iter().map(String::as_str).collect();
        next.elements.retain(|element| !removed.contains(element.name.as_str()));
        let mut additions: Vec<&PlyElementAdded> = elements.added.iter().collect();
        additions.sort_by_key(|entry| entry.index);
        for entry in additions {
            next.elements.insert(entry.index, entry.element.clone());
        }
    }
    next
}

impl MutationDiff<PlySnapshot> for PlyDiff {
    fn apply(&self, base: &PlySnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<PlySnapshot> {
        if let Some(diff) = &self.elements {
            validate_elements_diff(&base.elements, diff)?;
        }
        Ok(apply_ply_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `elements`: name-keyed transport (no renames — `AddElement`/`RemoveElement` only), one
    /// nested `rows` absorb per surviving modified element.
    fn absorb(&mut self, other: Self) {
        if other.format.is_some() {
            self.format = other.format;
        }
        if other.comments.is_some() {
            self.comments = other.comments;
        }
        self.elements = absorb_elements(self.elements.take(), other.elements);
    }
}

impl DiffAlgebra<PlySnapshot> for PlyDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) from `between`.
    fn inverse(&self, base: &PlySnapshot) -> Self {
        let mutated = apply_ply_diff_unchecked(self, base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): name-keyed matching over `elements`, each modified
    /// element recursing into `element_between`.
    fn between(base: &PlySnapshot, other: &PlySnapshot) -> Self {
        let format = (base.format != other.format).then_some(other.format);
        let comments = (base.comments != other.comments).then(|| other.comments.clone());
        let elements = if base.elements == other.elements {
            None
        } else {
            let base_names: HashSet<&str> = base.elements.iter().map(|e| e.name.as_str()).collect();
            let other_names: HashSet<&str> = other.elements.iter().map(|e| e.name.as_str()).collect();

            let removed: Vec<String> = base.elements.iter().filter(|e| !other_names.contains(e.name.as_str())).map(|e| e.name.clone()).collect();

            let mut modified = Vec::new();
            for be in &base.elements {
                if let Some(oe) = other.elements.iter().find(|o| o.name == be.name) {
                    let d = element_between(be, oe);
                    if !d.is_empty() {
                        modified.push(PlyElementModified { name: be.name.clone(), diff: d });
                    }
                }
            }

            let added: Vec<PlyElementAdded> = other.elements.iter().enumerate().filter(|(_, e)| !base_names.contains(e.name.as_str())).map(|(index, e)| PlyElementAdded { index, element: e.clone() }).collect();

            let d = PlyElementsDiff { removed, modified, added };
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };
        PlyDiff { format, comments, elements }
    }

    fn is_empty(&self) -> bool {
        self.format.is_none() && self.comments.is_none() && self.elements.as_ref().is_none_or(PlyElementsDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `PlyDiff` to short-circuit into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &PlySnapshot, next: &PlySnapshot) -> PlyDiff {
    PlyDiff::between(base, next)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_format(format: PlyFormat) -> PlyDiff {
    PlyDiff { format: Some(format), comments: None, elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_comments(comments: Vec<String>) -> PlyDiff {
    PlyDiff { format: None, comments: Some(comments), elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_element(index: usize, element: PlyElement) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![], added: vec![PlyElementAdded { index, element }] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_element(name: &str) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_element_field(name: &str, diff: PlyElementDiff) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![PlyElementModified { name: name.to_string(), diff }], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_row(element_name: &str, index: usize, row: PlyRow) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![], modified: vec![], added: vec![PlyRowAdded { index, row }] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_row(element_name: &str, index: usize) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![index], modified: vec![], added: vec![] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_row_property(element_name: &str, row_index: usize, property_name: &str, value: PlyValue) -> PlyDiff {
    diff_element_field(
        element_name,
        PlyElementDiff {
            count: None,
            properties: None,
            rows: Some(PlyRowsDiff { removed: vec![], modified: vec![PlyRowModified { index: row_index, diff: PlyRowDiff { fields: vec![PlyRowFieldChange { name: property_name.to_string(), value }] } }], added: vec![] }),
        },
    )
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `PlyDiff` — `#[derive(dsl::DslDiff)]` is not
/// usable (see the module doc comment for the real `cargo check` citation: `PlyProperty`/
/// `PlyValue` are data-carrying enums reachable from `PlyElementDiff::properties` and
/// `PlyRowFieldChange::value`, the 3a "enum-in-tree" blocker per `f6-recon-report.md` §3a).
///
/// **Grammar** (real, not `serde_json`), following the ticket's §5 template verbatim: one
/// space-separated `name=value` token per changed top-level field (absent token = unchanged).
/// Bytes/strings are lowercase hex (`enc_str`/`dec_str` — no external base64 dep, no escaping).
/// `Option<T>` values use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are positional
/// `[f1,f2,...]` tuples. Data-carrying enums (`PlyProperty`, `PlyValue`) use a single-uppercase
/// (or, for `PlyValue`'s eight scalar kinds, single-lowercase) tag prefix immediately followed by
/// the bracketed payload. Collection triples print as `{[removed];[modified];[added]}` — for the
/// index-keyed `rows` triple, `removed`/`modified` are index-keyed; for the name-keyed `elements`
/// triple, `removed`/`modified` are NAME-keyed (hex) while `added` stays index-keyed (matches
/// `PlyElementsDiff`'s own real shape — see `f6-ply-report.md` for why this deliberately deviates
/// from gif89a's uniform-index-keyed triple helper).
///
/// Worked example (captured from a real test run, see `diff_codec_text_binary_roundtrip_law`):
/// `format=6c comments=[68656c6c6f] elements={[666163e5];[];[76657274657865:[P:[...],R:{...}]]}`
/// (illustrative shape only — see the test for the literal printed string).
//#region 🔖️Primitives






//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs















//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives























//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame










//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoDiffCases
/// ✅️ Every representative `PlyDiff` shape (empty, plus a real `between()` result in BOTH
/// directions over `sweep_a()`/`sweep_b()`) — the single case list `diff_codec_text_binary_
/// roundtrip_law` (this file) AND `diff_grammar_conformance_law`/`protocol_walk_law`
/// (`⚙️engine/🦀️.rs`) all exercise. Covers every scalar field, the name-keyed `elements`
/// triple in all three flavors (removed/modified/added) simultaneously, the nested index-keyed
/// `rows` triple, the weak `properties` replace, and both `PlyProperty`/`PlyValue` enum tag
/// families (incl. `PlyValue::List`'s recursion).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_a() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["a".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] }],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_b() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::BinaryLittleEndian,
        comments: vec!["a".into(), "b".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 1,
                properties: vec![PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double }, PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double }],
                rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(-9.5)] }],
            },
            PlyElement { name: "edge".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }], rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }] },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<PlyDiff> {
    let a = sweep_a();
    let b = sweep_b();
    vec![PlyDiff::default(), <PlyDiff as DiffAlgebra<PlySnapshot>>::between(&a, &b), <PlyDiff as DiffAlgebra<PlySnapshot>>::between(&b, &a)]
}
//#endregion 🔖️DemoDiffCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::PlyValue;
//#endregion 🔁️Re-exports

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
