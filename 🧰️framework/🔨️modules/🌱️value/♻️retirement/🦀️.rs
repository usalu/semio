//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

pub trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
}

pub enum RetirementStep {
    Child(Box<dyn RetirementCursor>),
    Bytes(usize),
    Complete,
    BudgetExhausted,
}

pub trait RetirementCursor: Send {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep;
    fn terminal_is_empty(&self) -> bool;
    fn next_close_byte_demand(&self) -> Option<usize> {
        None
    }
}

struct Leaf<T: Copy + Send + 'static> {
    value: Option<T>,
    remaining: usize,
}
impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.remaining > 0 {
            if maximum_bytes == 0 {
                return RetirementStep::BudgetExhausted;
            }
            let bytes = maximum_bytes.min(self.remaining);
            self.remaining -= bytes;
            return RetirementStep::Bytes(bytes);
        }
        self.value.take();
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.remaining == 0
    }
}

pub fn leaf<T: Copy + Send + 'static>(value: T) -> Box<dyn RetirementCursor> {
    Box::new(Leaf { value: Some(value), remaining: size_of::<T>() })
}

#[macro_export]
macro_rules! artifact_retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl $crate::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::retirement::RetirementCursor> { $crate::retirement::leaf(self) }
        }
    )+ };
}

#[macro_export]
macro_rules! artifact_retirement_sequence {
    ($($field:expr),* $(,)?) => { $crate::retirement::sequence(vec![$($crate::retirement::RetireOwned::retirement($field)),*]) };
}

#[macro_export]
macro_rules! artifact_retire_struct {
    ($type:ty { $($field:ident),+ $(,)? }) => {
        impl $crate::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::retirement::RetirementCursor> {
                let Self { $($field),+ } = self;
                $crate::artifact_retirement_sequence![$($field),+]
            }
        }
    };
}

artifact_retire_leaf!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

struct Bytes(ManuallyDrop<Vec<u8>>);
impl RetirementCursor for Bytes {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.0.is_empty() {
            return RetirementStep::Complete;
        }
        if maximum_bytes == 0 {
            return RetirementStep::BudgetExhausted;
        }
        let bytes = maximum_bytes.min(self.0.len());
        let next = self.0.len() - bytes;
        self.0.truncate(next);
        RetirementStep::Bytes(bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Drop for Bytes {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned bytes retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl RetireOwned for String {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Bytes(ManuallyDrop::new(self.into_bytes())))
    }
}

/// ♻️ A vector retires element by element, each through its own retirement, unless its elements have no drop glue: then
/// nothing but their memory is released and a step releases a page of them at once (`maximum_bytes / size_of::<T>()`), so a
/// large byte buffer or number array costs its size over the grant in steps instead of three steps per element (a returned
/// 2 MiB `Vec<u8>` took ~6.3 M one-item pump turns). A grant narrower than one element retires that element on its own.
struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if !std::mem::needs_drop::<T>() && !self.0.is_empty() {
            let width = size_of::<T>();
            if width == 0 {
                self.0.clear();
                return RetirementStep::Bytes(0);
            }
            let count = (maximum_bytes / width).min(self.0.len());
            if count > 0 {
                let next = self.0.len() - count;
                self.0.truncate(next);
                return RetirementStep::Bytes(count * width);
            }
        }
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T: RetireOwned> Drop for Collection<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned collection retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl<T: RetireOwned> RetireOwned for Vec<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Collection(ManuallyDrop::new(self)))
    }
}

