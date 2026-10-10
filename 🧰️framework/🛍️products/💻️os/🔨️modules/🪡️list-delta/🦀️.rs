//! 🪡️ Positional list deltas — the sparse, typed, order-aware difference a mutation raises for an ordered list of identified rows.
//! A delta never carries an `order`/`reordered` id list and no `after` anchors: every ordered change is a positional row, and every
//! index is a COORDINATE of the base or of the after list, never an evolving position.
//!
//! - `removed: [{id, index}]` — `index` is the row's position in the BASE list (what the inverse reinserts at);
//! - `inserted: [{index, row}]` — `index` is the row's position in the AFTER list;
//! - `moved: [{id, from, to}]` — `from` is the BASE position, `to` the AFTER position;
//! - `modified: [{id, patch}]` — a sparse typed patch keyed by id (position-free).
//!
//! 🧭️ Committing a delta onto a base list (under the central applier's [`ApplyCapability`]): every removed and moved `id` is checked at
//! its base index; the after list has `|base| − removed + inserted` slots; inserted rows and moved rows take their after slots; every
//! other base row (an unmoved survivor) fills the remaining slots in base order; then the patches write. A key may be removed and
//! inserted again (a replacement).
//!
//! 🔁️ `inverse(base)` reads the base row by row — removed ← inserted rows at their after index, inserted ← removed rows read from the
//! base at their base index, moved ← `(id, to, from)`, modified ← each patch's own inverse over the base row — it never applies or
//! simulates the delta. `absorb` is base-free and coalesces per id with pure index arithmetic over the two deltas' coordinate sets:
//! insert∘remove → nothing, insert∘move → insert at the final slot, move∘move → one move, move∘remove → remove at the base index,
//! remove∘insert (same id) → replacement, patch∘patch → one patch, patch∘remove → dropped; the patch of an inserted row stays its own
//! `modified` entry, because folding it into the row would apply a diff outside the central applier.
//!
//! 🧬️ [`list_delta!`]/[`plain_list_delta!`] define the concrete wire types of one list and [`row_patch!`] the concrete sparse patch of a
//! row type; the generic algebra lives once in [`Parts`].
//!
//! @see ./🧪️tests/🔬️unit/🦀️.rs — the randomized sequence laws

use crate::{ApplyCapability, MutationApplyError};
use semio_framework_value::list::PagedList;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// 🔑️ A row with a stable identity inside its list.
pub trait Keyed {
    /// 🏷️ The identity type, unique within one list.
    type Key: Clone + Eq + Hash + std::fmt::Debug;
    /// 🏷️ The row's key.
    fn key(&self) -> Self::Key;
    /// 🧊️ Disposes a row the algebra displaced or dropped. The default is a plain drop; a row that owns a fail-closed root (it aborts
    /// on a bare drop) overrides it with its cold-disposal call.
    fn retire_cold(self)
    where
        Self: Sized,
    {
    }
}

/// 🔑️ The key extractor and disposal of a list whose rows are FOREIGN types that cannot implement [`Keyed`] here (the orphan rule): a
/// local marker type implements `KeyOf<Row>` and names itself in the macro as `key: Key = by Marker`. A marker must derive
/// `Clone, Debug, PartialEq`. [`ByKeyed`] is the marker of every row that implements [`Keyed`].
pub trait KeyOf<R> {
    /// 🏷️ The identity type, unique within one list.
    type Key: Clone + Eq + Hash + std::fmt::Debug;
    /// 🏷️ The key of `row`.
    fn key_of(row: &R) -> Self::Key;
    /// 🧊️ Disposes a row the algebra displaced or dropped; the default is a plain drop.
    fn retire_cold(row: R) {
        drop(row);
    }
}

/// 🔑️ The marker of every row type that implements [`Keyed`]; it is the default of [`Parts`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ByKeyed;

impl<R: Keyed> KeyOf<R> for ByKeyed {
    type Key = R::Key;
    fn key_of(row: &R) -> R::Key {
        row.key()
    }
    fn retire_cold(row: R) {
        Keyed::retire_cold(row);
    }
}

/// 🏷️ The key type of the list rows `R` under the marker `K`.
pub type KeyOfRow<R, K> = <K as KeyOf<R>>::Key;

/// 🔑️ A `(key, value)` pair is keyed by its first member: the rows of an ordered map kept as a list.
impl<K: Clone + Eq + Hash + std::fmt::Debug, V> Keyed for (K, V) {
    type Key = K;
    fn key(&self) -> K {
        self.0.clone()
    }
}

/// 🗂️ An ordered list of rows the delta can read by position.
pub trait ItemList<R> {
    /// 🔢️ The number of rows.
    fn count(&self) -> usize;
    /// 👁️ The row at `index`.
    fn at(&self, index: usize) -> Option<&R>;
    /// 📋️ The rows, cloned in order.
    fn to_rows(&self) -> Vec<R>;
}

