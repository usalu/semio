//! 🛬️ Canonical retained native receiving and original-close protocol.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,NativeReceivingFailure,OriginalEffectReply,OriginalEffectReplyComplete};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::{OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementFailure,OriginalRetirementCloseLoan};
use retained_clone::RetainedCloneRef;
pub use semio_framework_value_native_clone_engine::RetainedCursorRelease;

pub fn receiving_capacity(control:&NativeDecodeControl<'_>,bytes:usize)->Result<(),ValueError>{if control.owned_bytes().checked_add(bytes).is_none_or(|next|next>control.maximum_bytes()){Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit"))}else{Ok(())}}

/// 🧾️ The full original child, work and receiving causes remain independent.
pub type NativeReceivingChildFailure<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError=ValueError>=semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectFailure<E>;
/// 🧳️ Wraps one settled typed effect without flattening its independently owned causes.
pub trait NativeReceivingChildError:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError+Sized{fn into_native_failure(failure:NativeReceivingChildFailure<Self>)->NativeReceivingFailure;}
impl NativeReceivingChildError for ValueError{fn into_native_failure(failure:NativeReceivingChildFailure<Self>)->NativeReceivingFailure{NativeReceivingFailure::ValueEffect(failure)}}
impl NativeReceivingChildError for OriginalRetirementFailure{fn into_native_failure(failure:NativeReceivingChildFailure<Self>)->NativeReceivingFailure{NativeReceivingFailure::RetirementEffect(failure)}}
impl NativeReceivingChildError for OriginalReceivingPrimary{fn into_native_failure(failure:NativeReceivingChildFailure<Self>)->NativeReceivingFailure{NativeReceivingFailure::ReceivingEffect(failure)}}
/// 📬️ Resident receiving action and its complete original failure remain owned until funded settlement.
pub struct NativeReceivingChild<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError=ValueError>{
 pub effect:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffect<E>,
 pub demand:Option<RetirementDemand>,
 pub grant:Option<RetainedCloneGrant>,
 pub work:Option<u64>,
 pub ceiling:Option<u64>,
 pub request:Option<(usize,u64)>,
 pub tag:u8,
}
impl<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError> Default for NativeReceivingChild<E>{fn default()->Self{Self{effect:Default::default(),demand:None,grant:None,work:None,ceiling:None,request:None,tag:0}}}
impl<E:semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectError> NativeReceivingChild<E>{
 pub fn pending(&self)->bool{self.request.is_some()||self.demand.is_some()||self.effect.phase()!=semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectPhase::Empty}
 pub fn terminal_is_empty(&self)->bool{!self.pending()}
 /// 🚦️ Settles one exact original child effect and retains every independently owned refusal.
 /// 🪢️ Drives only the resident producer authority and retains its original action across Pending.
 pub fn poll<P>(&mut self,control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,grant:retained_clone::RetainedCloneGrant,work:u64,maximum_work_units:u64,authority:&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,&mut Option<semio_framework_value_native_decoding::native_decoding::OriginalEffectAction<RetainedCloneStep,E>>)->Result<bool,ValueError>)->crate::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure> where E:NativeReceivingChildError,P:semio_framework_value_native_decoding::native_decoding::OriginalEffectProducer<Output=RetainedCloneStep,Failure=E>{
  use semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectPoll;
  if self.ceiling.is_some_and(|old|maximum_work_units!=old)||self.ceiling.unwrap_or(maximum_work_units)<work||self.grant.is_some_and(|old|old!=grant){return if self.pending(){crate::RetainedWorkPoll::Pending{work:0}}else{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}}let _=*self.ceiling.get_or_insert(maximum_work_units);let grant=*self.grant.get_or_insert(grant);let selected=*self.demand.get_or_insert(demand);let units=*self.work.get_or_insert(work);match control.receive_original_effect(selected,units,&mut self.effect,authority,|control,effect,authority,action|{let selected=grant.restricted_by(effect.grant());let units=units.min(effect.maximum_work_units());let mut child=effect.reborrow(selected,units)?;invoke(control,&mut child,authority,action)}){   Ok(NativeReceivingEffectPoll::Pending{work})=>crate::RetainedWorkPoll::Pending{work},
   Ok(NativeReceivingEffectPoll::Complete(turn))=>{self.demand=None;self.grant=None;self.work=None;self.ceiling=None;self.tag=0;match turn.result{Ok(step)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:turn.work}),Err(failure)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(E::into_native_failure(failure)),work:turn.work})}},
   Err(error)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0}),
  }
 }
 /// 🪪️ Freezes and authenticates the actual resident local request before the original Source quote.
 pub fn poll_local<P>(&mut self,control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,grant:retained_clone::RetainedCloneGrant,work:u64,body:usize,maximum_work_units:u64,authority:&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut semio_framework_value_native_decoding::native_decoding::OriginalEffectAuthority<P>,&mut Option<semio_framework_value_native_decoding::native_decoding::OriginalEffectAction<RetainedCloneStep,E>>)->Result<bool,ValueError>)->crate::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure> where E:NativeReceivingChildError,P:semio_framework_value_native_decoding::native_decoding::OriginalEffectProducer<Output=RetainedCloneStep,Failure=E>{let request=(body,maximum_work_units);if self.request.is_some_and(|original|original!=request){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original local receiver request changed").into()),work:0})}self.request=Some(request);let turn=self.poll(control,demand,grant,work,maximum_work_units,authority,invoke);if matches!(turn,crate::RetainedWorkPoll::Complete(_))&&self.demand.is_none(){self.request=None}turn}

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

/// 🌱️ Supplies an inline child seed without allocating, releasing, calling out or unwinding.
pub trait NativeReceiverSeed<I:?Sized,T,C:NativeReceiver<I,T>>:Send+'static{fn seed(subset:bool)->Option<C>;/// 🧬️ Selects only this factory’s exact optional immutable seed template.
 fn seed_template(_subset:bool)->Option<semio_framework_value_native_storage::NativeSeedTemplate<C>>{None}}
pub trait NativeReceiver<I:?Sized,T>:Send+'static{
 /// 🧬️ Exposes an optional exact vacant native template for the requested seed variant.
 fn seed_template(_subset:bool)->Option<semio_framework_value_native_storage::NativeSeedTemplate<Self>> where Self:Sized{None}
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 /// 🧭️ Selects the actual resident child or local action before any original callback.
 fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>;
 /// ⏱️ Quotes the exact work of the next resident child or local action; a nested receiver must override it.
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<u64,ValueError>{match self.select_advance(original,body,u64::MAX)?{NativeAdvanceSelection::Local{work}=>Ok(work),NativeAdvanceSelection::Nested=>Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver lacks its nested work quote"))}}
 /// 🪢️ Polls only the selected resident child without mutating parent metadata.
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>;
 /// 📬️ Records one local action before its original borrowed reply can unwind.
 fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 /// 🔁️ Recovers only the same resident local producer action.
 fn recover_advance_local<'reply>(&mut self,_original:RetainedCloneRef<'_,I>,_body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 /// ⏳️ Reports only an actual resident original child/action that has begun and remains unresolved.
 fn advance_pending(&self)->bool;
 fn output_ready(&self)->bool;
 /// 📐️ Quotes the exact original output-slot publication.
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 /// ⏱️ Quotes the actual selected publication work.
 fn take_work_demand(&self)->Result<u64,ValueError>;
 /// 📤️ Publishes only into the caller-resident original output under the same effect.
 fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>;
 /// 🔁️ Recovers only the same original producer action without replaying a completed transfer.
 fn recover_take<'reply>(&mut self,_output:&mut Option<T>,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{Ok(reply.pending())}
 /// 🚪️ Observes this owner's separately paid close-start flag.
 fn is_closing(&self)->bool;
 /// 📐️ Quotes only this owner's close-start flag.
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 /// ⏱️ Quotes the actual close-start work.
 fn begin_close_work_demand(&self)->Result<u64,ValueError>;
 /// 🪢️ Starts only this owner under the same original effect.
 fn begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver lacks an original close work quote"))}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn requires_original_control(&self)->bool{false}
 fn requires_original_birth(&self)->bool{false}
 fn close_original_birth_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native receiver has no original birth to settle").into()),work:0})}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;
 fn terminal_is_empty(&self)->bool;
 /// ♻️ Supplies explicit evidence only for this actual empty receiver's bounded native destructor.
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>> where Self:Sized{None}
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


/// 🪢️ Settles one reply-shaped close action of any owner into a poll while charging its recorded work.
pub fn native_close_settle(effect:&mut crate::RetainedAllocationEffect<'_>,close:impl for<'r> FnOnce(&mut crate::RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>)->crate::OriginalRetirementPoll{
 use crate::RetainedWorkPoll::{Complete,Pending};let before=effect.performed_work();let mut slot=None;
 let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|close(&mut *effect,reply).map(|_|()));
 let work=effect.performed_work().saturating_sub(before);
 match(ran,slot.take()){(Err(error),_)=>Complete(RetainedWorkOutcome{result:Err(error.into()),work}),(Ok(()),Some(result))=>Complete(RetainedWorkOutcome{result,work}),(Ok(()),None)=>Pending{work}}
}

/// 🔁️ Drives a child producer through a stack reply and relays its recorded answer under the parent's error type.
pub fn native_relay<'r,T,Child,Parent:From<Child>>(reply:OriginalEffectReply<'r,T,Parent>,drive:impl for<'s> FnOnce(OriginalEffectReply<'s,T,Child>)->Result<OriginalEffectReplyComplete<'s,T,Child>,ValueError>)->Result<OriginalEffectReplyComplete<'r,T,Parent>,ValueError>{
 let mut slot=None;{let inner=OriginalEffectReply::borrow(&mut slot)?;drop(drive(inner)?);}
 Ok(match slot.take(){Some(result)=>reply.complete(result.map_err(Into::into)),None=>reply.pending()})
}
/// 🛑️ Records a funded-but-denied close action as zero progress into the borrowed reply.
pub fn native_close_denied<'r>(reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
/// 🌱️ Records one begun owned retirement into the borrowed close reply.
pub fn native_close_begun<'r>(turn:RetainedWorkOutcome<Option<RetainedCloneStep>>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
/// 📬️ Records one settled close poll into the borrowed reply, leaving a pending poll to its recovery.
pub fn native_reply_poll<'reply>(poll:crate::OriginalRetirementPoll,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>{match poll{crate::RetainedWorkPoll::Pending{..}=>reply.pending(),crate::RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)}}
pub fn receiving_child_body_budget(body:usize,metadata:usize)->usize{body.saturating_sub(metadata)}

/// 🚪️ Implements one owner's paid, nonrecursive close-start flag.
#[macro_export]
macro_rules! native_receiver_close_start{($vis:vis $field:ident)=>{
 $vis fn is_closing(&self)->bool{self.$field}
 $vis fn begin_close_demand(&self)->Result<$crate::RetirementDemand,$crate::ValueError>{Ok(if self.$field{Default::default()}else{$crate::RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()}})}
 $vis fn begin_close_work_demand(&self)->Result<u64,$crate::ValueError>{Ok(u64::from(!self.$field))}
 $vis fn begin_close_into<'reply>(&mut self,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{if self.$field{return Ok(reply.complete(Ok($crate::RetainedCloneStep::Complete(Default::default()))))}let demand=$crate::RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()};if !$crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?==0{return Ok(reply.complete(Ok($crate::RetainedCloneStep::Progress(Default::default()))))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err($crate::ValueError::literal($crate::ValueRefusalKind::WorkLimit,"native receiver close start exceeded its work authority").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}self.$field=true;Ok(reply.complete(Ok($crate::RetainedCloneStep::Complete($crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
 $vis fn recover_begin_close_into<'reply>(&mut self,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{self.begin_close_into(effect,reply)}
};}

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
macro_rules! box_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.pending_box.is_some(){return self.retirement.next_owner_work::<Box<T>>(true,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}Ok(1)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! fixed_array_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.live>0{return Ok(1)}if self.output.is_some(){return self.retirement.next_owner_work::<[T;N]>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! set_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.residual.is_some(){return self.retirement.next_owner_work::<Residual>(true,&None,body)}if self.cursor.is_some(){return self.retirement.next_owner_work::<UpdateCursor<()>>(true,&None,body)}if self.shared_value.is_some(){return self.retirement.next_owner_work::<SharedOwner<()>>(true,&None,body)}if self.shared_key.is_some(){return self.retirement.next_owner_work::<SharedOwner<String>>(true,&None,body)}if self.value.is_some(){return self.retirement.next_owner_work::<()>(true,&None,body)}if self.key.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.members.is_some(){return self.retirement.next_owner_work::<Members>(true,&None,body)}if self.map.is_some(){return self.retirement.next_owner_work::<OrderedMap<()>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! map_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.residual.is_some(){return self.retirement.next_owner_work::<Residual<V>>(true,&None,body)}if self.cursor.is_some(){return self.retirement.next_owner_work::<UpdateCursor<V>>(true,&None,body)}if self.shared_value.is_some(){return self.retirement.next_owner_work::<SharedOwner<V>>(true,&None,body)}if self.shared_key.is_some(){return self.retirement.next_owner_work::<SharedOwner<String>>(true,&None,body)}if self.value.is_some(){return self.retirement.next_owner_work::<V>(true,&None,body)}if self.key.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.map.is_some(){return self.retirement.next_owner_work::<OrderedMap<V>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! shared_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<R>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}

#[doc(hidden)]
#[macro_export]
macro_rules! list_close_work_quote{()=>{fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if let Some(child)=self.child.get(){if !child.terminal_is_empty(){return child.close_work_demand(body)}}if self.child.reserved()||self.child_retired.is_some(){return Ok(1)}if let Some(builder)=self.builder.get(){if !builder.terminal_is_empty(){return builder.close_work_demand(body)}}if self.builder.reserved()||self.builder_retired.is_some(){return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<PersistentList<T>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}};}


#[doc(hidden)]
#[macro_export]
macro_rules! list_original_close_bridge{()=>{fn requires_original_birth(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_birth()}if self.child.reserved()||self.child_retired.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}self.retirement.requires_original_birth()}fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.slots.terminal_is_empty(){return self.slots.close_original_birth_step(original,grant)}if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if let Some(c)=self.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,child)}}if let Some(c)=self.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,child)}}self.retirement.step_original_birth(original,grant)}fn requires_original_control(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_control()}if self.child.reserved()||self.child_retired.is_some(){return self.child.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}self.retirement.requires_original_control()} fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::list_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::list_original_close_route!(self,original,body,effect,reply,true)}};}
#[doc(hidden)]
#[macro_export]
macro_rules! shared_original_close_bridge{()=>{fn requires_original_birth(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_birth()}if !self.child.terminal_is_empty(){return self.child.requires_original_birth()}self.retirement.requires_original_birth()}fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.slots.terminal_is_empty(){return self.slots.close_original_birth_step(original,grant)}if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.child.terminal_is_empty()&&self.child.requires_original_birth(){return self.child.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}self.retirement.step_original_birth(original,grant)}fn requires_original_control(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_control()}if !self.child.terminal_is_empty(){return self.child.requires_original_control()}self.retirement.requires_original_control()} fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::shared_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::shared_original_close_route!(self,original,body,effect,reply,true)}};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_original_close_bridge{()=>{
 fn requires_original_birth(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::ordered_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::ordered_original_close_route!(self,original,body,effect,reply,true)}};}
