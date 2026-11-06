//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use crate::ErasedSnapshotRetirement;
use crate::retained_clone::RetainedCloneGrant;
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

#[path="📦️box/🦀️.rs"]
pub mod original_box;
use original_box::BoxedOwner;

#[path="⚖️result/🦀️.rs"]
mod original_result;

#[path="🧭️support/🦀️.rs"]
pub mod support;
pub use support::RetirementSupport;

pub trait RetireOwned: Send + 'static {
    /// 🛂️ Borrows the original owner before any retirement move.
    fn retirement_ready(&self)->bool{true}
    type RetirementCursor:RetirementCursor+'static;
    /// 🌱️ Builds only inline state after defining allocation admission; this producer must not allocate, release, call out or unwind.
    fn retirement_seed(self)->Self::RetirementCursor;
    fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(size_of::<Self::RetirementCursor>())}
    /// 🏡️ Quotes a native cursor that drains this same initialized shared allocation in place.
    fn shared_original_demands(&self)->Result<Option<crate::RetirementDemand>,crate::ValueError> where Self:Sized+Sync{Ok(None)}
    /// 🪙️ Quotes the actual authored in-place shared cursor constructor.
    fn shared_original_work_demand(&self)->Result<Option<u64>,crate::ValueError> where Self:Sized+Sync{Ok(None)}
    /// 🏠️ Admits one exact shared original without extracting its complete inline value; refusal retains the original slot.
    fn shared_original_retirement(_original:&mut Option<arc_custody::original::SharedOriginal<Self>>,_effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress)>> where Self:Sized+Sync{crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"owner has no in-place shared original retirement")),work:0}}
    /// 📦️ Quotes field-wise retirement on this exact unique Box allocation when explicitly authored.
    fn boxed_original_demands(&self)->Result<Option<crate::RetirementDemand>,crate::ValueError> where Self:Sized{Ok(None)}
    /// 📦️ Quotes the actual authored in-place unique cursor constructor.
    fn boxed_original_work_demand(&self)->Result<Option<u64>,crate::ValueError> where Self:Sized{Ok(None)}
    /// 🎟️ Transfers unique original allocation custody only after its actual cursor birth is funded.
    fn boxed_original_retirement(_original:&mut Option<original_box::BoxOriginal<Self>>,_effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress)>> where Self:Sized{crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"owner has no in-place original Box retirement")),work:0}}
    /// 🔐️ Whether this owner retires through a controlled cursor whose births and releases are priced up front. Required, never defaulted: an owner that forgot to declare it was refused by every factory that demands controlled retirement, and the refusal only surfaced after a fixture deadline.
    fn controlled_retirement_supported() -> bool;
    /// 🪢️ Composite owners propagate this borrowed type path; authored terminal owners retain their defining capability verdict.
    fn controlled_retirement_supported_in(_path:&RetirementSupport<'_>)->bool where Self:Sized{Self::controlled_retirement_supported()}
    /// 🧮️ Declares an element's complete inline payload work before a collection discards it.
    fn retirement_element_copy_bytes() -> usize { 0 }
}



/// 📏️ Source-slot extraction and defining seed initialization count their complete native inline headers once.
pub fn retirement_cursor_birth_demand<T:RetireOwned>(original:&T)->Result<crate::RetirementDemand,crate::ValueError>{
 if !original.retirement_ready()||!T::controlled_retirement_supported(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original cursor seed is not ready"))}
 let capacity_bytes=std::mem::size_of::<T::RetirementCursor>();if original.retirement_birth_bytes()!=Some(capacity_bytes){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original cursor birth differs from its defining layout"))}
 let copy_bytes=std::mem::size_of::<Option<T>>().checked_add(capacity_bytes).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"original cursor header transfer overflow"))?;
 Ok(crate::RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
}

pub fn retirement_cursor_birth_work()->u64{4}

pub use crate::retirement_contract::record_retirement_work;

/// 🏗️ Admits one exact cursor allocation before consuming its original resident input.
pub fn admit_retirement_cursor<T:RetireOwned>(original:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,crate::RetainedCloneProgress)>>{
 let Some(value)=original.as_ref()else{return crate::RetainedWorkOutcome{result:Ok(None),work:0}};
 let refusal=|kind,message|crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(kind,message)),work:0};
 let demand=match retirement_cursor_birth_demand(value){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};let layout=std::alloc::Layout::new::<T::RetirementCursor>();
 let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<layout.size()||grant.maximum_copy_bytes<demand.copy_bytes||effect.maximum_work_units()==0{return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
 if effect.actual().is_some(){return refusal(crate::ValueRefusalKind::InvariantViolated,"original cursor allocation marker is already occupied")}
 let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return crate::RetainedWorkOutcome{result:Err(error),work}};
 let pointer=if layout.size()==0{std::ptr::NonNull::<T::RetirementCursor>::dangling()}else{
  *effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}
  let Some(pointer)=std::ptr::NonNull::new(raw.cast::<T::RetirementCursor>())else{return crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::AllocationFailed,"original cursor allocation failed")),work}};pointer
 };
 let value=original.take().expect("admitted original cursor input");let owner=unsafe{pointer.as_ptr().write(value.retirement_seed());Box::from_raw(pointer.as_ptr())};
 crate::RetainedWorkOutcome{result:Ok(Some((owner,crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()}))),work}
}