struct DequeCollection<T:RetireOwned>{values:ManuallyDrop<std::collections::VecDeque<T>>,backing_bytes:usize}
impl<T:RetireOwned> RetirementCursor for DequeCollection<T>{
    fn close_step(&mut self,maximum_bytes:usize)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete}if maximum_bytes==0{return RetirementStep::BudgetExhausted}if let Some(value)=self.values.pop_back(){return RetirementStep::Child(value.retirement())}let released=maximum_bytes.min(self.backing_bytes);self.backing_bytes-=released;RetirementStep::Bytes(released)}
    fn terminal_is_empty(&self)->bool{self.values.is_empty()&&self.backing_bytes==0}
}
impl<T:RetireOwned> Drop for DequeCollection<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned deque retired before terminal-empty");unsafe{ManuallyDrop::drop(&mut self.values)}}}
impl<T:RetireOwned> RetireOwned for std::collections::VecDeque<T>{fn retirement(self)->Box<dyn RetirementCursor>{let backing_bytes=self.capacity()*std::mem::size_of::<T>();Box::new(DequeCollection{values:ManuallyDrop::new(self),backing_bytes})}}

struct VectorIterator<T:RetireOwned>(ManuallyDrop<std::vec::IntoIter<T>>);
impl<T:RetireOwned> RetirementCursor for VectorIterator<T> {
    fn close_step(&mut self,maximum_bytes:usize)->RetirementStep {if self.0.len()==0 {return RetirementStep::Complete;}if maximum_bytes==0 {return RetirementStep::BudgetExhausted;}RetirementStep::Child(self.0.next_back().unwrap().retirement())}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<T:RetireOwned> Drop for VectorIterator<T> {fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned iterator retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)}}}
impl<T:RetireOwned> RetireOwned for std::vec::IntoIter<T> {fn retirement(self)->Box<dyn RetirementCursor> {Box::new(VectorIterator(ManuallyDrop::new(self)))}}

struct UnorderedSet<T: RetireOwned>(ManuallyDrop<std::collections::hash_set::IntoIter<T>>);
impl<T: RetireOwned> RetirementCursor for UnorderedSet<T> {
    fn close_step(&mut self,_:usize)->RetirementStep {self.0.next().map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<T: RetireOwned> Drop for UnorderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)};}
}
impl<T: RetireOwned + std::hash::Hash + Eq> RetireOwned for std::collections::HashSet<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(UnorderedSet(ManuallyDrop::new(self.into_iter())))}
}

struct UnorderedMap<K: RetireOwned,V: RetireOwned>(ManuallyDrop<std::collections::hash_map::IntoIter<K,V>>);
impl<K: RetireOwned,V: RetireOwned> RetirementCursor for UnorderedMap<K,V> {
    fn close_step(&mut self,_:usize)->RetirementStep {self.0.next().map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<K: RetireOwned,V: RetireOwned> Drop for UnorderedMap<K,V> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned map retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)};}
}
impl<K: RetireOwned + std::hash::Hash + Eq,V: RetireOwned> RetireOwned for std::collections::HashMap<K,V> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(UnorderedMap(ManuallyDrop::new(self.into_iter())))}
}
struct OrderedSet<T: RetireOwned + Ord>(ManuallyDrop<std::collections::BTreeSet<T>>);
impl<T: RetireOwned + Ord> RetirementCursor for OrderedSet<T> {
    fn close_step(&mut self,_:usize)->RetirementStep {self.0.pop_first().map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.0.is_empty()}
}
impl<T: RetireOwned + Ord> Drop for OrderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.0.is_empty(),"owned ordered set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)};}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BTreeSet<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(OrderedSet(ManuallyDrop::new(self)))}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BinaryHeap<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.into_vec().retirement()}
}
impl<T: RetireOwned> RetireOwned for std::cmp::Reverse<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.0.retirement()}
}
impl<A: RetireOwned,B: RetireOwned,C: RetireOwned,D: RetireOwned> RetireOwned for (A,B,C,D) {
    fn retirement(self)->Box<dyn RetirementCursor> {artifact_retirement_sequence![self.0,self.1,self.2,self.3]}
}

