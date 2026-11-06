//! 📥️ Missing authored fields retain phased default producers under the actual caller grant.
use crate::{NativeDecodeControl,OriginalReceivingPrimary,RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::ManuallyDrop;
use semio_framework_value_ordered::{OrderedMap,OrderedSet};
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(crate::ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid")}
fn nested(mut d:RetirementDemand,copy_bytes:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy_bytes).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child_grant(mut g:RetainedCloneGrant,copy_bytes:usize)->RetainedCloneGrant{g.maximum_items=1;g.maximum_copy_bytes=g.maximum_copy_bytes.saturating_sub(copy_bytes);g.maximum_depth=g.maximum_depth.saturating_sub(1);g}
fn receiving_capacity(control:&NativeDecodeControl<'_>,bytes:usize)->Result<(),ValueError>{if control.owned_bytes().checked_add(bytes).is_none_or(|n|n>control.maximum_bytes()){Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit"))}else{Ok(())}}
/// 🌱️ Pure headers retain a genuine default producer instead of running arbitrary Default bodies.
/// ```compile_fail,E0277
/// use semio_framework_value_native_default::NativeDefault;
/// struct AllocatingDefault(String);
/// impl Default for AllocatingDefault{fn default()->Self{Self("allocated".to_owned())}}
/// let _=<AllocatingDefault as NativeDefault>::default_cursor();
/// ```
pub trait NativeDefault:RetireOwned+Sized{type Cursor:NativeDefaultCursor<Self>;/// 🌱️ Produces only a pure inline seed without allocating, releasing, calling out or unwinding.
fn default_cursor()->Self::Cursor;}
/// 🎮️ Normal work, owned output and cancellation expose independent actual CPU on every outcome.
pub trait NativeDefaultCursor<T:RetireOwned>:Send+'static{
 fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
 fn output_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native default lacks an original close work quote"))}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome;
 fn terminal_is_empty(&self)->bool;
}
/// 🥡️ First-party scalar and empty-owner definitions report their own genuine body effect.
pub trait NativeDefaultValue:RetireOwned+Sized{fn default_demand()->RetirementDemand;fn admit_default(output:&mut Option<Self>,grant:RetainedCloneGrant)->RetainedWorkOutcome;}
/// 🧺️ One leaf output stays resident before its paid take or controlled retirement.
pub struct NativeDefaultValueCursor<T:NativeDefaultValue>{output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool}
impl<T:NativeDefaultValue> NativeDefaultValueCursor<T>{pub fn new()->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),closing:false}}}
impl<T:NativeDefaultValue> NativeDefaultCursor<T> for NativeDefaultValueCursor<T>{
 fn demands(&self,_:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}Ok(if self.output.is_some(){Default::default()}else{T::default_demand()})}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,_:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{semio_framework_value_native_receiver_protocol::native_receive_zero_work!(maximum_work_units);if self.closing{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}if self.output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,T::default_demand()){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if let Err(error)=control.checkpoint(){return RetainedWorkOutcome{result:Err(error.into()),work:0}}control.receive_retained_work(g,T::admit_default(&mut self.output,g))}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output!(T,invalid);
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}self.retirement.next_owner_demands::<T>(self.output.is_some(),&None,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}self.retirement.next_owner_work::<T>(self.output.is_some(),&None,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let g=effect.grant();if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}let turn=self.retirement.begin_granted(&mut self.output,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}
 fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.retirement.is_empty()}
}
impl<T:NativeDefaultValue> Drop for NativeDefaultValueCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native default leaf requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}
macro_rules! empty_default{($copy:expr;$($ty:ty=>$value:expr),*$(,)?)=>{$(impl NativeDefaultValue for $ty{fn default_demand()->RetirementDemand{fixed($copy)}fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=Self::default_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some($value);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}}impl NativeDefault for $ty{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}})*};}
empty_default!(size_of::<Self>();()=>(),bool=>false,u8=>0,u16=>0,u32=>0,u64=>0,usize=>0,i8=>0,i16=>0,i32=>0,i64=>0,isize=>0,f32=>0.0,f64=>0.0);
empty_default!(0;String=>String::new());
macro_rules! generic_empty_default{(($($generic:tt)*),$ty:ty,$value:expr)=>{impl<$($generic)*> NativeDefaultValue for $ty{fn default_demand()->RetirementDemand{fixed(0)}fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=Self::default_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some($value);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}}impl<$($generic)*> NativeDefault for $ty{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}};}
generic_empty_default!((T:RetireOwned),Vec<T>,Vec::new());
generic_empty_default!((T:RetireOwned),Option<T>,None);
generic_empty_default!((T:RetireOwned,const N:usize),semio_framework_value_paged_list::PagedList<T,N>,Self::new());
#[path="📦️slot/🦀️.rs"]mod slot;pub use slot::NativeDefaultSlot;
#[path="🔢️array/🦀️.rs"]mod arrays;pub use arrays::NativeArrayDefaultCursor;
#[path="👯️pair/🦀️.rs"]mod pairs;pub use pairs::NativePairDefaultCursor;

impl<V:RetireOwned+Sync> NativeDefaultValue for OrderedMap<V>{
 fn default_demand()->RetirementDemand{fixed(0)}
 fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=<Self as NativeDefaultValue>::default_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::new());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}
}
impl<V:RetireOwned+Sync> NativeDefault for OrderedMap<V>{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}


impl NativeDefaultValue for OrderedSet{
 fn default_demand()->RetirementDemand{fixed(0)}
 fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=<Self as NativeDefaultValue>::default_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::new());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}
}
impl NativeDefault for OrderedSet{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}


#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod tests;