/// 🏗️ An [`ItemList`] the delta can rebuild.
pub trait BuildList<R>: ItemList<R> + Sized {
    /// 🏗️ A list holding `rows` in order.
    fn from_rows(rows: Vec<R>) -> Self;
    /// 📤️ The rows, moved out in order — the way a displaced list is handed to the cold-disposal hook row by row.
    fn into_rows(self) -> Vec<R>;
}

impl<R: Clone> ItemList<R> for [R] {
    fn count(&self) -> usize {
        self.len()
    }
    fn at(&self, index: usize) -> Option<&R> {
        self.get(index)
    }
    fn to_rows(&self) -> Vec<R> {
        self.to_vec()
    }
}

impl<R: Clone> ItemList<R> for Vec<R> {
    fn count(&self) -> usize {
        self.len()
    }
    fn at(&self, index: usize) -> Option<&R> {
        self.get(index)
    }
    fn to_rows(&self) -> Vec<R> {
        self.clone()
    }
}

impl<R: Clone> BuildList<R> for Vec<R> {
    fn from_rows(rows: Vec<R>) -> Self {
        rows
    }
    fn into_rows(self) -> Vec<R> {
        self
    }
}

impl<R: Clone> ItemList<R> for PagedList<R, { usize::MAX }> {
    fn count(&self) -> usize {
        self.len()
    }
    fn at(&self, index: usize) -> Option<&R> {
        self.get(index)
    }
    fn to_rows(&self) -> Vec<R> {
        self.iter().cloned().collect()
    }
}

impl<R: Clone> BuildList<R> for PagedList<R, { usize::MAX }> {
    fn from_rows(rows: Vec<R>) -> Self {
        let mut list = Self::new();
        for row in rows {
            list.push(row);
        }
        list
    }
    fn into_rows(mut self) -> Vec<R> {
        let mut rows = Vec::with_capacity(self.len());
        while let Some(row) = self.pop() {
            rows.push(row);
        }
        rows.reverse();
        rows
    }
}

/// 🩹 The sparse field patch of one row type: every field it carries is an absolute setter.
pub trait RowPatch<R>: Clone + PartialEq {
    /// ✍️ Writes the patch into the row it was raised for; reached only through [`Parts::commit_onto`] under the capability.
    fn commit_into(&self, row: &mut R, capability: ApplyCapability) -> Result<(), MutationApplyError>;
    /// ➕️ Composes `self` (row→mid) with `later` (mid→after) into row→after.
    fn absorb(&mut self, later: Self);
    /// 🔁️ The patch that, applied after `self`, restores `row` — the absolute pre-values of every field `self` carries.
    fn inverse(&self, row: &R) -> Self;
    /// 🕳️ Whether the patch carries nothing.
    fn is_empty(&self) -> bool;
    /// 🧊️ Disposes a patch the algebra displaced or dropped; the default is a plain drop, a patch that owns a fail-closed row overrides it.
    fn retire_cold(self)
    where
        Self: Sized,
    {
    }
}

/// 🕳️ The patch of a list whose rows are never patched in place (a row is replaced by removing and inserting it).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NoPatch;

