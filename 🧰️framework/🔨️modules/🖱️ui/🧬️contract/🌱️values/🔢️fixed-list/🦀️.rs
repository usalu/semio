//! 🔢️ Fixed list: the `UiFixedList` paged list with its iterators, indexing, serde and value conversions. See [`super`].

use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::native_decoding::{NativeReceive,owned::{NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,NativeMappedReceiver,NativeReceiveMap,tree::{NativeValueSource,fixed,receipt}}};

pub struct UiFixedList<T, const N: usize = UI_FIXED_LIST_ITEMS> {
    pub(super) storage: fixed_list_storage::PagedList<T, N>,
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for UiFixedList<T, N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq, const N: usize> PartialEq for UiFixedList<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<T: Eq, const N: usize> Eq for UiFixedList<T, N> {}

impl<T, const N: usize> Default for UiFixedList<T, N> {
    fn default() -> Self {
        Self { storage: Default::default() }
    }
}

/// 🧊️ Cold cloning allocates one page at a time and copies every initialized payload — the clone owns
/// exactly the pages its payloads need, never the source's spare backing; retained owners use
/// explicit cursors.
impl<T: Clone, const N: usize> Clone for UiFixedList<T, N> {
    fn clone(&self) -> Self {
        let mut result = Self::default();
        for item in self.iter() {
            if result.try_push(item.clone()).is_err() {
                unreachable!("cold clone admits at most the source's own logical length");
            }
        }
        result
    }
}

impl<T, const N: usize> UiFixedList<T, N> {
    /// 🪹️ Empty fixed metadata requires neither a heap owner nor lazy initialization.
    pub const fn empty() -> Self {
        Self { storage: fixed_list_storage::PagedList::empty() }
    }
    /// ❄️ Cold full-capacity reservation; interactive owners admit one page with try_reserve_one.
    pub fn try_reserve(&mut self) -> Result<bool, PagedListError> {
        self.storage.reserve_full()
    }

    pub fn allocated_bytes(&self) -> usize {
        self.storage.allocated_bytes()
    }
    pub fn has_reserved_slot(&self) -> bool {
        self.storage.has_reserved_slot()
    }
    pub fn next_allocation_bytes(&self) -> Result<usize, PagedListError> {
        self.storage.next_allocation_bytes()
    }
    /// 🧊️ Cold page admission at an explicitly synchronous boundary whose caller already charged the exact page bytes; retained owners use try_reserve_one_funded.
    pub fn try_reserve_one(&mut self, bytes: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        self.storage.reserve_one_cold(bytes)
    }
    /// 🎟️ Admits at most one backing page under the caller's own effect and returns the exact funded receipt; a grant that cannot carry the page leaves the list untouched and reports `Denied`.
    pub fn try_reserve_one_funded(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<PagedListReservationOutcome<RetainedCloneProgress>, PagedListReservationFailure> {
        self.storage.reserve_one_funded(effect)
    }
    /// ↩️ Admits at most one backing page toward `capacity` slots under the caller's own effect, whether or not a slot is already free; a list that already holds `capacity` slots, or a grant that cannot carry the page, leaves the list untouched.
    pub fn try_reserve_capacity_one_funded(&mut self, capacity: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<PagedListReservationOutcome<RetainedCloneProgress>, PagedListReservationFailure> {
        self.storage.reserve_capacity_one_funded(capacity, effect)
    }
    pub fn try_place_reserved(&mut self, source: &mut Option<T>, bytes: usize) -> Result<PagedListProgress, PagedListError> {
        self.storage.place_reserved(source, bytes)
    }
    pub fn next_release_allocation_bytes(&self)->Result<usize,PagedListError>{self.storage.next_release_allocation_bytes()}
    /// 🪜️ Borrows the original native page path depth before logical removal.
    pub fn next_pop_depth_demand(&self)->Result<usize,PagedListError>{self.storage.next_pop_depth_demand()}
    /// 👣️ Borrows the original native page path depth before physical release.
    pub fn next_release_depth_demand(&self)->Result<usize,PagedListError>{self.storage.next_release_depth_demand()}
    /// ♻️ Releases the selected empty page through the caller's own effect.
    pub fn release_empty_page(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<PagedListReleaseOutcome, PagedListReleaseFailure> {
        self.storage.release_empty_page(effect)
    }
    /// 🪶️ Releases one empty page inside an exclusive arena lease whose cursor owner pays one unit per progressed step.
    pub fn release_empty_page_leased(&mut self, maximum_bytes: usize) -> Result<PagedListProgress, PagedListError> {
        self.storage.release_empty_page_leased(maximum_bytes)
    }
    pub(crate) fn truncate_retired_last(&mut self) -> Result<(), PagedListError> {
        self.storage.truncate_retired_last()
    }

    /// 📥️ Moves one payload into previously admitted storage without allocation or capacity growth.
    pub fn try_push_reserved(&mut self, value: T) -> Result<(), T> {
        self.storage.push_reserved(value)
    }

    /// ⏭️ Admits backing pages until one slot is free — the page-exact reservation every retained [Design notes](README.md#source-notes-94).
    pub(super) fn reserve_next_slot(&mut self) -> Result<(), PagedListError> {
        while !self.has_reserved_slot() {
            let bytes = self.next_allocation_bytes()?;
            if !self.try_reserve_one(bytes).map_err(PagedListAllocationError::refusal)?.progressed {
                return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list page admission made no progress" });
            }
        }
        Ok(())
    }

    /// 📃️ Cold convenience combining page-exact allocation admission and one write; retained builders
    /// split them.
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len() == N || self.reserve_next_slot().is_err() {
            return Err(value);
        }
        self.try_push_reserved(value)
    }

    /// 📤️ Transfers the last initialized payload without destroying it or releasing its backing.
    pub fn pop(&mut self) -> Option<T> {
        self.storage.pop()
    }

    /// 🧾️ Releases only empty backing; a live payload rejects the operation without mutation.
    pub fn release_empty_allocation(&mut self) -> Result<bool, PagedListError> {
        if !self.is_empty() {
            return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "UI fixed-list payloads must be transferred before backing release" });
        }
        let released = !self.terminal_is_empty();
        while !self.terminal_is_empty() {
            self.release_empty_page_leased(usize::MAX)?;
        }
        Ok(released)
    }