/// 🦜️ Composes a composite's page-wise element width: its full padded size when it has no drop glue and every component declares its own plain width, otherwise zero so collections retire it element-wise. See [`RetireOwned::retirement_element_copy_bytes`].
pub fn composite_copy_bytes<T>(components: &[usize]) -> usize {
    if std::mem::needs_drop::<T>() || components.contains(&0) { 0 } else { size_of::<T>() }
}

pub enum RetirementStep {
    Pending(u128),
    Progress(crate::retained_clone::RetainedCloneProgress,u128),
    Child(Box<dyn RetirementCursor>),
    ChildProgress(Box<dyn RetirementCursor>,crate::retained_clone::RetainedCloneProgress,u128),
    Bytes(usize),
    ProcessedBytes(usize),
    Advanced,
    Failure(crate::OriginalRetirementFailure,u128),
    Complete(u128),
    BudgetExhausted,
}

impl RetirementStep {
    /// ⏱️ Reads explicit child work or the intrinsic bounded action, never a physical currency.
    pub fn work(&self)->u128{match self{Self::Pending(work)|Self::Progress(_,work)|Self::ChildProgress(_,_,work)|Self::Failure(_,work)|Self::Complete(work)=>*work,Self::BudgetExhausted=>0,Self::Child(_)|Self::Bytes(_)|Self::ProcessedBytes(_)|Self::Advanced=>1}}
}

/// 🧳️ Transfers exclusive retirement custody across threads; shared access belongs to an explicit owner gate.
pub trait RetirementCursor: Send {
    /// 👶️ Selects an owned original birth before any ordinary receiving effect opens.
    fn requires_original_birth(&self)->bool{false}
    fn close_original_birth_step(&mut self,_original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->RetirementStep{RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement cursor has no selected original birth").into(),0)}
    /// 🪪️ Reports whether the selected child must retain its original decoder loan.
    fn requires_original_control(&self)->bool{false}
    /// 🧷️ Forwards the same original control only when this cursor implements that authority.
    fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,_control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let _grant=effect.grant();RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement cursor lacks original controlled closure").into(),0)}
    /// 🧵️ Forwards the same original decoder loan without creating another authority.
    fn close_original_step(&mut self,_original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let _grant=effect.grant();RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement cursor lacks original closure").into(),0)}
    /// ⛽️ Quotes the selected original closure action before its effects.
    fn next_computational_work(&self,_body:usize)->Result<u64,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement cursor lacks a work quote"))}
    /// 🎟️ Borrows one original next-action quote without recursively re-querying individual currencies.
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        Ok(crate::RetirementDemand { copy_bytes: self.next_selected_work_byte_demand(maximum_body_bytes)?, capacity_bytes: self.next_birth_bytes(maximum_body_bytes).unwrap_or(0), release_bytes: self.next_close_byte_demand().unwrap_or(0), depth: self.next_depth_demand()? })
    }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep;
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

#[doc(hidden)]
pub struct Leaf<T: Copy + Send + 'static> {
    value: Option<T>,
}
#[cfg(test)]
#[path="🍃️leaf/🧪️tests/🦀️.rs"]
mod original_leaf_tests;
impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(self.value.is_some()))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{copy_bytes:if self.value.is_some(){size_of::<Option<T>>()}else{0},depth:usize::from(self.value.is_some()),..Default::default()})}

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.value.is_none(){return RetirementStep::Complete(0)}let demand=self.next_demand(0).expect("leaf header demand");if effect.maximum_work_units()==0||effect.grant().maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};self.value=None;RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

pub fn leaf<T:Copy+Send+'static>(value:T)->Leaf<T>{Leaf{value:Some(value)}}

