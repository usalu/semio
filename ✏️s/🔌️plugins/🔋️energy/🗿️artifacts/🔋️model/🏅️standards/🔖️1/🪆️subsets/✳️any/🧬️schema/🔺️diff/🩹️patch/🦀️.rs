//! 🩹 Sparse typed patches — the vocabulary the energy-model diff is spelled in. A [`FieldPatch`] names one
//! field (or one record) and carries only what changed; [`Rows`] keys a collection by entity id and carries
//! removed, inserted and modified rows. Every patch composes (`absorb`), negates against its base (`inverse`)
//! and reads the base row by row for its negative.

use super::splice::{Splice, SpliceFault};
use std::fmt::Debug;
use protocol::MutationApplyError;
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Traits
/// 🕳️ Whether a patch changes nothing.
pub trait Unchanged {
    fn unchanged(&self) -> bool;
}

/// 🩹 One sparse patch over `Self::Target`.
pub trait FieldPatch: Unchanged + Clone + PartialEq + Debug {
    type Target;
    /// ▶️ Writes the patch into `target`; only the diff's own capability-gated `apply` reaches this.
    fn commit_onto(&self, target: &mut Self::Target) -> Result<(), MutationApplyError>;
    /// ➕️ Composes `self` then `later` into one patch.
    fn absorb(&mut self, later: Self);
    /// ↩️ The patch that restores `base` after `self` was applied to it.
    fn inverse(&self, base: &Self::Target) -> Self;
}

/// 🔑️ A collection entry addressed by a stable key.
pub trait Row: Clone + PartialEq + Debug {
    type Key: Clone + Ord + Debug + ToValue + FromValue;
    fn key(&self) -> Self::Key;
}

/// 🩹 A patch over one keyed row.
pub trait RowPatch: FieldPatch<Target: Row> {
    fn key(&self) -> <Self::Target as Row>::Key;
}

pub(crate) fn refused(code: &'static str, message: impl Into<String>) -> MutationApplyError {
    MutationApplyError::new(code, message)
}

fn splice_refused(fault: SpliceFault) -> MutationApplyError {
    refused("diff.list-conflict", fault.to_string())
}
//#endregion 🔖️Traits

//#region 🔖️Set
impl<T> Unchanged for Option<T> {
    fn unchanged(&self) -> bool {
        self.is_none()
    }
}

/// 🎯️ `Some(value)` assigns the field; `None` leaves it alone.
impl<T: Clone + PartialEq + Debug> FieldPatch for Option<T> {
    type Target = T;
    fn commit_onto(&self, target: &mut T) -> Result<(), MutationApplyError> {
        if let Some(value) = self {
            *target = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.is_some() {
            *self = later;
        }
    }
    fn inverse(&self, base: &T) -> Self {
        self.as_ref().map(|_| base.clone())
    }
}
//#endregion 🔖️Set

//#region 🔖️OptionChange
/// 🎯️ A patch over an `Option<T>` field: absent leaves it, `Cleared` empties it, `Assigned` fills it. A typed
/// three-state, because `Option<Option<T>>` collapses "unchanged" and "now empty" onto one JSON `null`.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum OptionChange<T> {
    #[default]
    Unchanged,
    Cleared,
    Assigned(T),
}

impl<T> OptionChange<T> {
    /// 🎯️ The change that makes the field read `value`.
    pub fn assign(value: Option<T>) -> Self {
        value.map_or(Self::Cleared, Self::Assigned)
    }
}

impl<T> Unchanged for OptionChange<T> {
    fn unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
}

impl<T: Clone + PartialEq + Debug> FieldPatch for OptionChange<T> {
    type Target = Option<T>;
    fn commit_onto(&self, target: &mut Option<T>) -> Result<(), MutationApplyError> {
        match self {
            Self::Unchanged => {}
            Self::Cleared => *target = None,
            Self::Assigned(value) => *target = Some(value.clone()),
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if !later.unchanged() {
            *self = later;
        }
    }
    fn inverse(&self, base: &Option<T>) -> Self {
        if self.unchanged() {
            Self::Unchanged
        } else {
            Self::assign(base.clone())
        }
    }
}
//#endregion 🔖️OptionChange

//#region 🔖️Slots
/// 🔢️ One assigned array slot.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct Slot<T> {
    pub index: usize,
    pub value: T,
}

