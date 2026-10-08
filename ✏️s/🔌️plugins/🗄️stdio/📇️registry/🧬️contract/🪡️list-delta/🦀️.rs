//! 🪡️ The wire types of the stdio positional list deltas. The algebra — `commit_onto`, base-reading `inverse`, base-free `absorb` — is defined ONCE, in the kernel's
//! [`kernel::list_delta`] (`protocol::list_delta`); this module only declares the concrete wire shape of one stdio list around it:
//!
//! - `removed: [{id, index}]` — `index` is the row's position in the BASE list;
//! - `inserted: [{index, row}]` — `index` is the row's position in the AFTER list;
//! - `moved: [{id, from, to}]` — `from` is the BASE position, `to` the AFTER position;
//! - `modified: [{id, patch}]` — a sparse typed patch keyed by id.
//!
//! 🧬️ [`stdio_list_delta!`] is the kernel's `list_delta!` for rows whose wire types cannot derive `DslRecord` (an OPC part with its bytes, an XML part holding an XML
//! document diff): the same field names, the same methods, `ToValue`/`FromValue` only. It never reimplements an operation — every method forwards to
//! [`kernel::list_delta::Parts`].
//!
//! @see 🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta/🦀️.rs — the algebra and its randomized sequence laws

#[doc(hidden)]
pub use semio_framework_value_derive as __value_derive;

/// 🧩️ A positional list delta that composes with a later one of its own kind.
pub trait Composable: Sized {
    /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after.
    fn compose(&mut self, later: Self);
}

/// ➕️ Composes two optional deltas of one list, base-free: an absent side yields the other.
pub fn compose_optional<D: Composable>(first: Option<D>, second: Option<D>) -> Option<D> {
    match (first, second) {
        (None, value) | (value, None) => value,
        (Some(mut left), Some(right)) => {
            left.compose(right);
            Some(left)
        }
    }
}

/// 📍️ The after-list position of a row inserted into a list of `len` rows: `index` clamped to the end, the end when absent.
pub fn insertion_index(len: usize, index: Option<usize>) -> usize {
    index.map_or(len, |at| at.min(len))
}

