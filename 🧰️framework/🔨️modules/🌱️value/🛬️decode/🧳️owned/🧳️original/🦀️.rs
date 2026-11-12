//! 🧳️ Native receiving keeps its exact original source until the receiver and staged output settle.
use crate::{OriginalReceivingPrimary,native_decoding::{NativeReceivingEffect,NativeReceivingFailure}};
use crate::{RetainedWorkOutcome,RetainedWorkPoll};
use super::{NativeReceive,NativeReceiver};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::{RetainedCloneSource,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
#[path="🪞️projection/🦀️.rs"]mod projection;
#[path="🧭️staged-projection/🦀️.rs"]mod staged_projection;
pub use staged_projection::StagedProjection;
use projection::OriginalInput;
use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeSlotOperation,NativeSlotProducer};
use crate::native_decoding::OriginalEffectAuthority;
type Authority<I,T>=OriginalEffectAuthority<NativeSlotProducer<I,T,<T as NativeReceive<I>>::Receiver>>;
/// 🪪️ The registered codec supplies its genuine typed input source and required receiver without a cold conversion.
pub struct NativeReceivingOriginal<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync=I>{original:ManuallyDrop<Option<OriginalInput<I,A>>>,receiver:semio_framework_value_native_storage::NativeOwnedStorage<T::Receiver>,retired_header:Option<Box<T::Receiver>>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,action:semio_framework_value_native_receiver_protocol::NativeReceivingChild<OriginalReceivingPrimary>,authority:ManuallyDrop<Option<Authority<I,T>>>,backing:semio_framework_value_native_receiver_protocol::NativeReceiverBackingClose,close_reply:ManuallyDrop<Option<Result<RetainedCloneStep,crate::OriginalRetirementFailure>>>,close_route:Option<(crate::retained_clone::OriginalEffectInvocationIdentity,bool,u128)>,closing:bool}
fn cover_progress(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:a.copied_items.max(b.copied_items),copied_bytes:a.copied_bytes.max(b.copied_bytes),retained_capacity_bytes:a.retained_capacity_bytes.max(b.retained_capacity_bytes),released_bytes:a.released_bytes.max(b.released_bytes)}}
fn add_progress(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:a.copied_items.saturating_add(b.copied_items),copied_bytes:a.copied_bytes.saturating_add(b.copied_bytes),retained_capacity_bytes:a.retained_capacity_bytes.saturating_add(b.retained_capacity_bytes),released_bytes:a.released_bytes.saturating_add(b.released_bytes)}}
fn original_seed<I:Sync+'static,T:NativeReceive<I>>(_:bool)->Result<T::Receiver,ValueError>{Ok(T::receiver())}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native original receiving is not in the required funded phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}
fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> NativeReceivingOriginal<I,T,A>{
 pub fn new(original:RetainedCloneSource<I,A>)->Self{Self{original:ManuallyDrop::new(Some(OriginalInput::Source(original))),receiver:semio_framework_value_native_storage::NativeOwnedStorage::empty(),retired_header:None,output:ManuallyDrop::new(None),retirement:Default::default(),action:Default::default(),authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeSlotProducer::new()))),backing:semio_framework_value_native_receiver_protocol::NativeReceiverBackingClose::vacant(),close_reply:ManuallyDrop::new(None),close_route:None,closing:false}}
 pub fn borrow_input(&self)->Result<RetainedCloneRef<'_,I>,ValueError>{if self.closing{return Err(invalid())}self.original.as_ref().ok_or_else(invalid)?.try_borrow()}
 pub fn demands(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}if let Some(d)=self.action.demand{return Ok(d)}if !self.receiver.reserved(){return Ok(semio_framework_value_native_storage::NativeOwnedStorage::<T::Receiver>::reserve_demand())}if !self.receiver.initialized(){return Ok(semio_framework_value_native_storage::NativeOwnedStorage::<T::Receiver>::seed_demand())}let receiver=self.receiver.get().ok_or_else(||ValueError::invariant("get target absent"))?;nested(if receiver.output_ready(){receiver.take_demand()?}else{receiver.demands(self.borrow_input()?,body,maximum_work_units)?},0)}
 /// ⛽️ Quotes the exact work of the next resident action; a nested child action is quoted by the caller's ceiling because the child owns its own selection.
 pub fn advance_work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.closing||self.output.is_some(){return Err(invalid())}if let Some(work)=self.action.work{return Ok(work)}if !self.receiver.reserved(){return Ok(self.receiver.reserve_work_demand())}if !self.receiver.initialized(){return Ok(self.receiver.seed_work_demand())}let receiver=self.receiver.get().ok_or_else(||ValueError::invariant("get target absent"))?;if receiver.output_ready(){return receiver.take_work_demand()}match receiver.select_advance(self.borrow_input()?,body,maximum_work_units)?{NativeAdvanceSelection::Nested=>Ok(maximum_work_units),NativeAdvanceSelection::Local{work}=>Ok(work)}}
 /// 🔁️ Reports whether the resident action awaits original recovery.
 pub fn pending_recovery_required(&self)->bool{self.advance_pending()}
 /// ⏯️ Performs resident receiving actions while the turn grant and work still admit the next exact quote; yields only on a denied quote, a real Pending, a ready output or a pending yield.
 pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,witness:crate::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  use RetainedWorkPoll::{Complete,Pending};
  let(mut total,mut work,mut first)=(RetainedCloneProgress::default(),0u128,true);
  loop{
   let rem=RetainedCloneGrant{maximum_items:g.maximum_items.saturating_sub(total.copied_items),maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(total.copied_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(total.retained_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.saturating_sub(total.released_bytes),..g};
   let left=maximum_work_units.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX));
   if !first&&(left==0||rem.maximum_items==0||self.output_ready()||control.checkpoint().is_err()){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(total)),work})}
   match self.advance_once(control,witness,body,maximum_work_units,g,rem,left){
    Pending{work:pending}=>return if first{Pending{work:pending}}else{Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(total)),work:work+pending})},
    Complete(turn)=>{
     work+=turn.work;
     match turn.result{
      Err(mut failure)=>{let primary=match &mut failure{NativeReceivingFailure::Primary(primary)=>primary,NativeReceivingFailure::ValueEffect(error)=>&mut error.receiving,NativeReceivingFailure::RetirementEffect(error)=>&mut error.receiving,NativeReceivingFailure::ReceivingEffect(error)=>&mut error.receiving};primary.progress=add_progress(primary.progress,total);return Complete(RetainedWorkOutcome{result:Err(failure),work})},
      Ok(step)=>{let progress=step.progress();let idle=turn.work==0&&progress==RetainedCloneProgress::default();total=add_progress(total,progress);if let Ok(accepted)=control.construction_submitted(){total=cover_progress(total,accepted)}if idle{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(total)),work})}}
     }
    }
   }
   first=false;
  }
 }
 fn advance_once(&mut self,control:&mut NativeDecodeControl<'_>,witness:crate::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,rem:RetainedCloneGrant,left:u64)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  let done=|result:Result<RetainedCloneStep,NativeReceivingFailure>|->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:0})};
  if self.action.request.is_some_and(|request|request!=(body,maximum_work_units)){return done(Err(ValueError::invariant("original local receiver request changed").into()))}
  if left==0{return if self.action.demand.is_some(){RetainedWorkPoll::Pending{work:0}}else{done(Ok(RetainedCloneStep::Progress(Default::default())))}}
  let result:Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,NativeReceivingFailure>=(||{
   let d=self.demands(body,maximum_work_units)?;if !retained_clone_grant_admits(rem,d){return Ok(if self.action.demand.is_some(){RetainedWorkPoll::Pending{work:0}}else{done(Ok(RetainedCloneStep::Progress(Default::default())))})}
   if self.action.demand.is_none(){control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?}
   let input=self.original.as_ref().ok_or_else(invalid)?.try_borrow()?;let authority=self.authority.as_mut().ok_or_else(invalid)?;
   if !self.receiver.reserved()||self.action.tag==3{self.action.tag=3;let units=self.action.work.unwrap_or_else(||self.receiver.reserve_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Reserve(input,&mut self.receiver).with_witness(witness),effect,action)))}
   if !self.receiver.initialized()||self.action.tag==4{self.action.tag=4;let units=self.action.work.unwrap_or_else(||self.receiver.seed_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Seed(input,&mut self.receiver,false,original_seed::<I,T>,T::receiver_template(false)).with_witness(witness),effect,action)))}
   let receiver=self.receiver.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?;
   if receiver.output_ready()||self.action.tag==5{self.action.tag=5;let units=match self.action.work{Some(work)=>work,None=>receiver.take_work_demand()?};return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Publish(input,receiver,&mut self.output).with_witness(witness),effect,action)))}
   let selection=if self.action.tag==6{NativeAdvanceSelection::Local{work:self.action.work.ok_or_else(||ValueError::invariant("original local receiver action lost its work quote"))?}}else{receiver.select_advance(input,body,maximum_work_units)?};
   match selection{
    NativeAdvanceSelection::Nested=>Ok(receiver.advance_nested(input,control,witness,body,maximum_work_units,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}).map(|step|RetainedCloneStep::Progress(step.progress()))),
    NativeAdvanceSelection::Local{work}=>{if self.action.demand.is_none()&&work>left{return Ok(done(Ok(RetainedCloneStep::Progress(Default::default()))))}self.action.tag=6;Ok(self.action.poll_local(control,d,g,work,body,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Advance(input,receiver,body).with_witness(witness),effect,action)))}
   }
  })();
  match result{Ok(turn)=>turn,Err(error)=>done(Err(error))}
 }
 /// ⏳️ Reports an unsettled resident original action in the owner or its receiver; such an action must settle before close.
 pub fn advance_pending(&self)->bool{self.action.pending()||self.receiver.get().is_some_and(NativeReceiver::advance_pending)}
 pub fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_public_output!(T,invalid,output_ready);
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(pub closing);
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if !self.backing.terminal_is_empty()||self.receiver.reserved()||self.retired_header.is_some(){return self.backing.demands::<I,T,T::Receiver>(&self.receiver,&self.retired_header,body)}if !self.retirement.is_empty(){return nested(self.retirement.next_owner_demands::<T>(false,&None,body)?,0)}if self.output.is_some(){return nested(self.retirement.next_owner_demands::<T>(true,&None,body)?,0)}if self.authority.is_some(){return nested(self.retirement.next_owner_demands::<Authority<I,T>>(true,&None,body)?,0)}if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(fixed(0))}else{nested(original.next_close_demand(body)?,0)}}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if !self.backing.terminal_is_empty()||self.receiver.reserved()||self.retired_header.is_some(){return self.backing.work_demand::<I,T,T::Receiver>(&self.receiver,&self.retired_header,body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.authority.is_some(){return self.retirement.next_owner_work::<Authority<I,T>>(true,&None,body)}match self.original.as_ref(){Some(original)if !original.terminal_is_empty()=>original.next_close_work(body),Some(_)=>Ok(1),None=>Ok(0)}}
 fn close_inner<'r>(&mut self,mut original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  let fail=|error:ValueError,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>|Ok(reply.complete(Err(error.into())));
  if self.action.pending(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original receiver birth requires settlement"),reply)}
  if original.is_none()&&self.requires_original_control(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original receiver requires its original close loan"),reply)}
  if original.is_some()&&self.requires_original_birth(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect"),reply)}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  if !self.closing{return if recover{self.recover_begin_close_into(effect,reply)}else{self.begin_close_into(effect,reply)}}
  let g=effect.grant();let d=match self.close_demands(body){Ok(d)=>d,Err(error)=>return fail(error,reply)};if !retained_clone_grant_admits(g,d){return semio_framework_value_native_receiver_protocol::native_close_denied(reply)}
  let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};
  if !self.backing.terminal_is_empty()||self.receiver.reserved()||self.retired_header.is_some(){let reply=reply.progress();return match(original.as_deref_mut(),recover){(Some(original),false)=>self.backing.close_original_into::<I,T,T::Receiver>(&mut self.receiver,&mut self.retired_header,body,original,effect,reply),(Some(original),true)=>self.backing.recover_close_original_into::<I,T,T::Receiver>(&mut self.receiver,&mut self.retired_header,body,original,effect,reply),(None,false)=>self.backing.close_into::<I,T,T::Receiver>(&mut self.receiver,&mut self.retired_header,body,effect,reply),(None,true)=>self.backing.recover_close_into::<I,T,T::Receiver>(&mut self.receiver,&mut self.retired_header,body,effect,reply)}}
  if !self.retirement.is_empty(){let reply=reply.progress();let mut nested=effect.nested(child);let control=self.retirement.requires_original_control();return match(original.as_deref_mut().filter(|_|control),recover){(Some(original),false)=>self.retirement.step_original_into(original,body,&mut nested,reply),(Some(original),true)=>self.retirement.recover_step_original_into(original,body,&mut nested,reply),(None,false)=>self.retirement.step_into(body,&mut nested,reply),(None,true)=>self.retirement.recover_step_into(body,&mut nested,reply)}}
  if self.output.is_some(){if recover{return Err(invalid())}return semio_framework_value_native_receiver_protocol::native_close_begun(self.retirement.begin_granted(&mut self.output,&mut effect.nested(child)),reply)}
  if self.authority.is_some(){if recover{return Err(invalid())}return semio_framework_value_native_receiver_protocol::native_close_begun(self.retirement.begin_granted(&mut self.authority,&mut effect.nested(child)),reply)}
  if let Some(source)=self.original.as_mut(){if !source.terminal_is_empty(){return source.close_dispatch(original,body,&mut effect.nested(child),reply.progress(),recover)}if recover{return Err(invalid())}if !effect.performed(1)?{return fail(ValueError::literal(ValueRefusalKind::WorkLimit,"native original source release exceeded its work authority"),reply)}self.original.take();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(0)))))}
  Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))
 }
 fn close_borrowed<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.close_reply.is_some()&&self.close_route.is_none(){return Err(ValueError::invariant("native original receiver close reply lost its action"))}
  let invocation=effect.invocation().ok_or_else(||ValueError::invariant("native original receiver close lacks its original invocation"))?;let mode=if recover{crate::retained_clone::OriginalEffectCallbackMode::Recover}else{crate::retained_clone::OriginalEffectCallbackMode::Receive};if invocation.mode()!=mode{return Err(ValueError::invariant("native original receiver close changed invocation mode"))}
  let identity=invocation.identity();let is_original=original.is_some();
  let start=match self.close_route{Some((selected,was_original,start))if recover&&selected==identity&&was_original==is_original=>start,Some(_)=>return Err(ValueError::invariant("native original receiver close changed original action")),None if recover=>return Err(ValueError::invariant("native original receiver close has no pending action")),None=>{let start=effect.performed_work();self.close_route=Some((identity,is_original,start));start}};
  if self.close_reply.is_none(){let mut answer=None;{let inner=crate::retained_clone::OriginalEffectReply::borrow(&mut answer)?;drop(self.close_inner(original,body,effect,inner,recover)?);}*self.close_reply=answer;}
  if self.close_reply.is_none(){return Ok(reply.pending())}
  let work=effect.performed_work().checked_sub(start).ok_or_else(||ValueError::invariant("native original receiver close work regressed"))?;let result=self.close_reply.take().expect("original close reply selected");self.close_route=None;
  Ok(reply.complete(result.map(|step|match step{RetainedCloneStep::Complete(progress)if progress==RetainedCloneProgress::default()&&self.terminal_is_empty()=>RetirementStep::Complete(work),RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>RetirementStep::Progress(progress,work)})))
 }
 /// 👶️ Selects an unfinished birth before any general close effect is opened.
 pub fn requires_original_birth(&self)->bool{if let Some(receiver)=self.receiver.get(){if !receiver.terminal_is_empty(){return receiver.requires_original_birth()}}if self.receiver.reserved()||self.retired_header.is_some(){return false}self.retirement.requires_original_birth()}
 /// 🧷️ Resumes the same recipient’s pending physical birth without creating another marker.
 pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;if grant.maximum_items==0||grant.maximum_depth==0{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if let Some(receiver)=self.receiver.get_mut(){if !receiver.terminal_is_empty()&&receiver.requires_original_birth(){return receiver.close_original_birth_step(original,child)}}if !self.receiver.reserved()&&self.retired_header.is_none()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,child)}Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original receiver has no birth to settle").into()),work:0})}
 /// 🪪️ Reports whether the selected original receiver needs its own close loan.
 pub fn requires_original_control(&self)->bool{if self.action.pending(){return true}if let Some(original)=self.backing.original_route(){return original}if let Some(receiver)=self.receiver.get(){if !receiver.terminal_is_empty(){return receiver.requires_original_control()}}if self.receiver.reserved()||self.retired_header.is_some(){return false}self.retirement.requires_original_control()}
 pub fn terminal_is_empty(&self)->bool{self.receiver.terminal_is_empty()&&self.retired_header.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.action.terminal_is_empty()&&self.authority.is_none()&&self.original.is_none()&&self.backing.terminal_is_empty()&&self.close_reply.is_none()&&self.close_route.is_none()}
}
/// 🌱️ Inline seed starts the actual receiver close only after a funded retirement turn.
#[doc(hidden)]
pub struct NativeReceivingOriginalSeed<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync>{owner:ManuallyDrop<Option<NativeReceivingOriginal<I,T,A>>>}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> RetireOwned for NativeReceivingOriginal<I,T,A>{type RetirementCursor=NativeReceivingOriginalSeed<I,T,A>;fn retirement_seed(self)->Self::RetirementCursor{NativeReceivingOriginalSeed{owner:ManuallyDrop::new(Some(self))}}fn controlled_retirement_supported()->bool{true}}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> NativeReceivingOriginalSeed<I,T,A>{fn owner(&mut self)->Result<&mut NativeReceivingOriginal<I,T,A>,ValueError>{self.owner.as_mut().ok_or_else(||ValueError::invariant("retained original receiver"))}}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> RetirementCursor for NativeReceivingOriginalSeed<I,T,A>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.owner()?.close_borrowed(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.owner()?.close_borrowed(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.owner()?.close_borrowed(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.owner()?.close_borrowed(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.owner.as_ref().is_some_and(NativeReceivingOriginal::requires_original_birth)}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{let Some(owner)=self.owner.as_mut()else{return RetirementStep::Complete(0)};crate::retirement::erased_cursor_turn(owner.close_original_birth_step(original,grant))}
 fn requires_original_control(&self)->bool{self.owner.as_ref().is_some_and(NativeReceivingOriginal::requires_original_control)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.owner.as_ref(){None=>Ok(Default::default()),Some(owner)if owner.closing=>owner.close_demands(body),Some(owner)=>owner.begin_close_demand()}}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self.owner.as_ref(){None=>Ok(0),Some(owner)if owner.closing=>owner.close_work_demand(body),Some(owner)=>owner.begin_close_work_demand()}}
 fn terminal_is_empty(&self)->bool{self.owner.as_ref().is_none_or(NativeReceivingOriginal::terminal_is_empty)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> Drop for NativeReceivingOriginalSeed<I,T,A>{fn drop(&mut self){if !std::thread::panicking(){assert!(self.terminal_is_empty(),"original receiver seed requires funded closure");unsafe{ManuallyDrop::drop(&mut self.owner)}}}}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> RetirementCursor for NativeReceivingOriginal<I,T,A>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_borrowed(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_borrowed(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_borrowed(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_borrowed(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{Self::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{crate::retirement::erased_cursor_turn(Self::close_original_birth_step(self,original,grant))}
 fn requires_original_control(&self)->bool{Self::requires_original_control(self)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.close_work_demand(body)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync> Drop for NativeReceivingOriginal<I,T,A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native receiving keeps original custody until funded closure");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