/// 🔢️ A patch over a fixed-size array of `N` entries: the assigned slots, ascending by index.
#[derive(Clone, Debug, PartialEq)]
pub struct Slots<T, const N: usize>(pub Vec<Slot<T>>);

impl<T: semio_framework_value::retirement::RetireOwned, const N: usize> semio_framework_value::retirement::RetireOwned for Slots<T, N> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)
    }
    fn controlled_retirement_supported() -> bool {
        <Vec<Slot<T>> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()
    }
}

impl<T, const N: usize> Default for Slots<T, N> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T, const N: usize> Slots<T, N> {
    /// 🔢️ Assigns one slot.
    pub fn assigning(index: usize, value: T) -> Self {
        Self(vec![Slot { index, value }])
    }
}

impl<T: Clone + PartialEq, const N: usize> Slots<T, N> {
    /// 🔁️ Assigns exactly the slots of `was` whose entry differs in `now`.
    pub fn replacing(was: &[T; N], now: &[T; N]) -> Self {
        Self((0..N).filter(|index| was[*index] != now[*index]).map(|index| Slot { index, value: now[index].clone() }).collect())
    }
}

impl<T, const N: usize> Unchanged for Slots<T, N> {
    fn unchanged(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Clone + PartialEq + Debug, const N: usize> FieldPatch for Slots<T, N> {
    type Target = [T; N];
    fn commit_onto(&self, target: &mut [T; N]) -> Result<(), MutationApplyError> {
        for Slot { index, value } in &self.0 {
            *target.get_mut(*index).ok_or_else(|| refused("diff.slot-out-of-range", format!("slot {index} is outside an array of {N}")))? = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        for slot in later.0 {
            match self.0.binary_search_by_key(&slot.index, |own| own.index) {
                Ok(position) => self.0[position] = slot,
                Err(position) => self.0.insert(position, slot),
            }
        }
    }
    fn inverse(&self, base: &[T; N]) -> Self {
        Self(self.0.iter().filter_map(|slot| base.get(slot.index).map(|value| Slot { index: slot.index, value: value.clone() })).collect())
    }
}

impl<T: ToValue, const N: usize> ToValue for Slots<T, N> {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        self.0.to_value_controlled(control)
    }
}

impl<T: FromValue, const N: usize> FromValue for Slots<T, N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Vec::from_value(value).map(Self)
    }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        Vec::from_value_controlled(value, control).map(Self)
    }
}
//#endregion 🔖️Slots

//#region 🔖️Wire
#[derive(Clone, ToValueDerive, FromValueDerive)]
struct CutWire<K> {
    index: usize,
    key: K,
}

#[derive(Clone, ToValueDerive, FromValueDerive)]
struct PutWire<T> {
    index: usize,
    row: T,
}

fn cut_wires<K: Clone, T>(edit: &Splice<K, T>) -> Vec<CutWire<K>> {
    edit.cuts().iter().map(|(index, key)| CutWire { index: *index, key: key.clone() }).collect()
}

fn put_wires<K, T: Clone>(edit: &Splice<K, T>) -> Vec<PutWire<T>> {
    edit.puts().iter().map(|(index, row)| PutWire { index: *index, row: row.clone() }).collect()
}

fn splice_of<K, T>(cuts: Vec<CutWire<K>>, puts: Vec<PutWire<T>>) -> Splice<K, T> {
    Splice::new(cuts.into_iter().map(|cut| (cut.index, cut.key)).collect(), puts.into_iter().map(|put| (put.index, put.row)).collect())
}
//#endregion 🔖️Wire

//#region 🔖️ListEdit
/// 📋️ A patch over an ordered list: removals by base index and insertions by after index.
#[derive(Clone, Debug, PartialEq)]
pub struct ListEdit<T>(Splice<T, T>);

impl<T: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for ListEdit<T> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)
    }
    fn controlled_retirement_supported() -> bool {
        <Splice<T, T> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()
    }
}

impl<T> Default for ListEdit<T> {
    fn default() -> Self {
        Self(Splice::default())
    }
}

impl<T: Clone + PartialEq> ListEdit<T> {
    /// ➕️ Inserts `value` so it lands at `index`.
    pub fn inserting(index: usize, value: T) -> Self {
        Self(Splice::putting(index, value))
    }

    /// ✂️ Removes the entry at `index`, which holds `value`.
    pub fn removing_at(index: usize, value: T) -> Self {
        Self(Splice::cutting(index, value))
    }

