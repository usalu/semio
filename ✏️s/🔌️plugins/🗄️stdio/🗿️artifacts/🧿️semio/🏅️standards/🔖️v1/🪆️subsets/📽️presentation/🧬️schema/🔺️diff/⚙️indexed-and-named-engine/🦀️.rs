//! ⚙️ Generic indexed and named engine: apply, inverse, absorb and index transformation over index keyed and name keyed collection triples. See [`super`].

use super::*;

//#region 🔖️DiffTypes
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_indexed<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D))
where
    T: Clone,
{
    for m in &diff.modified {
        if let Some(item) = items.get_mut(m.index) {
            apply_item(item, &m.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(items.len());
        items.insert(at, add.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_indexed<T, D>(base_items: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T>
where
    T: Clone,
{
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(IndexModified { index: next_index, diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_items.get(idx) {
            added.push(IndexAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_index<T>(idx: usize, removed: &[usize], added: &[IndexAdded<T>]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_mid_origins<T>(base_len: usize, removed: &[usize], added: &[IndexAdded<T>]) -> Vec<ItemOrigin> {
    let mut mid: Vec<ItemOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ItemOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ItemOrigin::Added(k));
    }
    mid
}

#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
where
    T: Clone,
    D: Clone,
{
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified;
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(IndexModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_item(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    added.extend(d2.added);
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reproduces_order<K, T>(base: &[T], other: &[T], removed: &[K], added: &[NamedAdded<T>], key_of: &impl Fn(&T) -> K) -> bool
where
    K: PartialEq,
{
    let mut keys: Vec<K> = base.iter().map(key_of).filter(|k| !removed.contains(k)).collect();
    let mut ascending: Vec<&NamedAdded<T>> = added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(keys.len());
        keys.insert(at, key_of(&a.item));
    }
    keys.len() == other.len() && keys.iter().zip(other.iter().map(key_of)).all(|(produced, expected)| *produced == expected)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    let mut ascending: Vec<&NamedAdded<T>> = diff.added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(items.len());
        items.insert(at, a.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(|a| key_of(&a.item)).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added: Vec<NamedAdded<T>> = diff.removed.iter().filter_map(|k| base_items.iter().position(|i| &key_of(i) == k).map(|index| NamedAdded { index, item: base_items[index].clone() })).collect();
    added.sort_by_key(|a| a.index);
    NamedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, NamedAdded<T>>, d2: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(|a| key_of(&a.item)).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<NamedAdded<T>> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(&a.item))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(&a.item) == m2.key) {
            apply_item(&mut added.item, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(&a2.item);
        match working_added.iter_mut().find(|a| key_of(&a.item) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}

/// 🔧 Small `Option<T>` LWW-recursive-absorb helper (`None,x -> x; x,None -> x; Some,Some ->
/// Some(f(a,b))`) — factors out the same three-arm match repeated across every scalar-collection
/// pairing below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_opt<T>(a: Option<T>, b: Option<T>, f: impl FnOnce(T, T) -> T) -> Option<T> {
    match (a, b) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(f(x, y)),
    }
}
//#endregion 🔖️GenericNamedEngine
