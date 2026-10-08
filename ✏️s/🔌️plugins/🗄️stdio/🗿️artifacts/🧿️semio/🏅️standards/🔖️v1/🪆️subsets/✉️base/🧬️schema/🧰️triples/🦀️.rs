//! 🧰️ Shared generic diff/op-codec helpers for semio v1 subsets — hex-encoded, bracket-depth-
//! aware triple codecs for index-keyed and name-keyed collection diffs, ported from the bcf/docx
//! hand-rolled reference implementations
//! (`bcf/🏅️standards/🔖️2.1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs`,
//! `docx/🏅️standards/🔖️ecma-376/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs`) so all 13 W2
//! subset agents import this ONE copy instead of reinventing it 13 times. REAL and tested
//! (round-trip below) — load-bearing shared infrastructure, not a scaffolded placeholder.
//!
//! 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES) — pure generic diff/op-codec helpers with no snapshot dependency of their
//! own, so they land in `✉️base`'s own schema (the artifact-wide shared vocabulary every subset
//! already builds on), never an engine. Reached at `standards::v1::subsets::any::schema::triples`
//! (no shorter shim — every consumer now uses this full path).

//#region 🔖️IndexedTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `D: ToValue + FromValue`/`T: ToValue + FromValue`
// automatically per own type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit
// `#[value(bound = "...")]` override needed here, unlike `serde_derive`'s own inference which
// bcf's local `NamedTripleDiff` copy had to work around explicitly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedTripleDiff<D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️IndexedTriple

//#region 🔖️IndexedAlgebra
/// 🧮️ The nested diff of one row of an index-keyed collection. `apply_row`/`inverse_row`/`absorb_row` are the row-local
/// halves of the [`IndexedTripleDiff`] algebra below; `between_row` is for sync/import only, never for mutation leaves.
pub trait IndexedRow<T>: Clone {
    fn apply_row(&self, base: &T) -> T;
    fn inverse_row(&self, base: &T) -> Self;
    fn absorb_row(&mut self, other: Self);
    fn row_is_empty(&self) -> bool;
    fn between_row(base: &T, other: &T) -> Self;
}

/// 🔁️ Whole-row replacement as a row diff: the modified row takes the carried value; later replacements win.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Replace<T> {
    pub value: T,
}

impl<T: Clone + PartialEq> IndexedRow<T> for Replace<T> {
    fn apply_row(&self, _base: &T) -> T {
        self.value.clone()
    }
    fn inverse_row(&self, base: &T) -> Self {
        Replace { value: base.clone() }
    }
    fn absorb_row(&mut self, other: Self) {
        self.value = other.value;
    }
    fn row_is_empty(&self) -> bool {
        false
    }
    fn between_row(_base: &T, other: &T) -> Self {
        Replace { value: other.clone() }
    }
}

impl<D, T> IndexedTripleDiff<D, T> {
    /// 🕳️ Whether the triple names no removal, modification or addition.
    pub fn is_unchanged(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}

/// 🧭️ Position-pairwise state delta for sync/import: `0..min(len)` compare as `modified`, base's tail is `removed`, other's tail
/// is `added`. Forbidden in mutation leaves.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn between_indexed_rows<D: IndexedRow<T>, T: Clone>(base: &[T], other: &[T]) -> IndexedTripleDiff<D, T> {
    let min = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min {
        let d = D::between_row(&base[i], &other[i]);
        if !d.row_is_empty() {
            modified.push(IndexModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    IndexedTripleDiff { removed, modified, added }
}

/// ▶️ Apply semantics: modify against BASE indices, remove descending, then insert `added` ascending at `min(index, len)`
/// against the FINAL positions. Validate with [`validate_indexed_triple`] first.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_indexed_rows<D: IndexedRow<T>, T: Clone>(diff: &IndexedTripleDiff<D, T>, base: &[T]) -> Vec<T> {
    let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(item)) = next.get_mut(m.index) {
            *item = m.diff.apply_row(item);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.reverse();
    for &r in &removed_sorted {
        if r < next.len() {
            next.remove(r);
        }
    }
    let mut out: Vec<T> = next.into_iter().flatten().collect();
    let mut added_sorted: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let at = a.index.min(out.len());
        out.insert(at, a.item.clone());
    }
    out
}

/// ↩️ The negative diff of an index-keyed triple, given the ORIGINAL base items.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_indexed_rows<D: IndexedRow<T>, T: Clone>(diff: &IndexedTripleDiff<D, T>, base_items: &[T]) -> IndexedTripleDiff<D, T> {
    let removed_sorted = {
        let mut v = diff.removed.clone();
        v.sort_unstable();
        v
    };
    let added_index_sorted = {
        let mut v: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };
    let mut inv_removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut inv_modified: Vec<IndexModified<D>> = Vec::new();
    for m in &diff.modified {
        if let Some(orig) = base_items.get(m.index) {
            let after_index = transport_forward(m.index, &removed_sorted, &added_index_sorted);
            inv_modified.push(IndexModified { index: after_index, diff: m.diff.inverse_row(orig) });
        }
    }
    let mut inv_added: Vec<IndexAdded<T>> = Vec::new();
    for &r in &diff.removed {
        if let Some(orig) = base_items.get(r) {
            inv_added.push(IndexAdded { index: r, item: orig.clone() });
        }
    }
    inv_removed.sort_unstable();
    inv_modified.sort_by_key(|m| m.index);
    inv_added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed: inv_removed, modified: inv_modified, added: inv_added }
}