struct OrderedMap<K: RetireOwned + Ord, V: RetireOwned>(ManuallyDrop<std::collections::BTreeMap<K, V>>);
impl<K: RetireOwned + Ord, V: RetireOwned> RetirementCursor for OrderedMap<K, V> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop_first().map_or(RetirementStep::Complete, |entry| RetirementStep::Child(entry.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<K: RetireOwned + Ord, V: RetireOwned> Drop for OrderedMap<K, V> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned ordered map retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl<K: RetireOwned + Ord, V: RetireOwned> RetireOwned for std::collections::BTreeMap<K, V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(OrderedMap(ManuallyDrop::new(self)))
    }
}
impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.map_or_else(|| sequence(Vec::new()), RetireOwned::retirement)
    }
}
impl<T: RetireOwned> RetireOwned for Box<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        (*self).retirement()
    }
}
impl<T: RetireOwned, U: RetireOwned> RetireOwned for (T, U) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(self.0), deferred(self.1)])
    }
}
impl<T: RetireOwned, U: RetireOwned, V: RetireOwned> RetireOwned for (T, U, V) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(self.0), deferred(self.1), deferred(self.2)])
    }
}
impl<T: Copy + Send + 'static, const N: usize> RetireOwned for [T; N] {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        leaf(self)
    }
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Drop for Sequence {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned field sequence retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
pub fn sequence(fields: Vec<Box<dyn RetirementCursor>>) -> Box<dyn RetirementCursor> {
    Box::new(Sequence(ManuallyDrop::new(fields)))
}

struct Deferred<T: RetireOwned>(Option<T>);
impl<T: RetireOwned> RetirementCursor for Deferred<T> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.take().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl<T: RetireOwned> Drop for Deferred<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_none(), "deferred owned value retired before terminal-empty");
    }
}
pub fn deferred<T: RetireOwned>(value: T) -> Box<dyn RetirementCursor> {
    Box::new(Deferred(Some(value)))
}

struct ValueRetirement(ManuallyDrop<Option<crate::DslValue>>);
impl RetirementCursor for ValueRetirement {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        match self.0.take() {
            None | Some(crate::DslValue::Null) => RetirementStep::Complete,
            Some(crate::DslValue::Bool(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Number(value)) => RetirementStep::Child(match value {
                crate::Number::UInt(value) => value.retirement(),
                crate::Number::Int(value) => value.retirement(),
                crate::Number::Float(value) => value.retirement(),
            }),
            Some(crate::DslValue::String(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Bytes(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Array(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Object(value)) => RetirementStep::Child(value.retirement()),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl Drop for ValueRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_none(), "dynamic value retired before terminal-empty");
    }
}
impl RetireOwned for crate::DslValue {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(ValueRetirement(ManuallyDrop::new(Some(self))))
    }
}

struct CursorStack(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl CursorStack {
    fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        let (mut turns, mut released_items, mut released_bytes) = (0, 0, 0);
        while turns < maximum_items {
            let Some(cursor) = self.0.last_mut() else { break };
            match cursor.close_step(maximum_bytes - released_bytes) {
                RetirementStep::Child(child) => self.0.push(child),
                RetirementStep::Bytes(bytes) if bytes <= maximum_bytes - released_bytes => released_bytes += bytes,
                RetirementStep::Bytes(_) => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "owned retirement exceeded its exact byte grant")),
                RetirementStep::Complete => {
                    if !cursor.terminal_is_empty() {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "owned retirement reported Complete without a terminal-empty witness"));
                    }
                    self.0.pop();
                    released_items += 1;
                }
                RetirementStep::BudgetExhausted => {
                    if released_items == 0 && released_bytes == 0 && cursor.next_close_byte_demand().is_some_and(|demand| demand > maximum_bytes) {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "owned retirement byte grant is smaller than its next physical release"));
                    }
                    break;
                }
            }
            turns += 1;
        }
        Ok(if self.0.is_empty() && released_items == 0 && released_bytes == 0 { SnapshotRetirementStep::Complete } else { SnapshotRetirementStep::Pending { released_items, released_bytes } })
    }

    fn next_close_byte_demand(&self) -> usize {
        self.0.last().and_then(|cursor| cursor.next_close_byte_demand()).unwrap_or(1)
    }
}
impl Drop for CursorStack {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned cursor stack retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}

