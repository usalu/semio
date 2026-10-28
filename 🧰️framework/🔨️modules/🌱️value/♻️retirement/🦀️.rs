//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use crate::ErasedSnapshotRetirement;
use crate::retained_clone::RetainedCloneGrant;
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

#[path="📋️original-vector/🦀️.rs"]
pub mod original_vector;
#[path="📦️box/🦀️.rs"]
pub mod original_box;
use original_box::BoxedOwner;

#[path="⚖️result/🦀️.rs"]
mod original_result;

#[path="🧭️support/🦀️.rs"]
pub mod support;
pub use support::RetirementSupport;

pub trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
    fn retirement_birth_bytes(&self) -> Option<usize> { None }
    /// 🏡️ Quotes a native cursor that drains this same initialized shared allocation in place.
    fn shared_original_birth_bytes(&self)->Option<usize> where Self:Sized+Sync{None}
    /// 🏠️ Admits one exact shared original without extracting its complete inline value; refusal retains the original slot.
    fn shared_original_retirement(_original:&mut Option<arc_custody::original::SharedOriginal<Self>>,_grant:RetainedCloneGrant)->Result<Option<(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError> where Self:Sized+Sync{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"owner has no in-place shared original retirement"))}
    /// 📦️ Quotes field-wise retirement on this exact unique Box allocation when explicitly authored.
    fn boxed_original_demands(&self)->Result<Option<crate::RetirementDemand>,crate::ValueError> where Self:Sized{Ok(None)}
    /// 🎟️ Transfers unique original allocation custody only after its actual cursor birth is funded.
    fn boxed_original_retirement(_original:&mut Option<original_box::BoxOriginal<Self>>,_grant:RetainedCloneGrant)->Result<Option<(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError> where Self:Sized{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"owner has no in-place original Box retirement"))}
    /// 🔐️ Whether this owner retires through a controlled cursor whose births and releases are priced up front. Required, never defaulted: an owner that forgot to declare it was refused by every factory that demands controlled retirement, and the refusal only surfaced after a fixture deadline.
    fn controlled_retirement_supported() -> bool;
    /// 🪢️ Composite owners propagate this borrowed type path; authored terminal owners retain their defining capability verdict.
    fn controlled_retirement_supported_in(_path:&RetirementSupport<'_>)->bool where Self:Sized{Self::controlled_retirement_supported()}
    /// 🧮️ Declares an element's complete inline payload work before a collection discards it.
    fn retirement_element_copy_bytes() -> usize { 0 }
}

/// 🦜️ Composes a composite's page-wise element width: its full padded size when it has no drop glue and every component declares its own plain width, otherwise zero so collections retire it element-wise. See [`RetireOwned::retirement_element_copy_bytes`].
pub fn composite_copy_bytes<T>(components: &[usize]) -> usize {
    if std::mem::needs_drop::<T>() || components.contains(&0) { 0 } else { std::mem::size_of::<T>() }
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

/// 🧳️ Transfers exclusive retirement custody across threads; shared access belongs to an explicit owner gate.
pub trait RetirementCursor: Send {
    /// 🎟️ Borrows one original next-action quote without recursively re-querying individual currencies.
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        Ok(crate::RetirementDemand { copy_bytes: self.next_selected_work_byte_demand(maximum_body_bytes)?, capacity_bytes: self.next_birth_bytes(maximum_body_bytes).unwrap_or(0), release_bytes: self.next_close_byte_demand().unwrap_or(0), depth: self.next_depth_demand()? })
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep;
    fn terminal_is_empty(&self) -> bool;
    /// 🪆️ Admits the next structural cursor slot before a typed owner can return it.
    fn next_depth_demand(&self) -> Result<usize, crate::ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
    fn next_close_byte_demand(&self) -> Option<usize> {
        None
    }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(0)}
    /// 🎯️ Quotes payload work for the same event selected by the incoming body grant.
    fn next_selected_work_byte_demand(&self,_maximum_body_bytes:usize)->Result<usize,crate::ValueError> {self.next_work_byte_demand()}
    /// 💼️ Declares a separately funded child when a complete inline item exceeds this work grant.
    fn allows_admitted_narrow_work(&self) -> bool { false }
    fn next_birth_bytes(&self, _maximum_bytes: usize) -> Option<usize> { None }
    fn terminal_release_bytes(&self) -> Option<usize> { None }
}

