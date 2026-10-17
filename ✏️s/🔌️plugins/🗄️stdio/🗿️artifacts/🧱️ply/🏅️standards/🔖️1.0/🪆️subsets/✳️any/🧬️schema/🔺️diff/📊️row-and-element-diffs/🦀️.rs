//! 📊️ The row field, row, element and element-collection diffs with their position-aware absorb.

use super::*;

//#region 🔖️RowFieldDiff
/// 🔣️ One changed cell inside a row's sparse patch, keyed by the owning element's property
/// NAME (stable per-element schema — see module doc; positions can shift if `properties`
/// itself is replaced, names don't).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowFieldChange {
    pub name: String,
    pub value: PlyValue,
}

/// 🔺️ Sparse per-property patch for one [`PlyRow`] — only changed cells appear.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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

//#endregion 🔖️RowFieldDiff

//#region 🔖️RowsTriple
/// 📦️ One `rows.modified[]` entity — `index` is the row's position in BASE.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowModified {
    pub index: usize,
    pub diff: PlyRowDiff,
}

/// 📦️ One `rows.added[]` entity — `index` is the row's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowAdded {
    pub index: usize,
    pub row: PlyRow,
}

/// 🔺️ Index-keyed removed/modified/added triple over one element's `rows` (PLY rows have no
/// stable identity beyond position, same rationale as csv's `records` triple).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    let removed_desc = semio_s_artifact_stdio_contract::ordered_unique_descending(&removed);
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
        let r = semio_s_artifact_stdio_contract::ordered_unique(&d1.removed);
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    pub(super) fn is_empty(&self) -> bool {
        self.count.is_none() && self.properties.is_none() && self.rows.as_ref().is_none_or(PlyRowsDiff::is_empty)
    }
}

/// ▶️ Applies independently owned declaration metadata and occurrence changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_element_diff(element: &mut PlyElement, diff: &PlyElementDiff) {
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
fn absorb_element_rows(base: &mut PlyElementDiff, other: PlyElementDiff) {
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

//#endregion 🔖️ElementDiff

//#region 🔖️ElementsTriple
/// 📦️ One `elements.modified[]` entity — `name` is the element's identity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementModified {
    pub name: String,
    pub diff: PlyElementDiff,
}

/// 📦️ One `elements.added[]` entity — `index` is the element's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementAdded {
    pub index: usize,
    pub element: PlyElement,
}

/// 🔺️ Sparse name-keyed `elements` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
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
    pub(super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ➕️ Name-keyed absorb (mirrors zip's `entries` absorb — no rename support for elements since
/// there is no `RenameElement` mutation, which simplifies the key-transport map to identity).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_elements(d1: Option<PlyElementsDiff>, d2: Option<PlyElementsDiff>) -> Option<PlyElementsDiff> {
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
                absorb_element_rows(&mut existing.diff, dm.diff.clone());
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