struct OwnedRetirement<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    cursors: CursorStack,
}
impl<T: RetireOwned> ErasedSnapshotRetirement for OwnedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            self.cursors.0.push(value.retirement());
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.cursors.step(maximum_items, maximum_bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.cursors.0.is_empty()
    }

    fn next_close_byte_demand(&self) -> usize {
        if self.value.is_some() { 1 } else { self.cursors.next_close_byte_demand() }
    }
}
impl<T: RetireOwned> Drop for OwnedRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned value retired before terminal-empty");
    }
}
pub fn owned_retirement<T: RetireOwned>(value: T) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(OwnedRetirement { value: ManuallyDrop::new(Some(value)), cursors: CursorStack(ManuallyDrop::new(Vec::new())) })
}

struct ErasedCursor(Box<dyn ErasedSnapshotRetirement>);
impl RetirementCursor for ErasedCursor {
    fn close_step(&mut self,maximum_bytes:usize)->RetirementStep {if maximum_bytes==0 {return RetirementStep::BudgetExhausted;}match self.0.close_step(1,maximum_bytes).expect("typed nested retirement") {SnapshotRetirementStep::Pending {released_bytes,..}=>RetirementStep::Bytes(released_bytes),SnapshotRetirementStep::Complete=>RetirementStep::Complete,SnapshotRetirementStep::Blocked=>RetirementStep::BudgetExhausted}}
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
}
/// ♻️ Transfers an existing retirement frontier into its typed parent frontier.
pub fn erased_cursor(value:Box<dyn ErasedSnapshotRetirement>)->Box<dyn RetirementCursor> {Box::new(ErasedCursor(value))}

struct SharedRetirement<T: RetireOwned + Sync> {
    release_lease: bool,
    value: ManuallyDrop<Option<Arc<T>>>,
    owned: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl<T: RetireOwned + Sync> ErasedSnapshotRetirement for SharedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if maximum_items == 0 || (self.release_lease && maximum_bytes == 0) {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            if self.release_lease {
                if let Some(value) = Arc::into_inner(value) { *self.owned = Some(owned_retirement(value)); }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            match Arc::try_unwrap(value) {
                Ok(value) => *self.owned = Some(owned_retirement(value)),
                Err(shared) => {
                    *self.value = Some(shared);
                    return Ok(SnapshotRetirementStep::Blocked);
                }
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owned) = self.owned.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        match owned.close_step(maximum_items, maximum_bytes)? {
            SnapshotRetirementStep::Complete => {
                if !owned.terminal_is_empty() {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "shared retirement lacks its nested terminal witness"));
                }
                self.owned.take();
                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            step => Ok(step),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.owned.is_none()
    }
}
impl<T: RetireOwned + Sync> Drop for SharedRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "shared value retired before terminal-empty");
    }
}
pub fn shared_retirement<T: RetireOwned + Sync>(value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(SharedRetirement { release_lease: false, value: ManuallyDrop::new(Some(value)), owned: ManuallyDrop::new(None) })
}

/// 🔗️ Consumes one immutable lease, transferring only its last payload to bounded owned retirement.
pub fn shared_lease_retirement<T: RetireOwned + Sync>(value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(SharedRetirement { release_lease: true, value: ManuallyDrop::new(Some(value)), owned: ManuallyDrop::new(None) })
}

pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T: RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn ErasedSnapshotRetirement> {
        owned_retirement(value)
    }
}
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T: RetireOwned + Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retire(&self, value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
        shared_retirement(value)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="📦️allocation-return/🦀️.rs"]
pub mod allocation_return;