/// 🎞️ Measures the fixed constructor allocations of a deferred field sequence.
pub fn sequence_birth_bytes(fields: &[usize]) -> Option<usize> {
    fields.iter().try_fold(size_of::<Sequence>().checked_add(fields.len().checked_mul(size_of::<Box<dyn RetirementCursor>>())?)?, |total, bytes| total.checked_add(*bytes))
}

/// 💎️ Measures the exact inline ownership retained by one deferred field Box.
pub const fn deferred_birth_bytes<T: RetireOwned>() -> usize { size_of::<Deferred<T>>() }

/// 🏗️ Borrows the exact field type when a declarative owner measures its deferred scaffold.
pub fn deferred_birth_bytes_for<T: RetireOwned>(_: &T) -> usize { deferred_birth_bytes::<T>() }

/// 🍃️ Measures the exact scalar retirement scaffold allocation.
pub const fn leaf_birth_bytes<T: Copy + Send + 'static>() -> usize { size_of::<Leaf<T>>() }

struct Leaf<T: Copy + Send + 'static> {
    value: Option<T>,
}
#[cfg(test)]
#[path="🍃️leaf/🧪️tests/🦀️.rs"]
mod original_leaf_tests;
impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{depth:usize::from(self.value.is_some()),..Default::default()})}

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
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{release_bytes:if self.0.is_empty(){self.0.capacity()}else{0},depth:usize::from(!self.1),..Default::default()})}

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
/// 📩️ Retains the original byte allocation after its receiving parent admits this natural cursor birth.
pub fn bytes(original:Vec<u8>)->Box<dyn RetirementCursor>{Box::new(Bytes(ManuallyDrop::new(original),false))}

/// 📚️ Drains trivial elements as bounded logical work; the original backing allocation remains whole until terminal release.
struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{copy_bytes:self.next_selected_work_byte_demand(body)?,capacity_bytes:self.next_birth_bytes(body).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original collection child lacks a birth quote"))?,release_bytes:if self.0.is_empty(){self.0.capacity()*size_of::<T>()}else{0},depth:usize::from(!self.terminal_is_empty())})}

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
        let Some(birth) = self.next_birth_bytes(grant.maximum_copy_bytes) else { return RetirementStep::BudgetExhausted; };
        if grant.maximum_capacity_bytes < birth { return RetirementStep::BudgetExhausted; }
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty() && (self.0.capacity()==0 || size_of::<T>()==0)
    }
    fn next_close_byte_demand(&self)->Option<usize>{Some(if self.0.is_empty(){self.0.capacity()*size_of::<T>()}else{0})}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(if !std::mem::needs_drop::<T>()&&!self.0.is_empty(){T::retirement_element_copy_bytes()}else{0})}
    fn next_selected_work_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,crate::ValueError> {
        let atom=self.next_work_byte_demand()?;
        if maximum_body_bytes<atom{return Ok(if self.next_birth_bytes(maximum_body_bytes).is_some_and(|bytes|bytes!=0){0}else{atom})}
        let width=size_of::<T>();
        Ok(if width==0||atom==0{0}else{(maximum_body_bytes/width).min(self.0.len())*width})
    }
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
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}

struct DequeCollection<T: RetireOwned>(ManuallyDrop<std::collections::VecDeque<T>>);
impl<T: RetireOwned> RetirementCursor for DequeCollection<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.0.front().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original deque child lacks a birth quote"))?,depth:usize::from(!self.0.is_empty()),..Default::default()})}

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
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}

struct VectorIterator<T:RetireOwned>(ManuallyDrop<std::vec::IntoIter<T>>);
impl<T:RetireOwned> RetirementCursor for VectorIterator<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original consumed iterator does not expose its backing release"))}

    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}if self.0.len()==0 {return RetirementStep::Complete;}RetirementStep::Child(self.0.next_back().unwrap().retirement())}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<T:RetireOwned> Drop for VectorIterator<T> {fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned iterator retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)}}}
