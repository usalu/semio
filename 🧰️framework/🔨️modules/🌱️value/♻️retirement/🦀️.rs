//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use crate::retained_clone::RetainedCloneGrant;
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

#[path="📋️original-vector/🦀️.rs"]
pub mod original_vector;

#[path="⚖️result/🦀️.rs"]
mod original_result;

pub trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
    fn retirement_birth_bytes(&self) -> Option<usize> { None }
    fn controlled_retirement_supported() -> bool { false }
    /// 🧮️ Declares an element's complete inline payload work before a collection discards it.
    fn retirement_element_copy_bytes() -> usize { 0 }
}

pub enum RetirementStep {
    Progress(crate::retained_clone::RetainedCloneProgress),
    Child(Box<dyn RetirementCursor>),
    ChildProgress(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress),
    Bytes(usize),
    ProcessedBytes(usize),
    Advanced,
    Failure(crate::ValueError),
    Complete,
    BudgetExhausted,
}

pub trait RetirementCursor: Send {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep;
    fn terminal_is_empty(&self) -> bool;
    /// 🪆️ Admits the next structural cursor slot before a typed owner can return it.
    fn next_depth_demand(&self) -> Result<usize, crate::ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
    fn next_close_byte_demand(&self) -> Option<usize> {
        None
    }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(0)}
    /// 🪆️ Declares a separately funded child when a complete inline item exceeds this work grant.
    fn allows_admitted_narrow_work(&self) -> bool { false }
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
}
#[cfg(test)]
#[path="🍃️leaf/🧪️tests/🦀️.rs"]
mod original_leaf_tests;
impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return RetirementStep::BudgetExhausted; }
        self.value.take();
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(0)}
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

pub fn leaf<T: Copy + Send + 'static>(value: T) -> Box<dyn RetirementCursor> {
    Box::new(Leaf { value: Some(value) })
}

#[macro_export]
macro_rules! artifact_retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl $crate::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::retirement::RetirementCursor> { $crate::retirement::leaf(self) }
            fn retirement_birth_bytes(&self) -> Option<usize> { Some($crate::retirement::leaf_birth_bytes::<$type>()) }
            fn controlled_retirement_supported() -> bool { true }
            fn retirement_element_copy_bytes() -> usize { std::mem::size_of::<$type>() }
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

artifact_retire_leaf!(&'static str, (), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

struct Bytes(ManuallyDrop<Vec<u8>>, bool);
#[cfg(test)]
#[path="📦️bytes/🧪️tests/🦀️.rs"]
mod original_bytes_tests;
impl RetirementCursor for Bytes {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return RetirementStep::BudgetExhausted; }
        if !self.0.is_empty() {
            self.0.truncate(0);
            return RetirementStep::Advanced;
        }
        let bytes=self.0.capacity();
        if bytes!=0 {
            if bytes>grant.maximum_release_bytes { return RetirementStep::BudgetExhausted; }
            drop(std::mem::take(&mut *self.0));
            return RetirementStep::Bytes(bytes);
        }
        self.1=true;
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.1 }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(0)}
    fn next_close_byte_demand(&self) -> Option<usize> { Some(if self.0.is_empty(){self.0.capacity()}else{0}) }
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
    fn retirement(self) -> Box<dyn RetirementCursor> { bytes(self.into_bytes()) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Bytes>()) }
    fn controlled_retirement_supported() -> bool { true }
}
impl RetireOwned for std::path::PathBuf {
    fn retirement(self) -> Box<dyn RetirementCursor> { bytes(self.into_os_string().into_encoded_bytes()) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Bytes>()) }
    fn controlled_retirement_supported() -> bool { true }
}
/// 📦️ Quotes the actual byte-buffer cursor birth independently of any UTF8 completeness.
pub const fn bytes_birth_bytes()->usize{size_of::<Bytes>()}
/// 🧳️ Retains the original byte allocation after its receiving parent admits this natural cursor birth.
pub fn bytes(original:Vec<u8>)->Box<dyn RetirementCursor>{Box::new(Bytes(ManuallyDrop::new(original),false))}