#[macro_export]
macro_rules! artifact_retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl $crate::retirement::RetireOwned for $type {
            type RetirementCursor=$crate::retirement::Leaf<Self>;
            fn retirement_seed(self)->Self::RetirementCursor{$crate::retirement::leaf(self)}
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

#[path="🧱️record/🦀️.rs"]
pub mod record;
#[macro_export]
macro_rules! artifact_retire_struct {
 ($type:ty { $($field:ident),+ $(,)? })=>{
  impl $crate::retirement::RetireOwned for $type{
   type RetirementCursor=$crate::retirement::record::RetirementRecord<Self>;
   fn retirement_ready(&self)->bool{true $(&& $crate::retirement::RetireOwned::retirement_ready(&self.$field))+}
   fn retirement_seed(self)->Self::RetirementCursor{
    let Self{$($field),+}=self;let original=Self{$($field),+};
    unsafe{$crate::retirement::record::RetirementRecord::new(original,[$(stringify!($field)),+].len(),|root,ordinal|{let mut index=0;$(if ordinal==index{return $crate::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).$field))}index+=1;)+let _=index;Err($crate::ValueError::literal($crate::ValueRefusalKind::InvariantViolated,"record field ordinal exceeded its declaration"))},|root,ordinal,effect|{let mut index=0;$(if ordinal==index{return $crate::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).$field),effect)}index+=1;)+let _=index;$crate::retirement::RetirementStep::Failure($crate::ValueError::literal($crate::ValueRefusalKind::InvariantViolated,"record field ordinal exceeded its declaration").into(),0)})}
   }
   fn controlled_retirement_supported()->bool{$crate::retirement::support::controlled_retirement_supported::<Self>()}
   fn controlled_retirement_supported_in(path:&$crate::retirement::RetirementSupport<'_>)->bool{let original=::std::mem::MaybeUninit::<Self>::uninit();let root=original.as_ptr();true $(&& $crate::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).$field)},path))+}
  }
 };
}


/// 🌿️ Empty inline field metadata owns no child or allocation.
#[doc(hidden)]
pub struct RetirementFieldsEnd;
impl RetirementCursor for RetirementFieldsEnd{
 fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(0)}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
 fn close_step(&mut self,_:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{RetirementStep::Complete(0)}
 fn terminal_is_empty(&self)->bool{true}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(0)}
}
/// 🌳️ Concrete field seeds retain every original inline without building a vector of boxes.
#[doc(hidden)]
pub struct RetirementFieldsPair<L:RetirementCursor,R:RetirementCursor>{left:L,right:R,right_started:bool}
impl<L:RetirementCursor,R:RetirementCursor> RetirementFieldsPair<L,R>{
 pub fn new(left:L,right:R)->Self{Self{left,right,right_started:false}}
 fn active(&self)->Option<&dyn RetirementCursor>{if self.right_started{Some(&self.right)}else if self.left.terminal_is_empty(){None}else{Some(&self.left)}}
 fn step_with(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn RetirementCursor,&mut crate::RetainedAllocationEffect<'_>)->RetirementStep)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete(0)}
  let grant=effect.grant();let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()<work{return RetirementStep::BudgetExhausted}
  if !self.right_started&&self.left.terminal_is_empty(){let turn=crate::start_retained_close(&mut self.right_started,effect);return match turn.result{Ok(progress)=>RetirementStep::Progress(progress,turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}
  let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  let grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let mut child=match effect.reborrow(grant,effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if self.right_started{invoke(&mut self.right,&mut child)}else{invoke(&mut self.left,&mut child)}
 }
}
impl<L:RetirementCursor,R:RetirementCursor> RetirementCursor for RetirementFieldsPair<L,R>{
 fn requires_original_birth(&self)->bool{self.active().is_some_and(RetirementCursor::requires_original_birth)}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{if !self.requires_original_birth()||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}let child=if self.right_started{&mut self.right as &mut dyn RetirementCursor}else{&mut self.left as &mut dyn RetirementCursor};child.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}
 fn requires_original_control(&self)->bool{self.active().is_some_and(RetirementCursor::requires_original_control)}
 fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{if self.terminal_is_empty(){return Ok(0)}self.active().map_or(Ok(1),|child|child.next_computational_work(body))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let Some(child)=self.active()else{return Ok(crate::retained_close_start_demand(self.right_started))};let mut demand=child.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"retirement field depth overflow"))?;Ok(demand)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{self.step_with(effect,|child,effect|child.close_step(effect))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{self.step_with(effect,|child,effect|child.close_step_controlled(effect,control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{self.step_with(effect,|child,effect|child.close_original_step(original,effect))}
 fn terminal_is_empty(&self)->bool{self.right_started&&self.right.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
#[macro_export]
macro_rules! retirement_fields_type{
 ()=>{$crate::retirement::RetirementFieldsEnd};
 ($one:ty $(,)?)=>{$crate::retirement::Deferred<$one>};
 ($head:ty,$($tail:ty),+ $(,)?)=>{$crate::retirement::RetirementFieldsPair<$crate::retirement::Deferred<$head>,$crate::retirement_fields_type!($($tail),+)>};
}
#[macro_export]
macro_rules! retirement_fields_seed{
 ()=>{$crate::retirement::RetirementFieldsEnd};
 ($one:expr $(,)?)=>{$crate::retirement::deferred($one)};
 ($head:expr,$($tail:expr),+ $(,)?)=>{$crate::retirement::RetirementFieldsPair::new($crate::retirement::deferred($head),$crate::retirement_fields_seed!($($tail),+))};
}

artifact_retire_leaf!(&'static str, (), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
artifact_retire_leaf!(semio_framework_value_paged_list::PagedListReservationSelection,semio_framework_value_paged_list::PagedListReservationKind,semio_framework_value_paged_list::PagedListSelectionHeader);

#[doc(hidden)]
pub struct Bytes(ManuallyDrop<Vec<u8>>,bool);
#[cfg(test)]
#[path="📦️bytes/🧪️tests/🦀️.rs"]
mod original_bytes_tests;
impl RetirementCursor for Bytes {
    fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.1))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{copy_bytes:if self.1{0}else if !self.0.is_empty(){size_of::<usize>()}else if self.0.capacity()!=0{size_of::<Vec<u8>>()}else{size_of::<bool>()},release_bytes:if self.0.is_empty()&&!self.1{self.0.capacity()}else{0},depth:usize::from(!self.1),..Default::default()})}

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.1{return RetirementStep::Complete(0)}let demand=self.next_demand(0).expect("byte owner header demand");if effect.maximum_work_units()==0||effect.grant().maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};if !self.0.is_empty(){self.0.truncate(0)}else if self.0.capacity()!=0{drop(std::mem::take(&mut*self.0))}else{self.1=true}RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self) -> bool { self.1 }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
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
    type RetirementCursor=Bytes;
    fn retirement_seed(self)->Self::RetirementCursor{bytes(self.into_bytes())}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Bytes>()) }
    fn controlled_retirement_supported() -> bool { true }
}
impl RetireOwned for std::path::PathBuf {
    type RetirementCursor=Bytes;
    fn retirement_seed(self)->Self::RetirementCursor{bytes(self.into_os_string().into_encoded_bytes())}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Bytes>()) }
    fn controlled_retirement_supported() -> bool { true }
}
/// 📦️ Quotes the actual byte-buffer cursor birth independently of any UTF8 completeness.
pub const fn bytes_birth_bytes()->usize{size_of::<Bytes>()}
/// 📩️ Retains the original byte allocation after its receiving parent admits this natural cursor birth.
pub fn bytes(original:Vec<u8>)->Bytes{Bytes(ManuallyDrop::new(original),false)}