/// 🧬️ Defines the concrete wire types of one positional list delta: `$delta` (the list), `$removal` (`{id, index}`), `$insertion` (`{index, row}`), `$relocation`
/// (`{id, from, to}`) and `$modification` (`{id, patch}`), plus the [`kernel::list_delta::Keyed`] key of `$row`. Two key spellings: `key: field` (a `String` field of
/// the row, the list a `Vec<$row>`), or `list: $list, key: $key_ty = |row| expr`. `$row` must be local to the invoking crate.
#[macro_export]
macro_rules! stdio_list_delta {
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, key: $key:ident $(,)? }) => {
        $crate::stdio_list_delta! { $(#[$meta])* $vis $delta { removal: $removal, insertion: $insertion, relocation: $relocation, modification: $modification, row: $row, patch: $patch, list: Vec<$row>, key: String = |row| row.$key.clone() } }
    };
    ($(#[$meta:meta])* $vis:vis $delta:ident { removal: $removal:ident, insertion: $insertion:ident, relocation: $relocation:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, list: $list:ty, key: $key_ty:ty = |$item:ident| $key:expr $(,)? }) => {
        impl $crate::kernel::list_delta::Keyed for $row {
            type Key = $key_ty;
            fn key(&self) -> $key_ty {
                let $item = self;
                $key
            }
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::list_delta::__value_derive::ToValue, $crate::list_delta::__value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $vis struct $removal {
            pub id: $key_ty,
            pub index: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::list_delta::__value_derive::ToValue, $crate::list_delta::__value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $vis struct $insertion {
            pub index: usize,
            pub row: $row,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::list_delta::__value_derive::ToValue, $crate::list_delta::__value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $vis struct $relocation {
            pub id: $key_ty,
            pub from: usize,
            pub to: usize,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, $crate::list_delta::__value_derive::ToValue, $crate::list_delta::__value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $vis struct $modification {
            pub id: $key_ty,
            pub patch: $patch,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, $crate::list_delta::__value_derive::ToValue, $crate::list_delta::__value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $vis struct $delta {
            #[value(default, skip_serializing_if = "Vec::is_empty")]
            pub removed: Vec<$removal>,
            #[value(default, skip_serializing_if = "Vec::is_empty")]
            pub inserted: Vec<$insertion>,
            #[value(default, skip_serializing_if = "Vec::is_empty")]
            pub moved: Vec<$relocation>,
            #[value(default, skip_serializing_if = "Vec::is_empty")]
            pub modified: Vec<$modification>,
        }

        impl $delta {
            fn into_parts(self) -> $crate::kernel::list_delta::Parts<$row, $patch> {
                $crate::kernel::list_delta::Parts {
                    removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
                    inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
                    moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
                    modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
                }
            }

            fn from_parts(parts: $crate::kernel::list_delta::Parts<$row, $patch>) -> Self {
                Self {
                    removed: parts.removed.into_iter().map(|(id, index)| $removal { id, index }).collect(),
                    inserted: parts.inserted.into_iter().map(|(index, row)| $insertion { index, row }).collect(),
                    moved: parts.moved.into_iter().map(|(id, from, to)| $relocation { id, from, to }).collect(),
                    modified: parts.modified.into_iter().map(|(id, patch)| $modification { id, patch }).collect(),
                }
            }

            /// ➕️ The delta that inserts `row` at `index` of the after list.
            pub fn insertion(index: usize, row: $row) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::insertion(index, row))
            }

            /// ➖️ The delta that removes the row at `index` of `base`.
            pub fn removal<L: $crate::kernel::list_delta::ItemList<$row> + ?Sized>(base: &L, index: usize) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::removal(base, index))
            }

            /// ➖️ The delta that removes the rows at `indices` of `base`.
            pub fn removals<L: $crate::kernel::list_delta::ItemList<$row> + ?Sized>(base: &L, indices: &[usize]) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::removals(base, indices))
            }

            /// ➖️ The delta that removes the row `id` found at `index` of the base list.
            pub fn removal_by_id(id: impl Into<$key_ty>, index: usize) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::removal_by_id(id.into(), index))
            }

            /// ➖️ The delta that removes several rows, each named with its base index.
            pub fn removals_by_id(rows: Vec<($key_ty, usize)>) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::removals_by_id(rows))
            }

            /// ↕️ The delta that moves the row at `from` of `base` to `to` of the after list.
            pub fn relocation<L: $crate::kernel::list_delta::ItemList<$row> + ?Sized>(base: &L, from: usize, to: usize) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::relocation(base, from, to))
            }

            /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
            pub fn relocation_by_id(id: impl Into<$key_ty>, from: usize, to: usize) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::relocation_by_id(id.into(), from, to))
            }

            /// 🩹 The delta that patches the row `id`.
            pub fn modification(id: impl Into<$key_ty>, patch: $patch) -> Self {
                Self::from_parts($crate::kernel::list_delta::Parts::<$row, $patch>::modification(id.into(), patch))
            }

            /// ✍️ The list this delta turns `base` into; reached only from a diff type's own `apply`, under the central applier's capability.
            pub fn commit_onto(&self, base: &$list, capability: $crate::kernel::ApplyCapability) -> Result<$list, $crate::kernel::MutationApplyError> {
                self.clone().into_parts().commit_onto(base, capability)
            }

            /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after, in place.
            pub fn absorb(&mut self, later: Self) {
                let mut parts = std::mem::take(self).into_parts();
                parts.absorb(later.into_parts());
                *self = Self::from_parts(parts);
            }

            /// 🔁️ The negative delta: applied to what `self` makes of `base`, it restores `base`.
            pub fn inverse(&self, base: &$list) -> Self {
                Self::from_parts(self.clone().into_parts().inverse(base))
            }

            /// 🕳️ Whether the delta changes nothing.
            pub fn is_empty(&self) -> bool {
                self.clone().into_parts().is_empty()
            }
        }

        impl $crate::list_delta::Composable for $delta {
            fn compose(&mut self, later: Self) {
                self.absorb(later);
            }
        }
    };
}
