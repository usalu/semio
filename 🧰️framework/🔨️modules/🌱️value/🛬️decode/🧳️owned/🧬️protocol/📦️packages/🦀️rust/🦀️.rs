//! 🛬️ Canonical retained native receiving and original-close protocol.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,OriginalEffectReply,OriginalEffectReplyComplete};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll};
use retained_clone::RetainedCloneRef;

pub fn receiving_capacity(control:&NativeDecodeControl<'_>,bytes:usize)->Result<(),ValueError>{if control.owned_bytes().checked_add(bytes).is_none_or(|next|next>control.maximum_bytes()){Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit"))}else{Ok(())}}

/// 🧾️ The full original child, work and receiving causes remain independent.
pub type NativeReceivingChildFailure<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError=ValueError>=semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectFailure<E>;
/// 📬️ Resident receiving action and its complete original failure remain owned until funded settlement.
pub struct NativeReceivingChild<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError=ValueError>{
 pub effect:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffect<E>,
 pub demand:Option<RetirementDemand>,
 pub grant:Option<RetainedCloneGrant>,
 pub work:Option<u64>,
 pub ceiling:Option<u64>,
 pub request:Option<(usize,u64)>,
 pub tag:u8,
 pub failure:std::mem::ManuallyDrop<Option<NativeReceivingChildFailure<E>>>,
}
impl<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError> Default for NativeReceivingChild<E>{fn default()->Self{Self{effect:Default::default(),demand:None,grant:None,work:None,ceiling:None,request:None,tag:0,failure:std::mem::ManuallyDrop::new(None)}}}
impl<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError> NativeReceivingChild<E>{
 pub fn pending(&self)->bool{self.request.is_some()||self.demand.is_some()||self.effect.phase()!=semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectPhase::Empty}
 pub fn terminal_is_empty(&self)->bool{!self.pending()&&self.failure.is_none()}
 /// 🚦️ Settles one exact original child effect and retains every independently owned refusal.
 /// 🪢️ Drives only the resident producer authority and retains its original action across Pending.
 pub fn poll<P>(&mut self,control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,grant:retained_clone::RetainedCloneGrant,work:u64,maximum_work_units:u64,authority:&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,&mut Option<semio_framework_value_native_decoding::native_decoding::OriginalEffectAction<RetainedCloneStep,E>>)->Result<bool,ValueError>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary> where P:semio_framework_value_native_decoding::native_decoding::OriginalEffectProducer<Output=RetainedCloneStep,Failure=E>{
  use semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectPoll;
  if self.failure.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"original receiving failure remains resident").into()),work:0})}
  if self.ceiling.is_some_and(|old|maximum_work_units<old)||self.ceiling.unwrap_or(maximum_work_units)<work||self.grant.is_some_and(|old|old!=grant){return if self.pending(){crate::RetainedWorkPoll::Pending{work:0}}else{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}}let _=*self.ceiling.get_or_insert(maximum_work_units);let grant=*self.grant.get_or_insert(grant);let selected=*self.demand.get_or_insert(demand);let units=*self.work.get_or_insert(work);match control.receive_original_effect(selected,units,&mut self.effect,authority,|control,effect,authority,action|{let selected=grant.restricted_by(effect.grant());let units=units.min(effect.maximum_work_units());let mut child=effect.reborrow(selected,units)?;invoke(control,&mut child,authority,action)}){
   Ok(NativeReceivingEffectPoll::Pending{work})=>crate::RetainedWorkPoll::Pending{work},
   Ok(NativeReceivingEffectPoll::Complete(turn))=>{self.demand=None;self.grant=None;self.work=None;self.ceiling=None;self.tag=0;match turn.result{Ok(step)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:turn.work}),Err(failure)=>{let progress=failure.retained_progress();*self.failure=Some(failure);crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"original receiving child refused").with_retained_progress(progress).into()),work:turn.work})}}},
   Err(error)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0}),
  }
 }
 /// 🪪️ Freezes and authenticates the actual resident local request before the original Source quote.
 pub fn poll_local<P>(&mut self,control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,grant:retained_clone::RetainedCloneGrant,work:u64,body:usize,maximum_work_units:u64,authority:&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,&mut Option<semio_framework_value_native_decoding::native_decoding::OriginalEffectAction<RetainedCloneStep,E>>)->Result<bool,ValueError>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary> where P:semio_framework_value_native_decoding::native_decoding::OriginalEffectProducer<Output=RetainedCloneStep,Failure=E>{let request=(body,maximum_work_units);if self.request.is_some_and(|original|original!=request){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original local receiver request changed").into()),work:0})}self.request=Some(request);let turn=self.poll(control,demand,grant,work,maximum_work_units,authority,invoke);if matches!(turn,crate::RetainedWorkPoll::Complete(_))&&self.demand.is_none(){self.request=None}turn}
 pub fn failure_demand(&self,retirement:&retained_clone::RetainedCloneClose,body:usize)->Result<RetirementDemand,ValueError>{retirement.next_owner_demands::<NativeReceivingChildFailure<E>>(true,&None,body)}
 pub fn failure_work(&self,retirement:&retained_clone::RetainedCloneClose,body:usize)->Result<u64,ValueError>{retirement.next_owner_work::<NativeReceivingChildFailure<E>>(true,&None,body)}
 pub fn close_failure(&mut self,retirement:&mut retained_clone::RetainedCloneClose,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{let turn=retirement.begin_granted(&mut self.failure,effect);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}
}


/// 📐️ Quotes the actual source and destination option-header writes for one owned publication.
pub fn native_output_demand<T>(extra:usize)->Result<RetirementDemand,ValueError>{let copy_bytes=std::mem::size_of::<Option<T>>().checked_mul(2).and_then(|n|n.checked_add(extra)).ok_or_else(||ValueError::invariant("native output publication extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
/// 📤️ Transfers the exact resident value only after its original effect records the real action.
pub fn native_output_take<T>(source:&mut Option<T>,output:&mut Option<T>,extra:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
 let demand=match native_output_demand::<T>(extra){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
 if source.is_none()||output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native publication requires its original source and empty output slots").into()),work:0})}
 if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.maximum_work_units()==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
 match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}
 *output=source.take();crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1})
}

/// 📬️ Records one completed pure publication directly into the original borrowed producer reply.
pub fn native_record_pure_take<'reply>(turn:crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(match turn{crate::RetainedWorkPoll::Pending{..}=>reply.pending(),crate::RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})}

/// 🧭️ Selects the resident child action or one separately funded local receiver action.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeAdvanceSelection{Nested,Local{work:u64}}

pub trait NativeReceiver<I:?Sized,T>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 /// 🧭️ Selects the actual resident child or local action before any original callback.
 fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>;
 /// 🪢️ Polls only the selected resident child without mutating parent metadata.
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>;
 /// 📬️ Records one local action before its original borrowed reply can unwind.
 fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 /// 🔁️ Recovers only the same resident local producer action.
 fn recover_advance_local<'reply>(&mut self,_original:RetainedCloneRef<'_,I>,_body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 fn output_ready(&self)->bool;
 /// 📐️ Quotes the exact original output-slot publication.
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 /// ⏱️ Quotes the actual selected publication work.
 fn take_work_demand(&self)->Result<u64,ValueError>;
 /// 📤️ Publishes only into the caller-resident original output under the same effect.
 fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 /// 🔁️ Recovers only the same original producer action without replaying a completed transfer.
 fn recover_take<'reply>(&mut self,_output:&mut Option<T>,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver lacks an original close work quote"))}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll;
 fn requires_original_control(&self)->bool{false}
 fn requires_original_birth(&self)->bool{false}
 fn close_original_birth_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver has no original birth to settle").into()),work:0})}
 fn close_original_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_control(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver requires its original close port").into()),work:0})}self.close_step(effect)}
 fn terminal_is_empty(&self)->bool;
}

pub struct NativeReceivingBatch{left:RetainedCloneGrant,progress:RetainedCloneProgress,work:u128}
impl NativeReceivingBatch{
 pub fn new(left:RetainedCloneGrant)->Self{Self{left,progress:Default::default(),work:0}}
 pub fn grant(&self)->RetainedCloneGrant{self.left}
 /// ⏱️ Returns the same caller ceiling less actual child work already performed.
 pub fn remaining_work(&self,maximum:u64)->Result<u64,ValueError>{u128::from(maximum).checked_sub(self.work).and_then(|left|u64::try_from(left).ok()).ok_or_else(||ValueError::invariant("native receiving batch exceeded its original work ceiling"))}
 pub fn finish(self)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(self.progress)),work:self.work}}
 pub fn refuse(self,e:ValueError)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(e.with_retained_progress(self.progress).into()),work:self.work}}
 pub fn record(mut self,turn:RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,unposted:bool,complete:bool)->Result<Self,RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>>{
  self.work+=turn.work;let step=match turn.result{Ok(s)=>s,Err(mut e)=>{e.progress=self.progress.saturating_add(e.progress);return Err(RetainedWorkOutcome{result:Err(e),work:self.work})}};let p=step.progress();self.progress=match self.progress.checked_add(p){Ok(total)=>total,Err(error)=>return Err(self.refuse(error))};if complete{return Err(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:self.work})}if !unposted||p==RetainedCloneProgress::default(){return Err(self.finish())}
  self.left.maximum_items-=p.copied_items;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;if self.left.maximum_items==0{Err(self.finish())}else{Ok(self)}
 }
}


