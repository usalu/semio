//! 🧮️ Indexed triple algebra of the LAS diff: the target validation, the label simulation and the absorb and inverse of removed, modified and added rows. See [`super`].

use super::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, target: &str) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-index", "removal target must exist exactly once").at([target, &index.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-index", "modification target must exist exactly once and remain present").at([target, &index.to_string()]));
        }
    }
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-index", "addition target must be unique and within the evolving sequence").at([target, &index.to_string()]));
        }
        previous = Some(index);
    }
    Ok(())
}

//#region 🔖️IndexedAbsorb
/// 🏷️ Structural, base-free label used only inside [`absorb_indexed_triple`] to simulate the
/// two-step index-transform (base→mid via `d1`, mid→after via `d2`) — mirrors
/// `txt`'s `Lbl`/`simulate_labels`/`absorb_pair` pattern (own copy: no cross-artifact type
/// sharing), generalized once here over `vlrs` and `points` (same collection shape, same
/// artifact) instead of copy-pasted twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

/// ➡️ Simulates one collection-triple's position algebra over an abstract label array: remove
/// the given base/mid indices, then insert `added` labels ascending at `min(index, current_len)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<Lbl> = labels.into_iter().enumerate().filter(|(i, _)| !removed_set.contains(i)).map(|(_, l)| l).collect();
    let added_sorted = semio_s_artifact_stdio_contract::ordered_by_key(added, |(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

/// 🔺️ Borrowed removal, modification, and addition views for one indexed diff.
type IndexedDiffRef<'a, D, T> = (&'a [usize], &'a [(usize, D)], &'a [(usize, T)]);

/// ➕️ Generic index-keyed collection-triple absorb: `self` is base→mid (`d1`), `other` is
/// mid→after (`d2`). `merge_field_diff`/`patch_item` are the only per-entity-type logic —
/// everything else (index transport, annihilate-on-remove, patch-into-added) is the recipe's
/// normative algorithm, identical for `vlrs` and `points`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_indexed_triple<Item: Clone, D: Clone + Default + PartialEq>(
    (d1_removed, d1_modified, d1_added): IndexedDiffRef<'_, D, Item>,
    (d2_removed, d2_modified, d2_added): IndexedDiffRef<'_, D, Item>,
    absorb_field: impl Fn(&mut D, D),
    patch_item: impl Fn(&mut Item, &D),
) -> IndexedDiffParts<D, Item> {
    let max_ref =
        d1_removed.iter().copied().chain(d1_modified.iter().map(|(i, _)| *i)).chain(d1_added.iter().map(|(i, _)| *i)).chain(d2_removed.iter().copied()).chain(d2_modified.iter().map(|(i, _)| *i)).chain(d2_added.iter().map(|(i, _)| *i)).max();
    let l1 = max_ref.map_or(0, |m| m + 2);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added_lbl: Vec<(usize, Lbl)> = d1_added.iter().enumerate().map(|(j, (idx, _))| (*idx, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, d1_removed, &d1_added_lbl);

    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    let mut mid_pos_of_added1: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        match l {
            Lbl::Base(i) => {
                mid_pos_of_base.insert(*i, pos);
            }
            Lbl::Added1(j) => {
                mid_pos_of_added1.insert(*j, pos);
            }
            Lbl::Added2(_) => {}
        }
    }
    while mid_labels.len() < l1 {
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding, never referenced by mid_pos_of_base
    }

    let d2_added_lbl: Vec<(usize, Lbl)> = d2_added.iter().enumerate().map(|(k, (idx, _))| (*idx, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels, d2_removed, &d2_added_lbl);

    let d1_modified_at: HashMap<usize, &D> = d1_modified.iter().map(|(i, d)| (*i, d)).collect();
    let d2_modified_at: HashMap<usize, &D> = d2_modified.iter().map(|(i, d)| (*i, d)).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified: Vec<(usize, D)> = Vec::new();
    let mut added: Vec<(usize, Item)> = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let d1v = d1_modified_at.get(&i).copied().cloned();
                let d2v = mid_pos.and_then(|m| d2_modified_at.get(&m)).copied().cloned();
                let merged = match (d1v, d2v) {
                    (None, None) => None,
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (Some(mut a), Some(b)) => {
                        absorb_field(&mut a, b);
                        Some(a)
                    }
                };
                if let Some(d) = merged {
                    if d != D::default() {
                        modified.push((i, d));
                    }
                }
            }
            Lbl::Base(_) => {}
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let mut item = d1_added[j].1.clone();
                if let Some(m) = mid_pos {
                    if let Some(d) = d2_modified_at.get(&m) {
                        patch_item(&mut item, d);
                    }
                }
                added.push((pos, item));
            }
            Lbl::Added2(k) => {
                added.push((pos, d2_added[k].1.clone()));
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    (removed, modified, added)
}
//#endregion 🔖️IndexedAbsorb

//#region 🔖️IndexedInverse
/// ↩️ Negative rows for an index-keyed collection triple against its BASE items: added rows become removals at their final index,
/// removed rows return at their base index, and each modified row restores its base fields at the index the row has after the
/// diff. Every list comes back ascending, the normal form [`absorb_indexed_triple`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_indexed_triple<Item: Clone, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, Item)], base_items: &[Item], diff_inverse: impl Fn(&D, &Item) -> D) -> IndexedDiffParts<D, Item> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered(&removed);
    let mut added_final: Vec<usize> = added.iter().map(|(index, _)| *index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut inverse_modified: Vec<(usize, D)> = modified.iter().filter_map(|(index, diff)| base_items.get(*index).map(|item| (after_index(*index), diff_inverse(diff, item)))).collect();
    inverse_modified.sort_by_key(|(index, _)| *index);
    let inverse_added = removed_sorted.iter().filter_map(|index| base_items.get(*index).map(|item| (*index, item.clone()))).collect();
    (added_final, inverse_modified, inverse_added)
}
//#endregion 🔖️IndexedInverse