impl<R> RowPatch<R> for NoPatch {
    fn commit_into(&self, _row: &mut R, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        Ok(())
    }
    fn absorb(&mut self, _later: Self) {}
    fn inverse(&self, _row: &R) -> Self {
        Self
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl Keyed for String {
    type Key = String;
    fn key(&self) -> String {
        self.clone()
    }
}

/// 🧱️ The plain form of a positional list delta; the concrete wire types from [`list_delta!`] convert to and from it. `K` extracts the
/// row key and disposes displaced rows ([`ByKeyed`] for every [`Keyed`] row).
#[derive(Clone, Debug, PartialEq)]
pub struct Parts<R, Q, K: KeyOf<R> = ByKeyed> {
    /// ➖️ `(id, base index)` of the rows that leave.
    pub removed: Vec<(KeyOfRow<R, K>, usize)>,
    /// ➕️ `(after index, row)` of the rows that enter.
    pub inserted: Vec<(usize, R)>,
    /// ↕️ `(id, base index, after index)` of the rows that change position.
    pub moved: Vec<(KeyOfRow<R, K>, usize, usize)>,
    /// 🩹 One patch per id.
    pub modified: Vec<(KeyOfRow<R, K>, Q)>,
}

impl<R, Q, K: KeyOf<R>> Default for Parts<R, Q, K> {
    fn default() -> Self {
        Self { removed: Vec::new(), inserted: Vec::new(), moved: Vec::new(), modified: Vec::new() }
    }
}

fn refusal(code: &'static str, message: &'static str, section: &'static str, index: usize) -> MutationApplyError {
    MutationApplyError::new(code, message).at([section.to_string(), index.to_string()])
}

/// 🔢️ The position of `index` once every coordinate in `excluded` is skipped.
fn rank(excluded: &[usize], index: usize) -> usize {
    index - excluded.iter().filter(|skipped| **skipped < index).count()
}

/// 🔢️ The `nth` (0-based) coordinate that is not in `excluded`.
fn nth_free(excluded: &[usize], nth: usize) -> usize {
    let mut seen = 0;
    let mut candidate = 0;
    loop {
        if !excluded.contains(&candidate) {
            if seen == nth {
                return candidate;
            }
            seen += 1;
        }
        candidate += 1;
    }
}

/// 🧊️ Rows in flight inside one algebra step. Whatever is still held when the step ends — a removed row, a clone built for a step that
/// then refused — is disposed through `K::retire_cold`, never by a bare drop, because a row may own a fail-closed root.
struct Held<R, K: KeyOf<R>>(Vec<Option<R>>, std::marker::PhantomData<fn() -> K>);

impl<R, K: KeyOf<R>> Held<R, K> {
    fn new(rows: Vec<Option<R>>) -> Self {
        Self(rows, std::marker::PhantomData)
    }
}

impl<R, K: KeyOf<R>> Drop for Held<R, K> {
    fn drop(&mut self) {
        std::mem::take(&mut self.0).into_iter().flatten().for_each(K::retire_cold);
    }
}

impl<R: Clone + PartialEq, Q: RowPatch<R>, K: KeyOf<R>> Parts<R, Q, K> {
    /// ➕️ The delta that inserts `row` at `index` of the after list.
    pub fn insertion(index: usize, row: R) -> Self {
        Self { inserted: vec![(index, row)], ..Self::default() }
    }

    /// ➖️ The delta that removes the row at `index` of `base` (empty when `base` has no such row).
    pub fn removal<L: ItemList<R> + ?Sized>(base: &L, index: usize) -> Self {
        Self::removals(base, &[index])
    }

    /// ➖️ The delta that removes the rows at `indices` of `base` (indices past the end are skipped).
    pub fn removals<L: ItemList<R> + ?Sized>(base: &L, indices: &[usize]) -> Self {
        Self { removed: indices.iter().filter_map(|index| base.at(*index).map(|row| (K::key_of(row), *index))).collect(), ..Self::default() }
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: KeyOfRow<R, K>, index: usize) -> Self {
        Self { removed: vec![(id, index)], ..Self::default() }
    }

    /// ➖️ The delta that removes several rows, each named with its base index.
    pub fn removals_by_id(rows: Vec<(KeyOfRow<R, K>, usize)>) -> Self {
        Self { removed: rows, ..Self::default() }
    }

    /// ↕️ The delta that moves the row at `from` of `base` to `to` of the after list (empty when `base` has no such row).
    pub fn relocation<L: ItemList<R> + ?Sized>(base: &L, from: usize, to: usize) -> Self {
        Self { moved: base.at(from).map(|row| (K::key_of(row), from, to)).into_iter().collect(), ..Self::default() }
    }

    /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
    pub fn relocation_by_id(id: KeyOfRow<R, K>, from: usize, to: usize) -> Self {
        Self { moved: vec![(id, from, to)], ..Self::default() }
    }

    /// 🩹 The delta that patches the row `id`.
    pub fn modification(id: KeyOfRow<R, K>, patch: Q) -> Self {
        Self { modified: vec![(id, patch)], ..Self::default() }
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|(_, patch)| patch.is_empty())
    }

    /// 🧊️ Disposes every row and patch the delta owns through the cold-disposal hooks.
    pub fn retire_cold(self) {
        self.inserted.into_iter().for_each(|(_, row)| K::retire_cold(row));
        self.modified.into_iter().for_each(|(_, patch)| patch.retire_cold());
    }

    /// ✍️ The list this delta turns `base` into; reached only from a diff type's own `apply`, under the central applier's capability.
    /// Every row the step displaces or abandons (removed rows, the clones of a refused step) is disposed through `K::retire_cold`.
    pub fn commit_onto<L: BuildList<R>>(&self, base: &L, capability: ApplyCapability) -> Result<L, MutationApplyError> {
        let mut rows = Held::<R, K>::new(base.to_rows().into_iter().map(Some).collect());
        let mut taken = vec![false; rows.0.len()];
        for (index, (id, at)) in self.removed.iter().enumerate() {
            match rows.0.get(*at).and_then(Option::as_ref) {
                Some(row) if K::key_of(row) == *id && !taken[*at] => taken[*at] = true,
                Some(row) if K::key_of(row) == *id => return Err(refusal("mutation.apply.duplicate-target", "removed row is named twice", "removed", index)),
                _ => return Err(refusal("mutation.apply.missing-target", "removed row is not at its base index", "removed", index)),
            }
        }
        for (index, (id, from, _)) in self.moved.iter().enumerate() {
            match rows.0.get(*from).and_then(Option::as_ref) {
                Some(row) if K::key_of(row) == *id && !taken[*from] => taken[*from] = true,
                Some(row) if K::key_of(row) == *id => return Err(refusal("mutation.apply.duplicate-target", "moved row is named twice", "moved", index)),
                _ => return Err(refusal("mutation.apply.missing-target", "moved row is not at its base index", "moved", index)),
            }
        }
        let after_len = rows.0.len() - self.removed.len() + self.inserted.len();
        let mut slots = Held::<R, K>::new((0..after_len).map(|_| None).collect());
        for (index, (at, row)) in self.inserted.iter().enumerate() {
            match slots.0.get_mut(*at) {
                None => return Err(refusal("mutation.apply.invalid-add-index", "inserted row lies past the end of the after list", "inserted", index)),
                Some(slot) if slot.is_some() => return Err(refusal("mutation.apply.duplicate-target", "two rows take the same after index", "inserted", index)),
                Some(slot) => *slot = Some(row.clone()),
            }
        }
        for (index, (_, from, to)) in self.moved.iter().enumerate() {
            match slots.0.get_mut(*to) {
                None => return Err(refusal("mutation.apply.invalid-move-index", "moved row lies past the end of the after list", "moved", index)),
                Some(slot) if slot.is_some() => return Err(refusal("mutation.apply.duplicate-target", "two rows take the same after index", "moved", index)),
                Some(slot) => *slot = rows.0[*from].take(),
            }
        }
        let mut survivors = Held::<R, K>::new(rows.0.iter_mut().zip(&taken).map(|(row, gone)| if *gone { None } else { row.take() }).collect());
        let mut next_survivor = 0usize;
        for slot in slots.0.iter_mut().filter(|slot| slot.is_none()) {
            while survivors.0.get(next_survivor).is_some_and(Option::is_none) {
                next_survivor += 1;
            }
            *slot = survivors.0.get_mut(next_survivor).and_then(Option::take);
            if slot.is_none() {
                return Err(refusal("mutation.apply.missing-target", "the after list has more free slots than surviving rows", "inserted", 0));
            }
        }
        let mut after = Held::<R, K>::new(std::mem::take(&mut slots.0));
        let mut positions: HashMap<KeyOfRow<R, K>, usize> = HashMap::with_capacity(after.0.len());
        for (index, row) in after.0.iter().flatten().enumerate() {
            if positions.insert(K::key_of(row), index).is_some() {
                return Err(refusal("mutation.apply.duplicate-target", "two rows of the after list carry the same key", "inserted", index));
            }
        }
        for (index, (id, patch)) in self.modified.iter().enumerate() {
            let at = *positions.get(id).ok_or_else(|| refusal("mutation.apply.missing-target", "modified row does not exist", "modified", index))?;
            let row = after.0[at].as_mut().expect("a slot of the after list holds its row");
            patch.commit_into(row, capability).map_err(|error| error.under(["modified".to_string(), index.to_string()]))?;
        }
        Ok(L::from_rows(std::mem::take(&mut after.0).into_iter().flatten().collect()))
    }

    /// 🔁️ The negative delta: applied to what `self` makes of `base`, it restores `base`. Read row by row from `base`; nothing is simulated.
    pub fn inverse<L: ItemList<R> + ?Sized>(&self, base: &L) -> Self {
        let removed = self.inserted.iter().map(|(at, row)| (K::key_of(row), *at)).collect();
        let inserted = self.removed.iter().filter_map(|(_, at)| base.at(*at).map(|row| (*at, row.clone()))).collect();
        let moved = self.moved.iter().map(|(id, from, to)| (id.clone(), *to, *from)).collect();
        let created: HashSet<KeyOfRow<R, K>> = self.inserted.iter().map(|(_, row)| K::key_of(row)).collect();
        let modified = self
            .modified
            .iter()
            .filter(|(id, _)| !created.contains(id))
            .filter_map(|(id, patch)| (0..base.count()).filter_map(|at| base.at(at)).find(|row| K::key_of(row) == *id).map(|row| (id.clone(), patch.inverse(row))))
            .collect();
        Self { removed, inserted, moved, modified }
    }

    /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after, in place. A row or patch the composition cancels is
    /// disposed through the cold-disposal hooks.
    pub fn absorb(&mut self, later: Self) {
        let earlier = std::mem::take(self);
        let mut occupied_mid: Vec<usize> = earlier.inserted.iter().map(|(at, _)| *at).chain(earlier.moved.iter().map(|(_, _, to)| *to)).collect();
        occupied_mid.sort_unstable();
        let mut left_base: Vec<usize> = earlier.removed.iter().map(|(_, at)| *at).chain(earlier.moved.iter().map(|(_, from, _)| *from)).collect();
        left_base.sort_unstable();
        let mut left_mid: Vec<usize> = later.removed.iter().map(|(_, at)| *at).chain(later.moved.iter().map(|(_, from, _)| *from)).collect();
        left_mid.sort_unstable();
        let mut occupied_after: Vec<usize> = later.inserted.iter().map(|(at, _)| *at).chain(later.moved.iter().map(|(_, _, to)| *to)).collect();
        occupied_after.sort_unstable();
        let after_of_mid = |mid: usize| nth_free(&occupied_after, rank(&left_mid, mid));
        let base_of_mid = |mid: usize| nth_free(&left_base, rank(&occupied_mid, mid));
        let first_inserted: HashSet<KeyOfRow<R, K>> = earlier.inserted.iter().map(|(_, row)| K::key_of(row)).collect();
        let first_moved: HashSet<KeyOfRow<R, K>> = earlier.moved.iter().map(|(id, _, _)| id.clone()).collect();
        let removed_later: HashSet<KeyOfRow<R, K>> = later.removed.iter().map(|(id, _)| id.clone()).collect();
        let moved_later: HashMap<KeyOfRow<R, K>, usize> = later.moved.iter().map(|(id, _, to)| (id.clone(), *to)).collect();

        let mut removed = earlier.removed;
        let mut inserted: Vec<(usize, R)> = Vec::new();
        let mut moved: Vec<(KeyOfRow<R, K>, usize, usize)> = Vec::new();
        for (at, row) in earlier.inserted {
            let key = K::key_of(&row);
            if removed_later.contains(&key) {
                K::retire_cold(row);
                continue;
            }
            inserted.push((moved_later.get(&key).copied().unwrap_or_else(|| after_of_mid(at)), row));
        }
        for (id, from, to) in earlier.moved {
            if removed_later.contains(&id) {
                removed.push((id, from));
            } else {
                let target = moved_later.get(&id).copied().unwrap_or_else(|| after_of_mid(to));
                moved.push((id, from, target));
            }
        }
        for (id, mid) in &later.removed {
            if !first_inserted.contains(id) && !first_moved.contains(id) {
                removed.push((id.clone(), base_of_mid(*mid)));
            }
        }
        for (id, mid, to) in &later.moved {
            if !first_inserted.contains(id) && !first_moved.contains(id) {
                moved.push((id.clone(), base_of_mid(*mid), *to));
            }
        }
        inserted.extend(later.inserted);

        let mut modified: Vec<(KeyOfRow<R, K>, Q)> = Vec::new();
        for (id, patch) in earlier.modified {
            if removed_later.contains(&id) {
                patch.retire_cold();
            } else {
                modified.push((id, patch));
            }
        }
        for (id, patch) in later.modified {
            match modified.iter_mut().find(|(existing, _)| *existing == id) {
                Some((_, existing)) => existing.absorb(patch),
                None => modified.push((id, patch)),
            }
        }
        *self = Self { removed, inserted, moved, modified };
    }
}

/// 🚨️ The refusal a delta that cannot apply raises.
pub type ApplyError = MutationApplyError;

/// 🧬️ Defines the concrete wire types of one positional list delta: `$delta` (the list), `$removal` (`{id, index}`), `$insertion`
/// (`{index, row}`), `$relocation` (`{id, from, to}`) and `$modification` (`{id, patch}`), plus the [`Keyed`] key of `$row`. Three key
/// spellings: `key: id` (a `String` field of the row, the list a `Vec<$row>`), `list: $list, key: $key_ty = |row| expr`, or
/// `list: $list, key: $key_ty = keyed` for a row whose [`Keyed`] impl exists already (a `(key, value)` pair, or a row shared by several deltas), or
/// `list: $list, key: $key_ty = by Marker` for a FOREIGN row type that cannot implement [`Keyed`] here: the local `Marker` implements [`KeyOf`] (key extractor and
/// cold-disposal hook). Each spelling takes a trailing `, values_only` to skip the `DslRecord` derive;
/// the `by Marker` spelling takes `, native` to leave all representation implementations to an IO owner. Extra
/// derives and attributes go on every generated type through the leading `#[…]` attributes. The generic algebra is [`Parts`]; the
/// methods here only convert.
#[macro_export]
macro_rules! list_delta {
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = by $keyer:ty , native $(,)? }) => {
        $crate::list_delta! { @native_types [] [] [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $keyer } }
    };

    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, key: $key:ident $(,)? }) => {
        $crate::list_delta! { $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: Vec<$row>, key: String = |row| row.$key.clone() } }
    };
    (@types [$($dsl:path),*] $(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty, keyer: $keyer:ty $(,)? }) => {
        $crate::list_delta! { @native_types [$crate::__value_derive::ToValue, $crate::__value_derive::FromValue $(, $dsl)*] [#[value(rename_all = "camelCase")]] [#[value(rename_all = "camelCase", default)]] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $keyer } }
    };
    (@native_types [$($codec:path),*] [$($record_attrs:tt)*] [$($delta_attrs:tt)*] $(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty, keyer: $keyer:ty $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq $(, $codec)*)]
        $($record_attrs)*
        $vis struct $removal {
            pub id: $key_ty,
            pub index: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq $(, $codec)*)]
        $($record_attrs)*
        $vis struct $insertion {
            pub index: usize,
            pub row: $row,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq $(, $codec)*)]
        $($record_attrs)*
        $vis struct $relocation {
            pub id: $key_ty,
            pub from: usize,
            pub to: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq $(, $codec)*)]
        $($record_attrs)*
        $vis struct $modification {
            pub id: $key_ty,
            pub patch: $patch,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq $(, $codec)*)]
        $($delta_attrs)*
        $vis struct $delta {
            pub removed: Vec<$removal>,
            pub inserted: Vec<$insertion>,
            pub moved: Vec<$relocation>,
            pub modified: Vec<$modification>,
        }

        impl $delta {
            fn into_parts(self) -> $crate::list_delta::Parts<$row, $patch, $keyer> {
                $crate::list_delta::Parts {
                    removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
                    inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
                    moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
                    modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
                }
            }

            fn from_parts(parts: $crate::list_delta::Parts<$row, $patch, $keyer>) -> Self {
                Self {
                    removed: parts.removed.into_iter().map(|(id, index)| $removal { id, index }).collect(),
                    inserted: parts.inserted.into_iter().map(|(index, row)| $insertion { index, row }).collect(),
                    moved: parts.moved.into_iter().map(|(id, from, to)| $relocation { id, from, to }).collect(),
                    modified: parts.modified.into_iter().map(|(id, patch)| $modification { id, patch }).collect(),
                }
            }

            /// ➕️ The delta that inserts `row` at `index` of the after list.
            pub fn insertion(index: usize, row: $row) -> Self {
                Self::from_parts($crate::list_delta::Parts::insertion(index, row))
            }

            /// ➖️ The delta that removes the row at `index` of `base`.
            pub fn removal<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, index: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::removal(base, index))
            }

            /// ➖️ The delta that removes the rows at `indices` of `base`.
            pub fn removals<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, indices: &[usize]) -> Self {
                Self::from_parts($crate::list_delta::Parts::removals(base, indices))
            }

            /// ➖️ The delta that removes the row `id` found at `index` of the base list.
            pub fn removal_by_id(id: impl Into<$key_ty>, index: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::removal_by_id(id.into(), index))
            }

            /// ➖️ The delta that removes several rows, each named with its base index.
            pub fn removals_by_id(rows: Vec<($key_ty, usize)>) -> Self {
                Self::from_parts($crate::list_delta::Parts::removals_by_id(rows))
            }

            /// ↕️ The delta that moves the row at `from` of `base` to `to` of the after list.
            pub fn relocation<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, from: usize, to: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::relocation(base, from, to))
            }

            /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
            pub fn relocation_by_id(id: impl Into<$key_ty>, from: usize, to: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::relocation_by_id(id.into(), from, to))
            }

            /// 🩹 The delta that patches the row `id`.
            pub fn modification(id: impl Into<$key_ty>, patch: $patch) -> Self {
                Self::from_parts($crate::list_delta::Parts::modification(id.into(), patch))
            }

            /// ✍️ The list this delta turns `base` into; reached only from a diff type's own `apply`, under the central applier's capability.
            pub fn commit_onto(&self, base: &$list, capability: $crate::ApplyCapability) -> Result<$list, $crate::list_delta::ApplyError> {
                let parts = self.clone().into_parts();
                let committed = parts.commit_onto(base, capability);
                parts.retire_cold();
                committed
            }

            /// ➕️ Composes `self` with the delta `later` applied after it.
            pub fn absorb(&mut self, later: Self) {
                let mut parts = std::mem::take(self).into_parts();
                parts.absorb(later.into_parts());
                *self = Self::from_parts(parts);
            }

            /// 🔁️ The negative delta over `base`, read row by row.
            pub fn inverse(&self, base: &$list) -> Self {
                let parts = self.clone().into_parts();
                let inverse = parts.inverse(base);
                parts.retire_cold();
                Self::from_parts(inverse)
            }

            /// 🧊️ Disposes every row and patch the delta owns through the cold-disposal hooks of its key marker.
            pub fn retire_cold(self) {
                self.into_parts().retire_cold();
            }

            /// 🧊️ Disposes a displaced list row by row through the cold-disposal hook of its key marker.
            pub fn retire_list(list: $list) {
                $crate::list_delta::BuildList::<$row>::into_rows(list).into_iter().for_each(<$keyer as $crate::list_delta::KeyOf<$row>>::retire_cold);
            }

            /// 🕳️ Whether the delta changes nothing.
            pub fn is_empty(&self) -> bool {
                self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| $crate::list_delta::RowPatch::<$row>::is_empty(&entry.patch))
            }
        }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = |$item:ident| $key:expr $(,)? }) => {
        impl $crate::list_delta::Keyed for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                let $item = self;
                $key
            }
        }

        $crate::list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = keyed $(,)? }) => {
        $crate::list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = by $keyer:ty $(,)? }) => {
        $crate::list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $keyer } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = by $keyer:ty , values_only $(,)? }) => {
        $crate::list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $keyer } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, key: $key:ident , values_only $(,)? }) => {
        $crate::list_delta! { $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: Vec<$row>, key: String = |row| row.$key.clone(), values_only } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = |$item:ident| $key:expr , values_only $(,)? }) => {
        impl $crate::list_delta::Keyed for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                let $item = self;
                $key
            }
        }

        $crate::list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = keyed , values_only $(,)? }) => {
        $crate::list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
}

