//! 🧬️ The installed receiver retains the exact replacement payload until every original typed projection returns.
use crate::{ArtifactOperationWire,ArtifactOriginalOperationReceiving};
use semio_os_spr::InputReplacement;
use semio_framework_pack_json::{retained_alias,retained_alias_demand};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply,RetainedCloneSourceAdmission};
use semio_framework_value_native_owned::{NativeAllocationIdentity,native_decoding::{NativeDecodeControl,NativeReceivingFailure,OriginalReceivingPrimary}};
use semio_os_store_one_item_publication::native_source::{pay,polled};
use std::{mem::size_of,sync::Arc};
/// 🧳️ The original input row, native decoder ledger and retirement recipient remain bound through cancellation.
pub struct OriginalReplacementReceiving<M:Sync>{admission:Option<RetainedCloneSourceAdmission<Vec<u8>,Vec<u8>>>,source:Option<RetainedCloneSource<Vec<u8>>>,wire:Option<RetainedOwnedProjection<Vec<u8>>>,receiver:Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,input:Option<RetainedOwnedProjection<M>>,binding:Option<(NativeAllocationIdentity,u64,usize)>,closing:bool,pending_work:Option<u128>,returned:Option<Vec<u8>>,take_reply:Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,take_pending:bool}
impl<M:Sync> Default for OriginalReplacementReceiving<M>{fn default()->Self{Self{admission:None,source:None,wire:None,receiver:None,input:None,binding:None,closing:false,pending_work:None,returned:None,take_reply:None,take_pending:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"replacement receiving requires its original input row, decoder and retirement recipient")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)}
fn primary(failure:OriginalReceivingPrimary)->ValueError{let progress=failure.progress;failure.primary.or(failure.quote_refusal).or(failure.allocation_refusal).unwrap_or_else(invalid).with_retained_progress(progress)}
fn refused(failure:NativeReceivingFailure)->ValueError{let progress=failure.retained_progress();match failure{NativeReceivingFailure::Primary(failure)=>primary(failure),NativeReceivingFailure::ValueEffect(effect)=>effect.child.or(effect.producer_refusal).or(effect.work_refusal).unwrap_or_else(||primary(effect.receiving)).with_retained_progress(progress),NativeReceivingFailure::RetirementEffect(effect)=>effect.child.map(OriginalRetirementFailure::into_value_error).or(effect.producer_refusal).or(effect.work_refusal).unwrap_or_else(||primary(effect.receiving)).with_retained_progress(progress),NativeReceivingFailure::ReceivingEffect(effect)=>effect.child.map(primary).or(effect.producer_refusal).or(effect.work_refusal).unwrap_or_else(||primary(effect.receiving)).with_retained_progress(progress)}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<M:Sync+'static> OriginalReplacementReceiving<M>{
 fn validate(&self,row:&InputReplacement,ledger:NativeAllocationIdentity,recipient:Option<u64>)->Result<(NativeAllocationIdentity,u64,usize),ValueError>{let identity=(ledger,recipient.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"replacement receiving requires the original native retirement recipient"))?,row as*const _ as usize);if self.binding.is_some_and(|original|original!=identity){return Err(invalid())}Ok(identity)}
 /// 🎟️ Reports actual source custody without exposing or fabricating its projection authority.
 pub fn retains_source(&self)->bool{self.binding.is_some()}
 /// 📬️ A ready projection still belongs to the original receiver until its funded transfer.
 pub fn ready(&self)->bool{self.input.is_some()}
 /// 👁️ Borrows the unchanged original input wire while its real receiving source retains custody.
 pub fn borrow_payload(&self)->Result<semio_framework_value::RetainedCloneRef<'_,Vec<u8>>,ValueError>{if self.closing{return Err(invalid())}self.source.as_ref().ok_or_else(invalid)?.try_borrow()}
 /// 📏️ Prices one actual source capture, wire alias, receiver birth or native receiving step.
 pub fn demands(&self,codec:&ArtifactOperationWire<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.source.is_none(){if let Some(admission)=self.admission.as_ref(){return match admission.phase(){"ready"=>Ok(header::<Option<RetainedCloneSource<Vec<u8>>>>()),"projectionStart"=>admission.owned_projection_demand(),_=>admission.next_demand()}}let mut d=header::<Self>();d.copy_bytes+=size_of::<Vec<u8>>();return Ok(d)}
  if self.wire.is_none()&&self.receiver.is_none(){return retained_alias_demand(self.source.as_ref().ok_or_else(invalid)?.try_borrow()?)}
  if self.receiver.is_none(){return Ok((codec.original_receiving.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"installed operation wire has no original receiving capability"))?.demands)())}
  let receiver=self.receiver.as_ref().unwrap();if receiver.ready(){return Ok(header::<Option<RetainedOwnedProjection<M>>>())}receiver.demands(body)
 }
 /// 🪡️ Every native birth uses the same decoder and independent grant inside one original effect; the source Vec is never copied or replaced.
 pub fn advance_native(&mut self,row:&mut InputReplacement,codec:&ArtifactOperationWire<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,maximum_work:u64)->Result<RetainedCloneProgress,ValueError>{
  let identity=self.validate(row,control.ledger_identity(),control.retirement_recipient_identity())?;if self.closing{return Err(invalid())}if self.ready(){return Ok(Default::default())}
  let d=self.demands(codec,grant.maximum_copy_bytes)?;if maximum_work==0||!permits(grant,d){return Ok(Default::default())}
  control.checkpoint()?;if d.capacity_bytes!=0{control.admit_turn_capacity(grant.maximum_capacity_bytes)?;}
  if self.source.is_some()&&self.wire.is_some()&&self.receiver.is_none(){return (codec.original_receiving.as_ref().ok_or_else(invalid)?.admit)(&mut self.wire,&mut self.receiver,factory,control,grant)}
  if let Some(receiver)=self.receiver.as_mut(){
   if receiver.ready(){let turn=receiver.take(grant);return match turn.result?{Some((input,p))=>{self.input=Some(input);Ok(p)},None=>Ok(Default::default())}}
   let poll=receiver.advance(control,grant.maximum_copy_bytes,maximum_work,grant);return self.settle_receive(poll);
  }
  let child=RetainedCloneGrant{maximum_items:1,..grant};
  let Some(result)=control.with_allocated_effect(d,maximum_work,|_,effect|{
   let result=self.advance_local(row,identity,d,child,effect);
   let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
   if d.capacity_bytes!=0&&progress.retained_capacity_bytes!=0&&effect.actual().is_none(){*effect.actual()=Some(progress.retained_capacity_bytes);}
   match result{Ok(progress) if effect.performed_work()==0&&progress!=RetainedCloneProgress::default()=>pay(effect,1).map(|()|progress),other=>other}
  })? else{return Ok(Default::default())};
  result
 }
