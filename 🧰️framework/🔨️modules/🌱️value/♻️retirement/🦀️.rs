//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

pub trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
    fn retirement_birth_bytes(&self) -> Option<usize> { None }
    fn controlled_retirement_supported() -> bool { false }
}

pub enum RetirementStep {
    Child(Box<dyn RetirementCursor>),
    Bytes(usize),
    ProcessedBytes(usize),
    Complete,
    BudgetExhausted,
}

pub trait RetirementCursor: Send {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep;
    fn terminal_is_empty(&self) -> bool;
    fn next_close_byte_demand(&self) -> Option<usize> {
        None
    }
    fn next_work_byte_demand(&self) -> usize { 0 }
    fn next_birth_bytes(&self, _maximum_bytes: usize) -> Option<usize> { None }
    fn terminal_release_bytes(&self) -> Option<usize> { None }
}

/// 🧮️ Measures the fixed constructor allocations of a deferred field sequence.
pub fn sequence_birth_bytes(fields: &[usize]) -> Option<usize> {
    fields.iter().try_fold(size_of::<Sequence>().checked_add(fields.len().checked_mul(size_of::<Box<dyn RetirementCursor>>())?)?, |total, bytes| total.checked_add(*bytes))
}

/// 🧮️ Measures the exact inline ownership retained by one deferred field Box.
pub const fn deferred_birth_bytes<T: RetireOwned>() -> usize { size_of::<Deferred<T>>() }

/// 🧮️ Borrows the exact field type when a declarative owner measures its deferred scaffold.
pub fn deferred_birth_bytes_for<T: RetireOwned>(_: &T) -> usize { deferred_birth_bytes::<T>() }

/// 🧮️ Measures the exact scalar retirement scaffold allocation.
pub const fn leaf_birth_bytes<T: Copy + Send + 'static>() -> usize { size_of::<Leaf<T>>() }

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
            return RetirementStep::ProcessedBytes(bytes);
        }
        self.value.take();
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.remaining == 0
    }
    fn next_work_byte_demand(&self) -> usize { usize::from(self.remaining != 0) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(usize::from(self.remaining != 0)) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

pub fn leaf<T: Copy + Send + 'static>(value: T) -> Box<dyn RetirementCursor> {
    Box::new(Leaf { value: Some(value), remaining: size_of::<T>() })
}

#[macro_export]
macro_rules! artifact_retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl $crate::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::retirement::RetirementCursor> { $crate::retirement::leaf(self) }
            fn retirement_birth_bytes(&self) -> Option<usize> { Some($crate::retirement::leaf_birth_bytes::<$type>()) }
            fn controlled_retirement_supported() -> bool { true }
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
                $crate::retirement::sequence(vec![$($crate::retirement::deferred($field)),+])
            }
            fn retirement_birth_bytes(&self) -> Option<usize> {
                $crate::retirement::sequence_birth_bytes(&[$($crate::retirement::deferred_birth_bytes_for(&self.$field)),+])
            }
            fn controlled_retirement_supported() -> bool { true }
        }
    };
}

artifact_retire_leaf!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