    /// ✂️ Removes the entry of `list` at `index`.
    pub fn removing_index(list: &[T], index: usize) -> Self {
        list.get(index).map_or_else(Self::default, |value| Self::removing_at(index, value.clone()))
    }

    /// ✂️ Removes every entry of `list` that `matches`.
    pub fn removing_where(list: &[T], matches: impl Fn(&T) -> bool) -> Self {
        Self(Splice::new(list.iter().enumerate().filter(|(_, entry)| matches(entry)).map(|(index, entry)| (index, entry.clone())).collect(), Vec::new()))
    }

    /// 🔁️ Makes `was` read `now`, touching only the positions whose entry differs.
    pub fn replacing(was: &[T], now: &[T]) -> Self {
        Self(Splice::replacing(was, now))
    }

    /// ➡️ Moves the entry of `list` at `from` so it lands at `to`.
    pub fn moving(list: &[T], from: usize, to: usize) -> Self {
        let Some(entry) = list.get(from) else { return Self::default() };
        let mut edit = Self(Splice::new(vec![(from, entry.clone())], vec![(to, entry.clone())]));
        edit.0.settle();
        edit
    }
}

impl<T> Unchanged for ListEdit<T> {
    fn unchanged(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Clone + PartialEq + Debug> FieldPatch for ListEdit<T> {
    type Target = Vec<T>;
    fn commit_onto(&self, target: &mut Vec<T>) -> Result<(), MutationApplyError> {
        *target = self.0.commit_onto(std::mem::take(target), |entry, key| entry == key).map_err(splice_refused)?;
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.0.absorb(later.0);
        self.0.settle();
    }
    fn inverse(&self, base: &Vec<T>) -> Self {
        Self(self.0.inverse(base, T::clone))
    }
}

/// 📋️ The wire form of an edit: two arrays, each omitted when empty.
#[derive(ToValueDerive, FromValueDerive)]
struct ListEditWire<T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    removed: Vec<CutWire<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    inserted: Vec<PutWire<T>>,
}

impl<T: ToValue + Clone> ToValue for ListEdit<T> {
    fn to_value(&self) -> DslValue {
        ListEditWire { removed: cut_wires(&self.0), inserted: put_wires(&self.0) }.to_value()
    }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        ListEditWire { removed: cut_wires(&self.0), inserted: put_wires(&self.0) }.to_value_controlled(control)
    }
}

impl<T: FromValue> FromValue for ListEdit<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let wire = ListEditWire::<T>::from_value(value)?;
        Ok(Self(splice_of(wire.removed, wire.inserted)))
    }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let wire = ListEditWire::<T>::from_value_controlled(value, control)?;
        Ok(Self(splice_of(wire.removed, wire.inserted)))
    }
}
//#endregion 🔖️ListEdit

//#region 🔖️Rows
/// 🗂️ A patch over a collection keyed by entity id. `edit` removes rows by base index and inserts rows by after
/// index; `modified` patches the rows that stay, one patch per key.
#[derive(Clone, Debug, PartialEq)]
pub struct Rows<P: RowPatch> {
    edit: Splice<<P::Target as Row>::Key, P::Target>,
    modified: Vec<P>,
}

impl<P: RowPatch + semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for Rows<P>
where
    P::Target: semio_framework_value::retirement::RetireOwned,
    <P::Target as Row>::Key: semio_framework_value::retirement::RetireOwned,
{
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement((self.edit, self.modified))
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&(Splice::<<P::Target as Row>::Key, P::Target>::default(), Vec::<P>::new()))
    }
    fn controlled_retirement_supported() -> bool {
        <(Splice<<P::Target as Row>::Key, P::Target>, Vec<P>) as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()
    }
}

impl<P: RowPatch> Default for Rows<P> {
    fn default() -> Self {
        Self { edit: Splice::default(), modified: Vec::new() }
    }
}

impl<P: RowPatch> Rows<P> {
    /// 🩹 Patches one existing row.
    pub fn modifying(patch: P) -> Self {
        Self { edit: Splice::default(), modified: vec![patch] }
    }

    /// ➕️ Inserts `row` so it lands at `index`.
    pub fn inserting(index: usize, row: P::Target) -> Self {
        Self { edit: Splice::putting(index, row), modified: Vec::new() }
    }