#[doc(hidden)]
pub fn native_receiver_backing_demands<I:?Sized,T,C:NativeReceiver<I,T>>(storage:&semio_framework_value_native_storage::NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize)->Result<RetirementDemand,ValueError>{
 if let Some(cursor)=retired.as_ref(){if !cursor.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid"))}return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<C>>>(),release_bytes:std::alloc::Layout::new::<C>().size(),depth:1,..Default::default()})}if let Some(cursor)=storage.get(){if !cursor.terminal_is_empty(){return {let mut d=cursor.close_demands(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid"))?;Ok(d)}}return Ok(semio_framework_value_native_storage::NativeOwnedStorage::<C>::box_demand())}Ok(storage.release_demand())
}
#[doc(hidden)]
pub fn close_native_receiver_backing<I:?Sized,T,C:NativeReceiver<I,T>>(storage:&mut semio_framework_value_native_storage::NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
 use crate::RetainedWorkPoll::{Complete,Pending};let g=effect.grant();let d=match native_receiver_backing_demands::<I,T,C>(storage,retired,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !crate::retained_clone::retained_clone_grant_admits(g,d)||effect.maximum_work_units()==0{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
 if retired.is_some(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native receiver release exceeded its work authority").into()),work:1}),Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let layout=std::alloc::Layout::new::<C>();let pointer=Box::into_raw(retired.take().expect("retired owner selected"));if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1})}
 if let Some(cursor)=storage.get_mut(){if !cursor.terminal_is_empty(){return match cursor.close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units()).expect("retirement child exceeds original effect")){Pending{work}=>Pending{work},Complete(turn)=>Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}return storage.take_box_into(retired,effect).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}let turn=storage.release(effect);Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})
}