/// 📚️ Drains trivial elements as bounded logical work; the original backing allocation remains whole until terminal release.
#[path="📋️collection/🦀️.rs"]
mod collection;
pub use collection::{Collection,DequeCollection};

fn retirement_input_step<T:RetireOwned>(slot:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let turn=admit_retirement_cursor(slot,effect);match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}



#[doc(hidden)]
pub struct VectorIterator<T:RetireOwned>(ManuallyDrop<std::vec::IntoIter<T>>);
impl<T:RetireOwned> RetirementCursor for VectorIterator<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original consumed iterator does not expose its backing release"))}

    fn close_step(&mut self,_:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"consumed iterator backing release is not observable").into(),0)}
    fn terminal_is_empty(&self)->bool {self.0.len()==0}
}
impl<T:RetireOwned> Drop for VectorIterator<T> {fn drop(&mut self) {assert!(std::thread::panicking() || self.0.len()==0,"owned iterator retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.0)}}}
/// 🔓️ Stays uncontrolled on purpose: a consumed `Vec` iterator hides its backing capacity, so its release cannot be quoted exactly; it is only an internal cursor field of parents that price their own scaffold, never a payload, snapshot or factory input.
impl<T:RetireOwned> RetireOwned for std::vec::IntoIter<T> {type RetirementCursor=VectorIterator<T>;fn retirement_seed(self)->Self::RetirementCursor {VectorIterator(ManuallyDrop::new(self))} fn controlled_retirement_supported()->bool{false}}

/// 🧺️ Drains one hashed element per turn; the standard table backing is not priced, only the element scaffolds and this cursor shell.
/// 📏️ Exact bytes of the standard hash table backing that holds `capacity` slots of `T`: buckets are the next power of two the load factor admits, the control bytes follow the slots after one group of padding (a 16-byte SSE2 group on x86, 8 bytes elsewhere). The retirement law in `🛂️manifest/🧪️tests/🪟️resolved-host-context` measures it against the real allocator.
fn std_hash_table_bytes<T>(capacity:usize)->usize{
    if capacity==0{return 0}
    const GROUP:usize=if cfg!(all(any(target_arch="x86",target_arch="x86_64"),target_feature="sse2")){16}else{8};
    let buckets=if capacity<8{capacity+1}else{capacity*8/7};
    (buckets*size_of::<T>()).next_multiple_of(align_of::<T>().max(GROUP))+buckets+GROUP
}

/// 🌀️ Drains one hashed element per turn; its terminal release is the cursor shell plus the standard table backing the iterator still owns.
#[doc(hidden)]
pub struct UnorderedSet<T: RetireOwned>{head:ManuallyDrop<Option<T>>,rest:ManuallyDrop<std::collections::hash_set::IntoIter<T>>,table_bytes:usize}
impl<T: RetireOwned> UnorderedSet<T> {
    fn new(rest:std::collections::hash_set::IntoIter<T>,table_bytes:usize)->Self {Self{head:ManuallyDrop::new(None),rest:ManuallyDrop::new(rest),table_bytes}}
}
impl<T: RetireOwned> RetirementCursor for UnorderedSet<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.head.as_ref(){return retirement_cursor_birth_demand(value)}Ok(crate::RetirementDemand{copy_bytes:if self.rest.len()==0{0}else{size_of::<Self>()},depth:usize::from(self.rest.len()!=0),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.head.is_some(){return retirement_input_step(&mut self.head,effect)}if self.rest.len()==0{return RetirementStep::Complete(0)}let grant=effect.grant();let demand=self.next_demand(0).expect("hashed input metadata demand");if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};*self.head=self.rest.next();RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self)->bool {self.head.is_none()&&self.rest.len()==0}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>()+self.table_bytes)}
}
impl<T: RetireOwned> Drop for UnorderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<T: RetireOwned + std::hash::Hash + Eq> RetireOwned for std::collections::HashSet<T> {
    type RetirementCursor=UnorderedSet<T>;
    fn retirement_seed(self)->Self::RetirementCursor{let table=std_hash_table_bytes::<T>(self.capacity());UnorderedSet::new(self.into_iter(),table)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedSet<T>>())}
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
}

