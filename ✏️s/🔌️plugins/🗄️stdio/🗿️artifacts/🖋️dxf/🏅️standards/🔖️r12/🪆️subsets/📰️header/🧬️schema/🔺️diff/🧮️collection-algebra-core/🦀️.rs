//! 🧮️ Collection algebra of the DXF diff: the index-keyed and name-keyed element traits with their generic apply, inverse and absorb-pair cores. See [`super`].

use std::collections::HashSet;

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub type NamedDiffParts<D, T> = (Vec<String>, Vec<(String, D)>, Vec<(usize, T)>);

//#region 🔖️IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the two index-keyed collections (`DxfEntity`,
/// `DxfBlock` — the latter's own nested `entities` field reuses `DxfEntity`'s impl directly).
pub trait DxfIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    fn diff_is_empty(d: &Self::Diff) -> bool;
    fn field_changes(a: &Self, b: &Self) -> Self::Diff;
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff;
    fn diff_apply(d: &Self::Diff, item: &mut Self);
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

/// ▶️ Applies a `(removed, modified, added)` triple to a base array — modified on BASE
/// positions first, then removed descending, then added ascending clamped to `min(index,len)`
/// (recipe's normative apply order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn generic_apply<T: DxfIndexElem>(base: &[T], removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (idx, d) in modified {
        T::diff_apply(d, &mut items[*idx]);
    }
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_desc {
        items.remove(idx);
    }
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

/// ↩️ Negative rows of an index-keyed triple against its BASE items: added rows become removals at their final index, removed
/// rows return at their base index, and each modified row restores its base fields at the index the row has after the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn generic_inverse<T: DxfIndexElem>(removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)], base: &[T]) -> IndexedDiffParts<T::Diff, T> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&removed);
    let mut added_final: Vec<usize> = added.iter().map(|(index, _)| *index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut inverse_modified: Vec<(usize, T::Diff)> = modified.iter().filter_map(|(index, diff)| base.get(*index).map(|item| (after_index(*index), T::diff_inverse(diff, item)))).collect();
    inverse_modified.sort_by_key(|(index, _)| *index);
    let inverse_added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|item| (*index, item.clone()))).collect();
    (added_final, inverse_modified, inverse_added)
}

/// ↩️ Negative rows of a name-keyed triple against its BASE items: added entries become removals by name, removed entries return
/// at their base index, and each modified entry restores its base fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn named_inverse<T: DxfNamedElem>(removed: &[String], modified: &[(String, T::Diff)], added: &[(usize, T)], base: &[T]) -> NamedDiffParts<T::Diff, T> {
    let mut added_ordered: Vec<&(usize, T)> = added.iter().collect();
    added_ordered.sort_by_key(|(index, _)| *index);
    let inverse_removed = added_ordered.iter().map(|(_, item)| item.key().to_string()).collect();
    let inverse_modified = modified.iter().filter_map(|(key, diff)| base.iter().find(|item| item.key() == key).map(|item| (key.clone(), T::diff_inverse(diff, item)))).collect();
    let mut inverse_added: Vec<(usize, T)> = removed.iter().filter_map(|key| base.iter().position(|item| item.key() == key).map(|index| (index, base[index].clone()))).collect();
    inverse_added.sort_by_key(|(index, _)| *index);
    (inverse_removed, inverse_modified, inverse_added)
}

