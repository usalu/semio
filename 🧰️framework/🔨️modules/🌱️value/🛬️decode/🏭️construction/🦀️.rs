//! 🏭️ Retained native construction requires original physical grants and retains born output before receipt acceptance.
use super::NativeDecodeControl;
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},shared_issuance::{SealedShared,SharedIssuer}};
#[path="🧾️result/🦀️.rs"]
mod result;
#[path="📦️effect/🦀️.rs"]mod effect;
pub use effect::{NativeReceivingConstructionBinding,NativeReceivingSuccessIdentity,NativeResidentSuccessAck,OriginalEffectAction,OriginalEffectAuthority,OriginalEffectCallbackMode,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,NativeReceivingEffectState,NativeReceivingEffectOutcome,NativeEffectCapturePlan,NativeReceivingEffect,NativeReceivingEffectPoll,NativeReceivingEffectError,NativeReceivingEffectFailure,NativeReceivingEffectPhase};
pub use result::{NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};

/// 🧭️ One actual held construction scope exposes no allocation or work issuance methods.
pub struct NativeConstructionScope<'scope,'decode>{original:&'scope mut NativeDecodeControl<'decode>,grant:RetainedCloneGrant,work:u64}
impl NativeConstructionScope<'_, '_>{
 pub fn checkpoint(&mut self)->Result<(),ValueError>{self.original.checkpoint()}
 pub fn remaining_work_units(&self)->Result<u64,ValueError>{self.original.remaining_prepared_work_units().map(|available|available.min(self.work))}
 pub fn retained_grant(&self)->RetainedCloneGrant{self.grant}
 /// 🪟️ Narrows the same held scope without preparing or settling another reservation.
 pub fn with_limits<T>(&mut self,grant:RetainedCloneGrant,work:u64,run:impl FnOnce(&mut NativeConstructionScope<'_, '_>)->T)->Result<T,ValueError>{let work=work.min(self.remaining_work_units()?);let g=self.grant;let grant=RetainedCloneGrant{maximum_items:g.maximum_items.min(grant.maximum_items),maximum_copy_bytes:g.maximum_copy_bytes.min(grant.maximum_copy_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.min(grant.maximum_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.min(grant.maximum_release_bytes),maximum_depth:g.maximum_depth.min(grant.maximum_depth)};Ok(run(&mut NativeConstructionScope{original:&mut *self.original,grant,work}))}
 pub fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.original.ledger_identity()}
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.original.retirement_recipient_identity()}
}
/// 🛟️ Borrows the actual typed callback owner for original receiving recovery.
pub enum NativeRecoveryLoan<'a>{Value(&'a mut NativeReceivingEffect<ValueError>),Retirement(&'a mut NativeReceivingEffect<crate::OriginalRetirementFailure>)}
/// 📦️ The original caller retains empty shared storage and every pending callback until funded publication or release.
pub struct NativeSharedAdmission<T:Send+Sync+'static>{effect:NativeReceivingEffect,sealed:std::mem::ManuallyDrop<Option<crate::shared_issuance::SealedSharedBirth<T>>>,issuer:Option<SharedIssuer<T>>,arc:std::mem::ManuallyDrop<Option<std::sync::Arc<std::mem::MaybeUninit<T>>>>}
impl<T:Send+Sync+'static> Default for NativeSharedAdmission<T>{fn default()->Self{Self{effect:Default::default(),sealed:std::mem::ManuallyDrop::new(None),issuer:None,arc:std::mem::ManuallyDrop::new(None)}}}
impl<T:Send+Sync+'static> NativeSharedAdmission<T>{
 /// 🎯️ Declares the actual retained shared-admission frame and its current typed phase.
 pub fn capture_plan(&self)->Result<Option<NativeEffectCapturePlan>,ValueError>{Ok(Some(self.effect.capture_plan()))}

 /// 🛟️ Loans this admission's actual pending effect for same-recipient recovery.
 pub fn has_pending_recovery(&self)->bool{self.effect.has_pending_recovery()}
 pub fn pending_recovery(&mut self)->Option<&mut NativeReceivingEffect>{self.effect.pending_recovery()}
 /// 🔁️ Completes a callback-resolved turn through its unchanged original work and construction loans.
 pub fn resume(&mut self,control:&mut NativeDecodeControl<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{control.finish_receiving(&mut self.effect)}
 pub fn receiving_effect(&mut self)->&mut NativeReceivingEffect{&mut self.effect}
 pub fn phase(&self)->NativeReceivingEffectPhase{self.effect.phase()}
 /// ▶️ Allows a fresh turn or the same grant after its interrupted work quote was resolved.
 pub fn can_advance(&self)->bool{matches!(self.effect.phase(),NativeReceivingEffectPhase::Empty|NativeReceivingEffectPhase::Granted)}
 pub fn has_birth(&self)->bool{self.sealed.is_some()||self.arc.is_some()}
 pub fn shared_demand(&self)->RetirementDemand{if self.sealed.is_some(){Self::publication_demand()}else{NativeOwnedDecodeControl::shared_birth_demand::<T>()}}
 pub fn arc_demand(&self)->RetirementDemand{if self.arc.is_some(){Self::publication_demand()}else{NativeOwnedDecodeControl::arc_birth_demand::<T>()}}
 /// ⛽️ Quotes the original sealed birth or publication without advancing it.
 pub fn shared_work_demand(&self)->Result<u64,ValueError>{if !self.can_advance()||self.arc.is_some(){return Err(ValueError::invariant("sealed admission requires its resolved original frame"))}Ok(if self.sealed.is_some(){crate::shared_issuance::SealedSharedBirth::<T>::publication_work_demand()}else{1+SealedShared::<T>::prepare_birth_work_demand()})}
 /// 🖼️ Quotes the original Arc allocation or publication.
 pub fn arc_work_demand(&self)->Result<u64,ValueError>{if !self.can_advance()||self.sealed.is_some(){return Err(ValueError::invariant("Arc admission requires its resolved original frame"))}Ok(1)}
 fn publication_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
 pub fn terminal_is_empty(&self)->bool{self.effect.phase()==NativeReceivingEffectPhase::Empty&&!self.has_birth()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{if self.effect.phase()!=NativeReceivingEffectPhase::Empty{return Err(ValueError::invariant("original shared allocation callback requires resolution"))}if let Some(birth)=self.sealed.as_ref(){if !birth.terminal_is_empty(){return Ok(birth.next_close_demand())}return Ok(RetirementDemand{copy_bytes:size_of::<Option<crate::shared_issuance::SealedSharedBirth<T>>>()+size_of::<Option<SharedIssuer<T>>>(),depth:1,..Default::default()})}if self.arc.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<std::sync::Arc<std::mem::MaybeUninit<T>>>>(),release_bytes:crate::retirement::arc_custody::shared_allocation_bytes::<T>(),depth:1,..Default::default()})}Ok(Default::default())}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let grant=effect.grant();let demand=match self.close_demand(){Ok(d)=>d,Err(error)=>return Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}if let Some(birth)=self.sealed.as_mut(){if !birth.terminal_is_empty(){let turn=birth.close_step(0,&mut effect.nested(grant));return Complete(crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}}if self.sealed.is_some()||self.arc.is_some(){if effect.maximum_work_units()==0{return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"shared admission close exceeded its work authority").into()),work:1}),Err(error)=>return Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:1})}if self.sealed.is_some(){self.sealed.take();self.issuer.take();}else{self.arc.take();}return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),work:1})}Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0})}

}
impl<T:Send+Sync+'static> Drop for NativeSharedAdmission<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original shared admission requires callback settlement and funded birth closure")}}
pub use semio_framework_value_control_contract::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionLoan,OriginalConstructionIssuerIdentity,OriginalConstructionAction,OriginalConstructionPhase,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete};
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_state:&'owner mut super::work::WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_state=self.control.work.state.take();}}
pub(super) fn run<T,E:From<ValueError>>(original:&mut NativeDecodeControl<'_>,recipient:&mut OriginalConstructionLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 if original.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction cannot substitute its original receiving authority")));}
 let mut construction=recipient.reborrow();
 let mut forwarded=match &mut original.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(port)=>Some(&mut **port)};
 let mut work_loan=original.work.recipient.as_mut().map(|loan|loan.reborrow());
 let control=NativeDecodeControl{work:super::work::Work{state:original.work.state.take(),recipient:work_loan.as_mut()},ledger:original.ledger.take(),scope_maximum:original.scope_maximum,construction:Some(&mut construction),receiving:original.receiving,maximum_bytes:original.maximum_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut *original.callback,source:original.source,retirement:original.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){None=>super::allocation::Binding::Local,Some(port)=>super::allocation::Binding::Forwarded(&mut **port)}};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,ledger:&mut original.ledger,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth,work_state:&mut original.work.state};operation(&mut scope.control)
}
/// 🎟️ Quotes one original construction action and either cancels it unspent or posts its actual progress, always settling the action.
fn quote_construction(loan:&mut OriginalConstructionLoan<'_>,work:&semio_framework_value_control_contract::OriginalWorkBorrow<'_>,demand:RetirementDemand,progress:Option<RetainedCloneProgress>)->Result<RetainedCloneGrant,ValueError>{
 let pending=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original construction reply is pending");
 let mut slot=None;loan.begin(demand,&mut slot,work)?;
 let outcome=(||{
  let action=slot.as_mut().ok_or_else(pending)?;loan.quote(action)?;
  let grant=loan.recover_grant(action)?.ok_or_else(pending)??;
  match progress{
   None=>loan.cancel(action,false,Default::default(),None)?,
   Some(progress)=>{if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original construction grant no longer admits the posted progress"))}loan.post(action,progress,(progress.retained_capacity_bytes!=0).then_some(progress.retained_capacity_bytes))?;loan.recover_accept(action)?.ok_or_else(pending)?;}
  }
  Ok(grant)
 })();
 if outcome.is_err(){if let Some(action)=slot.as_mut(){if matches!(action.phase(),OriginalConstructionPhase::Fresh|OriginalConstructionPhase::Granted){loan.cancel(action,false,Default::default(),None)?}}}
 if let Some(action)=slot.as_ref(){match action.phase(){OriginalConstructionPhase::GrantRefused=>loan.finish(&mut slot)?,OriginalConstructionPhase::Settled if action.accept_succeeded()==Some(true)=>loan.finish_accepted(&mut slot)?,OriginalConstructionPhase::Settled if action.recorded_accept_status().is_none()=>loan.finish(&mut slot)?,OriginalConstructionPhase::Settled if action.accept_succeeded()==Some(false)=>{let refusal=loan.finish_refused(&mut slot)?;return Err(refusal)},_=>{}}}
 outcome
}
impl NativeDecodeControl<'_>{
 /// 🔭️ Reads actual original construction submissions without issuing another grant.
 pub fn construction_submitted(&self)->Result<RetainedCloneProgress,ValueError>{self.construction.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?.submitted()}
 /// 🛬️ One original physical recipient funds and records a bounded typed construction transition.
 pub fn receive_construction_step(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,NativeConstructionStepFailure<ValueError>>{
  self.receive_construction_step_typed(demand,|control,grant|{let turn=operation(control,grant);let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};NativeConstructionStepResult{result:turn.result,progress,work:turn.work}}).into_step_outcome()
 }
 /// 🧬️ Typed output or primary failure remains resident even when its physical recipient refuses the same effects.
 pub fn receive_construction_step_typed<T,E:From<ValueError>>(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->NativeConstructionStepResult<T,E>)->NativeConstructionStepOutcome<T,E>{
  let admission=self.checkpoint().and_then(|()|self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction step requires its original recipient")).and_then(|loan|quote_construction(loan,&semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work),demand,None)));
  let grant=match admission{Ok(grant)=>grant,Err(error)=>return NativeConstructionStepOutcome{result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default(),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return NativeConstructionStepOutcome{result:None,receipt_refusal:None,progress:Default::default(),work:0}}
  let NativeConstructionStepResult{result,progress,work}=operation(self,grant);
  let receipt_refusal=(||{if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"constructed progress exceeds its original grant"))}let loan=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;let posted=RetirementDemand{copy_bytes:progress.copied_bytes,capacity_bytes:progress.retained_capacity_bytes,release_bytes:progress.released_bytes,depth:usize::from(grant.maximum_depth>0)};quote_construction(loan,&semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work),posted,Some(progress)).map(|_|())})().err().map(|error|error.with_retained_progress(progress));
  NativeConstructionStepOutcome{result:Some(result),receipt_refusal,progress,work}
 }

 /// 🧷️ Admits sealed field ownership only under this hop's explicitly supplied original construction recipient.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if input.is_none()||output.is_some()||owner.arc.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native sealed admission requires its original slots").into()),work:0}}
  let demand=owner.shared_demand();let born=owner.sealed.is_some();let frame=&mut owner.effect;let slot=&mut *owner.sealed;let saved=&mut owner.issuer;
   self.receive_allocation_effect(demand,frame,!born,if born{crate::shared_issuance::SealedSharedBirth::<T>::publication_work_demand()}else{1+SealedShared::<T>::prepare_birth_work_demand()},|effect|{if !born{let mut birth=None;let turn=SealedShared::<T>::prepare_birth(&mut birth,effect);if let Some(birth)=birth{*slot=Some(birth);*saved=Some(issuer)}turn}else{let Some(authority)=saved.as_ref().copied()else{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original sealed issuer absent")),work:0}};let Some(birth)=slot.as_mut()else{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("sealed birth absent")),work:0}};let turn=birth.publish(input,authority,output,effect);if output.is_some(){slot.take();saved.take();}turn}})
 }
 /// 🖼️ Original receiving allocates an ordinary immutable Arc without replacing its recipient.
 pub fn receive_arc<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<std::sync::Arc<T>>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if input.is_none()||output.is_some()||owner.sealed.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native Arc admission requires its original slots").into()),work:0}}
  let demand=owner.arc_demand();let born=owner.arc.is_some();let frame=&mut owner.effect;let slot=&mut *owner.arc;
  self.receive_allocation(demand,frame,!born,1,|grant,actual|{if !born{let turn=crate::retirement::arc_custody::try_uninitialized_arc::<T>(grant,actual.expect("allocated birth marker"));let result=turn.result.map(|(value,progress)|{*slot=Some(value);crate::RetainedCloneStep::Progress(progress)});crate::RetainedWorkOutcome{result,work:turn.work}}else{let mut work=0;let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("Arc birth absent"))?;let target=std::sync::Arc::get_mut(birth).ok_or_else(||ValueError::invariant("original Arc birth is not unique"))?;target.write(input.take().ok_or_else(||ValueError::invariant("original Arc input absent"))?);work=1;let birth=slot.take().ok_or_else(||ValueError::invariant("Arc birth absent"))?;*output=Some(unsafe{birth.assume_init()});Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}})
 }
 /// 🎟️ Recovers only the grant deposited in this actual original construction action.
 pub fn construction_grant(&mut self,demand:RetirementDemand,action:&mut Option<OriginalConstructionAction>)->Result<Option<Result<RetainedCloneGrant,ValueError>>,ValueError>{
  if action.is_none(){self.checkpoint()?}let work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work);let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;
  if action.is_none(){recipient.begin(demand,action,&work)?}let original=action.as_mut().ok_or_else(||ValueError::invariant("original construction action absent"))?;if original.demand()!=demand{return Err(ValueError::invariant("original construction demand changed"))}
  if original.phase()==OriginalConstructionPhase::Fresh{recipient.quote(original)?}
  if original.phase()==OriginalConstructionPhase::Granted{return Ok(Some(Ok(original.grant().ok_or_else(||ValueError::invariant("original construction grant absent"))?)))}
  recipient.recover_grant(original)
 }
 /// 🧾️ Records the actual performed span and recovers its original producer-owned verdict.
 pub fn submit_constructed_progress(&mut self,action:&mut OriginalConstructionAction,progress:RetainedCloneProgress,actual:Option<usize>)->Result<Option<semio_framework_value_control_contract::OriginalConstructionAcceptStatus>,ValueError>{
  let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;
  if action.phase()==OriginalConstructionPhase::Granted{recipient.post(action,progress,actual)?}else if action.progress()!=Some(progress)||action.actual()!=actual{return Err(ValueError::invariant("original construction receipt changed"))}
  recipient.recover_accept(action)
 }
 /// 🔓️ Publishes the original completion under this control’s actual Work and construction loans.
 pub fn publish_construction_completion(&mut self,completion:&mut semio_framework_value_control_contract::OriginalConstructionCompletion,payout:&mut OriginalConstructionAction,proof:&semio_framework_value_control_contract::OriginalWorkActionProof)->Result<bool,ValueError>{let work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work);let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;recipient.publish_completion(completion,payout,&work,proof)}
 /// ♻️ Retires a published receiving completion through this exact original construction loan.
 pub fn retire_receiving_completion(&mut self,receiving:&mut crate::OriginalReceivingPrimary,payout:&OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let recipient=match self.construction.as_deref_mut(){Some(recipient)=>recipient,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient")),work:0}};receiving.retire_receipt_completion(recipient,payout,effect)}
 /// 🏁️ Releases the same action only after every recorded original reply returned.
 pub fn finish_constructed_action(&mut self,action:&mut Option<OriginalConstructionAction>)->Result<(),ValueError>{let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;if action.as_ref().and_then(|original|original.recorded_accept_status())==Some(semio_framework_value_control_contract::OriginalConstructionAcceptStatus::Accepted){recipient.finish_accepted(action)}else{recipient.finish(action)}}
 /// ↩️ Cancels an unspent granted action and releases it without posting any physical effect.
 pub fn cancel_constructed_action(&mut self,action:&mut Option<OriginalConstructionAction>)->Result<(),ValueError>{let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;let original=action.as_mut().ok_or_else(||ValueError::invariant("original construction action absent"))?;if matches!(original.phase(),OriginalConstructionPhase::Fresh|OriginalConstructionPhase::Granted){recipient.cancel(original,false,Default::default(),None)?}recipient.finish(action)}
 /// 🧾️ Posts the actual span under a granted action, then releases the action with the recipient's verdict.
 pub fn settle_constructed_progress(&mut self,action:&mut Option<OriginalConstructionAction>,progress:RetainedCloneProgress)->Result<(),ValueError>{
  if progress==RetainedCloneProgress::default(){return self.cancel_constructed_action(action)}
  let original=action.as_mut().ok_or_else(||ValueError::invariant("original construction action absent"))?;
  let status=self.submit_constructed_progress(original,progress,(progress.retained_capacity_bytes!=0).then_some(progress.retained_capacity_bytes))?;
  let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction requires its original recipient"))?;
  match status{
   Some(semio_framework_value_control_contract::OriginalConstructionAcceptStatus::Accepted)=>recipient.finish_accepted(action),
   Some(semio_framework_value_control_contract::OriginalConstructionAcceptStatus::Refused)=>Err(recipient.finish_refused(action)?),
   None=>Err(ValueError::invariant("original construction reply is pending")),
  }
 }
 /// 🏗️ Lends a held construction scope to the original resident producer and reply.
 pub fn receive_prepared_construction<P>(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect<P::Failure>,work:u64,authority:&mut OriginalEffectAuthority<P>,invoke:impl FnOnce(&mut NativeConstructionScope<'_,'_>,&mut crate::RetainedAllocationEffect<'_>,&mut OriginalEffectAuthority<P>,&mut Option<OriginalEffectAction<crate::RetainedCloneStep,P::Failure>>)->Result<bool,ValueError>)->Result<NativeReceivingEffectPoll<P::Failure>,ValueError>where P:OriginalEffectProducer<Output=crate::RetainedCloneStep>,P::Failure:NativeReceivingEffectError{
  self.receive_original_effect(demand,work,frame,authority,|original,effect,authority,action|{let grant=effect.grant();invoke(&mut NativeConstructionScope{original,grant,work},effect,authority,action)})
 }
 /// 🏗️ Lends one original construction recipient through this receiving hop without changing its cumulative decoder receipt.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut OriginalConstructionLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{run(self,recipient,operation)}
 /// 🏗️ Advances the actual retained value effect under original construction and work loans.
 /// 🎟️ Runs one allocation producer directly on the original effect lent by the receiving frame.
 pub fn receive_allocation_effect(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect,allocating:bool,work_demand:u64,operation:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.receive_effect_typed(demand,frame,allocating,work_demand,operation);crate::RetainedWorkOutcome{result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child.or(receiving.allocation_refusal.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}),work:turn.work}}
 pub fn receive_allocation(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect,allocating:bool,work_demand:u64,operation:impl FnOnce(RetainedCloneGrant,Option<&mut Option<usize>>)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.receive_effect_typed(demand,frame,allocating,work_demand,|effect|{let grant=effect.grant();let turn=operation(grant,Some(effect.actual()));let owed=u64::try_from(turn.work.saturating_sub(effect.performed_work())).map_err(|_|ValueError::invariant("original allocation work overflow"));let recorded=match owed{Ok(0)=>Ok(true),Ok(owed)=>effect.performed(owed),Err(error)=>Err(error)};let result=match recorded{Ok(true)=>turn.result,Ok(false)=>Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original allocation work exceeded its authority")),Err(error)=>Err(error)};crate::RetainedWorkOutcome{result,work:turn.work}});crate::RetainedWorkOutcome{result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child.or(receiving.allocation_refusal.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}),work:turn.work}}
 /// ♻️ Advances the actual retained retirement effect without flattening its failure.
 pub fn receive_retirement(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect<crate::OriginalRetirementFailure>,allocating:bool,work_demand:u64,operation:impl FnOnce(RetainedCloneGrant,Option<&mut Option<usize>>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.receive_effect_typed(demand,frame,allocating,work_demand,|effect|{let grant=effect.grant();operation(grant,Some(effect.actual()))});crate::RetainedWorkOutcome{result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.retirement_failure=failure.child.or(receiving.retirement_failure.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}),work:turn.work}}
 fn receive_effect_typed<E:NativeReceivingEffectError>(&mut self,demand:RetirementDemand,frame:&mut NativeReceivingEffect<E>,allocating:bool,work_demand:u64,operation:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,E>)->crate::RetainedWorkOutcome<RetainedCloneProgress,NativeReceivingEffectFailure<E>>{
   if frame.phase()==NativeReceivingEffectPhase::Empty{if let Err(error)=frame.quote(self,demand){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}else if !frame.matches_demand(demand){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original shared demand changed during recovery").into()),work:0}}
   if frame.phase()==NativeReceivingEffectPhase::Granted{let admission=match frame.prepare_work(self,work_demand){Ok(Some(admitted))=>Ok(admitted),Ok(None)=>return crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0},Err(refusal)=>Err(refusal)};match admission{Ok(true)=>{let result=if allocating{frame.effect_controlled(self,|_,effect|operation(effect))}else{frame.effect_unallocated_controlled(self,|_,effect|operation(effect))};if let Err(error)=result{return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}},Ok(false)=>{if let Err(error)=frame.cancel_unstarted(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}},Err(refusal)=>{if let Err((error,refusal))=frame.record_work_refusal(self,refusal){let mut failure:NativeReceivingEffectFailure<E>=error.into();failure.work_refusal=Some(refusal);return crate::RetainedWorkOutcome{result:Err(failure),work:0}}if let Err(error)=frame.cancel_unstarted(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}}}
   self.finish_receiving_typed(frame)
  }
  /// 🧾️ Settles the original receiving effect with the same construction and work issuers.
  pub fn finish_receiving(&mut self,frame:&mut NativeReceivingEffect)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.finish_receiving_typed(frame);crate::RetainedWorkOutcome{result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child.or(receiving.allocation_refusal.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}),work:turn.work}}
  /// ♻️ Settles the original retirement child while preserving its complete failure custody.
  pub fn finish_retirement_receiving(&mut self,frame:&mut NativeReceivingEffect<crate::OriginalRetirementFailure>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.finish_receiving_typed(frame);crate::RetainedWorkOutcome{result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.retirement_failure=failure.child.or(receiving.retirement_failure.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}),work:turn.work}}
  fn finish_receiving_typed<E:NativeReceivingEffectError>(&mut self,frame:&mut NativeReceivingEffect<E>)->crate::RetainedWorkOutcome<RetainedCloneProgress,NativeReceivingEffectFailure<E>>{
   if matches!(frame.phase(),NativeReceivingEffectPhase::EffectStarted|NativeReceivingEffectPhase::EffectFinished){
    if let Some(work)=frame.child_work(){
     if !frame.work_is_settled(){
      let settled=if frame.work_commit_is_pending(){frame.retry_original_work(self)}else{frame.settle_original_work(self)};
      if let Err(error)=settled{return crate::RetainedWorkOutcome{result:Err(error.into()),work:if self.has_pending_work(){0}else{frame.claim_child_work()}}}
     }
    }else if frame.phase()==NativeReceivingEffectPhase::EffectFinished&&self.has_pending_work(){if let Err(error)=frame.cancel_unperformed_work(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}}
   }
   if frame.phase()==NativeReceivingEffectPhase::EffectStarted&&frame.work_is_settled()&&frame.allocation_progress().is_some(){if let Err(error)=frame.finish_allocation(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:frame.claim_child_work()}}}
   if frame.phase()==NativeReceivingEffectPhase::EffectStarted{return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original allocation callback requires resolution").into()),work:frame.claim_child_work()}}
   if frame.phase()==NativeReceivingEffectPhase::EffectFinished{if let Err(error)=frame.receipt(self){return crate::RetainedWorkOutcome{result:Err(error.into()),work:frame.claim_child_work()}}}
  match frame.take(self){Ok(turn)=>crate::RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work},Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}
 }

}
/// 🎛️ Construction borrows the original decoder and recipient without deriving retained authority from cumulative totals.
pub struct NativeOwnedDecodeControl<'operation,'decode,'recipient>{original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut OriginalConstructionLoan<'recipient>}
impl<'operation,'decode,'recipient> NativeOwnedDecodeControl<'operation,'decode,'recipient>{
 /// 🔗️ The required recipient belongs to the same owning operation that lends the original decode control.
 pub fn new(original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut OriginalConstructionLoan<'recipient>)->Self{Self{original,recipient}}
 /// 📏️ Original payload and resulting source header movement accompany the exact sealed backing birth.
 pub fn shared_birth_demand<T:Send+Sync+'static>()->RetirementDemand{SealedShared::<T>::prepare_birth_demand().unwrap_or(RetirementDemand{capacity_bytes:usize::MAX,depth:1,..Default::default()})}
 /// 📐️ An ordinary shared backing quotes the actual standard Arc layout.
 pub fn arc_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{capacity_bytes:crate::retirement::arc_custody::shared_allocation_bytes::<T>(),depth:1,..Default::default()}}
 /// 📦️ The exact original payload or born owner survives every native primary and receipt refusal.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,owner:&mut NativeSharedAdmission<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  match self.original.with_construction_recipient(self.recipient,|control|Ok::<_,ValueError>(control.receive_shared(input,output,issuer,owner))){Ok(turn)=>turn,Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}
 }

}

#[path="📦️backing/🦀️.rs"]mod backing;
pub use backing::{NativeBackingTurn,allocate_empty_vec};