struct Bytes(ManuallyDrop<Vec<u8>>, bool);
impl RetirementCursor for Bytes {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if !self.0.is_empty() {
            if maximum_bytes == 0 { return RetirementStep::BudgetExhausted; }
            let bytes=maximum_bytes.min(self.0.len());
            let next=self.0.len()-bytes;
            self.0.truncate(next);
            return RetirementStep::ProcessedBytes(bytes);
        }
        let bytes=self.0.capacity();
        if bytes!=0 {
            if bytes>maximum_bytes { return RetirementStep::BudgetExhausted; }
            drop(std::mem::take(&mut *self.0));
            return RetirementStep::Bytes(bytes);
        }
        self.1=true;
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.1 }
    fn next_work_byte_demand(&self) -> usize { usize::from(!self.0.is_empty()) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(if self.0.is_empty(){self.0.capacity()}else{1}) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
impl Drop for Bytes {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.1, "owned bytes retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl RetireOwned for String {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(Bytes(ManuallyDrop::new(self.into_bytes()), false)) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Bytes>()) }
    fn controlled_retirement_supported() -> bool { true }
}

/// ♻️ Drains trivial elements as bounded logical work; the original backing allocation remains whole until terminal release.
struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if !std::mem::needs_drop::<T>() && !self.0.is_empty() {
            let width = size_of::<T>();
            if width == 0 {
                self.0.clear();
                return RetirementStep::ProcessedBytes(0);
            }
            let count = (maximum_bytes / width).min(self.0.len());
            if count > 0 {
                let next = self.0.len() - count;
                self.0.truncate(next);
                return RetirementStep::ProcessedBytes(count * width);
            }
        }
        if self.0.is_empty() && self.0.capacity()!=0 && size_of::<T>()!=0 {
            let bytes=self.0.capacity()*size_of::<T>();
            if bytes>maximum_bytes{return RetirementStep::BudgetExhausted;}
            drop(std::mem::take(&mut*self.0));return RetirementStep::Bytes(bytes);
        }
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty() && (self.0.capacity()==0 || size_of::<T>()==0)
    }
    fn next_close_byte_demand(&self)->Option<usize>{Some(if self.0.is_empty(){self.0.capacity()*size_of::<T>()}else{1})}
    fn next_work_byte_demand(&self) -> usize { if !std::mem::needs_drop::<T>() && !self.0.is_empty() { size_of::<T>() } else { 0 } }
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> {
        if self.0.is_empty() || (!std::mem::needs_drop::<T>() && (size_of::<T>() == 0 || maximum_bytes >= size_of::<T>())) { Some(0) }
        else { self.0.last()?.retirement_birth_bytes() }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
impl<T: RetireOwned> Drop for Collection<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned collection retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl<T: RetireOwned> RetireOwned for Vec<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Collection(ManuallyDrop::new(self)))
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Collection<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

struct DequeCollection<T: RetireOwned>(ManuallyDrop<std::collections::VecDeque<T>>);
impl<T: RetireOwned> RetirementCursor for DequeCollection<T> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop_front().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool { self.0.is_empty() }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        self.0.front().map_or(Some(0), RetireOwned::retirement_birth_bytes)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        size_of::<Self>().checked_add(self.0.capacity().checked_mul(size_of::<T>())?)
    }
}
impl<T: RetireOwned> Drop for DequeCollection<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned deque retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) }
    }
}
impl<T: RetireOwned> RetireOwned for std::collections::VecDeque<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(DequeCollection(ManuallyDrop::new(self))) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<DequeCollection<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

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
    fn retirement_birth_bytes(&self) -> Option<usize> { self.as_ref().map_or_else(|| sequence_birth_bytes(&[]), RetireOwned::retirement_birth_bytes) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}