/// 🗺️ Drains one hashed entry per turn; its terminal release is the cursor shell plus the standard table backing the iterator still owns.
#[doc(hidden)]
pub struct UnorderedMap<K: RetireOwned,V: RetireOwned>{head:ManuallyDrop<Option<(K,V)>>,rest:ManuallyDrop<std::collections::hash_map::IntoIter<K,V>>,table_bytes:usize}
impl<K: RetireOwned,V: RetireOwned> UnorderedMap<K,V> {
    fn new(rest:std::collections::hash_map::IntoIter<K,V>,table_bytes:usize)->Self {Self{head:ManuallyDrop::new(None),rest:ManuallyDrop::new(rest),table_bytes}}
}
impl<K: RetireOwned,V: RetireOwned> RetirementCursor for UnorderedMap<K,V> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.head.as_ref(){return retirement_cursor_birth_demand(value)}Ok(crate::RetirementDemand{copy_bytes:if self.rest.len()==0{0}else{size_of::<Self>()},depth:usize::from(self.rest.len()!=0),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.head.is_some(){return retirement_input_step(&mut self.head,effect)}if self.rest.len()==0{return RetirementStep::Complete(0)}let grant=effect.grant();let demand=self.next_demand(0).expect("hashed input metadata demand");if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};*self.head=self.rest.next();RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self)->bool {self.head.is_none()&&self.rest.len()==0}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>()+self.table_bytes)}
}
impl<K: RetireOwned,V: RetireOwned> Drop for UnorderedMap<K,V> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"owned map retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.head);ManuallyDrop::drop(&mut self.rest)};}
}
impl<K: RetireOwned + std::hash::Hash + Eq,V: RetireOwned> RetireOwned for std::collections::HashMap<K,V> {
    type RetirementCursor=UnorderedMap<K,V>;
    fn retirement_seed(self)->Self::RetirementCursor{let table=std_hash_table_bytes::<(K,V)>(self.capacity());UnorderedMap::new(self.into_iter(),table)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<UnorderedMap<K,V>>())}
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<K>()&&path.supports::<V>()}
}

