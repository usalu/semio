//! 🧮️ The generic index-keyed removed, modified and added triple with its apply, validate, absorb and inverse algebra.

use super::*;

//#region 🔖️CommentsTriple
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedDiff<T, D> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexedModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexedAdded<T>>,
}

impl<T, D> IndexedDiff<T, D> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedAdded<T> {
    pub index: usize,
    pub item: T,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_indexed<T: Clone, D>(base: &[T], diff: &IndexedDiff<T, D>, apply_item: impl Fn(&T, &D) -> T) -> Vec<T> {
    let mut kept: Vec<(usize, T)> = base.iter().enumerate().filter(|(i, _)| !diff.removed.contains(i)).map(|(i, t)| (i, t.clone())).collect();
    for m in &diff.modified {
        if let Some(entry) = kept.iter_mut().find(|(i, _)| *i == m.index) {
            entry.1 = apply_item(&entry.1, &m.diff);
        }
    }
    let mut result: Vec<T> = kept.into_iter().map(|(_, t)| t).collect();
    let mut adds = diff.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let idx = a.index.min(result.len());
        result.insert(idx, a.item);
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_indexed<T, D>(base: &[T], diff: &IndexedDiff<T, D>, validate_item: impl Fn(&T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()> {
    let mut removed = HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated"));
        }
    }
    let mut modified = HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated"));
        }
        validate_item(&base[entry.index], &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unrank_excluding(rank: usize, excluded_sorted: &[usize]) -> usize {
    let mut candidate = rank;
    loop {
        let next = rank + count_le(excluded_sorted, candidate);
        if next == candidate {
            return candidate;
        }
        candidate = next;
    }
}

/// ➕️ Structural, total, base-free absorb — see mp4's `absorb_indexed` for the full derivation
/// (identical algorithm, adapted from gif 89a's `absorb_indexed_collection`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn absorb_indexed<T: Clone, D: Clone>(d1: &mut IndexedDiff<T, D>, d2: IndexedDiff<T, D>, absorb_item: impl Fn(&mut D, D), apply_item_diff: impl Fn(&mut T, &D)) {
    let removed1_sorted = semio_s_artifact_stdio_contract::ordered(&d1.removed);
    let mut added1_index_sorted: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    added1_index_sorted.sort_unstable();
    let removed2_sorted = semio_s_artifact_stdio_contract::ordered(&d2.removed);
    let mut added2_index_sorted: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    added2_index_sorted.sort_unstable();

    let mut merged_added: Vec<IndexedAdded<T>> = std::mem::take(&mut d1.added);
    let mut annihilated: HashSet<usize> = Default::default();

    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|a| a.index != r2);
        } else {
            let post_remove_rank = rank_excluding(r2, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            merged_removed_base.push(base_index);
        }
    }
    merged_removed_base.sort_unstable();
    merged_removed_base.dedup();

    let mut modified_map: BTreeMap<usize, D> = std::mem::take(&mut d1.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for m2 in d2.modified {
        if annihilated.contains(&m2.index) {
            continue;
        }
        if added1_index_sorted.binary_search(&m2.index).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|a| a.index == m2.index) {
                apply_item_diff(&mut entry.item, &m2.diff);
            }
        } else {
            let post_remove_rank = rank_excluding(m2.index, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            match modified_map.get_mut(&base_index) {
                Some(existing) => absorb_item(existing, m2.diff),
                None => {
                    modified_map.insert(base_index, m2.diff);
                }
            }
        }
    }

    let mut merged_added_final: Vec<IndexedAdded<T>> = merged_added
        .into_iter()
        .map(|a| {
            let after_pos = if removed2_sorted.binary_search(&a.index).is_ok() {
                a.index
            } else {
                let post_remove_rank = rank_excluding(a.index, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            IndexedAdded { index: after_pos, item: a.item }
        })
        .collect();
    merged_added_final.extend(d2.added);
    merged_added_final.sort_by_key(|a| a.index);

    d1.removed = merged_removed_base;
    d1.modified = modified_map.into_iter().map(|(index, diff)| IndexedModified { index, diff }).collect();
    d1.added = merged_added_final;
}

/// ↩️ Negative rows for an indexed collection triple against its BASE items: added rows become removals at their final index,
/// removed rows return at their base index, and each modified row restores its base value at the index the row has after the
/// diff. Every list comes back ascending, the normal form [`absorb_indexed`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_indexed<T: Clone, D>(diff: &IndexedDiff<T, D>, base: &[T], inverse_item: impl Fn(&D, &T) -> D) -> IndexedDiff<T, D> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<IndexedModified<D>> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|item| IndexedModified { index: after_index(row.index), diff: inverse_item(&row.diff, item) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|item| IndexedAdded { index: *index, item: item.clone() })).collect();
    IndexedDiff { removed: added_final, modified, added }
}
//#endregion 🔖️CommentsTriple