/// 🧬️ [`list_delta!`] for a list whose rows are never patched in place (a tag, a reference, a plain value): the delta has only
/// `removed`, `inserted` and `moved`. Same three key spellings as [`list_delta!`], without `modification`/`patch`.
#[macro_export]
macro_rules! plain_list_delta {
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, key: $key:ident $(,)? }) => {
        $crate::plain_list_delta! { $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: Vec<$row>, key: String = |row| row.$key.clone() } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: String $(,)? }) => {
        $crate::plain_list_delta! { @wire [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: String, list: Vec<String>, key: String, keyer: $crate::list_delta::ByKeyed } }
    };
    (@types [$($dsl:path),*] $(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty, keyer: $keyer:ty $(,)? }) => {
        $crate::plain_list_delta! { @wire [$($dsl),*] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $keyer } }
    };
    (@wire [$($dsl:path),*] $(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty, keyer: $keyer:ty $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue $(, $dsl)*)]
        #[value(rename_all = "camelCase")]
        $vis struct $removal {
            pub id: $key_ty,
            pub index: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue $(, $dsl)*)]
        #[value(rename_all = "camelCase")]
        $vis struct $insertion {
            pub index: usize,
            pub row: $row,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue $(, $dsl)*)]
        #[value(rename_all = "camelCase")]
        $vis struct $relocation {
            pub id: $key_ty,
            pub from: usize,
            pub to: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue $(, $dsl)*)]
        #[value(rename_all = "camelCase", default)]
        $vis struct $delta {
            pub removed: Vec<$removal>,
            pub inserted: Vec<$insertion>,
            pub moved: Vec<$relocation>,
        }

        impl $delta {
            fn into_parts(self) -> $crate::list_delta::Parts<$row, $crate::list_delta::NoPatch, $keyer> {
                $crate::list_delta::Parts {
                    removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
                    inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
                    moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
                    modified: Vec::new(),
                }
            }

            fn from_parts(parts: $crate::list_delta::Parts<$row, $crate::list_delta::NoPatch, $keyer>) -> Self {
                Self {
                    removed: parts.removed.into_iter().map(|(id, index)| $removal { id, index }).collect(),
                    inserted: parts.inserted.into_iter().map(|(index, row)| $insertion { index, row }).collect(),
                    moved: parts.moved.into_iter().map(|(id, from, to)| $relocation { id, from, to }).collect(),
                }
            }

            /// ➕️ The delta that inserts `row` at `index` of the after list.
            pub fn insertion(index: usize, row: $row) -> Self {
                Self::from_parts($crate::list_delta::Parts::insertion(index, row))
            }

            /// ➖️ The delta that removes the row at `index` of `base`.
            pub fn removal<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, index: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::removal(base, index))
            }

            /// ➖️ The delta that removes the rows at `indices` of `base`.
            pub fn removals<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, indices: &[usize]) -> Self {
                Self::from_parts($crate::list_delta::Parts::removals(base, indices))
            }

            /// ➖️ The delta that removes the row `id` found at `index` of the base list.
            pub fn removal_by_id(id: impl Into<$key_ty>, index: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::removal_by_id(id.into(), index))
            }

            /// ➖️ The delta that removes several rows, each named with its base index.
            pub fn removals_by_id(rows: Vec<($key_ty, usize)>) -> Self {
                Self::from_parts($crate::list_delta::Parts::removals_by_id(rows))
            }

            /// ↕️ The delta that moves the row at `from` of `base` to `to` of the after list.
            pub fn relocation<L: $crate::list_delta::ItemList<$row> + ?Sized>(base: &L, from: usize, to: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::relocation(base, from, to))
            }

            /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
            pub fn relocation_by_id(id: impl Into<$key_ty>, from: usize, to: usize) -> Self {
                Self::from_parts($crate::list_delta::Parts::relocation_by_id(id.into(), from, to))
            }

            /// ✍️ The list this delta turns `base` into; reached only from a diff type's own `apply`, under the central applier's capability.
            pub fn commit_onto(&self, base: &$list, capability: $crate::ApplyCapability) -> Result<$list, $crate::list_delta::ApplyError> {
                let parts = self.clone().into_parts();
                let committed = parts.commit_onto(base, capability);
                parts.retire_cold();
                committed
            }

            /// ➕️ Composes `self` with the delta `later` applied after it.
            pub fn absorb(&mut self, later: Self) {
                let mut parts = std::mem::take(self).into_parts();
                parts.absorb(later.into_parts());
                *self = Self::from_parts(parts);
            }

            /// 🔁️ The negative delta over `base`, read row by row.
            pub fn inverse(&self, base: &$list) -> Self {
                let parts = self.clone().into_parts();
                let inverse = parts.inverse(base);
                parts.retire_cold();
                Self::from_parts(inverse)
            }

            /// 🧊️ Disposes every row and patch the delta owns through the cold-disposal hooks of its key marker.
            pub fn retire_cold(self) {
                self.into_parts().retire_cold();
            }

            /// 🧊️ Disposes a displaced list row by row through the cold-disposal hook of its key marker.
            pub fn retire_list(list: $list) {
                $crate::list_delta::BuildList::<$row>::into_rows(list).into_iter().for_each(<$keyer as $crate::list_delta::KeyOf<$row>>::retire_cold);
            }

            /// 🕳️ Whether the delta changes nothing.
            pub fn is_empty(&self) -> bool {
                self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty()
            }
        }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = |$item:ident| $key:expr $(,)? }) => {
        impl $crate::list_delta::Keyed for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                let $item = self;
                $key
            }
        }

        $crate::plain_list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = keyed $(,)? }) => {
        $crate::plain_list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = by $keyer:ty $(,)? }) => {
        $crate::plain_list_delta! { @types [$crate::__dsl_record_derive::DslRecord] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $keyer } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = by $keyer:ty , values_only $(,)? }) => {
        $crate::plain_list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $keyer } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, key: $key:ident , values_only $(,)? }) => {
        $crate::plain_list_delta! { $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: Vec<$row>, key: String = |row| row.$key.clone(), values_only } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: String , values_only $(,)? }) => {
        $crate::plain_list_delta! { @wire [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: String, list: Vec<String>, key: String, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = |$item:ident| $key:expr , values_only $(,)? }) => {
        impl $crate::list_delta::Keyed for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                let $item = self;
                $key
            }
        }

        $crate::plain_list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, row: $row:ty, list: $list:ty, key: $key_ty:ty = keyed , values_only $(,)? }) => {
        $crate::plain_list_delta! { @types [] $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, row: $row, list: $list, key: $key_ty, keyer: $crate::list_delta::ByKeyed } }
    };
}