/// 🔓️ Stays uncontrolled on purpose: a consumed `Vec` iterator hides its backing capacity, so its release cannot be quoted exactly; it is only an internal cursor field of parents that price their own scaffold, never a payload, snapshot or factory input.
impl<T:RetireOwned> RetireOwned for std::vec::IntoIter<T> {fn retirement(self)->Box<dyn RetirementCursor> {Box::new(VectorIterator(ManuallyDrop::new(self)))} fn controlled_retirement_supported()->bool{false}}

/// 🧺️ Drains one hashed element per turn; the standard table backing is not priced, only the element scaffolds and this cursor shell.
/// 📏️ Exact bytes of the standard hash table backing that holds `capacity` slots of `T`: buckets are the next power of two the load factor admits, the control bytes follow the slots after one group of padding (a 16-byte SSE2 group on x86, 8 bytes elsewhere). The retirement law in `🛂️manifest/🧪️tests/🪟️resolved-host-context` measures it against the real allocator.
fn std_hash_table_bytes<T>(capacity:usize)->usize{
    if capacity==0{return 0}
    const GROUP:usize=if cfg!(all(any(target_arch="x86",target_arch="x86_64"),target_feature="sse2")){16}else{8};
    let buckets=if capacity<8{capacity+1}else{capacity*8/7};
    (buckets*size_of::<T>()).next_multiple_of(align_of::<T>().max(GROUP))+buckets+GROUP
}

/// 🌀️ Drains one hashed element per turn; its terminal release is the cursor shell plus the standard table backing the iterator still owns.
struct UnorderedSet<T: RetireOwned>{head:ManuallyDrop<Option<T>>,rest:ManuallyDrop<std::collections::hash_set::IntoIter<T>>,table_bytes:usize}
impl<T: RetireOwned> UnorderedSet<T> {
    fn new(mut rest:std::collections::hash_set::IntoIter<T>,table_bytes:usize)->Self {Self{head:ManuallyDrop::new(rest.next()),rest:ManuallyDrop::new(rest),table_bytes}}
}
impl<T: RetireOwned> RetirementCursor for UnorderedSet<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original hashed child lacks a birth quote"))?,depth:usize::from(self.head.is_some()),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}let head=self.head.take();*self.head=self.rest.next();head.map_or(RetirementStep::Complete,|value|RetirementStep::Child(value.retirement()))}
    fn terminal_is_empty(&self)->bool {self.head.is_none()}
    fn next_birth_bytes(&self,_:usize)->Option<usize> {self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>()+self.table_bytes)}
}
impl<T: RetireOwned> Drop for UnorderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<T: RetireOwned + std::hash::Hash + Eq> RetireOwned for std::collections::HashSet<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {let table=std_hash_table_bytes::<T>(self.capacity());Box::new(UnorderedSet::new(self.into_iter(),table))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedSet<T>>())}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}

/// 🗺️ Drains one hashed entry per turn; its terminal release is the cursor shell plus the standard table backing the iterator still owns.
struct UnorderedMap<K: RetireOwned,V: RetireOwned>{head:ManuallyDrop<Option<(K,V)>>,rest:ManuallyDrop<std::collections::hash_map::IntoIter<K,V>>,table_bytes:usize}
impl<K: RetireOwned,V: RetireOwned> UnorderedMap<K,V> {
    fn new(mut rest:std::collections::hash_map::IntoIter<K,V>,table_bytes:usize)->Self {Self{head:ManuallyDrop::new(rest.next()),rest:ManuallyDrop::new(rest),table_bytes}}
}
impl<K: RetireOwned,V: RetireOwned> RetirementCursor for UnorderedMap<K,V> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original hashed entry lacks a birth quote"))?,depth:usize::from(self.head.is_some()),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}let head=self.head.take();*self.head=self.rest.next();head.map_or(RetirementStep::Complete,|entry|RetirementStep::Child(entry.retirement()))}
    fn terminal_is_empty(&self)->bool {self.head.is_none()}
    fn next_birth_bytes(&self,_:usize)->Option<usize> {self.head.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>()+self.table_bytes)}
}
impl<K: RetireOwned,V: RetireOwned> Drop for UnorderedMap<K,V> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned map retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<K: RetireOwned + std::hash::Hash + Eq,V: RetireOwned> RetireOwned for std::collections::HashMap<K,V> {
    fn retirement(self)->Box<dyn RetirementCursor> {let table=std_hash_table_bytes::<(K,V)>(self.capacity());Box::new(UnorderedMap::new(self.into_iter(),table))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedMap<K,V>>())}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<K>()&&path.supports::<V>()}
}