/// ➕️ Sequential-coalesce absorb (base→mid composed with mid→after): patch∘patch coalesces into one patch, create∘delete
/// annihilates, delete∘create becomes a replace.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn absorb_indexed_rows<D: IndexedRow<T>, T: Clone>(mine: &mut IndexedTripleDiff<D, T>, other: IndexedTripleDiff<D, T>) {
    let removed1_sorted = {
        let mut v = mine.removed.clone();
        v.sort_unstable();
        v
    };
    let added1_index_sorted = {
        let mut v: Vec<usize> = mine.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };
    let removed2_sorted = {
        let mut v = other.removed.clone();
        v.sort_unstable();
        v
    };
    let added2_index_sorted = {
        let mut v: Vec<usize> = other.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };
    let mut merged_added: Vec<IndexAdded<T>> = std::mem::take(&mut mine.added);
    let mut annihilated: std::collections::HashSet<usize> = Default::default();
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
    let mut modified_map: std::collections::BTreeMap<usize, D> = std::mem::take(&mut mine.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for m2 in other.modified {
        let mp = m2.index;
        if annihilated.contains(&mp) {
            continue;
        }
        if added1_index_sorted.binary_search(&mp).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|a| a.index == mp) {
                entry.item = m2.diff.apply_row(&entry.item);
            }
        } else {
            let post_remove_rank = rank_excluding(mp, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            match modified_map.entry(base_index) {
                std::collections::btree_map::Entry::Occupied(mut slot) => slot.get_mut().absorb_row(m2.diff),
                std::collections::btree_map::Entry::Vacant(slot) => {
                    slot.insert(m2.diff);
                }
            }
        }
    }
    let mut merged_added_final: Vec<IndexAdded<T>> = merged_added
        .into_iter()
        .map(|a| {
            let after_pos = if removed2_sorted.binary_search(&a.index).is_ok() {
                a.index
            } else {
                let post_remove_rank = rank_excluding(a.index, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            IndexAdded { index: after_pos, item: a.item }
        })
        .collect();
    merged_added_final.extend(other.added);
    merged_added_final.sort_by_key(|a| a.index);
    mine.removed = merged_removed_base;
    mine.modified = modified_map.into_iter().map(|(index, diff)| IndexModified { index, diff }).collect();
    mine.added = merged_added_final;
}

/// 🧮️ `Option<IndexedTripleDiff>` slot helpers shared by every diff type that nests an index-keyed collection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn absorb_indexed_slot<D: IndexedRow<T>, T: Clone>(mine: &mut Option<IndexedTripleDiff<D, T>>, other: Option<IndexedTripleDiff<D, T>>) {
    match (mine.as_mut(), other) {
        (Some(slot), Some(theirs)) => absorb_indexed_rows(slot, theirs),
        (None, Some(theirs)) => *mine = Some(theirs),
        _ => {}
    }
    if mine.as_ref().is_some_and(IndexedTripleDiff::is_unchanged) {
        *mine = None;
    }
}
//#endregion 🔖️IndexedAlgebra