/// 🔃️ Drains one ordered element per turn; the standard node backing is not priced, only the element scaffolds and this cursor shell.
#[doc(hidden)]
pub struct OrderedSet<T: RetireOwned + Ord>(ManuallyDrop<std::collections::BTreeSet<T>>,ManuallyDrop<Option<T>>);
impl<T: RetireOwned + Ord> RetirementCursor for OrderedSet<T> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.1.as_ref(){return retirement_cursor_birth_demand(value)}Ok(crate::RetirementDemand{copy_bytes:if self.0.is_empty(){0}else{size_of::<Self>()},depth:usize::from(!self.0.is_empty()),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.1.is_some(){return retirement_input_step(&mut self.1,effect)}if self.0.is_empty(){return RetirementStep::Complete(0)}let demand=self.next_demand(0).expect("ordered input metadata demand");if effect.maximum_work_units()==0||effect.grant().maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};*self.1=self.0.pop_first();RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self)->bool {self.0.is_empty()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
    fn terminal_release_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
}
impl<T: RetireOwned + Ord> Drop for OrderedSet<T> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.0.is_empty(),"owned ordered set retired before terminal-empty");unsafe {ManuallyDrop::drop(&mut self.1);ManuallyDrop::drop(&mut self.0)};}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BTreeSet<T> {
    type RetirementCursor=OrderedSet<T>;
    fn retirement_seed(self)->Self::RetirementCursor{OrderedSet(ManuallyDrop::new(self),ManuallyDrop::new(None))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<OrderedSet<T>>())}
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T: RetireOwned + Ord> RetireOwned for std::collections::BinaryHeap<T> {
    type RetirementCursor=<Vec<T> as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor {self.into_vec().retirement_seed()}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Collection<T>>())}
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T: RetireOwned> RetireOwned for std::cmp::Reverse<T> {
    type RetirementCursor=T::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor {self.0.retirement_seed()}
    fn retirement_birth_bytes(&self)->Option<usize> {self.0.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
    fn retirement_element_copy_bytes()->usize {T::retirement_element_copy_bytes()}
}

/// 📇️ Drains one ordered entry per turn; the standard node backing is not priced, only the entry scaffolds and this cursor shell.
#[doc(hidden)]
pub struct OrderedMap<K: RetireOwned + Ord, V: RetireOwned>(ManuallyDrop<std::collections::BTreeMap<K, V>>,ManuallyDrop<Option<(K,V)>>);
impl<K: RetireOwned + Ord, V: RetireOwned> RetirementCursor for OrderedMap<K, V> {
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.1.as_ref(){return retirement_cursor_birth_demand(value)}Ok(crate::RetirementDemand{copy_bytes:if self.0.is_empty(){0}else{size_of::<Self>()},depth:usize::from(!self.0.is_empty()),..Default::default()})}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.1.is_some(){return retirement_input_step(&mut self.1,effect)}if self.0.is_empty(){return RetirementStep::Complete(0)}let demand=self.next_demand(0).expect("ordered input metadata demand");if effect.maximum_work_units()==0||effect.grant().maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};*self.1=self.0.pop_first();RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()&&self.1.is_none()
    }
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
impl<K: RetireOwned + Ord, V: RetireOwned> Drop for OrderedMap<K, V> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_empty(), "owned ordered map retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.1);ManuallyDrop::drop(&mut self.0) };
    }
}
impl<K: RetireOwned + Ord, V: RetireOwned> RetireOwned for std::collections::BTreeMap<K, V> {
    type RetirementCursor=OrderedMap<K, V>;
    fn retirement_seed(self)->Self::RetirementCursor{
        OrderedMap(ManuallyDrop::new(self),ManuallyDrop::new(None))}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<OrderedMap<K, V>>()) }
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<K>()&&path.supports::<V>()}
}
impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement_ready(&self)->bool{self.as_ref().is_none_or(RetireOwned::retirement_ready)}
    type RetirementCursor=Optional<T>;
    fn retirement_seed(self)->Self::RetirementCursor{
        Optional { original:ManuallyDrop::new(Some(self)) }}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Optional<T>>()) }
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
    fn retirement_element_copy_bytes() -> usize { composite_copy_bytes::<Self>(&[T::retirement_element_copy_bytes()]) }
}
#[doc(hidden)]
pub struct Optional<T:RetireOwned>{original:ManuallyDrop<Option<Option<T>>>}
impl<T:RetireOwned> RetirementCursor for Optional<T>{
 fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(self.original.is_some()))}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{match self.original.as_ref(){None=>Ok(Default::default()),Some(None)=>Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<Option<T>>>(),depth:1,..Default::default()}),Some(Some(value))=>{let mut demand=retirement_cursor_birth_demand(value)?;demand.depth+=1;Ok(demand)}}}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{
  if self.original.is_none(){return RetirementStep::Complete(0)}let grant=effect.grant();let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if effect.maximum_work_units()==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  if self.original.as_ref().is_some_and(Option::is_none){let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};self.original.take();return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
  let mut child=match effect.reborrow(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant},effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  let turn=admit_retirement_cursor(self.original.as_mut().expect("original optional slot retained"),&mut child);match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Drop for Optional<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"optional original abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}
/// 📏️ Prices the concrete cursor that retains an original typed box.
pub const fn boxed_owner_birth_bytes<T:RetireOwned>()->usize{size_of::<BoxedOwner<T>>()}
impl<T: RetireOwned> RetireOwned for Box<T> {
    fn retirement_ready(&self)->bool{T::retirement_ready(self.as_ref())}
    type RetirementCursor=BoxedOwner<T>;
    fn retirement_seed(self)->Self::RetirementCursor{ BoxedOwner::new(self)}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<BoxedOwner<T>>()) }
    fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
}
macro_rules! retire_tuple {
    ($($type:ident:$index:tt),+) => {
        impl<$($type:RetireOwned),+> RetireOwned for ($($type,)+) {
            type RetirementCursor=crate::retirement_fields_type!($($type),+);
            fn retirement_ready(&self)->bool{true$(&&self.$index.retirement_ready())+}
            fn retirement_seed(self)->Self::RetirementCursor{crate::retirement_fields_seed!($(self.$index),+)}
            fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(size_of::<Self::RetirementCursor>())}
            fn controlled_retirement_supported()->bool {crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{true$(&&path.supports::<$type>())+}
            fn retirement_element_copy_bytes()->usize {composite_copy_bytes::<Self>(&[$($type::retirement_element_copy_bytes()),+])}
        }
    };
}
retire_tuple!(A:0,B:1);
retire_tuple!(A:0,B:1,C:2);
retire_tuple!(A:0,B:1,C:2,D:3);
retire_tuple!(A:0,B:1,C:2,D:3,E:4);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6,H:7);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6,H:7,I:8);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6,H:7,I:8,J:9);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6,H:7,I:8,J:9,K:10);
retire_tuple!(A:0,B:1,C:2,D:3,E:4,F:5,G:6,H:7,I:8,J:9,K:10,L:11);