    /// 🔒️ A retained list is terminal only after both payloads and its backing owner are gone.
    pub fn terminal_is_empty(&self) -> bool {
        self.storage.terminal_is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.storage.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.storage.get_mut(index)
    }

    pub fn last(&self) -> Option<&T> {
        self.get(self.len().checked_sub(1)?)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len().checked_sub(1)?)
    }

    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        let last = self.pop()?;
        if index == self.len() {
            Some(last)
        } else {
            Some(std::mem::replace(self.get_mut(index)?, last))
        }
    }

    pub fn iter_mut(&mut self) -> UiFixedListIterMut<'_, T, N> {
        UiFixedListIterMut { inner: self.storage.iter_mut() }
    }

    pub fn iter(&self) -> UiFixedListIter<'_, T, N> {
        UiFixedListIter { owner: self, next: 0, end: self.len() }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, const N: usize> std::ops::Index<usize> for UiFixedList<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("fixed-list index must be within admitted length")
    }
}

pub struct UiFixedListIter<'a, T, const N: usize> {
    pub(super) owner: &'a UiFixedList<T, N>,
    pub(super) next: usize,
    pub(super) end: usize,
}

impl<'a, T, const N: usize> Iterator for UiFixedListIter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            return None;
        }
        let index = self.next;
        self.next += 1;
        self.owner.get(index)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.end - self.next;
        (length, Some(length))
    }
}

impl<T, const N: usize> DoubleEndedIterator for UiFixedListIter<'_, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            return None;
        }
        self.end -= 1;
        self.owner.get(self.end)
    }
}

impl<T, const N: usize> ExactSizeIterator for UiFixedListIter<'_, T, N> {}

pub struct UiFixedListIterMut<'a, T, const N: usize> {
    inner: fixed_list_storage::PagedIterMut<'a, T>,
}

impl<'a, T, const N: usize> Iterator for UiFixedListIterMut<'a, T, N> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T, const N: usize> ExactSizeIterator for UiFixedListIterMut<'_, T, N> {}

