//! 🧭️ The actual native receiving control driver expands at its defining control boundary.
#[macro_export]
macro_rules! native_receiving_control_driver {()=>{
impl NativeDecodeControl<'_>{
 /// 🧵️ Drives the same resident producer through original work, allocation and construction replies.
 pub fn receive_effect<E:NativeReceivingEffectError>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<E>,operation:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<NativeReceivingEffectPoll<E>,ValueError>{
  use NativeReceivingEffectPhase as Phase;
  if frame.phase()==Phase::Empty{frame.quote(self,demand)?}else if !frame.matches_demand(demand){return Err(ValueError::invariant("original receiving demand changed"))}
  if frame.phase()==Phase::GrantStarted&&!frame.recover_grant(self)?{return Ok(NativeReceivingEffectPoll::Pending{work:frame.claim_child_work()})}
  if frame.phase()==Phase::Granted{match frame.prepare_work(self,maximum_work)?{None=>return Ok(NativeReceivingEffectPoll::Pending{work:0}),Some(false)=>frame.cancel_unstarted(self)?,Some(true)=>{}}}
  if frame.phase()==Phase::EffectStarted&&frame.child_work().is_none(){if let Some(progress)=frame.allocation_progress(){if matches!(progress.phase(),super::super::allocation::NativeDecodeAllocationPhase::QuoteStarted|super::super::allocation::NativeDecodeAllocationPhase::QuoteRefused){if !frame.recover_allocation_callback(self)?{return Ok(NativeReceivingEffectPoll::Pending{work:0})}}}}
  let fresh=frame.phase()==Phase::Granted;
  let resumed=frame.phase()==Phase::EffectStarted&&frame.child_work().is_none()&&frame.allocation_progress().is_some_and(|progress|!progress.operation_started()&&matches!(progress.phase(),super::super::allocation::NativeDecodeAllocationPhase::Absent|super::super::allocation::NativeDecodeAllocationPhase::QuoteAccepted));
  if fresh||resumed{if resumed{frame.resume_effect_controlled(self,operation)?}else if demand.capacity_bytes==0{frame.effect_unallocated_controlled(self,operation)?}else{frame.effect_controlled(self,operation)?}}
  self.finish_effect_poll(frame)
 }
 /// 🏭️ Lends one original effect to an actual producer whose reply remains in this frame.
 pub fn receive_original_effect<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>)->Result<bool,ValueError>)->Result<NativeReceivingEffectPoll<P::Failure>,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{
  match self.drive_original_effect_with(demand,maximum_work,frame,authority,invoke)?{NativeReceivingEffectState::Pending{work}=>Ok(NativeReceivingEffectPoll::Pending{work}),NativeReceivingEffectState::Ready{work}=>{let mut turn=frame.take(self)?;turn.work=turn.work.checked_add(work).expect("original native completion work is already bounded by its recorded total");Ok(NativeReceivingEffectPoll::Complete(turn))}}
 }
 /// 🔐️ Drives the actual producer while its complete typed outcome stays in the original frame.
 pub fn receive_original_effect_retained<'turn,'source,P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'turn,'source>)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError,'source:'turn{
  self.drive_original_effect_with(demand,maximum_work,frame,authority,|_,effect,authority,action|authority.drive(context,effect,action))
 }
 /// 🧵️ Lends the same native control to a source-bound producer while its result remains resident.
 pub fn receive_original_effect_retained_with<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>)->Result<bool,ValueError>)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{self.drive_original_effect_with(demand,maximum_work,frame,authority,invoke)}
 fn drive_original_effect_with<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>)->Result<bool,ValueError>)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{self.drive_original_effect_bound_with(demand,maximum_work,frame,authority,|control,effect,authority,action,_|invoke(control,effect,authority,action),false,false)}
 /// 🧾️ Keeps the actual accepted action and finished Work proof for paid completion publication.
 pub fn receive_original_effect_preserving_receipt_with<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding)->Result<bool,ValueError>)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{self.drive_original_effect_bound_with(demand,maximum_work,frame,authority,invoke,true,false)}
 /// 🧬️ Preserves the same finished Work proof for a later separately funded success acknowledgement.
 pub fn receive_original_effect_retaining_work_with<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding)->Result<bool,ValueError>)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{self.drive_original_effect_bound_with(demand,maximum_work,frame,authority,invoke,false,true)}
 fn drive_original_effect_bound_with<P>(&mut self,demand:RetirementDemand,maximum_work:u64,frame:&mut NativeReceivingEffect<P::Failure>,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeDecodeControl<'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<RetainedCloneStep,P::Failure>>,&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding)->Result<bool,ValueError>,retain_receipt:bool,retain_work:bool)->Result<NativeReceivingEffectState,ValueError>where P:OriginalEffectProducer<Output=RetainedCloneStep>,P::Failure:NativeReceivingEffectError{
  frame.bind_original(self)?;use NativeReceivingEffectPhase as Phase;
  if !frame.prepare_original(self,demand,maximum_work)?{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}
  if frame.phase()==Phase::EffectStarted&&frame.child_work().is_none(){
   if frame.allocation_progress().is_some_and(|state|matches!(state.phase(),super::super::allocation::NativeDecodeAllocationPhase::QuoteStarted|super::super::allocation::NativeDecodeAllocationPhase::QuoteRefused)){if !frame.recover_allocation_callback(self)?{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}}
   if frame.allocation_progress().is_some_and(|state|state.phase()==super::super::allocation::NativeDecodeAllocationPhase::QuoteRefused){frame.finish_allocation(self)?}else if !frame.drive_original_producer_bound(self,authority,invoke)?{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}
  }
  self.finish_effect_retained_mode(frame,retain_receipt,retain_work)
 }
 fn finish_effect_poll<E:NativeReceivingEffectError>(&mut self,frame:&mut NativeReceivingEffect<E>)->Result<NativeReceivingEffectPoll<E>,ValueError>{match self.finish_effect_retained(frame)?{NativeReceivingEffectState::Pending{work}=>Ok(NativeReceivingEffectPoll::Pending{work}),NativeReceivingEffectState::Ready{work}=>{let mut turn=frame.take(self)?;turn.work=turn.work.checked_add(work).expect("original native completion work is already bounded by its recorded total");Ok(NativeReceivingEffectPoll::Complete(turn))}}}
 fn finish_effect_retained<E:NativeReceivingEffectError>(&mut self,frame:&mut NativeReceivingEffect<E>)->Result<NativeReceivingEffectState,ValueError>{self.finish_effect_retained_mode(frame,false,false)}
 fn finish_effect_retained_mode<E:NativeReceivingEffectError>(&mut self,frame:&mut NativeReceivingEffect<E>,retain_receipt:bool,retain_work:bool)->Result<NativeReceivingEffectState,ValueError>{
  use NativeReceivingEffectPhase as Phase;
  if frame.child_work().is_some(){if !frame.work_is_settled(){if !(if frame.work_commit_is_pending(){frame.retry_original_work(self)?}else{frame.settle_original_work(self)?}){return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}}}
  if frame.phase()==Phase::EffectStarted{
   if let Some(progress)=frame.allocation_progress(){
    if matches!(progress.phase(),super::super::allocation::NativeDecodeAllocationPhase::QuoteStarted|super::super::allocation::NativeDecodeAllocationPhase::SettlementStarted|super::super::allocation::NativeDecodeAllocationPhase::QuoteRefused){if !frame.recover_allocation_callback(self)?{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}}
    if frame.child_work().is_none()&&frame.allocation_progress().is_some_and(|progress|!progress.operation_started()&&progress.phase()==super::super::allocation::NativeDecodeAllocationPhase::QuoteAccepted){return Ok(NativeReceivingEffectState::Pending{work:0})}
    frame.finish_allocation(self)?;if frame.phase()==Phase::EffectStarted{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}
   }else if frame.child_work().is_some(){frame.finish_unallocated(self)?}else{return Ok(NativeReceivingEffectState::Pending{work:0})}
  }
  if frame.phase()==Phase::EffectFinished{if frame.child_work().is_none(){frame.cancel_unperformed_work(self)?}frame.receipt(self)?}
  if frame.phase()==Phase::ReceiptStarted&&!frame.recover_receipt(self)?{return Ok(NativeReceivingEffectState::Pending{work:frame.claim_child_work()})}
  if retain_receipt{if frame.finished_work_proof().is_none(){frame.finish_accepted_work_for_payout(self)?}}else if retain_work&&frame.child_work().is_some()&&frame.has_accepted_receipt(){frame.prepare_transfer_retaining_work(self)?}else{frame.prepare_transfer(self)?}Ok(NativeReceivingEffectState::Ready{work:frame.claim_child_work()})
 }
}
}}