/// 🧱️ Drains a fixed array of owning elements one child cursor per turn, last element first.
#[doc(hidden)]
pub struct FixedArray<T: RetireOwned, const N: usize> {
    items: ManuallyDrop<[Option<T>; N]>,
}
impl<T: RetireOwned, const N: usize> RetirementCursor for FixedArray<T, N> {
    fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(u64::from(!self.terminal_is_empty()))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.items.iter().rev().flatten().next().map_or(Ok(Default::default()),retirement_cursor_birth_demand)}

    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let Some(slot)=self.items.iter_mut().rev().find(|slot|slot.is_some())else{return RetirementStep::Complete(0)}; retirement_input_step(slot,effect)
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

impl<T: RetireOwned, const N: usize> RetireOwned for [T; N] {
 type RetirementCursor=FixedArray<T,N>;
 fn retirement_seed(self)->Self::RetirementCursor{FixedArray{items:ManuallyDrop::new(self.map(Some))}}
 fn retirement_ready(&self)->bool{self.iter().all(RetireOwned::retirement_ready)}
 fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}
 fn retirement_element_copy_bytes()->usize{if plain_elements::<T>(){size_of::<Self>()}else{0}}
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(!self.0.is_empty()))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{depth:usize::from(!self.0.is_empty()),..Default::default()})}

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        self.0.pop().map_or(RetirementStep::Complete(0), RetirementStep::Child)
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