/// ♻️ Drains trivial elements as bounded logical work; the original backing allocation remains whole until terminal release.
struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = if self.0.is_empty() { grant.maximum_release_bytes } else { grant.maximum_copy_bytes };
        if !std::mem::needs_drop::<T>() && T::retirement_element_copy_bytes() != 0 && !self.0.is_empty() {
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
    fn next_close_byte_demand(&self)->Option<usize>{Some(if self.0.is_empty(){self.0.capacity()*size_of::<T>()}else{0})}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(if !std::mem::needs_drop::<T>()&&!self.0.is_empty(){T::retirement_element_copy_bytes()}else{0})}
    fn allows_admitted_narrow_work(&self)->bool{!std::mem::needs_drop::<T>()&&!self.0.is_empty()&&T::retirement_element_copy_bytes()!=0}
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> {
        if self.0.is_empty() || (!std::mem::needs_drop::<T>() && T::retirement_element_copy_bytes() != 0 && maximum_bytes >= T::retirement_element_copy_bytes()) { Some(0) }
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
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
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
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}if self.0.len()==0 {return RetirementStep::Complete;}RetirementStep::Child(self.0.next_back().unwrap().retirement())}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<T:RetireOwned> Drop for VectorIterator<T> {fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned iterator retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)}}}
impl<T:RetireOwned> RetireOwned for std::vec::IntoIter<T> {fn retirement(self)->Box<dyn RetirementCursor> {Box::new(VectorIterator(ManuallyDrop::new(self)))}}

/// 🧺️ Drains one hashed element per turn; the standard table backing is not priced, only the element scaffolds and this cursor shell.
struct UnorderedSet<T: RetireOwned>{head:ManuallyDrop<Option<T>>,rest:ManuallyDrop<std::collections::hash_set::IntoIter<T>>}
impl<T: RetireOwned> UnorderedSet<T> {
    fn new(mut rest:std::collections::hash_set::IntoIter<T>)->Self {Self{head:ManuallyDrop::new(rest.next()),rest:ManuallyDrop::new(rest)}}
}
impl<T: RetireOwned> RetirementCursor for UnorderedSet<T> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}let head=self.head.take();*self.head=self.rest.next();head.map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.head.is_none()}
    fn next_birth_bytes(&self,_:usize)->Option<usize> {self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
}
impl<T: RetireOwned> Drop for UnorderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<T: RetireOwned + std::hash::Hash + Eq> RetireOwned for std::collections::HashSet<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(UnorderedSet::new(self.into_iter()))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedSet<T>>())}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}

/// 🧺️ Drains one hashed entry per turn; the standard table backing is not priced, only the entry scaffolds and this cursor shell.
struct UnorderedMap<K: RetireOwned,V: RetireOwned>{head:ManuallyDrop<Option<(K,V)>>,rest:ManuallyDrop<std::collections::hash_map::IntoIter<K,V>>}
impl<K: RetireOwned,V: RetireOwned> UnorderedMap<K,V> {
    fn new(mut rest:std::collections::hash_map::IntoIter<K,V>)->Self {Self{head:ManuallyDrop::new(rest.next()),rest:ManuallyDrop::new(rest)}}
}
impl<K: RetireOwned,V: RetireOwned> RetirementCursor for UnorderedMap<K,V> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}let head=self.head.take();*self.head=self.rest.next();head.map_or(RetirementStep::Complete,|entry|RetirementStep::Child(entry.retirement()))}
    fn terminal_is_empty(&self)->bool {self.head.is_none()}
    fn next_birth_bytes(&self,_:usize)->Option<usize> {self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
}
impl<K: RetireOwned,V: RetireOwned> Drop for UnorderedMap<K,V> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned map retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<K: RetireOwned + std::hash::Hash + Eq,V: RetireOwned> RetireOwned for std::collections::HashMap<K,V> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(UnorderedMap::new(self.into_iter()))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedMap<K,V>>())}
    fn controlled_retirement_supported()->bool {K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}