    /// ✂️ Removes the row of `list` addressed by `key`.
    pub fn removing(list: &[P::Target], key: &<P::Target as Row>::Key) -> Self {
        Self::removing_where(list, |row| row.key() == *key)
    }

    /// ✂️ Removes every row of `list` that `matches`.
    pub fn removing_where(list: &[P::Target], matches: impl Fn(&P::Target) -> bool) -> Self {
        Self { edit: Splice::new(list.iter().enumerate().filter(|(_, row)| matches(row)).map(|(index, row)| (index, row.key())).collect(), Vec::new()), modified: Vec::new() }
    }

    /// 🔎️ The patches of the rows that stay.
    pub fn modified(&self) -> &[P] {
        &self.modified
    }

    /// 🔎️ The inserted rows with their after index.
    pub fn inserted(&self) -> &[(usize, P::Target)] {
        self.edit.puts()
    }

    /// 🔎️ The removed keys with their base index.
    pub fn removed(&self) -> &[(usize, <P::Target as Row>::Key)] {
        self.edit.cuts()
    }

    fn normalized(mut self) -> Self {
        self.modified.retain(|patch| !patch.unchanged());
        self.modified.sort_by_key(|patch| patch.key());
        self
    }
}

impl<P: RowPatch> Unchanged for Rows<P> {
    fn unchanged(&self) -> bool {
        self.edit.is_empty() && self.modified.iter().all(Unchanged::unchanged)
    }
}

impl<P: RowPatch> FieldPatch for Rows<P> {
    type Target = Vec<P::Target>;
    fn commit_onto(&self, target: &mut Vec<P::Target>) -> Result<(), MutationApplyError> {
        for patch in &self.modified {
            let key = patch.key();
            let row = target.iter_mut().find(|row| row.key() == key).ok_or_else(|| refused("diff.target-missing", "a row patch names a row the collection does not hold"))?;
            patch.commit_onto(row)?;
        }
        *target = self.edit.commit_onto(std::mem::take(target), |row, key| row.key() == *key).map_err(splice_refused)?;
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let Self { edit: later_edit, modified: later_modified } = later;
        let later_cut_keys: Vec<_> = later_edit.cuts().iter().map(|(_, key)| key.clone()).collect();
        self.modified.retain(|patch| !later_cut_keys.contains(&patch.key()));
        let (cuts, mut puts) = std::mem::take(&mut self.edit).into_parts();
        for patch in later_modified {
            let key = patch.key();
            if later_cut_keys.contains(&key) {
                continue;
            }
            let folded = puts.iter_mut().find(|(_, row)| row.key() == key).is_some_and(|(_, row)| {
                let mut next = row.clone();
                let applied = patch.commit_onto(&mut next).is_ok();
                if applied {
                    *row = next;
                }
                applied
            });
            if folded {
                continue;
            }
            match self.modified.iter_mut().find(|own| own.key() == key) {
                Some(own) => own.absorb(patch),
                None => self.modified.push(patch),
            }
        }
        self.edit = Splice::new(cuts, puts);
        self.edit.absorb(later_edit);
        *self = std::mem::take(self).normalized();
    }
    fn inverse(&self, base: &Vec<P::Target>) -> Self {
        Self {
            edit: self.edit.inverse(base, Row::key),
            modified: self.modified.iter().filter_map(|patch| base.iter().find(|row| row.key() == patch.key()).map(|row| patch.inverse(row))).collect(),
        }
        .normalized()
    }
}

/// 🗂️ The wire form of [`Rows`]: three arrays, each omitted when empty.
#[derive(ToValueDerive, FromValueDerive)]
struct RowsWire<K, R, P> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    removed: Vec<CutWire<K>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    inserted: Vec<PutWire<R>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    modified: Vec<P>,
}

impl<P: RowPatch + ToValue> RowsWire<<P::Target as Row>::Key, P::Target, P>
where
    P::Target: ToValue,
{
    fn of(rows: &Rows<P>) -> Self {
        Self { removed: cut_wires(&rows.edit), inserted: put_wires(&rows.edit), modified: rows.modified.clone() }
    }
}

impl<P: RowPatch + ToValue> ToValue for Rows<P>
where
    P::Target: ToValue,
{
    fn to_value(&self) -> DslValue {
        RowsWire::of(self).to_value()
    }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        RowsWire::of(self).to_value_controlled(control)
    }
}