impl<T, const N: usize> std::ops::IndexMut<usize> for UiFixedList<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("fixed-list index must be within admitted length")
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a UiFixedList<T, N> {
    type Item = &'a T;
    type IntoIter = UiFixedListIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// 📚️ Cold owned iteration transfers the initialized sequence; retained owners use pop and explicit backing release.
impl<T, const N: usize> IntoIterator for UiFixedList<T, N> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut values = Vec::with_capacity(self.len());
        while let Some(value) = self.pop() {
            values.push(value);
        }
        values.reverse();
        self.release_empty_allocation().expect("cold iteration released empty pages");
        values.into_iter()
    }
}

impl<T: Serialize, const N: usize> Serialize for UiFixedList<T, N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for value in self {
            sequence.serialize_element(value)?;
        }
        sequence.end()
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for UiFixedList<T, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        struct FixedListVisitor<T, const N: usize>(std::marker::PhantomData<T>);
        impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for FixedListVisitor<T, N> {
            type Value = UiFixedList<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded fixed UI list")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut values = UiFixedList::default();
                while let Some(value) = access.next_element::<T>()? {
                    if values.try_push(value).is_err() {
                        return Err(serde::de::Error::custom(format!("UiFixedList exceeds {N} items")));
                    }
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(FixedListVisitor::<T, N>(std::marker::PhantomData))
    }
}

/// 🌱️ Hand-written, not derived — generic over `T`, and `#[derive(ToValue, FromValue)]` does not
/// reach through a hand-rolled container's iterator; mirrors the `Serialize`/`Deserialize` impls
/// above exactly (a bare JSON array). Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
impl<T: ::semio_framework_value::ToValue, const N: usize> ::semio_framework_value::ToValue for UiFixedList<T, N> {
    fn to_value(&self) -> ::semio_framework_value::DslValue {
        ::semio_framework_value::DslValue::Array(self.iter().map(::semio_framework_value::ToValue::to_value).collect())
    }
}

impl<T: ::semio_framework_value::FromValue, const N: usize> ::semio_framework_value::FromValue for UiFixedList<T, N> {
    fn from_value(value: ::semio_framework_value::DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        let items = match value {
            ::semio_framework_value::DslValue::Array(items) => items,
            other => return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}"))),
        };
        let mut values = UiFixedList::default();
        for item in items {
            let item = <T as ::semio_framework_value::FromValue>::from_value(item)?;
            if values.try_push(item).is_err() {
                return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiFixedList exceeds {N} items")));
            }
        }
        Ok(values)
    }
}

impl<T:RetireOwned,const N:usize> RetireOwned for UiFixedList<T,N>{
    type RetirementCursor=<fixed_list_storage::PagedList<T,N> as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{self.storage.retirement_seed()}
    fn retirement_ready(&self)->bool{self.storage.retirement_ready()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.storage.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{<fixed_list_storage::PagedList<T,N> as RetireOwned>::controlled_retirement_supported()}
}

impl<T:RetireOwned,const N:usize> NativeReceiveMap<fixed_list_storage::PagedList<T,N>> for UiFixedList<T,N>{fn from_native(storage:fixed_list_storage::PagedList<T,N>)->Self{Self{storage}}}

impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,const N:usize> NativeReceive<I> for UiFixedList<T,N>{
    type Receiver=NativeMappedReceiver<I,fixed_list_storage::PagedList<T,N>,Self>;
    fn receiver()->Self::Receiver{Self::Receiver::new()}
}

impl<T:RetireOwned,const N:usize> NativeDefaultValue for UiFixedList<T,N>{
    fn default_demand()->RetirementDemand{fixed(size_of::<Option<Self>>())}
    fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
    fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut ::semio_framework_value::RetainedAllocationEffect<'_>,reply: ::semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<::semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>,::semio_framework_value::ValueError>{::semio_framework_value_native_owned::native_decoding::owned::native_admit_default(output,<Self as ::semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue>::default_demand(),||Self::empty(),effect,reply)}
fn recover_default<'reply>(_:&mut Option<Self>,_:&mut ::semio_framework_value::RetainedAllocationEffect<'_>,reply: ::semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<::semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>,::semio_framework_value::ValueError>{Ok(reply.pending())}
}
impl<T:RetireOwned,const N:usize> NativeDefault for UiFixedList<T,N>{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