#[doc(hidden)]
#[macro_export]
macro_rules! list_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if $s.requires_original_birth(){return Ok($reply.complete(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into())))}
if !$s.slots.terminal_is_empty()&&$s.slots.requires_original_control(){return if $recover{$s.slots.recover_close_original_step($original,$body,$effect,$reply)}else{$s.slots.close_original_step($original,$body,$effect,$reply)}}
let g=$effect.grant();let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};
if let Some(c)=$s.child.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){return if $recover{c.recover_close_original_step($original,$body,&mut $effect.nested(child),$reply)}else{c.close_original_step($original,$body,&mut $effect.nested(child),$reply)}}}
if let Some(c)=$s.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){return if $recover{c.recover_close_original_step($original,$body,&mut $effect.nested(child),$reply)}else{c.close_original_step($original,$body,&mut $effect.nested(child),$reply)}}}
if $s.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply)}else{$s.retirement.step_original_into($original,$body,$effect,$reply)}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! shared_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if $s.requires_original_birth(){return Ok($reply.complete(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into())))}
if !$s.slots.terminal_is_empty()&&$s.slots.requires_original_control(){return if $recover{$s.slots.recover_close_original_step($original,$body,$effect,$reply)}else{$s.slots.close_original_step($original,$body,$effect,$reply)}}
if !$s.child.terminal_is_empty()&&$s.child.requires_original_control(){let g=$effect.grant();let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};return if $recover{$s.child.recover_close_original_step($original,$body,&mut $effect.nested(child),$reply)}else{$s.child.close_original_step($original,$body,&mut $effect.nested(child),$reply)}}
if $s.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply)}else{$s.retirement.step_original_into($original,$body,$effect,$reply)}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if !$s.child.terminal_is_empty()&&$s.child.requires_original_control(){return if $recover{$s.child.recover_close_original_into($original,$body,$effect,$reply)}else{$s.child.close_original_into($original,$body,$effect,$reply)}}
if !$s.retirement.is_empty()&&$s.retirement.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply)}else{$s.retirement.step_original_into($original,$body,$effect,$reply)}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_close_demands{($($field:ident:$ty:ty),* $(,)?)=>{
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if !self.child.terminal_is_empty(){return self.child.close_demands(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}
  $(if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)})*
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_close_step{($($field:ident),* $(,)?)=>{
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::ordered_close_route!(self,body,effect,reply,false;$($field),*)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::ordered_close_route!(self,body,effect,reply,true;$($field),*)}
};}
#[doc(hidden)]
#[macro_export]
macro_rules! ordered_close_route{($s:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr;$($field:ident),*)=>{{
let g=$effect.grant();
let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};
if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}
if !$s.child.terminal_is_empty(){let mut nested=$effect.nested(g);return if $recover{$s.child.recover_close_into($body,&mut nested,$reply)}else{$s.child.close_into($body,&mut nested,$reply)}}
if !$s.retirement.is_empty(){return if $recover{$s.retirement.recover_step_into($body,$effect,$reply)}else{$s.retirement.step_into($body,$effect,$reply)}}
if $recover{return Err(invalid())}
$(if $s.$field.is_some(){return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.$field,$effect),$reply)})*
if $recover{RetainedCloneBinding::recover_close_into(&mut $s.binding,$body,$effect,$reply)}else{RetainedCloneBinding::close_into(&mut $s.binding,$body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! scalar_close_route{($s:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
let g=$effect.grant();
let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};
if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}

if !$s.retirement.is_empty(){return if $recover{$s.retirement.recover_step_into($body,$effect,$reply)}else{$s.retirement.step_into($body,$effect,$reply)}}
if $s.output.is_some(){if $recover{return Err(invalid())}return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.output,$effect),$reply)}
if $recover{RetainedCloneBinding::recover_close_into(&mut $s.binding,$body,$effect,$reply)}else{RetainedCloneBinding::close_into(&mut $s.binding,$body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! option_close_route{($s:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
let g=$effect.grant();
let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};
if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}

if let Some(query)=$s.query.as_mut(){
if !query.close_started(){if $recover{return Err(invalid())}let turn=query.begin_close_paid($effect);return Ok($reply.complete(turn.result.map_err(Into::into)))}
if !query.terminal_is_empty(){let mut nested=$effect.reborrow(child_grant(g,0),$effect.maximum_work_units())?;return if $recover{query.recover_close_into($body,&mut nested,$reply.progress())}else{query.close_into($body,&mut nested,$reply.progress())}}
if $recover||$effect.remaining_work_units()?==0{return if $recover{Err(invalid())}else{$crate::native_close_denied($reply)}}
if !$effect.performed(1)?{return Ok($reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"optional query header exceeded its work authority").into())))}
*$s.query=None;return Ok($reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
if !$s.child.terminal_is_empty(){let mut nested=$effect.reborrow(g,$effect.maximum_work_units())?;if !$s.child.is_closing(){return if $recover{$s.child.recover_begin_close_into(&mut nested,$reply)}else{$s.child.begin_close_into(&mut nested,$reply)}}return if $recover{$s.child.recover_close_into($body,&mut nested,$reply)}else{$s.child.close_into($body,&mut nested,$reply)}}
if !$s.retirement.is_empty(){return if $recover{$s.retirement.recover_step_into($body,$effect,$reply)}else{$s.retirement.step_into($body,$effect,$reply)}}
if $recover{return Err(invalid())}
if $s.query_output.is_some(){return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.query_output,$effect),$reply)}
if $s.staged.is_some(){return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.staged,$effect),$reply)}
RetainedCloneBinding::close_into(&mut $s.binding,$body,$effect,$reply)
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! option_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if let Some(query)=$s.query.as_mut(){if !query.terminal_is_empty()&&query.requires_original_control(){let g=$effect.grant();if g.maximum_items==0||g.maximum_depth==0{return $crate::native_close_denied($reply)}let mut nested=$effect.reborrow(child_grant(g,0),$effect.maximum_work_units())?;return if $recover{query.recover_close_original_into($original,$body,&mut nested,$reply)}else{query.close_original_into($original,$body,&mut nested,$reply)}}}
if !$s.child.terminal_is_empty()&&$s.child.requires_original_control(){return if $recover{$s.child.recover_close_original_into($original,$body,$effect,$reply)}else{$s.child.close_original_into($original,$body,$effect,$reply)}}
if !$s.retirement.is_empty()&&$s.retirement.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply)}else{$s.retirement.step_original_into($original,$body,$effect,$reply)}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! result_close_route{($s:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
let g=$effect.grant();
let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};
if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}