/// ⏳️ A pending receive keeps its receiver and records the newly performed work; only a completed turn clears it.
 fn settle_receive(&mut self,poll:RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>)->Result<RetainedCloneProgress,ValueError>{
  match poll{RetainedWorkPoll::Pending{work}=>{self.pending_work=Some(self.pending_work.unwrap_or(0).saturating_add(work));Ok(Default::default())},RetainedWorkPoll::Complete(turn)=>{self.pending_work=None;turn.result.map(|step|step.progress()).map_err(refused)}}
 }
 /// ⏳️ Observes the cumulative work of receives that are still pending; it is never a debit, the committed work is.
 pub fn pending_receive_work(&self)->Option<u128>{self.pending_work}
 fn advance_local(&mut self,row:&mut InputReplacement,identity:(NativeAllocationIdentity,u64,usize),d:RetirementDemand,child:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  if self.source.is_none(){
   let Some(admission)=self.admission.as_mut()else{let InputReplacement::Input{payload,..}=row else{return Err(invalid())};pay(effect,1)?;let bytes=std::mem::take(payload);self.admission=Some(RetainedCloneSource::<Vec<u8>>::admission_owned(bytes,()));self.binding=Some(identity);return Ok(progress(d))};
   match admission.phase(){"ready"=>{let step=admission.take_output(&mut self.source,&mut effect.nested(child)).result?;if admission.terminal_is_empty(){self.admission=None;}return Ok(step.progress())},"projectionStart"=>return Ok(admission.project_owned(&mut effect.nested(child)).result?.progress()),_=>return Ok(admission.advance(&mut effect.nested(child)).result?.progress())}
  }
  let Some((wire,p))=retained_alias(self.source.as_ref().ok_or_else(||ValueError::invariant("source absent"))?.try_borrow()?,&mut effect.nested(child))? else{return Ok(Default::default())};self.wire=Some(wire);Ok(p)
 }
 /// 🤝️ Transfers only the original typed projection after authenticating the same decoder and input recipient.
 pub fn take_input_native(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.binding.is_some_and(|(ledger,recipient,_)|ledger!=control.ledger_identity()||Some(recipient)!=control.retirement_recipient_identity()){return Err(invalid())}if output.is_some(){return Err(invalid())}let d=header::<Option<RetainedOwnedProjection<M>>>();if self.input.is_none()||!permits(grant,d){return Ok(Default::default())}*output=self.input.take();Ok(progress(d))}
 /// 🛑️ Stops receiving before original semantic producers return their projection aliases.
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close();}}
 /// 📏️ Includes actual receiver release, source handback and the original binding header.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{input.close_demand(body)}}
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(RetirementDemand{release_bytes:receiver.allocation_bytes(),..header::<Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>>()})}else{receiver.close_demands(body)}}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<Vec<u8>>>>())}else{source.next_take_demand(body)}}
  Ok(header::<Option<(u64,u64,usize)>>())
 }
 /// ↩️ Restores the same original Vec to the exact source row after every typed and wire alias has closed.
 pub fn close_native(&mut self,row:&mut InputReplacement,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   self.validate(row,control.ledger_identity(),control.retirement_recipient_identity())?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(admission)=self.admission.as_mut(){if admission.phase()=="ready"{let step=admission.take_output(&mut self.source,effect).result?;if admission.terminal_is_empty(){self.admission=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}if admission.phase()=="projectionStart"{return Ok(RetainedCloneStep::Progress(admission.project_owned(effect).result?.progress()))}let InputReplacement::Input{payload,..}=row else{return Err(invalid().into())};let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((bytes,()))=returned{*payload=bytes;}if admission.terminal_is_empty(){self.admission=None;}return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}
   if let Some(input)=self.input.as_mut(){if input.terminal_is_empty(){self.input=None;return Ok(RetainedCloneStep::Progress(progress(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(input.close_step(grant.maximum_copy_bytes,effect))?.progress()))}
   if let Some(receiver)=self.receiver.as_mut(){if !receiver.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(receiver.close_step(control,effect))?.progress()))}self.receiver=None;return Ok(RetainedCloneStep::Progress(progress(d)))}
   if let Some(wire)=self.wire.as_mut(){if wire.terminal_is_empty(){self.wire=None;return Ok(RetainedCloneStep::Progress(progress(d)))}control.charge(d.capacity_bytes)?;return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(wire.close_step(grant.maximum_copy_bytes,effect))?.progress()))}
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty()&&!self.take_pending{self.source=None;return Ok(RetainedCloneStep::Progress(progress(d)))}let InputReplacement::Input{payload,..}=row else{return Err(invalid().into())};if !payload.is_empty()||payload.capacity()!=0{return Err(invalid().into())}if !self.take_pending{control.charge(d.capacity_bytes)?;}if self.take_reply.is_none(){let reply=OriginalEffectReply::borrow(&mut self.take_reply)?;let done=if self.take_pending{source.recover_take_authority_into(&mut self.returned,grant.maximum_copy_bytes,effect,reply)?}else{source.take_authority_into(&mut self.returned,grant.maximum_copy_bytes,effect,reply)?};drop(done);}return match self.take_reply.take(){None=>{self.take_pending=true;Err(RetirementPollExit::Pending(0))},Some(result)=>{self.take_pending=false;let step=result?;if let Some(bytes)=self.returned.take(){*payload=bytes;}Ok(RetainedCloneStep::Progress(step.progress()))}}}
   self.binding=None;Ok(RetainedCloneStep::Complete(progress(d)))
  })();
  RetirementPollExit::settle(effect,result)
 }
 /// 🧾️ Settles one finished receiving scope; the scope's accepted physical spend is its receipt.
 fn settled(cx:&mut semio_framework_job::StepContext<'_>,outcome:Option<semio_framework_job::OriginalNativeReceivingOutcome>,stepped:Option<Result<RetainedCloneProgress,ValueError>>)->Result<RetainedCloneProgress,semio_framework_job::OriginalTurnFailure>{
  let outcome=outcome.ok_or_else(invalid)?;cx.settle_receiving_scope()?;let progress=outcome.retained_progress();
  if outcome.source_refusal.is_some()||outcome.call.is_err(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original replacement receiving scope refused").with_retained_progress(progress).into())}
  Ok(stepped.ok_or_else(invalid)??)
 }
 /// 🧵️ The initializer's actual Job boundary forwards its original receiving continuation and reconciles the same receipt.
 pub fn advance(&mut self,row:&mut InputReplacement,codec:&ArtifactOperationWire<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  let maximum_work=cx.fuel_remaining();let mut outcome=None;let mut stepped=None;let mut committed=0;
  if !cx.with_native_receiving(&mut outcome,|control,grant|{let before=control.actual_work();stepped=Some(self.advance_native(row,codec,factory,control,grant,maximum_work));committed=control.actual_work().saturating_sub(before);Ok(())})?{return Ok(false)}
  cx.consume_fuel(u64::try_from(committed).unwrap_or(u64::MAX));Self::settled(cx,outcome,stepped)?;if self.pending_work.is_some(){return Ok(false)}Ok(self.ready())
 }
 /// 🤝️ Transfers the already authenticated original typed projection under the same Job receiving continuation.
 pub fn take_input(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  let mut outcome=None;let mut stepped=None;
  if !cx.with_native_receiving(&mut outcome,|control,grant|{stepped=Some(self.take_input_native(output,control,grant));Ok(())})?{return Ok(false)}
  Self::settled(cx,outcome,stepped)?;Ok(output.is_some())
 }
 /// ♻️ The original cancellation scope supplies its actual decoder loan; no receiving callback is invented for closure.
 pub fn close(&mut self,row:&mut InputReplacement,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}
  cx.with_original_retirement(|cx|{
   let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
   let Some((poll,performed))=cx.with_original_effect(grant,|cx,effect|cx.with_native_retirement(|control|Ok::<_,ValueError>(self.close_native(row,control,effect))))? else{return Ok(false)};
   let result=polled(poll?);let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
   cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));cx.consume_retained(progress)?;result?;Ok(self.terminal_is_empty())
  })
 }
}
impl<M:Sync> OriginalReplacementReceiving<M>{
 /// 🫙️ Every original parser, payload, typed projection and decoder binding has settled.
 pub fn terminal_is_empty(&self)->bool{self.admission.is_none()&&self.source.is_none()&&self.wire.is_none()&&self.receiver.is_none()&&self.input.is_none()&&self.binding.is_none()&&self.returned.is_none()&&!self.take_pending}
}
impl<M:Sync> Drop for OriginalReplacementReceiving<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original replacement payload must return to its exact input row before Drop")}}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn a_receive_that_pends_first_keeps_its_state_and_then_completes(){
  let mut receiving=OriginalReplacementReceiving::<Vec<u8>>::default();
  let first=receiving.settle_receive(RetainedWorkPoll::Pending{work:3}).unwrap();
  assert_eq!(first,RetainedCloneProgress::default());assert_eq!(receiving.pending_receive_work(),Some(3));
  receiving.settle_receive(RetainedWorkPoll::Pending{work:2}).unwrap();assert_eq!(receiving.pending_receive_work(),Some(5));
  let done=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};
  let second=receiving.settle_receive(RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(done)),work:1})).unwrap();
  assert_eq!(second,done);assert_eq!(receiving.pending_receive_work(),None);
 }
}