#[doc(hidden)]
pub struct Deferred<T:RetireOwned>{original:ManuallyDrop<Option<T>>}
impl<T:RetireOwned> RetirementCursor for Deferred<T>{
    fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(self.original.is_some()))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{let Some(original)=self.original.as_ref()else{return Ok(Default::default())};let mut demand=retirement_cursor_birth_demand(original)?;demand.depth+=1;Ok(demand)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{
  if self.original.is_none(){return RetirementStep::Complete(0)}let grant=effect.grant();if grant.maximum_depth<2{return RetirementStep::BudgetExhausted}
  let mut child=match effect.reborrow(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant},effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  let turn=admit_retirement_cursor(&mut self.original,&mut child);match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
 }

 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),RetireOwned::retirement_birth_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(if self.original.is_some(){2}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Drop for Deferred<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"deferred original abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}
pub fn deferred<T:RetireOwned>(value:T)->Deferred<T>{Deferred{original:ManuallyDrop::new(Some(value))}}

/// 🖼️ Borrows the exact erased ownership frame before its original value moves.
pub const fn owned_retirement_birth_bytes<T: RetireOwned>() -> usize { controlled::controlled_retirement_birth_bytes::<T>() }

/// ↩️ Returns the original owner when its erased frame has not been admitted.
pub fn admit_owned_retirement<T:RetireOwned>(value:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError>{let turn=controlled::admit_typed_controlled_retirement(value,effect);crate::RetainedWorkOutcome{result:turn.result.map(|owner|owner.map(|(owner,progress)|(owner as Box<dyn ErasedSnapshotRetirement>,progress))),work:turn.work}}

/// 🖱️ Keeps the exact original in its caller slot until its installed issuer admits the whole frame.


/// 🔀️ Moves one exact erased turn into the typed cursor frontier without rebuilding its failure.
pub fn erased_cursor_turn(poll:crate::OriginalRetirementPoll)->RetirementStep{match poll{crate::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>match turn.result{Err(error)=>RetirementStep::Failure(error,turn.work),Ok(crate::retained_clone::RetainedCloneStep::Complete(progress))if progress==crate::retained_clone::RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),Ok(crate::retained_clone::RetainedCloneStep::Progress(progress)|crate::retained_clone::RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work)}}}

struct ErasedCursor(Box<dyn ErasedSnapshotRetirement>);
impl RetirementCursor for ErasedCursor {
    fn requires_original_control(&self)->bool{self.0.requires_original_control()}
    fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();erased_cursor_turn(self.0.close_step_controlled(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control))}
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();erased_cursor_turn(self.0.close_original_step(original,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect")))}
    fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.0.next_computational_work(body)}
    fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.0.next_demand(body)}

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();erased_cursor_turn(self.0.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect")))}
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

#[doc(hidden)]
pub struct PagedListRetirement<T: RetireOwned, const N: usize> {
    owner: ManuallyDrop<crate::list::PagedList<T, N>>,
    released: bool,
    pending:ManuallyDrop<Option<T>>,
}

impl<T: RetireOwned, const N: usize> RetirementCursor for PagedListRetirement<T, N> {
    fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(u64::from(!self.released))}
    fn next_demand(&self, body: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        if self.released{return Ok(Default::default())}
        if self.pending.is_none()&&!self.owner.is_empty()&&!std::mem::needs_drop::<T>()&&size_of::<T>()!=0&&T::retirement_element_copy_bytes()==size_of::<T>(){return self.owner.plain_tail_discard_demand(usize::MAX,body).map(|d|d.physical).map_err(Into::into)}
        if let Some(value)=self.pending.as_ref(){return retirement_cursor_birth_demand(value)}
        if !self.owner.is_empty()&&std::mem::needs_drop::<T>(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()})}if self.owner.is_empty(){if self.owner.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<crate::list::PagedList<T,N>>()+size_of::<bool>(),depth:1,..Default::default()})}return Ok(crate::RetirementDemand{copy_bytes:self.owner.next_release_copy_byte_demand().map_err(crate::ValueError::from)?,release_bytes:self.owner.next_release_allocation_bytes().map_err(crate::ValueError::from)?,depth:self.owner.next_release_depth_demand().map_err(crate::ValueError::from)?,..Default::default()})}
        Ok(crate::RetirementDemand {
            copy_bytes: self.next_selected_work_byte_demand(body)?,
            capacity_bytes: self.next_birth_bytes(body).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner, "original paged child lacks a birth quote"))?,
            release_bytes: self.next_close_byte_demand().unwrap_or(0),
            depth: self.next_depth_demand()?,
        })
    }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
        if self.released{return RetirementStep::Complete(0)}if grant.maximum_items==0||effect.maximum_work_units()==0{return RetirementStep::BudgetExhausted}if self.pending.is_some(){return retirement_input_step(&mut self.pending,effect)}
        if !self.owner.is_empty()&&!std::mem::needs_drop::<T>()&&size_of::<T>()!=0&&T::retirement_element_copy_bytes()==size_of::<T>(){
          let turn=self.owner.discard_plain_tail(effect);return match turn.result{Ok(p)if p!=Default::default()=>RetirementStep::Progress(p,turn.work),Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
        }
        if !self.owner.is_empty(){let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};*self.pending=self.owner.pop();return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
        if self.owner.terminal_is_empty(){let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};unsafe{ManuallyDrop::drop(&mut self.owner)};self.released=true;return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
        let turn=self.owner.release_empty_page(effect);match turn.result{Ok(crate::list::PagedListReleaseOutcome::Denied)=>RetirementStep::BudgetExhausted,Ok(crate::list::PagedListReleaseOutcome::Ready(progress))=>RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:progress.placed_bytes,released_bytes:progress.released_allocation_bytes,..Default::default()},turn.work),Err(crate::list::PagedListReleaseFailure::Work(error))=>RetirementStep::Failure(error.into(),turn.work),Err(crate::list::PagedListReleaseFailure::Release(error))=>RetirementStep::Failure(crate::ValueError::from(error).into(),turn.work)}
    }

    fn terminal_is_empty(&self) -> bool {
        self.released&&self.pending.is_none()
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
    type RetirementCursor=PagedListRetirement<T, N>;
    fn retirement_seed(self)->Self::RetirementCursor{
        PagedListRetirement { owner: ManuallyDrop::new(self), released: false, pending:ManuallyDrop::new(None) }}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<PagedListRetirement<T, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}


#[doc(hidden)]
pub struct OwnedAtomicU64Retirement(Option<std::sync::atomic::AtomicU64>);
impl RetireOwned for std::sync::atomic::AtomicU64{
 type RetirementCursor=OwnedAtomicU64Retirement;
    fn retirement_seed(self)->Self::RetirementCursor{OwnedAtomicU64Retirement(Some(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OwnedAtomicU64Retirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for OwnedAtomicU64Retirement{
 fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::RetirementDemand{copy_bytes:if self.0.is_some(){size_of::<Option<std::sync::atomic::AtomicU64>>()}else{0},depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if self.0.is_none(){return RetirementStep::Complete(0)}let demand=self.next_demand(0).expect("atomic owner header demand");if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};self.0=None;RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.0.is_none().then_some(size_of::<Self>())}
}