pub fn receiving_child_body_budget(body:usize,metadata:usize)->usize{body.saturating_sub(metadata)}

#[doc(hidden)]
#[macro_export]
macro_rules! scalar_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! string_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.cursor.close_work_demand(body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! option_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! sequence_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<C>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<C>(true,&None,body)}if self.storage.is_some(){return self.retirement.next_owner_work::<C::Storage>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! box_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.pending_box.is_some(){return self.retirement.next_owner_work::<Box<T>>(true,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}Ok(1)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! fixed_array_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.live>0{return Ok(1)}if self.output.is_some(){return self.retirement.next_owner_work::<[T;N]>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! set_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.ordered.failure.is_some(){return self.ordered.failure_work(&self.retirement,body)}if self.cleanup.failure.is_some(){return self.cleanup.failure_work(&self.retirement,body)}if self.residual.is_some(){return self.retirement.next_owner_work::<Residual>(true,&None,body)}if self.cursor.is_some(){return self.retirement.next_owner_work::<UpdateCursor<()>>(true,&None,body)}if self.shared_value.is_some(){return self.retirement.next_owner_work::<SharedOwner<()>>(true,&None,body)}if self.shared_key.is_some(){return self.retirement.next_owner_work::<SharedOwner<String>>(true,&None,body)}if self.value.is_some(){return self.retirement.next_owner_work::<()>(true,&None,body)}if self.key.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.members.is_some(){return self.retirement.next_owner_work::<Members>(true,&None,body)}if self.map.is_some(){return self.retirement.next_owner_work::<OrderedMap<()>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! map_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.ordered.failure.is_some(){return self.ordered.failure_work(&self.retirement,body)}if self.cleanup.failure.is_some(){return self.cleanup.failure_work(&self.retirement,body)}if self.residual.is_some(){return self.retirement.next_owner_work::<Residual<V>>(true,&None,body)}if self.cursor.is_some(){return self.retirement.next_owner_work::<UpdateCursor<V>>(true,&None,body)}if self.shared_value.is_some(){return self.retirement.next_owner_work::<SharedOwner<V>>(true,&None,body)}if self.shared_key.is_some(){return self.retirement.next_owner_work::<SharedOwner<String>>(true,&None,body)}if self.value.is_some(){return self.retirement.next_owner_work::<V>(true,&None,body)}if self.key.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.map.is_some(){return self.retirement.next_owner_work::<OrderedMap<V>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! shared_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if let Some(child)=self.child.get(){if !child.terminal_is_empty(){return child.close_work_demand(body)}}if self.child.reserved()||self.retired_header.is_some(){return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<R>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! list_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if let Some(child)=self.child.get(){if !child.terminal_is_empty(){return child.close_work_demand(body)}}if self.child.reserved()||self.child_retired.is_some(){return Ok(1)}if let Some(builder)=self.builder.get(){if !builder.terminal_is_empty(){return builder.close_work_demand(body)}}if self.builder.reserved()||self.builder_retired.is_some(){return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<PersistentList<T>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}


#[doc(hidden)]
#[macro_export]
macro_rules! list_original_close_bridge{()=>{fn requires_original_birth(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_birth()}if self.child.reserved()||self.child_retired.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}self.retirement.requires_original_birth()}fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.slots.terminal_is_empty(){return self.slots.close_original_birth_step(original,grant)}if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if let Some(c)=self.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,child)}}if let Some(c)=self.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,child)}}self.retirement.step_original_birth(original,grant)}fn requires_original_control(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_control()}if self.child.reserved()||self.child_retired.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}self.retirement.requires_original_control()}fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into()),work:0})}if !self.slots.terminal_is_empty()&&self.slots.requires_original_control(){return self.slots.close_original_step(original,effect)}let g=effect.grant();let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};if let Some(c)=self.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){return c.close_original_step(original,&mut effect.reborrow(child,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}}if let Some(c)=self.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){return c.close_original_step(original,&mut effect.reborrow(child,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}}if self.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! shared_original_close_bridge{()=>{fn requires_original_birth(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_birth()}if self.child.reserved()||self.retired_header.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}self.retirement.requires_original_birth()}fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.slots.terminal_is_empty(){return self.slots.close_original_birth_step(original,grant)}if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if let Some(c)=self.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}}self.retirement.step_original_birth(original,grant)}fn requires_original_control(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_control()}if self.child.reserved()||self.retired_header.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}self.retirement.requires_original_control()}fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into()),work:0})}if !self.slots.terminal_is_empty()&&self.slots.requires_original_control(){return self.slots.close_original_step(original,effect)}if let Some(c)=self.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){let g=effect.grant();let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};return c.close_original_step(original,&mut effect.reborrow(child,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}}if self.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! ordered_original_close_bridge{()=>{
 fn requires_original_birth(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{(self.ordered.pending()||self.cleanup.pending())||if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.ordered.pending()||self.cleanup.pending(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original ordered receiving action requires settlement").into()),work:0})}if !self.child.terminal_is_empty()&&self.child.requires_original_control(){return self.child.close_original_step(original,effect)}if !self.retirement.is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! ordered_shared_admission{($owner:ident,$control:ident,$demand:ident,$grant:ident,$work:ident,$maximum:ident,$input:ident,$output:ident)=>{{
 let units=match $owner.ordered.work{Some(units)=>units,None=>SharedOwner::admission_work(&$owner.$input,&$owner.$output)?};
 let turn=$owner.ordered.poll($control,$demand,$grant,units,$maximum,|_,effect|{let turn=SharedOwner::admit_into(&mut $owner.$input,&mut $owner.$output,effect);RetainedWorkOutcome{result:turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())),work:turn.work}});
 $work=turn.work;return turn.result
}};}