/// 🩹 Defines the concrete sparse patch `$patch` of the row type `$row`: every `set` field is an `Option` absolute setter, every
/// `nest` field a positional list delta of a list the row owns.
#[macro_export]
macro_rules! row_patch {
    ($(#[$meta:meta])* $vis:vis $patch:ident of $row:ty { set { $($set:ident : $set_ty:ty),* $(,)? } $(nest { $($nest:ident : $nest_delta:ty),* $(,)? })? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, $crate::__value_derive::ToValue, $crate::__value_derive::FromValue, $crate::__dsl_record_derive::DslRecord)]
        #[value(rename_all = "camelCase", default)]
        $vis struct $patch {
            $(pub $set: Option<$set_ty>,)*
            $($(pub $nest: $nest_delta,)*)?
        }

        impl $crate::list_delta::RowPatch<$row> for $patch {
            fn commit_into(&self, row: &mut $row, capability: $crate::ApplyCapability) -> Result<(), $crate::list_delta::ApplyError> {
                $(if let Some(value) = &self.$set {
                    row.$set = value.clone();
                })*
                $($({
                    let next = self.$nest.commit_onto(&row.$nest, capability).map_err(|error| error.under([stringify!($nest)]))?;
                    <$nest_delta>::retire_list(std::mem::replace(&mut row.$nest, next));
                })*)?
                Ok(())
            }

            fn absorb(&mut self, later: Self) {
                $(if later.$set.is_some() {
                    self.$set = later.$set;
                })*
                $($(self.$nest.absorb(later.$nest);)*)?
            }

            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($set: self.$set.as_ref().map(|_| row.$set.clone()),)*
                    $($($nest: self.$nest.inverse(&row.$nest),)*)?
                }
            }

            fn is_empty(&self) -> bool {
                true $(&& self.$set.is_none())* $($(&& self.$nest.is_empty())*)?
            }

            fn retire_cold(self) {
                $($(self.$nest.retire_cold();)*)?
            }
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