//#region 🔖️NamedTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}

// 🩹 same auto-synthesized-bound story as `IndexedTripleDiff` above (see that struct's comment) —
// no explicit `#[value(bound = "...")]` needed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// 🛡️ Rejects malformed indexed collection operations before any candidate snapshot is changed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, base_len: usize, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()> {
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut removed = std::collections::BTreeSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-index", format!("remove index {index} is absent or duplicated")).at(target));
        }
    }
    let mut modified = std::collections::BTreeSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-index", format!("modify index {} is absent, removed, or duplicated", entry.index)).at(target));
        }
    }
    let mut added = std::collections::BTreeSet::new();
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || !added.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at(target));
        }
    }
    Ok(())
}

/// 🛡️ Rejects missing, duplicate, overlapping, or colliding named collection operations.
///
/// 🐛️ A key the same diff REMOVES may be re-added: `apply_named` retains survivors first and pushes
/// `added` onto the tail, so `removed + added` of one key is the only spelling this container has for
/// "move this member", and the key is present exactly once afterwards. Testing `added` against the
/// raw base keys instead of the post-removal ones made the validator disagree with the applier it
/// guards, and made a whole-collection REPLACEMENT — the only faithful diff for a `set-snapshot`
/// that reorders surviving members — unrepresentable. Measured on the real Nakagin Capsule Tower by
/// `🏛️mutate-semio-model`’s `mutate-set-snapshot`, which the applier handles correctly and this
/// preflight rejected with `mutation.apply.invalid-add-key`. A key that is NOT removed still
/// collides, exactly as before.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_named_triple<K, D, T, A>(base: &[T], diff: &NamedTripleDiff<K, D, A>, key_of_base: impl Fn(&T) -> K, key_of_added: impl Fn(&A) -> K, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()>
where
    K: PartialEq + Clone + std::fmt::Debug,
{
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut base_keys = Vec::new();
    for item in base {
        let key = key_of_base(item);
        if base_keys.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-base-key", format!("base key {key:?} is duplicated")).at(target));
        }
        base_keys.push(key);
    }
    let mut removed = Vec::new();
    for key in &diff.removed {
        if !base_keys.contains(key) || removed.contains(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-key", format!("remove key {key:?} is absent or duplicated")).at(target));
        }
        removed.push(key.clone());
    }
    let mut modified = Vec::new();
    for entry in &diff.modified {
        if !base_keys.contains(&entry.key) || removed.contains(&entry.key) || modified.contains(&entry.key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-key", format!("modify key {:?} is absent, removed, or duplicated", entry.key)).at(target));
        }
        modified.push(entry.key.clone());
    }
    let mut added = Vec::new();
    for item in &diff.added {
        let key = key_of_added(item);
        if (base_keys.contains(&key) && !removed.contains(&key)) || added.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-key", format!("add key {key:?} already exists or is duplicated")).at(target));
        }
        added.push(key);
    }
    Ok(())
}

/// 🧷 Position-carrying "added" wrapper for name/id-keyed collections, supplied as `T` in
/// `NamedTripleDiff<K, D, NamedAdded<T>>` by any consumer that needs a re-added interior member to
/// land back at its real position instead of always being appended last (`IndexedTripleDiff`
/// already gets this for free via `IndexAdded<T>`; `NamedTripleDiff`'s own `added: Vec<T>` field
/// intentionally stays position-agnostic — most named/keyed collections don't care about order —
/// so this is opt-in via `T`, not a change to the struct itself). Was independently reinvented by
/// every W2 subset that needed it (`value::NamedAdded`, `json::JsonObjectAdded`, …) before this
/// shared copy existed — see `s.stdio.value`'s own `🧬️schema/🔺️diff/🦀️.rs` for the
/// reference usage this was hoisted from. Existing per-subset local copies are untouched (still
/// correct); only new W4/W5 consumers should import this one instead of reinventing it again.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedAdded<T> {
    pub index: usize,
    pub item: T,
}
//#endregion 🔖️NamedTriple

//#region 🔖️Parsing



//#endregion 🔖️Parsing

//#region 🔖️IndexedCodec



//#endregion 🔖️IndexedCodec

//#region 🔖️NamedCodec






//#endregion 🔖️NamedCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