if !$s.ok.terminal_is_empty(){let mut nested=$effect.nested(child_grant(g,0));return if $recover{$s.ok.recover_close_into($body,&mut nested,$reply)}else{$s.ok.close_into($body,&mut nested,$reply)}}
if !$s.err.terminal_is_empty(){let mut nested=$effect.nested(child_grant(g,0));return if $recover{$s.err.recover_close_into($body,&mut nested,$reply)}else{$s.err.close_into($body,&mut nested,$reply)}}
if !$s.retirement.is_empty(){return if $recover{$s.retirement.recover_step_into($body,$effect,$reply)}else{$s.retirement.step_into($body,$effect,$reply)}}
if $recover{return Err(invalid())}
if $s.staged_ok.is_some(){return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.staged_ok,$effect),$reply)}
if $s.staged_err.is_some(){return $crate::native_close_begun($s.retirement.begin_granted(&mut $s.staged_err,$effect),$reply)}
let mut nested=$effect.nested(g);if $recover{$s.selector.recover_close_into($body,&mut nested,$reply)}else{$s.selector.close_into($body,&mut nested,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! result_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
let grant=$effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return $crate::native_close_denied($reply)}
if !$s.ok.terminal_is_empty()&&$s.ok.requires_original_control(){return if $recover{$s.ok.recover_close_original_into($original,$body,&mut $effect.nested(child_grant(grant,0)),$reply)}else{$s.ok.close_original_into($original,$body,&mut $effect.nested(child_grant(grant,0)),$reply)}}
if !$s.err.terminal_is_empty()&&$s.err.requires_original_control(){return if $recover{$s.err.recover_close_original_into($original,$body,&mut $effect.nested(child_grant(grant,0)),$reply)}else{$s.err.close_original_into($original,$body,&mut $effect.nested(child_grant(grant,0)),$reply)}}
if !$s.retirement.is_empty()&&$s.retirement.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply)}else{$s.retirement.step_original_into($original,$body,$effect,$reply)}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! native_mapped_close_route{($s:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if $s.requires_original_control(){return Ok($reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped child requires its original close port").into())))}
if $s.terminal_is_empty(){return Ok($reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
let g=$effect.grant();
let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};
if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}

if !$s.inner.terminal_is_empty(){let mut nested=$effect.nested(child(g));return if $recover{$s.inner.recover_close_into($body,&mut nested,$reply.progress())}else{$s.inner.close_into($body,&mut nested,$reply.progress())}}
if !$s.retirement.is_empty(){return if $recover{$s.retirement.recover_step_into($body,$effect,$reply.progress())}else{$s.retirement.step_into($body,$effect,$reply.progress())}}
if $s.pending.is_none()&&$s.output.is_none()&&$s.binding.is_some(){return if $recover{crate::retained_clone::RetainedCloneBinding::recover_close_into(&mut $s.binding,$body,$effect,$reply)}else{crate::retained_clone::RetainedCloneBinding::close_into(&mut $s.binding,$body,$effect,$reply)}}
if $recover{return Err(invalid())}
if $s.pending.is_some(){$crate::native_close_begun($s.retirement.begin_granted(&mut $s.pending,$effect),$reply)}else{$crate::native_close_begun($s.retirement.begin_granted(&mut $s.output,$effect),$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! native_mapped_original_close_route{($s:ident,$original:ident,$body:ident,$effect:ident,$reply:ident,$recover:expr)=>{{
if $s.requires_original_birth(){return Ok($reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into())))}
let g=$effect.grant();
if !$s.inner.terminal_is_empty()&&$s.inner.requires_original_control(){let d=match $s.close_demands($body){Ok(d)=>d,Err(error)=>return Ok($reply.complete(Err(error.into())))};if !retained_clone_grant_admits(g,d){return $crate::native_close_denied($reply)}let mut nested=$effect.nested(child(g));return if $recover{$s.inner.recover_close_original_into($original,$body,&mut nested,$reply.progress())}else{$s.inner.close_original_into($original,$body,&mut nested,$reply.progress())}}
if $s.inner.terminal_is_empty()&&$s.retirement.requires_original_control(){return if $recover{$s.retirement.recover_step_original_into($original,$body,$effect,$reply.progress())}else{$s.retirement.step_original_into($original,$body,$effect,$reply.progress())}}
if $recover{$s.recover_close_step($body,$effect,$reply)}else{$s.close_step($body,$effect,$reply)}
}};}
#[doc(hidden)]
#[macro_export]
macro_rules! native_receive_zero_work{($work:expr)=>{if $work==0{return $crate::RetainedWorkOutcome{result:Ok($crate::retained_clone::RetainedCloneStep::Progress(Default::default())),work:0}}};}

#[doc(hidden)]
#[macro_export]
macro_rules! scalar_close_engine{()=>{ fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 $crate::scalar_close_work_quote!();
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::scalar_close_route!(self,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::scalar_close_route!(self,body,effect,reply,true)}
 fn close_original_step<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::scalar_close_route!(self,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::scalar_close_route!(self,body,effect,reply,true)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! option_close_engine{()=>{
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(query)=self.query.as_ref(){if !query.close_started(){return Ok(query.begin_close_demand())}if !query.terminal_is_empty(){return nested(query.close_demand(body)?,0)}return Ok(fixed(std::mem::size_of::<Option<NativeScalarQueryFrame<I>>>()))}if !self.child.terminal_is_empty(){return if self.child.is_closing(){self.child.close_demands(body)}else{self.child.begin_close_demand()}}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.query_output.is_some(){return self.retirement.next_owner_demands::<Result<NativeSourceScalar,ValueError>>(true,&None,body)}if self.staged.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if let Some(query)=self.query.as_ref(){if !query.close_started(){return Ok(query.begin_close_work_demand())}if !query.terminal_is_empty(){return query.close_work_demand(body)}return Ok(1)}if !self.child.terminal_is_empty(){return if self.child.is_closing(){self.child.close_work_demand(body)}else{self.child.begin_close_work_demand()}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.query_output.is_some(){return self.retirement.next_owner_work::<Result<NativeSourceScalar,ValueError>>(true,&None,body)}if self.staged.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::option_close_route!(self,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::option_close_route!(self,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{if let Some(query)=self.query.as_ref(){return !query.terminal_is_empty()&&query.requires_original_birth()}if !self.child.terminal_is_empty(){self.child.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if let Some(query)=self.query.as_mut(){if !query.terminal_is_empty(){if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}return query.close_original_birth_step(original,child_grant(grant,0))}}if !self.child.terminal_is_empty(){self.child.close_original_birth_step(original,grant)}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if let Some(query)=self.query.as_ref(){return !query.terminal_is_empty()&&query.requires_original_control()}if !self.child.terminal_is_empty(){self.child.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::option_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::option_original_close_route!(self,original,body,effect,reply,true)}};}

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
  if self.phase==3&&self.staged.is_none(){let child=child_grant(g,0);let mut loan=effect.nested(child);return match self.child.take(&mut self.staged,&mut loan){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
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
  if selected.index==0&&self.staged_ok.is_none(){let mut child=effect.nested(child_grant(g,0));return match self.ok.take(&mut self.staged_ok,&mut child){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
  if selected.index!=0&&self.staged_err.is_none(){let mut child=effect.nested(child_grant(g,0));return match self.err.take(&mut self.staged_err,&mut child){crate::RetainedWorkPoll::Pending{work}=>crate::RetainedWorkPoll::Pending{work},crate::RetainedWorkPoll::Complete(turn)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original publication work exceeded its authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=if selected.index==0{match self.staged_ok.take(){Some(value)=>Ok(value),None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})}}else{match self.staged_err.take(){Some(value)=>Err(value),None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:1})}};*output=Some(value);self.taken=true;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1})
 })();$crate::native_record_pure_take(turn,reply)}
};}

#[doc(hidden)]
#[macro_export]
macro_rules! result_close_engine{()=>{ fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.ok.terminal_is_empty(){return nested(self.ok.close_demands(body)?,0)}if !self.err.terminal_is_empty(){return nested(self.err.close_demands(body)?,0)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.staged_ok.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.staged_err.is_some(){return self.retirement.next_owner_demands::<E>(true,&None,body)}self.selector.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.ok.terminal_is_empty(){return self.ok.close_work_demand(body)}if !self.err.terminal_is_empty(){return self.err.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.staged_ok.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.staged_err.is_some(){return self.retirement.next_owner_work::<E>(true,&None,body)}self.selector.close_work_demand(body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::result_close_route!(self,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::result_close_route!(self,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{if !self.ok.terminal_is_empty(){self.ok.requires_original_birth()}else if !self.err.terminal_is_empty(){self.err.requires_original_birth()}else{self.retirement.requires_original_birth()}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.ok.terminal_is_empty(){self.ok.close_original_birth_step(original,child_grant(grant,0))}else if !self.err.terminal_is_empty(){self.err.close_original_birth_step(original,child_grant(grant,0))}else{self.retirement.step_original_birth(original,grant)}}
 fn requires_original_control(&self)->bool{if !self.ok.terminal_is_empty(){self.ok.requires_original_control()}else if !self.err.terminal_is_empty(){self.err.requires_original_control()}else{self.retirement.requires_original_control()}}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::result_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::result_original_close_route!(self,original,body,effect,reply,true)}
};}

#[path="🎛️slot.rs"]mod slot;
pub use slot::{NativeSlotOperation,NativeSlotProducer,NativeSlotContext};
#[path="🎛️local.rs"]mod local;
pub use local::{NativeLocalOperation,NativeLocalProducer,NativeLocalAuthority,native_receive_local};
/// 🧾️ Runs one grant-priced child step under the original effect and records the work it reports exactly once.
pub fn native_grant_step(effect:&mut crate::RetainedAllocationEffect<'_>,step:impl FnOnce(RetainedCloneGrant,u64)->RetainedWorkOutcome)->crate::RetainedWorkPoll<RetainedCloneStep,ValueError>{
 let turn=step(effect.grant(),effect.maximum_work_units());
 let recorded=u64::try_from(turn.work.saturating_sub(effect.performed_work())).map_err(|_|ValueError::invariant("native grant step work overflow")).and_then(|owed|effect.performed(owed));
 let result=match recorded{Ok(true)=>turn.result,Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native grant step exceeded its original work")),Err(error)=>Err(error)};
 crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:turn.work})
}
/// 🪢️ Drives one authored ordered-owner effect through a nested child under its resident local authority.
pub fn native_poll_local<E:NativeReceivingChildError+Send+'static>(child:&mut NativeReceivingChild<E>,control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,grant:retained_clone::RetainedCloneGrant,work:u64,maximum_work_units:u64,authority:&mut NativeLocalAuthority<E>,operation:&mut dyn FnMut(&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll<RetainedCloneStep,E>)->crate::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
 child.poll(control,demand,grant,work,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeLocalOperation::Poll(operation),effect,action))
}

#[doc(hidden)]
#[macro_export]
macro_rules! native_mapped_receiver_engine {()=>{
impl<I:NativeValueSource+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>NativeReceiver<I,U>for NativeMappedReceiver<I,T,U>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}input.check_binding(&self.binding)?;if self.binding.is_none(){return Ok(fixed(input.binding_copy_bytes()))}if self.pending.is_some(){return Ok(fixed(std::mem::size_of::<T>().saturating_add(std::mem::size_of::<U>())))}if self.inner.ready(){return nested(self.inner.take_demand()?)}nested(self.inner.demands(input,body,maximum_work_units)?)}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<$crate::NativeAdvanceSelection,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}input.check_binding(&self.binding)?;if self.binding.is_none(){return Ok($crate::NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}if self.pending.is_some(){return Ok($crate::NativeAdvanceSelection::Local{work:1})}if self.inner.ready(){return Ok($crate::NativeAdvanceSelection::Local{work:self.inner.take_work_demand()?})}Ok($crate::NativeAdvanceSelection::Nested)}
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,$crate::NativeReceivingFailure>{if self.closing||self.output.is_some()||self.binding.is_none()||self.pending.is_some()||self.inner.ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}self.inner.advance(input,control,witness,body,maximum_work_units,child(g)).map(|s|RetainedCloneStep::Progress(s.progress()))}
 fn advance_pending(&self)->bool{self.inner.advance_pending()}
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<$crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   if self.pending.is_some(){if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native mapped action exceeded its original work").into())}let value=self.pending.take().ok_or_else(||ValueError::invariant("pending absent"))?;*self.output=Some(U::from_native(value));return Ok(Some(RetainedCloneStep::Complete(receipt(d))))}
   if self.inner.ready(){return match self.inner.take(&mut *self.pending,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}}
   Err(invalid().into())})();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output!(U,invalid);
 $crate::native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return nested(self.inner.close_demands(body)?)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<U>(true,&None,body)}if self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.inner.terminal_is_empty(){return self.inner.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<U>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::native_mapped_close_route!(self,body,effect,reply,false)}
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::native_mapped_close_route!(self,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if !self.inner.terminal_is_empty()&&self.inner.requires_original_birth(){return self.inner.close_original_birth_step(original,child(grant))}if self.inner.terminal_is_empty()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"native mapped owner has no original birth to settle").into()),work:0})}
 fn requires_original_control(&self)->bool{if !self.inner.terminal_is_empty(){return self.inner.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::native_mapped_original_close_route!(self,original,body,effect,reply,false)}
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:$crate::OriginalEffectReply<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>)->Result<$crate::OriginalEffectReplyComplete<'reply,$crate::RetainedCloneStep,$crate::OriginalRetirementFailure>,$crate::ValueError>{$crate::native_mapped_original_close_route!(self,original,body,effect,reply,true)}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
}
};}

#[path="../../♻️backing/🦀️.rs"]mod backing;
pub use backing::NativeReceiverBackingClose;