impl<P: RowPatch + FromValue> FromValue for Rows<P>
where
    P::Target: FromValue,
{
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let wire = RowsWire::<<P::Target as Row>::Key, P::Target, P>::from_value(value)?;
        Ok(Self { edit: splice_of(wire.removed, wire.inserted), modified: wire.modified })
    }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let wire = RowsWire::<<P::Target as Row>::Key, P::Target, P>::from_value_controlled(value, control)?;
        Ok(Self { edit: splice_of(wire.removed, wire.inserted), modified: wire.modified })
    }
}
//#endregion 🔖️Rows

//#region 🔖️Fields
/// 🧩️ Maps a field kind (a marker type) and the field's declared type to the patch that edits it.
pub trait Field<T> {
    type Patch: FieldPatch + Default;
}

/// 🎯️ Assigns a scalar-like field: `Option<T>`.
pub struct Set;
/// 🎯️ Sets or clears an `Option<T>` field: `OptionChange<T>`.
pub struct Opt;
/// 📋️ Edits an ordered list of `T`: `ListEdit<T>`.
pub struct List;
/// 🔢️ Assigns slots of a fixed array: `Slots<T, N>`.
pub struct Arr;
/// 🗂️ Patches a collection of rows addressed by patch type `P`: `Rows<P>`.
pub struct Coll;
/// 🧱️ Patches a nested record with its own patch `P`.
pub struct Rec;

impl<T: Clone + PartialEq + Debug> Field<T> for Set {
    type Patch = Option<T>;
}

impl<T: Clone + PartialEq + Debug> Field<T> for Opt {
    type Patch = OptionChange<T>;
}

impl<T: Clone + PartialEq + Debug> Field<T> for List {
    type Patch = ListEdit<T>;
}

impl<T: Clone + PartialEq + Debug, const N: usize> Field<[T; N]> for Arr {
    type Patch = Slots<T, N>;
}

impl<P: RowPatch> Field<P> for Coll {
    type Patch = Rows<P>;
}

impl<P: FieldPatch + Default> Field<P> for Rec {
    type Patch = P;
}
//#endregion 🔖️Fields

//#region 🔖️Macros
macro_rules! patch {
    (@ $(#[$meta:meta])* $patch:ident for $row:ty; [$($key:ident: $key_ty:ty)?]; $($kind:ident $field:ident: $ty:ty),*) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
        pub struct $patch {
            $(pub $key: $key_ty,)?
            $(#[value(default, skip_serializing_if = "Unchanged::unchanged")] pub $field: <$kind as Field<$ty>>::Patch,)*
        }

        impl $patch {
            pub fn of($($key: $key_ty)?) -> Self {
                Self { $($key,)? $($field: Default::default(),)* }
            }
        }

        impl Unchanged for $patch {
            fn unchanged(&self) -> bool {
                true $(&& self.$field.unchanged())*
            }
        }

        impl FieldPatch for $patch {
            type Target = $row;
            fn commit_onto(&self, target: &mut $row) -> Result<(), MutationApplyError> {
                $(self.$field.commit_onto(&mut target.$field)?;)*
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $(self.$field.absorb(later.$field);)*
            }
            fn inverse(&self, base: &$row) -> Self {
                Self { $($key: self.$key.clone(),)? $($field: self.$field.inverse(&base.$field),)* }
            }
        }
    };
}

macro_rules! row_patch {
    ($(#[$meta:meta])* $patch:ident for $row:ty { key $key:ident: $key_ty:ty; $($kind:ident $field:ident: $ty:ty),* $(,)? }) => {
        patch!(@ $(#[$meta])* $patch for $row; [$key: $key_ty]; $($kind $field: $ty),*);

        impl Row for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                self.$key.clone()
            }
        }

        impl RowPatch for $patch {
            fn key(&self) -> $key_ty {
                self.$key.clone()
            }
        }
    };
}

macro_rules! record_patch {
    ($(#[$meta:meta])* $patch:ident for $row:ty { $($kind:ident $field:ident: $ty:ty),* $(,)? }) => {
        patch!(@ $(#[$meta])* $patch for $row; []; $($kind $field: $ty),*);

        impl Default for $patch {
            fn default() -> Self {
                Self::of()
            }
        }
    };
}

pub(crate) use {patch, record_patch, row_patch};
//#endregion 🔖️Macros