/// 🧺️ Drains one ordered element per turn; the standard node backing is not priced, only the element scaffolds and this cursor shell.
struct OrderedSet<T: RetireOwned + Ord>(ManuallyDrop<std::collections::BTreeSet<T>>);
impl<T: RetireOwned + Ord> RetirementCursor for OrderedSet<T> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}self.0.pop_first().map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.0.is_empty()}
    fn next_birth_bytes(&self,_:usize)->Option<usize> {self.0.first().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
}
impl<T: RetireOwned + Ord> Drop for OrderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.0.is_empty(),"owned ordered set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)};}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BTreeSet<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(OrderedSet(ManuallyDrop::new(self)))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<OrderedSet<T>>())}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BinaryHeap<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.into_vec().retirement()}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Collection<T>>())}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}
impl<T: RetireOwned> RetireOwned for std::cmp::Reverse<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.0.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize> {self.0.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
    fn retirement_element_copy_bytes()->usize {T::retirement_element_copy_bytes()}
}

/// 🧺️ Drains one ordered entry per turn; the standard node backing is not priced, only the entry scaffolds and this cursor shell.
struct OrderedMap<K: RetireOwned + Ord, V: RetireOwned>(ManuallyDrop<std::collections::BTreeMap<K, V>>);
impl<K: RetireOwned + Ord, V: RetireOwned> RetirementCursor for OrderedMap<K, V> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        self.0.pop_first().map_or(RetirementStep::Complete, |entry| RetirementStep::Child(entry.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        if self.0.is_empty() { Some(0) } else { sequence_birth_bytes(&[deferred_birth_bytes::<K>(), deferred_birth_bytes::<V>()]) }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
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
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<OrderedMap<K, V>>()) }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() && V::controlled_retirement_supported() }
}
impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Optional { original:ManuallyDrop::new(Some(self)) })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Optional<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}
struct Optional<T:RetireOwned>{original:ManuallyDrop<Option<Option<T>>>}
impl<T:RetireOwned> RetirementCursor for Optional<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
 if grant.maximum_items==0{return RetirementStep::BudgetExhausted}if self.original.is_none(){return RetirementStep::Complete}
  let Some(birth)=self.next_birth_bytes(grant.maximum_copy_bytes)else{return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"optional original lacks controlled child birth"))};if grant.maximum_capacity_bytes<birth{return RetirementStep::BudgetExhausted}
  if grant.maximum_depth<self.next_depth_demand().unwrap_or(usize::MAX){return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"optional original child exceeds admitted depth"))}
  let progress=crate::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()};
  match self.original.take().unwrap(){Some(value)=>RetirementStep::ChildProgress(value.retirement(),progress),None=>RetirementStep::Progress(progress)}
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{self.original.as_ref().and_then(Option::as_ref).map_or(Some(0),RetireOwned::retirement_birth_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(if self.original.as_ref().is_some_and(Option::is_some){2}else{usize::from(self.original.is_some())})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Drop for Optional<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"optional original abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}
/// 📏️ Prices the concrete cursor that retains an original typed box.
pub const fn boxed_owner_birth_bytes<T:RetireOwned>()->usize{size_of::<BoxedOwner<T>>()}
struct BoxedOwner<T: RetireOwned> {
    boxed: ManuallyDrop<Option<Box<T>>>,
    value: ManuallyDrop<Option<T>>,
}
impl<T: RetireOwned> RetirementCursor for BoxedOwner<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = grant.maximum_release_bytes;
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
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}
macro_rules! retire_tuple {
    ($($type:ident:$index:tt),+) => {
        impl<$($type:RetireOwned),+> RetireOwned for ($($type,)+) {
            fn retirement(self)->Box<dyn RetirementCursor> {sequence(vec![$(deferred(self.$index)),+])}
            fn retirement_birth_bytes(&self)->Option<usize> {sequence_birth_bytes(&[$(deferred_birth_bytes::<$type>()),+])}
            fn controlled_retirement_supported()->bool {true$(&&$type::controlled_retirement_supported())+}
        }
    };
}
retire_tuple!(A:0,B:1);
retire_tuple!(A:0,B:1,C:2);
retire_tuple!(A:0,B:1,C:2,D:3);
retire_tuple!(A:0,B:1,C:2,D:3,E:4);

/// 🧱️ Drains a fixed array of owning elements one child cursor per turn, last element first.
struct FixedArray<T: RetireOwned, const N: usize> {
    items: ManuallyDrop<[Option<T>; N]>,
}
impl<T: RetireOwned, const N: usize> RetirementCursor for FixedArray<T, N> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        self.items.iter_mut().rev().find_map(Option::take).map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool { self.items.iter().all(Option::is_none) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { self.items.iter().rev().flatten().next().map_or(Some(0), RetireOwned::retirement_birth_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl<T: RetireOwned, const N: usize> Drop for FixedArray<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned fixed array retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.items) };
    }
}
fn plain_elements<T: RetireOwned>() -> bool { !std::mem::needs_drop::<T>() && T::retirement_element_copy_bytes() != 0 }