#[doc(hidden)]
#[macro_export]
macro_rules! ordered_failure_accessors{()=>{
 /// 🔎️ Borrows every original ordered-child and receiving refusal without moving its owner.
 pub fn ordered_failure(&self)->Option<&$crate::NativeReceivingChildFailure>{self.ordered.failure.as_ref()}
 /// 🔎️ Borrows every original retirement-child and receiving refusal without moving its owner.
 pub fn cleanup_failure(&self)->Option<&$crate::NativeReceivingChildFailure<crate::OriginalRetirementFailure>>{self.cleanup.failure.as_ref()}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! ordered_cleanup_guard{($owner:ident,$child:ident)=>{if $owner.cleanup.demand.is_none()&&($owner.$child.requires_original_birth()||$owner.$child.requires_original_control()){return Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original cleanup requires its original close loan").into())}};}

#[doc(hidden)]
#[macro_export]
macro_rules! ordered_close_demands{($($field:ident:$ty:ty),* $(,)?)=>{
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if !self.child.terminal_is_empty(){return self.child.close_demands(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}
  if self.ordered.failure.is_some(){return self.ordered.failure_demand(&self.retirement,body)}
  if self.cleanup.failure.is_some(){return self.cleanup.failure_demand(&self.retirement,body)}
  $(if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)})*
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_close_step{($($field:ident),* $(,)?)=>{
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if self.ordered.pending()||self.cleanup.pending(){return Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}if !self.child.terminal_is_empty(){return self.child.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}if self.ordered.failure.is_some(){return Complete(self.ordered.close_failure(&mut self.retirement,effect))}if self.cleanup.failure.is_some(){return Complete(self.cleanup.close_failure(&mut self.retirement,effect))}$(if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})})*RetainedCloneBinding::close_one(&mut self.binding,effect)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! native_receive_zero_work{($work:expr)=>{if $work==0{return $crate::RetainedWorkOutcome{result:Ok($crate::retained_clone::RetainedCloneStep::Progress(Default::default())),work:0}}};}

#[doc(hidden)]
#[macro_export]
macro_rules! scalar_close_engine{()=>{ fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 $crate::scalar_close_work_quote!();
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}RetainedCloneBinding::close_one(&mut self.binding,effect)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! option_close_engine{()=>{
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.staged.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.staged.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.child.terminal_is_empty(){return self.child.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}if self.staged.is_some(){let turn=self.retirement.begin_granted(&mut self.staged,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}RetainedCloneBinding::close_one(&mut self.binding,effect)}
 fn requires_original_birth(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty()&&self.child.requires_original_control(){return self.child.close_original_step(original,effect)}if !self.retirement.is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! scalar_output_engine{()=>{
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}$crate::native_output_demand::<T>(std::mem::size_of::<bool>())}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{if !self.output_ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}let turn=$crate::native_output_take(&mut self.output,output,std::mem::size_of::<bool>(),effect);if matches!(&turn,crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(_)),..})){self.taken=true}turn})();$crate::native_record_pure_take(turn,reply)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! option_output_engine{()=>{
 fn output_ready(&self)->bool{!self.closing&&matches!(self.phase,2|3)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}if self.phase==3&&self.staged.is_none(){return nested(self.child.take_demand()?,0)}let source=if self.phase==3{std::mem::size_of::<Option<T>>()}else{0};let bytes=source.checked_add(std::mem::size_of::<Option<Option<T>>>()).and_then(|n|n.checked_add(std::mem::size_of::<u8>())).ok_or_else(invalid)?;Ok(fixed(bytes))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;if self.phase==3&&self.staged.is_none(){self.child.take_work_demand()}else{Ok(1)}}
 fn take<'reply>(&mut self,output:&mut Option<Option<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let d=match self.take_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}let g=effect.grant();if !retained_clone_grant_admits(g,d)||effect.maximum_work_units()==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if self.phase==3&&self.staged.is_none(){let child=child_grant(g,0);let mut loan=match effect.reborrow(child,effect.maximum_work_units()){Ok(loan)=>loan,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};return match self.child.take(&mut self.staged,&mut loan){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=if self.phase==2{None}else{self.staged.take()};*output=Some(value);self.phase=4;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1})
 })();$crate::native_record_pure_take(turn,reply)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! native_owned_output{($ty:ty,$invalid:ident)=>{
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err($invalid())}$crate::native_output_demand::<$ty>(0)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<$ty>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Err($invalid().into())))}$crate::native_record_pure_take($crate::native_output_take(&mut self.output,output,0,effect),reply)}
};}
#[doc(hidden)]
#[macro_export]
macro_rules! native_owned_output_state{($ty:ty,$invalid:ident,$field:ident,$next:expr)=>{
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err($invalid())}$crate::native_output_demand::<$ty>(std::mem::size_of_val(&self.$field))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<$ty>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Err($invalid().into())))}let turn=$crate::native_output_take(&mut self.output,output,std::mem::size_of_val(&self.$field),effect);if matches!(&turn,crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(_)),..})){self.$field=$next}$crate::native_record_pure_take(turn,reply)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! native_owned_public_output{($ty:ty,$invalid:ident,$ready:ident)=>{
 /// 📐️ Quotes publication into the caller's original output slot.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.$ready(){return Err($invalid())}$crate::native_output_demand::<$ty>(0)}
 /// ⏱️ Quotes the selected actual publication work.
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 /// 📤️ Publishes the exact resident value through the caller's original effect.
 pub fn take(&mut self,output:&mut Option<$ty>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{if !self.$ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err($invalid().into()),work:0})}$crate::native_output_take(&mut self.output,output,0,effect)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! result_output_engine{()=>{ fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}let selected=self.selector.result().ok_or_else(invalid)?;if selected.index==0&&self.staged_ok.is_none(){return nested(self.ok.take_demand()?,0)}if selected.index!=0&&self.staged_err.is_none(){return nested(self.err.take_demand()?,0)}let source=if selected.index==0{std::mem::size_of::<Option<T>>()}else{std::mem::size_of::<Option<E>>()};let bytes=source.checked_add(std::mem::size_of::<Option<Result<T,E>>>()).and_then(|n|n.checked_add(std::mem::size_of::<bool>())).ok_or_else(invalid)?;Ok(fixed(bytes))}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;let selected=self.selector.result().ok_or_else(invalid)?;if selected.index==0&&self.staged_ok.is_none(){self.ok.take_work_demand()}else if selected.index!=0&&self.staged_err.is_none(){self.err.take_work_demand()}else{Ok(1)}}
 fn take<'reply>(&mut self,output:&mut Option<Result<T,E>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let d=match self.take_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if output.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}let g=effect.grant();if !retained_clone_grant_admits(g,d)||effect.maximum_work_units()==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let selected=match self.selector.result(){Some(selected)=>selected,None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})};
  if selected.index==0&&self.staged_ok.is_none(){let mut child=match effect.reborrow(child_grant(g,0),effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};return match self.ok.take(&mut self.staged_ok,&mut child){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
  if selected.index!=0&&self.staged_err.is_none(){let mut child=match effect.reborrow(child_grant(g,0),effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};return match self.err.take(&mut self.staged_err,&mut child){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=if selected.index==0{match self.staged_ok.take(){Some(value)=>Ok(value),None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})}}else{match self.staged_err.take(){Some(value)=>Err(value),None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})}};*output=Some(value);self.taken=true;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1})
 })();$crate::native_record_pure_take(turn,reply)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! result_close_engine{()=>{ fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.ok.terminal_is_empty(){return nested(self.ok.close_demands(body)?,0)}if !self.err.terminal_is_empty(){return nested(self.err.close_demands(body)?,0)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.staged_ok.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.staged_err.is_some(){return self.retirement.next_owner_demands::<E>(true,&None,body)}self.selector.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.ok.terminal_is_empty(){return self.ok.close_work_demand(body)}if !self.err.terminal_is_empty(){return self.err.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.staged_ok.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.staged_err.is_some(){return self.retirement.next_owner_work::<E>(true,&None,body)}self.selector.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.ok.terminal_is_empty(){return self.ok.close_step(&mut effect.reborrow(child_grant(g,0),effect.maximum_work_units()).expect("retirement child exceeds original effect"))}if !self.err.terminal_is_empty(){return self.err.close_step(&mut effect.reborrow(child_grant(g,0),effect.maximum_work_units()).expect("retirement child exceeds original effect"))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect)}if self.staged_ok.is_some(){let turn=self.retirement.begin_granted(&mut self.staged_ok,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}if self.staged_err.is_some(){let turn=self.retirement.begin_granted(&mut self.staged_err,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}self.selector.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn requires_original_birth(&self)->bool{if !self.ok.terminal_is_empty(){self.ok.requires_original_birth()}else if !self.err.terminal_is_empty(){self.err.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.ok.terminal_is_empty(){self.ok.close_original_birth_step(original,child_grant(grant,0))}else if !self.err.terminal_is_empty(){self.err.close_original_birth_step(original,child_grant(grant,0))}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if !self.ok.terminal_is_empty(){self.ok.requires_original_control()}else if !self.err.terminal_is_empty(){self.err.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.ok.terminal_is_empty()&&self.ok.requires_original_control(){return self.ok.close_original_step(original,&mut effect.reborrow(child_grant(grant,0),effect.maximum_work_units()).expect("selected result child exceeds its original effect"))}if !self.err.terminal_is_empty()&&self.err.requires_original_control(){return self.err.close_original_step(original,&mut effect.reborrow(child_grant(grant,0),effect.maximum_work_units()).expect("selected result child exceeds its original effect"))}if !self.retirement.is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
};}

#[path="🎛️slot.rs"]mod slot;
pub use slot::{NativeSlotOperation,NativeSlotProducer};

#[doc(hidden)]
#[macro_export]
macro_rules! native_mapped_receiver_engine {()=>{
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>NativeReceiver<I,U>for NativeMappedReceiver<I,T,U>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}input.check_binding(&self.binding)?;if self.binding.is_none(){return Ok(fixed(input.binding_copy_bytes()))}if self.pending.is_some(){return Ok(fixed(std::mem::size_of::<T>().saturating_add(std::mem::size_of::<U>())))}if self.inner.output_ready(){return nested(self.inner.take_demand()?)}nested(self.inner.demands(input,body,maximum_work_units)?)}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{semio_framework_value_native_receiver_protocol::native_receive_zero_work!(maximum_work_units);let mut work=0;let result=(||{let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;if self.binding.is_none(){let turn=input.bind(&mut self.binding,g);let turn=control.receive_retained_work(g,crate::RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work});work=turn.work;return turn.result.map_err(Into::into)}if self.pending.is_some(){let value=self.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?;*self.output=Some(U::from_native(value));let turn=control.receive_retained_work(g,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1});work=turn.work;return turn.result.map_err(Into::into)}if self.inner.output_ready(){let turn=self.inner.take(child(g));work=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.pending=Some(value);return Ok(RetainedCloneStep::Progress(p))}let turn=self.inner.advance(input,control,body,maximum_work_units,child(g));work=turn.work;turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))})();RetainedWorkOutcome{result,work}}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output!(U,invalid);
 fn begin_close(&mut self){self.closing=true;self.inner.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return nested(self.inner.close_demands(body)?)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<U>(true,&None,body)}if self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<U>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;if self.requires_original_control(){return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped child requires its original close port").into()),work:0})}if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty(){return self.inner.close_step(&mut effect.reborrow(child(g),effect.maximum_work_units()).expect("retirement child exceeds original effect")).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|step|RetainedCloneStep::Progress(step.progress()))}if self.pending.is_none()&&self.output.is_none()&&self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_one(&mut self.binding,effect)}let turn=if self.pending.is_some(){self.retirement.begin_granted(&mut self.pending,effect)}else{self.retirement.begin_granted(&mut self.output,effect)};Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
 fn requires_original_birth(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty()&&self.inner.requires_original_birth(){return self.inner.close_original_birth_step(original,child(grant))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped owner has no original birth to settle").into()),work:0})}
 fn requires_original_control(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into()),work:0})}let g=effect.grant();if !self.inner.terminal_is_empty()&&self.inner.requires_original_control(){let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}return self.inner.close_original_step(original,&mut effect.reborrow(child(g),effect.maximum_work_units()).expect("retirement child exceeds original effect"))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
};}