/// 🔃️ Drains one ordered element per turn; the standard node backing is not priced, only the element scaffolds and this cursor shell.
struct OrderedSet<T: RetireOwned + Ord>(ManuallyDrop<std::collections::BTreeSet<T>>);
impl<T: RetireOwned + Ord> RetirementCursor for OrderedSet<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.0.first().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original ordered child lacks a birth quote"))?,depth:usize::from(!self.0.is_empty()),..Default::default()})}

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
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BinaryHeap<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.into_vec().retirement()}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Collection<T>>())}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T: RetireOwned> RetireOwned for std::cmp::Reverse<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {self.0.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize> {self.0.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
    fn retirement_element_copy_bytes()->usize {T::retirement_element_copy_bytes()}
}

/// 📇️ Drains one ordered entry per turn; the standard node backing is not priced, only the entry scaffolds and this cursor shell.
struct OrderedMap<K: RetireOwned + Ord, V: RetireOwned>(ManuallyDrop<std::collections::BTreeMap<K, V>>);
impl<K: RetireOwned + Ord, V: RetireOwned> RetirementCursor for OrderedMap<K, V> {
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.next_birth_bytes(body).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original ordered entry lacks a birth quote"))?,depth:usize::from(!self.0.is_empty()),..Default::default()})}

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
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<K>()&&path.supports::<V>()}
}
impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Optional { original:ManuallyDrop::new(Some(self)) })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Optional<T>>()) }
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
    fn retirement_element_copy_bytes() -> usize { composite_copy_bytes::<Self>(&[T::retirement_element_copy_bytes()]) }
}
struct Optional<T:RetireOwned>{original:ManuallyDrop<Option<Option<T>>>}
impl<T:RetireOwned> RetirementCursor for Optional<T>{
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.next_birth_bytes(body).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original optional child lacks a birth quote"))?,depth:self.next_depth_demand()?,..Default::default()})}

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
impl<T: RetireOwned> RetireOwned for Box<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(BoxedOwner::new(self)) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<BoxedOwner<T>>()) }
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
macro_rules! retire_tuple {
    ($($type:ident:$index:tt),+) => {
        impl<$($type:RetireOwned),+> RetireOwned for ($($type,)+) {
            fn retirement(self)->Box<dyn RetirementCursor> {sequence(vec![$(deferred(self.$index)),+])}
            fn retirement_birth_bytes(&self)->Option<usize> {sequence_birth_bytes(&[$(deferred_birth_bytes::<$type>()),+])}
            fn controlled_retirement_supported()->bool {crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{true$(&&path.supports::<$type>())+}
            fn retirement_element_copy_bytes()->usize {composite_copy_bytes::<Self>(&[$($type::retirement_element_copy_bytes()),+])}
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
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.items.iter().rev().flatten().next().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original array child lacks a birth quote"))?,depth:usize::from(!self.terminal_is_empty()),..Default::default()})}

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

/// 🔧️ Drains a fixed array of trivial elements as bounded logical copy work, as [`Collection`] does, instead of one child cursor per element.
struct PlainArray<T: RetireOwned, const N: usize> {
    items: ManuallyDrop<[T; N]>,
    remaining: usize,
}
impl<T: RetireOwned, const N: usize> RetirementCursor for PlainArray<T, N> {
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{copy_bytes:self.next_selected_work_byte_demand(body)?,capacity_bytes:self.next_birth_bytes(body).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original array child lacks a birth quote"))?,depth:usize::from(self.remaining!=0),..Default::default()})}

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
        if count == 0 {
            let Some(birth)=self.next_birth_bytes(grant.maximum_copy_bytes) else {return RetirementStep::BudgetExhausted;};
            if birth==0||grant.maximum_capacity_bytes<birth||grant.maximum_depth==0 {return RetirementStep::BudgetExhausted;}
            let value=unsafe {std::ptr::read(&self.items[self.remaining-1])};
            self.remaining-=1;
            return RetirementStep::Child(value.retirement());
        }
        self.remaining -= count;
        RetirementStep::ProcessedBytes(count * width)
    }
    fn terminal_is_empty(&self) -> bool { self.remaining == 0 }
    fn next_work_byte_demand(&self) -> Result<usize, crate::ValueError> { Ok(if self.remaining != 0 { T::retirement_element_copy_bytes() } else { 0 }) }
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,crate::ValueError> {
        let atom=self.next_work_byte_demand()?;
        if body<atom{return Ok(if self.next_birth_bytes(body).is_some_and(|bytes|bytes!=0){0}else{atom})}
        let width=size_of::<T>();
        Ok(if width==0{0}else{(body/width).min(self.remaining)*width})
    }
    fn allows_admitted_narrow_work(&self) -> bool { self.remaining != 0 }
    fn next_birth_bytes(&self,body:usize)->Option<usize> {
        if self.remaining!=0&&body<T::retirement_element_copy_bytes() {self.items[self.remaining-1].retirement_birth_bytes()}else {Some(0)}
    }
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
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
    fn retirement_element_copy_bytes() -> usize { if plain_elements::<T>() { size_of::<Self>() } else { 0 } }
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{depth:usize::from(!self.0.is_empty()),..Default::default()})}

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
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{capacity_bytes:self.original.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original deferred child lacks a birth quote"))?,depth:if self.original.is_some(){2}else{0},..Default::default()})}

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

/// 🖼️ Borrows the exact erased ownership frame before its original value moves.
pub const fn owned_retirement_birth_bytes<T: RetireOwned>() -> usize { controlled::controlled_retirement_birth_bytes::<T>() }

/// ↩️ Returns the original owner when its erased frame has not been admitted.
pub fn admit_owned_retirement<T: RetireOwned>(value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, crate::retained_clone::RetainedCloneProgress), (crate::ValueError, T)> {
    controlled::admit_typed_controlled_retirement(value, grant).map(|(owner, progress)| (owner as Box<dyn ErasedSnapshotRetirement>, progress))
}

/// 🖱️ Keeps the exact original in its caller slot until its installed issuer admits the whole frame.


struct ErasedCursor(Box<dyn ErasedSnapshotRetirement>);
impl RetirementCursor for ErasedCursor {
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.0.next_demand(body)}

    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
        match self.0.close_step(grant) {
            Err(error)=>RetirementStep::Failure(error),
            Ok(crate::retained_clone::RetainedCloneStep::Complete(progress)) if progress==crate::retained_clone::RetainedCloneProgress::default()=>RetirementStep::Complete,
            Ok(crate::retained_clone::RetainedCloneStep::Progress(progress)|crate::retained_clone::RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),
        }
    }
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.0.next_copy_byte_demand()}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,crate::ValueError> {Ok(self.0.next_demand(body)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.0.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.0.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError> {self.0.next_depth_demand()}
    fn terminal_release_bytes(&self)->Option<usize> {self.0.terminal_is_empty().then_some(size_of::<Self>()+size_of_val(self.0.as_ref()))}
}

/// ⏩️ Transfers an existing admitted retirement into its typed parent frontier.
/// 🧮️ Declares the existing erased cursor shell before its parent admits ownership.
pub const fn erased_cursor_birth_bytes()->usize {size_of::<ErasedCursor>()}
pub fn erased_cursor(value:Box<dyn ErasedSnapshotRetirement>)->Box<dyn RetirementCursor> {Box::new(ErasedCursor(value))}

pub use semio_framework_value_allocation_return as allocation_return;

#[path = "🎮️controlled/🦀️.rs"]
pub mod controlled;

#[path="🔗️arc-custody/🦀️.rs"]
pub mod arc_custody;
#[path="🔐️mutex/🦀️.rs"]
mod owned_mutex;



#[path="🎟️turn/🦀️.rs"]
pub mod turn;





#[path="🎟️frame/🦀️.rs"]
pub mod frame;

pub struct PagedListRetirement<T: RetireOwned, const N: usize> {
    owner: ManuallyDrop<crate::list::PagedList<T, N>>,
    released: bool,
}

impl<T: RetireOwned, const N: usize> RetirementCursor for PagedListRetirement<T, N> {
    fn next_demand(&self, body: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        Ok(crate::RetirementDemand {
            copy_bytes: self.next_selected_work_byte_demand(body)?,
            capacity_bytes: self.next_birth_bytes(body).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner, "original paged child lacks a birth quote"))?,
            release_bytes: self.next_close_byte_demand().unwrap_or(0),
            depth: self.next_depth_demand()?,
        })
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = if self.owner.is_empty() { grant.maximum_release_bytes } else { grant.maximum_copy_bytes };
        if self.released {
            return RetirementStep::Complete;
        }
        let width = size_of::<T>();
        if width != 0 && !std::mem::needs_drop::<T>() && !self.owner.is_empty() {
            let count = (maximum_bytes / width).min(self.owner.len());
            if count != 0 {
                for _ in 0..count {
                    self.owner.pop();
                }
                return RetirementStep::ProcessedBytes(count * width);
            }
        }
        if let Some(value) = self.owner.pop() {
            return RetirementStep::Child(value.retirement());
        }
        if self.owner.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.owner) };
            self.released = true;
            return RetirementStep::Complete;
        }
        let Ok(required) = self.owner.next_release_allocation_bytes() else { return RetirementStep::BudgetExhausted; };
        if required > maximum_bytes {
            return RetirementStep::BudgetExhausted;
        }
        match self.owner.release_empty_page(maximum_bytes) {
            Ok(progress) if progress.progressed => RetirementStep::Bytes(progress.released_allocation_bytes),
            _ => RetirementStep::BudgetExhausted,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.released
    }

    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(if !self.released && !self.owner.is_empty() && !std::mem::needs_drop::<T>() { size_of::<T>() } else { 0 })}
    fn next_selected_work_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,crate::ValueError> {
        let atom=self.next_work_byte_demand()?;
        if maximum_body_bytes<atom{return Ok(if self.next_birth_bytes(maximum_body_bytes).is_some_and(|bytes|bytes!=0){0}else{atom})}
        let width=size_of::<T>();
        Ok(if width==0||atom==0{0}else{(maximum_body_bytes/width).min(self.owner.len())*width})
    }

    fn allows_admitted_narrow_work(&self)->bool { !self.released&&!self.owner.is_empty()&&!std::mem::needs_drop::<T>()&&self.owner.get(self.owner.len()-1).is_some_and(|value|value.retirement_birth_bytes().is_some_and(|bytes|bytes!=0)) }

    fn next_close_byte_demand(&self) -> Option<usize> {
        if self.released || !self.owner.is_empty() || self.owner.terminal_is_empty() {
            Some(0)
        } else {
            self.owner.next_release_allocation_bytes().ok()
        }
    }
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> {
        if self.released || self.owner.is_empty() || (!std::mem::needs_drop::<T>() && size_of::<T>() != 0 && maximum_bytes >= size_of::<T>()) { Some(0) }
        else { self.owner.get(self.owner.len() - 1)?.retirement_birth_bytes() }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

impl<T: RetireOwned, const N: usize> Drop for PagedListRetirement<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.released, "paged list retired before terminal-empty");
    }
}

impl<T: RetireOwned, const N: usize> RetireOwned for crate::list::PagedList<T, N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(PagedListRetirement { owner: ManuallyDrop::new(self), released: false })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<PagedListRetirement<T, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}


struct OwnedAtomicU64Retirement(Option<std::sync::atomic::AtomicU64>);
impl RetireOwned for std::sync::atomic::AtomicU64{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OwnedAtomicU64Retirement(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OwnedAtomicU64Retirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for OwnedAtomicU64Retirement{
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.0.is_none(){return RetirementStep::Complete}if grant.maximum_items==0||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}self.0=None;RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,..Default::default()})}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.0.is_none().then_some(size_of::<Self>())}
}

#[cfg(test)]
#[path="🧪️tests/🎯️selected/🦀️.rs"]
mod original_selected_body_tests;
