//! 📥️ Missing authored fields retain phased default producers under the actual caller grant.
use crate::{NativeDecodeControl,OriginalReceivingPrimary,RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::ManuallyDrop;
use semio_framework_value_ordered::{OrderedMap,OrderedSet};
use semio_framework_value_native_receiver_protocol::{NativeLocalProducer,NativeLocalOperation,NativeLocalAuthority,native_receive_local as receive_local};
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(crate::ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid")}
fn nested(mut d:RetirementDemand,copy_bytes:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy_bytes).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child_grant(mut g:RetainedCloneGrant,copy_bytes:usize)->RetainedCloneGrant{g.maximum_items=1;g.maximum_copy_bytes=g.maximum_copy_bytes.saturating_sub(copy_bytes);g.maximum_depth=g.maximum_depth.saturating_sub(1);g}
#[derive(Clone,Copy,PartialEq,Eq)]
struct DefaultCloseLoanBinding{ledger:crate::NativeAllocationIdentity,recipient:Option<u64>,work:crate::OriginalWorkBindingIdentity}
fn default_close_binding(original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,effect:&crate::RetainedAllocationEffect<'_>)->Result<Option<DefaultCloseLoanBinding>,ValueError>{match original{None=>Ok(None),Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("default close changed its original Work Source"))}Ok(Some(DefaultCloseLoanBinding{ledger:original.ledger_identity(),recipient:original.retirement_recipient_identity(),work}))}}}
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
 fn select_advance(&self,body:usize,maximum_work_units:u64)->Result<semio_framework_value_native_receiver_protocol::NativeAdvanceSelection,ValueError>;
 fn advance_nested(&mut self,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>;
 fn advance_local<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 fn recover_advance_local<'reply>(&mut self,_body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 /// ⏳️ Reports only an actual resident original child/action that has begun and remains unresolved.
 fn advance_pending(&self)->bool;
 fn output_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 fn recover_take<'reply>(&mut self,_output:&mut Option<T>,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 fn is_closing(&self)->bool;
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn begin_close_work_demand(&self)->Result<u64,ValueError>;
 fn begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native default lacks an original close work quote"))}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
 fn requires_original_control(&self)->bool;
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🥡️ First-party defaults publish their original owned value through a borrowed reply.
pub trait NativeDefaultValue:RetireOwned+Sized{
 fn default_demand()->RetirementDemand;
 fn default_work_demand()->Result<u64,ValueError>;
 fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 fn recover_default<'reply>(output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
}
/// 🥡️ Publishes one first-party default into the caller's original output slot through a borrowed reply.
pub fn native_admit_default<'reply,T>(output:&mut Option<T>,demand:RetirementDemand,make:impl FnOnce()->T,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if output.is_some(){return Ok(reply.complete(Err(invalid().into())))}if !retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native default exceeded its work authority").into())))}*output=Some(make());Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(demand)))))}
/// 🧺️ One leaf output stays resident before its paid take or controlled retirement.
pub struct NativeDefaultValueCursor<T:NativeDefaultValue>{output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool}
impl<T:NativeDefaultValue> NativeDefaultValueCursor<T>{pub fn new()->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),closing:false}}}
impl<T:NativeDefaultValue> NativeDefaultCursor<T> for NativeDefaultValueCursor<T>{
 fn demands(&self,_:usize,_:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}Ok(if self.output.is_some(){Default::default()}else{T::default_demand()})}
 fn select_advance(&self,_:usize,_:u64)->Result<semio_framework_value_native_receiver_protocol::NativeAdvanceSelection,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}Ok(semio_framework_value_native_receiver_protocol::NativeAdvanceSelection::Local{work:T::default_work_demand()?})}
 fn advance_nested(&mut self,_:&mut NativeDecodeControl<'_>,_:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn advance_local<'reply>(&mut self,_:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{T::admit_default(&mut self.output,effect,reply)}
 fn recover_advance_local<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{T::recover_default(&mut self.output,effect,reply)}
 fn advance_pending(&self)->bool{false}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}semio_framework_value_native_receiver_protocol::native_output_demand::<T>(0)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Err(invalid().into())))}semio_framework_value_native_receiver_protocol::native_record_pure_take(semio_framework_value_native_receiver_protocol::native_output_take(&mut self.output,output,0,effect),reply)}
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}self.retirement.next_owner_demands::<T>(self.output.is_some(),&None,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}self.retirement.next_owner_work::<T>(self.output.is_some(),&None,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_birth()||self.requires_original_control(){return Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"default leaf requires its selected original close"))}let g=effect.grant();if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let demand=self.close_demands(body)?;if !retained_clone_grant_admits(g,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply)}let turn=self.retirement.begin_granted(&mut self.output,effect);Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.retirement.recover_step_into(body,effect,reply)}
 fn requires_original_birth(&self)->bool{self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.retirement.step_original_birth(original,grant)}
 fn requires_original_control(&self)->bool{self.retirement.requires_original_control()}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_birth(){return Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle default leaf Birth before close"))}if self.retirement.requires_original_control(){return self.retirement.step_original_into(original,body,effect,reply)}self.close_step(body,effect,reply)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.retirement.recover_step_original_into(original,body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.retirement.is_empty()}
}
impl<T:NativeDefaultValue> Drop for NativeDefaultValueCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native default leaf requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output)}}}}
macro_rules! pure_default_methods{($value:expr)=>{
 fn default_demand()->RetirementDemand{fixed(size_of::<Option<Self>>())}
 fn default_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if output.is_some(){return Ok(reply.complete(Err(invalid().into())))}let d=Self::default_demand();if !retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native default exceeded its work authority").into())))}*output=Some($value);Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
 fn recover_default<'reply>(_:&mut Option<Self>,_:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
};}
macro_rules! empty_default{($($ty:ty=>$value:expr),*$(,)?)=>{$(impl NativeDefaultValue for $ty{pure_default_methods!($value);}impl NativeDefault for $ty{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}})*};}
empty_default!(()=>(),bool=>false,u8=>0,u16=>0,u32=>0,u64=>0,usize=>0,i8=>0,i16=>0,i32=>0,i64=>0,isize=>0,f32=>0.0,f64=>0.0,String=>String::new(),OrderedSet=>OrderedSet::new());
macro_rules! generic_empty_default{(($($generic:tt)*),$ty:ty,$value:expr)=>{impl<$($generic)*> NativeDefaultValue for $ty{pure_default_methods!($value);}impl<$($generic)*> NativeDefault for $ty{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{NativeDefaultValueCursor::new()}}};}
generic_empty_default!((T:RetireOwned),Vec<T>,Vec::new());
generic_empty_default!((T:RetireOwned),Option<T>,None);
generic_empty_default!((T:RetireOwned,const N:usize),semio_framework_value_paged_list::PagedList<T,N>,Self::new());
generic_empty_default!((V:RetireOwned+Sync),OrderedMap<V>,Self::new());
#[path="📦️slot/🦀️.rs"]mod slot;pub use slot::NativeDefaultSlot;
macro_rules! native_composite_close_start{()=>{
 fn is_closing(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.advance_pending()||self.close_reply.is_some()||self.birth_request.is_some(){return Err(invalid())}Ok(if self.closing{Default::default()}else{fixed(std::mem::size_of::<bool>())})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_demand()?;Ok(u64::from(!self.closing))}
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let d=match self.begin_close_demand(){Ok(d)=>d,Err(error)=>return Ok(reply.complete(Err(error.into())))};if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"default composite close start exceeded original Work").into())))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::native_decoding::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::native_decoding::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.begin_close_into(effect,reply)}
};}
#[path="🔢️array/🦀️.rs"]mod arrays;pub use arrays::NativeArrayDefaultCursor;
#[path="👯️pair/🦀️.rs"]mod pairs;pub use pairs::NativePairDefaultCursor;

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod tests;