/// 🧱️ Drains a fixed array of trivial elements as bounded logical copy work, as [`Collection`] does, instead of one child cursor per element.
struct PlainArray<T: RetireOwned, const N: usize> {
    items: ManuallyDrop<[T; N]>,
    remaining: usize,
}
impl<T: RetireOwned, const N: usize> RetirementCursor for PlainArray<T, N> {
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        if self.remaining == 0 { return RetirementStep::Complete; }
        let width = size_of::<T>();
        if width == 0 {
            self.remaining = 0;
            return RetirementStep::ProcessedBytes(0);
        }
        let count = (grant.maximum_copy_bytes / width).min(self.remaining);
        if count == 0 { return RetirementStep::BudgetExhausted; }
        self.remaining -= count;
        RetirementStep::ProcessedBytes(count * width)
    }
    fn terminal_is_empty(&self) -> bool { self.remaining == 0 }
    fn next_work_byte_demand(&self) -> Result<usize, crate::ValueError> { Ok(if self.remaining != 0 { T::retirement_element_copy_bytes() } else { 0 }) }
    fn allows_admitted_narrow_work(&self) -> bool { self.remaining != 0 }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl<T: RetireOwned, const N: usize> Drop for PlainArray<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned plain array retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.items) };
    }
}
impl<T: RetireOwned, const N: usize> RetireOwned for [T; N] {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        if plain_elements::<T>() { return Box::new(PlainArray { items: ManuallyDrop::new(self), remaining: N }); }
        Box::new(FixedArray { items: ManuallyDrop::new(self.map(Some)) })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(if plain_elements::<T>() { size_of::<PlainArray<T, N>>() } else { size_of::<FixedArray<T, N>>() }) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
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

struct Deferred<T:RetireOwned>{original:ManuallyDrop<Option<T>>}
impl<T:RetireOwned> RetirementCursor for Deferred<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}if self.original.is_none(){return RetirementStep::Complete}
  let Some(birth)=self.next_birth_bytes(grant.maximum_copy_bytes)else{return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"deferred original lacks controlled child birth"))};if grant.maximum_capacity_bytes<birth{return RetirementStep::BudgetExhausted}
  if grant.maximum_depth<2{return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"deferred original child exceeds admitted depth"))}
  let value=self.original.take().unwrap();RetirementStep::ChildProgress(value.retirement(),crate::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()})
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(if self.original.is_some(){2}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Drop for Deferred<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"deferred original abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}
pub fn deferred<T:RetireOwned>(value:T)->Box<dyn RetirementCursor>{Box::new(Deferred{original:ManuallyDrop::new(Some(value))})}

struct ValueRetirement(ManuallyDrop<Option<crate::DslValue>>);
impl RetirementCursor for ValueRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
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
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        match self.0.as_ref() {
            None | Some(crate::DslValue::Null) => Some(0),
            Some(crate::DslValue::Bool(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Number(value)) => match value {
                crate::Number::UInt(value) => value.retirement_birth_bytes(),
                crate::Number::Int(value) => value.retirement_birth_bytes(),
                crate::Number::Float(value) => value.retirement_birth_bytes(),
            },
            Some(crate::DslValue::String(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Bytes(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Array(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Object(value)) => value.retirement_birth_bytes(),
        }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
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
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<ValueRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
}

/// 🧱️ Borrows the exact erased ownership frame before its original value moves.
pub const fn owned_retirement_birth_bytes<T: RetireOwned>() -> usize { controlled::controlled_retirement_birth_bytes::<T>() }

/// 🎟️ Returns the original owner when its erased frame has not been admitted.
pub fn admit_owned_retirement<T: RetireOwned>(value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, crate::retained_clone::RetainedCloneProgress), (crate::ValueError, T)> {
    controlled::admit_typed_controlled_retirement(value, grant).map(|(owner, progress)| (owner as Box<dyn ErasedSnapshotRetirement>, progress))
}

/// 🎟️ Keeps the exact original in its caller slot until its installed issuer admits the whole frame.
pub fn admit_original_owned_retirement<T>(original:&mut Option<T>,factory:&dyn ArtifactOwnedValueRetirementFactory<T>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError> {
    let Some(value)=original.as_ref()else{return Ok(None)};
    let refusal=if grant.maximum_items==0{Some((crate::ValueRefusalKind::WorkLimit,"owned retirement requires one admitted item"))}else if grant.maximum_depth==0{Some((crate::ValueRefusalKind::DepthLimit,"owned retirement requires admitted depth"))}else if factory.retirement_birth_bytes(value)>grant.maximum_capacity_bytes{Some((crate::ValueRefusalKind::OwnershipLimit,"owned retirement exceeds admitted frame capacity"))}else{None};
    if let Some((kind,message))=refusal{return Err(crate::ValueError::literal(kind,message));}
    match factory.retire_owned(original.take().expect("observed exact original owned value"),grant){Ok(admitted)=>Ok(Some(admitted)),Err((error,value))=>{*original=Some(value);Err(error)}}
}

struct ErasedCursor(Box<dyn ErasedSnapshotRetirement>);
impl RetirementCursor for ErasedCursor {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        match self.0.close_step(grant) {
            Err(error)=>RetirementStep::Failure(error),
            Ok(crate::retained_clone::RetainedCloneStep::Complete(progress)) if progress==crate::retained_clone::RetainedCloneProgress::default()=>RetirementStep::Complete,
            Ok(crate::retained_clone::RetainedCloneStep::Progress(progress)|crate::retained_clone::RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),
        }
    }
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.0.next_copy_byte_demand()}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.0.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.0.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError> {self.0.next_depth_demand()}
    fn terminal_release_bytes(&self)->Option<usize> {self.0.terminal_is_empty().then_some(size_of::<Self>()+size_of_val(self.0.as_ref()))}
}

/// ♻️ Transfers an existing admitted retirement into its typed parent frontier.
/// 🧮️ Declares the existing erased cursor shell before its parent admits ownership.
pub const fn erased_cursor_birth_bytes()->usize {size_of::<ErasedCursor>()}
pub fn erased_cursor(value:Box<dyn ErasedSnapshotRetirement>)->Box<dyn RetirementCursor> {Box::new(ErasedCursor(value))}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&T)->usize {owned_retirement_birth_bytes::<T>()}
    fn retire_owned(&self,value:T,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress),(crate::ValueError,T)> {admit_owned_retirement(value,grant)}
}

/// 🔗️ Retires one store-owned snapshot alias as a lease: the last alias to close retires the payload, every other alias only drops its count, so sequentially closed owners of one Arc never wait on each other.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned+Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&Arc<T>)->usize {shared::shared_retirement_birth_bytes::<T>()}
    fn retire(&self,value:Arc<T>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress),(crate::ValueError,Arc<T>)> {shared::admit_shared_retirement(value,grant,true)}
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[path="📦️allocation-return/🦀️.rs"]
pub mod allocation_return;

#[path = "🎮️controlled/🦀️.rs"]
pub mod controlled;

#[path="🔗️shared/🦀️.rs"]
pub mod shared;
#[path="🔐️mutex/🦀️.rs"]
mod owned_mutex;

#[path="📋️queue/🦀️.rs"]
pub mod queue;

#[path="🎟️turn/🦀️.rs"]
pub mod turn;

#[cfg(test)]
#[path="📋️queue/🧪️tests/🎮️ownership/🦀️.rs"]
mod queue_tests;

#[path="📦️aliases/🦀️.rs"]
pub mod aliases;

#[path="🎟️frame/🦀️.rs"]
pub mod frame;