struct BoxedOwner<T: RetireOwned> {
    boxed: ManuallyDrop<Option<Box<T>>>,
    value: ManuallyDrop<Option<T>>,
}
impl<T: RetireOwned> RetirementCursor for BoxedOwner<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.boxed.is_some() {
            let bytes = size_of::<T>();
            if bytes > maximum_bytes { return RetirementStep::BudgetExhausted; }
            *self.value = Some(*self.boxed.take().unwrap());
            return RetirementStep::Bytes(bytes);
        }
        if let Some(value) = self.value.take() { return RetirementStep::Child(value.retirement()); }
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.boxed.is_none() && self.value.is_none() }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(if self.boxed.is_some() { size_of::<T>() } else { 0 }) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { self.value.as_ref().map_or(Some(0), RetireOwned::retirement_birth_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl<T: RetireOwned> Drop for BoxedOwner<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "boxed retirement abandoned before terminal-empty");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.boxed); ManuallyDrop::drop(&mut self.value); } }
    }
}
impl<T: RetireOwned> RetireOwned for Box<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(BoxedOwner { boxed: ManuallyDrop::new(Some(self)), value: ManuallyDrop::new(None) }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<BoxedOwner<T>>()) }
    fn controlled_retirement_supported() -> bool { true }
}
impl<T: RetireOwned, U: RetireOwned> RetireOwned for (T, U) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(self.0), deferred(self.1)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { sequence_birth_bytes(&[deferred_birth_bytes::<T>(), deferred_birth_bytes::<U>()]) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() && U::controlled_retirement_supported() }
}
impl<T: RetireOwned, U: RetireOwned, V: RetireOwned> RetireOwned for (T, U, V) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(self.0), deferred(self.1), deferred(self.2)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { sequence_birth_bytes(&[deferred_birth_bytes::<T>(), deferred_birth_bytes::<U>(), deferred_birth_bytes::<V>()]) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() && U::controlled_retirement_supported() && V::controlled_retirement_supported() }
}
impl<T: Copy + Send + 'static, const N: usize> RetireOwned for [T; N] {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        leaf(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(leaf_birth_bytes::<Self>()) }
    fn controlled_retirement_supported() -> bool { true }
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { size_of::<Self>().checked_add(self.0.capacity().checked_mul(size_of::<Box<dyn RetirementCursor>>())?) }
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
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { self.0.as_ref().map_or(Some(0), RetireOwned::retirement_birth_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
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
        if self.0.is_empty() && self.0.capacity()!=0 {
            let bytes=self.0.capacity()*size_of::<Box<dyn RetirementCursor>>();
            if maximum_items==0 || bytes>maximum_bytes { return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}); }
            drop(std::mem::take(&mut *self.0));
            return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:bytes});
        }
        let (mut turns, mut released_items, mut released_bytes, mut processed_bytes) = (0, 0, 0, 0);
        while turns < maximum_items {
            let Some(cursor) = self.0.last_mut() else { break };
            if cursor.terminal_is_empty() {
                let bytes=cursor.terminal_release_bytes().unwrap_or_else(||size_of_val(cursor.as_ref()));
                if bytes>maximum_bytes-released_bytes-processed_bytes { break; }
                drop(self.0.pop());
                released_items+=1;released_bytes+=bytes;turns+=1;
                continue;
            }
            match cursor.close_step(maximum_bytes - released_bytes - processed_bytes) {
                RetirementStep::Child(child) => self.0.push(child),
                RetirementStep::Bytes(bytes) if bytes <= maximum_bytes - released_bytes - processed_bytes => released_bytes += bytes,
                RetirementStep::ProcessedBytes(bytes) if bytes <= maximum_bytes - released_bytes - processed_bytes => processed_bytes += bytes,
                RetirementStep::ProcessedBytes(_) => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "owned retirement exceeded its exact work byte grant")),
                RetirementStep::Bytes(_) => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "owned retirement exceeded its exact byte grant")),
                RetirementStep::Complete => {
                    if !cursor.terminal_is_empty() {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "owned retirement reported Complete without a terminal-empty witness"));
                    }
                    let bytes=cursor.terminal_release_bytes().unwrap_or_else(||size_of_val(cursor.as_ref()));
                    if bytes>maximum_bytes-released_bytes-processed_bytes { break; }
                    drop(self.0.pop());
                    released_items += 1;released_bytes+=bytes;
                }
                RetirementStep::BudgetExhausted => {
                    if maximum_bytes != 0 && released_items == 0 && released_bytes == 0 && processed_bytes == 0 && cursor.next_close_byte_demand().is_some_and(|demand| demand > maximum_bytes) {
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
        self.0.last().map_or_else(||self.0.capacity()*size_of::<Box<dyn RetirementCursor>>(),|cursor|if cursor.terminal_is_empty(){cursor.terminal_release_bytes().unwrap_or_else(||size_of_val(cursor.as_ref()))}else{cursor.next_close_byte_demand().unwrap_or(1).max(usize::from(cursor.next_work_byte_demand()!=0))})
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
        if maximum_items == 0 || maximum_bytes < self.next_close_byte_demand() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.value.is_some() {
            self.cursors.0.try_reserve_exact(1).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "owned retirement cursor slot allocation failed"))?;
        }
        if let Some(value) = self.value.take() {
            self.cursors.0.push(value.retirement());
            if self.cursors.0.last().is_some_and(|cursor|cursor.next_birth_bytes(maximum_bytes)==Some(0)) && self.cursors.next_close_byte_demand()<=maximum_bytes {
                return self.cursors.step(maximum_items,maximum_bytes);
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.cursors.step(maximum_items, maximum_bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.cursors.0.is_empty() && self.cursors.0.capacity()==0
    }

    fn next_close_byte_demand(&self) -> usize {
        self.value.as_ref().map_or_else(|| self.cursors.next_close_byte_demand(), |value| value.retirement_birth_bytes().unwrap_or(0).saturating_add(size_of::<Box<dyn RetirementCursor>>()).max(1))
    }
}
impl<T: RetireOwned> Drop for OwnedRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned value retired before terminal-empty");
    }
}
/// 🧱️ Borrows the exact typed erased ownership frame before its original value moves.
pub const fn owned_retirement_birth_bytes<T: RetireOwned>() -> usize { size_of::<OwnedRetirement<T>>() }
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
    fn next_close_byte_demand(&self) -> usize {
        if let Some(value) = self.value.as_ref() {
            return if Arc::strong_count(value) == 1 {
                std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<T>()).expect("shared retirement Arc layout").0.pad_to_align().size().saturating_add(size_of::<OwnedRetirement<T>>())
            } else { 1 };
        }
        self.owned.as_ref().map_or(0, |owned| if owned.terminal_is_empty() { size_of_val(owned.as_ref()) } else { owned.next_close_byte_demand() })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if maximum_items == 0 || (self.release_lease && maximum_bytes == 0) || maximum_bytes < self.next_close_byte_demand() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            let arc_bytes = std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<T>()).expect("shared retirement Arc layout").0.pad_to_align().size();
            if self.release_lease {
                let released_bytes = if let Some(value) = Arc::into_inner(value) { *self.owned = Some(owned_retirement(value)); arc_bytes } else { 0 };
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
            }
            match Arc::try_unwrap(value) {
                Ok(value) => *self.owned = Some(owned_retirement(value)),
                Err(shared) => {
                    *self.value = Some(shared);
                    return Ok(SnapshotRetirementStep::Blocked);
                }
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: arc_bytes });
        }
        let Some(owned) = self.owned.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        if owned.terminal_is_empty() {
            let released_bytes = size_of_val(owned.as_ref());
            drop(self.owned.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        match owned.close_step(maximum_items, maximum_bytes)? {
            SnapshotRetirementStep::Complete => {
                if !owned.terminal_is_empty() {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "shared retirement lacks its nested terminal witness"));
                }
                Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
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
/// 🧮️ Exact borrowed admission for the shared retirement cursor frame.
pub fn shared_retirement_birth_bytes<T: RetireOwned + Sync>() -> usize { std::mem::size_of::<SharedRetirement<T>>() }

pub fn shared_retirement<T: RetireOwned + Sync>(value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(SharedRetirement { release_lease: false, value: ManuallyDrop::new(Some(value)), owned: ManuallyDrop::new(None) })
}

/// 🔗️ Consumes one immutable lease, transferring only its last payload to bounded owned retirement.
pub fn shared_lease_retirement<T: RetireOwned + Sync>(value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(SharedRetirement { release_lease: true, value: ManuallyDrop::new(Some(value)), owned: ManuallyDrop::new(None) })
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
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
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T: RetireOwned + Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<T>) -> usize { shared_retirement_birth_bytes::<T>() }

    fn retire(&self, value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
        shared_retirement(value)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="📦️allocation-return/🦀️.rs"]
pub mod allocation_return;

#[path = "🎮️controlled/🦀️.rs"]
pub mod controlled;