/// 🏷️ Structural, base-free label used only inside [`generic_absorb_pair`] to simulate the
/// two-step position transform (base→mid via `d1`, mid→after via `d2`) — mirrors `stdio.txt`'s
/// proven `Lbl`/`simulate_labels`/`absorb_pair` shape (also mirrored verbatim in `stdio.obj`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

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

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) into a single base→after triple.
#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn generic_absorb_pair<T: DxfIndexElem>(d1_removed: &[usize], d1_modified: &[(usize, T::Diff)], d1_added: &[(usize, T)], d2_removed: &[usize], d2_modified: &[(usize, T::Diff)], d2_added: &[(usize, T)]) -> IndexedDiffParts<T::Diff, T> {
    use std::collections::HashMap;
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
        mid_labels.push(Lbl::Base(usize::MAX));
    }

    let d2_added_lbl: Vec<(usize, Lbl)> = d2_added.iter().enumerate().map(|(k, (idx, _))| (*idx, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels.clone(), d2_removed, &d2_added_lbl);

    let d2_modified_at: HashMap<usize, &T::Diff> = d2_modified.iter().map(|(i, d)| (*i, d)).collect();
    let d1_modified_at: HashMap<usize, &T::Diff> = d1_modified.iter().map(|(i, d)| (*i, d)).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified: Vec<(usize, T::Diff)> = Vec::new();
    let mut added: Vec<(usize, T)> = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let d2d = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).cloned();
                let d1d = d1_modified_at.get(&i).copied().cloned();
                let combined = match (d1d, d2d) {
                    (None, None) => None,
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (Some(mut a), Some(b)) => {
                        T::diff_absorb(&mut a, b);
                        Some(a)
                    }
                };
                if let Some(d) = combined {
                    if !T::diff_is_empty(&d) {
                        modified.push((i, d));
                    }
                }
            }
            Lbl::Base(_) => {}
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let (_, base_item) = &d1_added[j];
                let mut item = base_item.clone();
                if let Some(m) = mid_pos {
                    if let Some(d2d) = d2_modified_at.get(&m) {
                        T::diff_apply(d2d, &mut item);
                    }
                }
                added.push((pos, item));
            }
            Lbl::Added2(k) => {
                let (_, item) = &d2_added[k];
                added.push((pos, item.clone()));
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    (removed, modified, added)
}
//#endregion 🔖️IndexCollectionCore

//#region 🔖️NamedCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the four name-keyed collections (`DxfHeaderVar`,
/// `DxfLayer`, `DxfStyle`, `DxfLinetype`) — same shape as `stdio.obj`'s `HasFaces`/group pattern,
/// generalized into a trait since there are four distinct named types here, not two structurally
/// identical ones. No rename tracking: nothing in this artifact's mutation vocabulary renames a
/// header var/layer/style/linetype in place (matches `stdio.obj`'s groups/objects, not `stdio.zip`'s
/// entries).
pub trait DxfNamedElem: Clone + PartialEq {
    type Diff: Clone + PartialEq + Default;
    fn key(&self) -> &str;
    fn field_changes(a: &Self, b: &Self) -> Self::Diff;
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff;
    fn diff_apply(d: &Self::Diff, item: &mut Self);
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn named_apply<T: DxfNamedElem>(base: &[T], removed: &[String], modified: &[(String, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (key, d) in modified {
        for item in &mut items {
            if item.key() == key {
                T::diff_apply(d, item);
            }
        }
    }
    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    items.retain(|it| !removed_set.contains(&it.key()));
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn named_absorb_pair<T: DxfNamedElem>(d1_removed: &[String], d1_modified: &[(String, T::Diff)], d1_added: &[(usize, T)], d2_removed: &[String], d2_modified: &[(String, T::Diff)], d2_added: &[(usize, T)]) -> NamedDiffParts<T::Diff, T> {
    let added_keys: HashSet<String> = d1_added.iter().map(|(_, t)| t.key().to_string()).collect();
    let mut merged_removed: Vec<String> = Vec::from(d1_removed);
    let mut annihilated: HashSet<String> = HashSet::new();
    for key in d2_removed {
        if added_keys.contains(key) {
            annihilated.insert(key.clone());
        } else if !merged_removed.contains(key) {
            merged_removed.push(key.clone());
        }
    }
    let mut merged_modified: Vec<(String, T::Diff)> = d1_modified.iter().filter(|(k, _)| !merged_removed.contains(k)).cloned().collect();
    let mut merged_added: Vec<(usize, T)> = d1_added.iter().filter(|(_, t)| !annihilated.contains(t.key())).cloned().collect();

    for (key, d2d) in d2_modified {
        if added_keys.contains(key) {
            if annihilated.contains(key) {
                continue;
            }
            if let Some((_, item)) = merged_added.iter_mut().find(|(_, t)| t.key() == key) {
                T::diff_apply(d2d, item);
            }
        } else {
            if merged_removed.contains(key) {
                continue;
            }
            if let Some((_, existing)) = merged_modified.iter_mut().find(|(k, _)| k == key) {
                T::diff_absorb(existing, d2d.clone());
            } else {
                merged_modified.push((key.clone(), d2d.clone()));
            }
        }
    }
    merged_added.extend(d2_added.iter().cloned());
    (merged_removed, merged_modified, merged_added)
}
//#endregion 🔖️NamedCollectionCore
