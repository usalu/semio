//! 👶️ Original failure-node birth retains work, physical output and callback custody between turns.
use crate::{NativeAllocationRefusal,NativeAllocationRequest,OriginalFailureStorage,OriginalFailureStoragePool,OriginalRetirementFailure,OriginalWorkActionIdentity,OriginalWorkActionProof,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
/// 🚦️ Pending retains its original action; completion transfers that action's actual verdict.
pub enum OriginalFailureStorageStep<T,E=NativeAllocationRefusal>{Pending,Complete(Result<T,E>)}
/// 🧭️ The original owner preserves each effect and settlement boundary before invoking a callback.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum OriginalFailureStorageBirthPhase{#[default]Empty,WorkPrepare,Quote,Allocate,Allocating,WorkPartial,WorkCommit,Post,CancelAllocation,CancelWork,Finishing,Settled}
/// 🧳️ The original recipient owns every unfinished cause, node, proof and physical invoice.
#[derive(Default)]
pub struct OriginalFailureStorageBirth{phase:OriginalFailureStorageBirthPhase,requested:Option<RetainedCloneGrant>,quoted:Option<RetainedCloneGrant>,demand:RetirementDemand,request:Option<NativeAllocationRequest>,actual:Option<usize>,progress:RetainedCloneProgress,work:u128,reported_work:u128,performed:Option<crate::OriginalWorkPerformedReceipt>,work_claim:std::mem::ManuallyDrop<Option<Result<u128,ValueError>>>,partial_work:std::mem::ManuallyDrop<Option<crate::OriginalWorkPartialOutcome>>,proof:Option<OriginalWorkActionProof>,action:Option<OriginalWorkActionIdentity>,excess_refusal:Option<ValueError>,primary:Option<ValueError>,work_refusal:Option<ValueError>,receipt_refusal:Option<ValueError>,allocation_refusal:Option<NativeAllocationRefusal>}
#[path="⏱️performed/🦀️.rs"]mod performed;
pub use performed::OriginalFailurePerformedScope;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl OriginalFailureStorageBirth{
 pub fn phase(&self)->OriginalFailureStorageBirthPhase{self.phase}
 pub fn terminal_is_empty(&self)->bool{self.phase==OriginalFailureStorageBirthPhase::Empty}
 pub fn requested_grant(&self)->Option<RetainedCloneGrant>{self.requested}
 pub fn quoted_grant(&self)->Option<RetainedCloneGrant>{self.quoted}
 pub fn demand(&self)->RetirementDemand{self.demand}
 pub fn request(&self)->Option<NativeAllocationRequest>{self.request}
 pub fn actual(&self)->Option<usize>{self.actual}
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 pub fn work(&self)->u128{self.work}
 pub fn work_proof(&self)->Option<&OriginalWorkActionProof>{self.proof.as_ref()}
 pub fn work_action(&self)->Option<OriginalWorkActionIdentity>{self.action}
 pub fn work_action_output(&mut self)->Result<&mut Option<OriginalWorkActionIdentity>,ValueError>{if self.phase!=OriginalFailureStorageBirthPhase::WorkPrepare{return Err(invalid("failure birth is outside original work admission"))}Ok(&mut self.action)}
 pub fn work_action_slot(&mut self)->Result<&mut Option<OriginalWorkActionIdentity>,ValueError>{if !matches!(self.phase,OriginalFailureStorageBirthPhase::WorkPrepare|OriginalFailureStorageBirthPhase::WorkCommit|OriginalFailureStorageBirthPhase::CancelWork|OriginalFailureStorageBirthPhase::Finishing){return Err(invalid("failure birth cannot mutate its original action in this phase"))}Ok(&mut self.action)}
 pub fn finishing_work_slots(&mut self)->Result<(&mut Option<OriginalWorkActionProof>,&mut Option<OriginalWorkActionIdentity>),ValueError>{if self.phase!=OriginalFailureStorageBirthPhase::Finishing{return Err(invalid("failure birth is outside original action finish"))}Ok((&mut self.proof,&mut self.action))}
 /// 🛡️ Keeps a preflight refusal without replacing any earlier actual cause.
 pub fn retain_protocol_refusal(&mut self,error:ValueError)->Result<(),ValueError>{if self.phase==OriginalFailureStorageBirthPhase::Empty{return Err(error)}if self.receipt_refusal.is_none(){self.receipt_refusal=Some(error);return Ok(())}if self.excess_refusal.is_none(){self.excess_refusal=Some(error);return Ok(())}Err(error)}
 fn retain_work_refusal(&mut self,error:ValueError)->Result<(),ValueError>{if self.work_refusal.is_none(){self.work_refusal=Some(error);Ok(())}else{self.retain_protocol_refusal(error)}}
 pub fn excess_refusal(&self)->Option<&ValueError>{self.excess_refusal.as_ref()}
 pub fn excess_refusal_demand(&self)->RetirementDemand{crate::original_refusal_transfer_demand(&self.excess_refusal)}
 /// 📬️ Transfers an excess original refusal only through its caller's actual selected effect.
 pub fn take_excess_refusal(&mut self,output:&mut Option<ValueError>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{crate::transfer_original_refusal(&mut self.excess_refusal,output,effect)}

 /// 🪪️ Retains the actual source request before the original allocation quote begins.
 pub fn install_request(&mut self,request:NativeAllocationRequest)->Result<(),ValueError>{if self.phase!=OriginalFailureStorageBirthPhase::Quote||request.bytes!=self.demand.capacity_bytes||request.owned_bytes.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes||self.request.is_some_and(|original|original!=request){return Err(invalid("failure birth requires its unchanged original allocation request"))}self.request=Some(request);Ok(())}
 /// 📥️ Lends only the original work-commit output slot, before its reservation clears.
 pub fn work_proof_output(&mut self)->Result<&mut Option<OriginalWorkActionProof>,ValueError>{if self.phase!=OriginalFailureStorageBirthPhase::WorkCommit{return Err(invalid("failure birth is outside original work commit"))}Ok(&mut self.proof)}
 /// 📤️ Lends the retained proof only after every allocation callback has completed.
 pub fn finishing_work_proof(&mut self)->Result<&mut Option<OriginalWorkActionProof>,ValueError>{if self.phase!=OriginalFailureStorageBirthPhase::Finishing{return Err(invalid("failure birth is outside original action finish"))}Ok(&mut self.proof)}
 pub fn work_refusal(&self)->Option<&ValueError>{self.work_refusal.as_ref()}
 pub fn receipt_refusal(&self)->Option<&ValueError>{self.receipt_refusal.as_ref()}
 /// 📥️ Recovers the resident partial reply on the same original Work Source.
 pub fn settle_performed(&mut self,work:&mut crate::OriginalWork<'_>)->OriginalFailureStorageStep<(),ValueError>{
  if self.phase!=OriginalFailureStorageBirthPhase::WorkPartial{return OriginalFailureStorageStep::Complete(Err(invalid("failure birth is outside original partial settlement")))}
  if let Some(result)=self.work_claim.take(){if let Err(error)=result{return OriginalFailureStorageStep::Complete(Err(error))}}
  if self.performed.is_some(){return OriginalFailureStorageStep::Pending}
  if self.partial_work.is_none(){if let Err(error)=work.validate_action_source(self.action){return OriginalFailureStorageStep::Complete(Err(error))}match work.partial_pending_for_action(self.action){Ok(true)=>match work.settle_partial_for_action_into(self.action,&mut self.partial_work){Ok(true)=>{},Ok(false)=>return OriginalFailureStorageStep::Pending,Err(error)=>return OriginalFailureStorageStep::Complete(Err(error))},Ok(false)=>return OriginalFailureStorageStep::Complete(Ok(())),Err(error)=>return OriginalFailureStorageStep::Complete(Err(error))}}
  let reply=self.partial_work.take().expect("original partial reply remains resident");let(identity,_,total,admitted,refusal)=reply.into_parts();if let Some(error)=refusal{let retained=self.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains its actual partial Source refusal")}
  if Some(identity.action_identity())!=self.action||total!=self.work{return OriginalFailureStorageStep::Complete(Err(invalid("original failure partial receipt changed")))}
  if !admitted{let retained=self.retain_work_refusal(ValueError::literal(ValueRefusalKind::WorkLimit,"original failure work exceeded its quote"));assert!(retained.is_ok(),"failure birth retains its actual overrun")}
  OriginalFailureStorageStep::Complete(Ok(()))
 }

 fn emit(&mut self)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{
  let work=self.work.checked_sub(self.reported_work).expect("original failure work regressed");self.reported_work=self.work;
  if self.phase!=OriginalFailureStorageBirthPhase::Settled{return crate::RetainedWorkPoll::Pending{work}}
  assert!(self.partial_work.is_none()&&self.proof.is_none()&&self.action.is_none()&&self.performed.is_none()&&self.work_claim.is_none()&&self.excess_refusal.is_none(),"failure birth retains an original output or action");
  let progress=self.progress;let failed=self.primary.is_some()||self.work_refusal.is_some()||self.receipt_refusal.is_some()||self.allocation_refusal.is_some();
  let result=if failed{Err(OriginalRetirementFailure::from_causes(self.primary.take(),self.work_refusal.take(),self.receipt_refusal.take(),self.allocation_refusal.take()).with_retained_progress(progress))}else{Ok(RetainedCloneStep::Progress(progress))};
  self.phase=OriginalFailureStorageBirthPhase::Empty;*self=Self::default();crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})
 }

}
impl Drop for OriginalFailureStorageBirth{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original failure birth requires its original settlement and closure")}}
/// ⛽️ All services belong to the same original recipient and retained birth state.
pub trait OriginalFailureFunding{
 fn birth(&mut self)->&mut OriginalFailureStorageBirth;
 fn performed_scope(&mut self)->Result<OriginalFailurePerformedScope<'_>,ValueError>;
 fn settle_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>;
 fn quote(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>;
 fn post(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->OriginalFailureStorageStep<()>;
 fn cancel(&mut self)->OriginalFailureStorageStep<()>;
 fn pool(&mut self)->&mut OriginalFailureStoragePool;
 fn prepare(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>;
 fn commit(&mut self,work:u128)->Result<(),ValueError>;
 fn cancel_work(&mut self)->Result<(),ValueError>;
 fn finish_work(&mut self)->Result<(),ValueError>;
}
impl OriginalFailureFunding for crate::OriginalRetirementCloseLoan<'_>{
 fn birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn performed_scope(&mut self)->Result<OriginalFailurePerformedScope<'_>,ValueError>{self.failure_storage_performed_scope()}
 fn settle_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>{self.settle_failure_storage_performed()}
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->OriginalFailureStorageStep<()>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit(&mut self,work:u128)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}
}
/// 🧾️ Advances one original birth without replaying its allocator or a started callback.
pub fn refill_original_failure_storage(funding:&mut (impl OriginalFailureFunding+?Sized),grant:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{
 use OriginalFailureStorageBirthPhase as Phase;
 if funding.birth().phase!=Phase::Empty&&funding.birth().requested!=Some(grant){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("failure birth requires its original requested grant").into()),work:0})}
 if funding.birth().phase==Phase::Empty{let demand=OriginalFailureStorage::birth_demand();if !crate::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let state=funding.birth();state.requested=Some(grant);state.demand=demand;state.phase=Phase::WorkPrepare;}
 loop{if funding.birth().excess_refusal.is_some(){return funding.birth().emit()}match funding.birth().phase{
  Phase::Empty=>return funding.birth().emit(),
  Phase::WorkPrepare=>match funding.prepare(1){OriginalFailureStorageStep::Pending=>return funding.birth().emit(),OriginalFailureStorageStep::Complete(Ok(true))=>funding.birth().phase=Phase::Quote,OriginalFailureStorageStep::Complete(Ok(false))=>{let state=funding.birth();state.phase=if state.action.is_some(){Phase::CancelWork}else{Phase::Settled}},OriginalFailureStorageStep::Complete(Err(error))=>{let state=funding.birth();let retained=state.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains every original work refusal");state.phase=if state.action.is_some(){Phase::CancelWork}else{Phase::Settled}}},
  Phase::Quote=>{let demand=funding.birth().demand;let original=funding.birth().requested.expect("retained failure grant");match funding.quote(demand,original){OriginalFailureStorageStep::Pending=>return funding.birth().emit(),OriginalFailureStorageStep::Complete(Err(error))=>{let state=funding.birth();state.allocation_refusal=Some(error);state.phase=Phase::CancelWork},OriginalFailureStorageStep::Complete(Ok(quoted))=>{let state=funding.birth();state.quoted=Some(quoted);state.phase=if crate::retained_clone_grant_admits(quoted,state.demand){Phase::Allocate}else{Phase::CancelAllocation}}}},
  Phase::Allocate=>{let result=match funding.performed_scope(){Ok(mut scope)=>{scope.allocate_and_publish();Ok(())},Err(error)=>Err(error)};if let Err(error)=result{let state=funding.birth();let retained=state.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains its actual entry refusal");return state.emit()}},
  Phase::Allocating=>return funding.birth().emit(),
  Phase::WorkPartial=>match funding.settle_performed(){OriginalFailureStorageStep::Pending=>return funding.birth().emit(),OriginalFailureStorageStep::Complete(Ok(()))=>funding.birth().phase=Phase::WorkCommit,OriginalFailureStorageStep::Complete(Err(error))=>{let state=funding.birth();let retained=state.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains its actual partial refusal");return state.emit()}},
  Phase::WorkCommit=>{let work=funding.birth().work;let result=funding.commit(work);let state=funding.birth();if let Err(error)=result{let retained=state.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains every original commit refusal")}if state.proof.is_none(){if state.work_refusal.is_none(){state.work_refusal=Some(invalid("original failure work has no committed action proof"))}state.phase=Phase::WorkCommit;return state.emit()}state.phase=if state.actual.is_some(){Phase::Post}else{Phase::CancelAllocation};},
  Phase::Post=>{let quoted=funding.birth().quoted.expect("retained quoted grant");let progress=funding.birth().progress;match funding.post(quoted,progress){OriginalFailureStorageStep::Pending=>return funding.birth().emit(),OriginalFailureStorageStep::Complete(result)=>{let state=funding.birth();state.allocation_refusal=result.err();state.phase=Phase::Finishing}}},
  Phase::CancelAllocation=>match funding.cancel(){OriginalFailureStorageStep::Pending=>return funding.birth().emit(),OriginalFailureStorageStep::Complete(result)=>{let state=funding.birth();state.allocation_refusal=result.err();state.phase=if state.proof.is_some(){Phase::Finishing}else{Phase::CancelWork}}},
  Phase::CancelWork=>{let result=funding.cancel_work();let state=funding.birth();match result{Ok(())=>state.phase=Phase::Settled,Err(error)=>{let retained=state.retain_work_refusal(error);assert!(retained.is_ok(),"failure birth retains every original work refusal");state.phase=Phase::CancelWork;return state.emit()}}},
  Phase::Finishing=>{let result=funding.finish_work();let state=funding.birth();if let Err(error)=result{let retained=state.retain_protocol_refusal(error);assert!(retained.is_ok(),"failure finish retains its actual refusal");state.phase=Phase::Finishing;return state.emit()}if state.proof.is_some(){let retained=state.retain_protocol_refusal(invalid("original failure action was not consumed"));assert!(retained.is_ok(),"failure birth retains the original unfinished-proof refusal");state.phase=Phase::Finishing;return state.emit()}state.phase=Phase::Settled;},
  Phase::Settled=>return funding.birth().emit(),
 }}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
