//! 🫴️ Retains original decoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,RetainedWorkOutcome,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
#[path="♻️retirement/🦀️.rs"]
mod retirement;

/// 🪑️ Stack-resident recipient capacity is admitted before a decoder can create partial owners.
pub struct NativeDecodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) close:ManuallyDrop<semio_framework_value_retirement::FactoryTicketClose>,pub(super) funding:NativeDecodeRetirementFunding}
/// 🧾️ Keeps the actual recipient currencies separate from its physical owner slot.
pub(super) struct NativeDecodeRetirementFunding{pub(super) reserved:bool,pub(super) identity:u64,pub(super) failure_pool:semio_framework_value_retirement_contract::OriginalFailureStoragePool,pub(super) failure_allocation:Option<crate::NativeAllocationTransaction>,pub(super) failure_port:Option<Option<crate::NativeAllocationIdentity>>,pub(super) failure_ledger:Option<crate::NativeAllocationIdentity>,pub(super) failure_verdict:Option<Result<(),crate::NativeAllocationRefusal>>,pub(super) failure_protocol_refusal:Option<ValueError>,pub(super) failure_birth:semio_framework_value_retirement_contract::OriginalFailureStorageBirth}
impl NativeDecodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),close:ManuallyDrop::new(Default::default()),funding:NativeDecodeRetirementFunding{reserved:false,identity,failure_pool:Default::default(),failure_allocation:None,failure_port:None,failure_ledger:None,failure_verdict:None,failure_protocol_refusal:None,failure_birth:Default::default()}}}
    /// 🪪️ Identifies the original retained recipient.
    pub fn identity(&self)->u64{self.funding.identity}
    /// 🧺️ Reports whether this original recipient holds a prepaid collision node.
    pub fn has_failure_storage(&self)->bool{!self.funding.failure_pool.is_empty()}
    /// 📏️ Quotes transfer of an original protocol refusal that exceeded Birth custody.
    pub fn failure_protocol_refusal_demand(&self)->crate::RetirementDemand{semio_framework_value_retirement_contract::original_refusal_transfer_demand(&self.funding.failure_protocol_refusal)}
    /// 📬️ Transfers the exact overflow refusal only through the caller’s selected effect.
    pub fn take_failure_protocol_refusal(&mut self,output:&mut Option<ValueError>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{semio_framework_value_retirement_contract::transfer_original_refusal(&mut self.funding.failure_protocol_refusal,output,effect)}
    /// 📏️ Quotes this recipient’s selected spare-node release without borrowing a control.
    pub fn failure_storage_release_demand(&self)->crate::RetirementDemand{semio_framework_value_retirement::original_failure_storage_release_demand(&self.funding.failure_pool)}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{!self.terminal_is_empty()}
    /// 🚧️ Reports a returned owner, reserved slot or unsettled failure birth that still needs closing, excluding idle spare failure custody.
    pub fn has_pending_close(&self)->bool{self.funding.reserved||self.owner.is_some()||self.pending_failure()}
    /// 🔒️ Reserves the empty slot before any construction can create a partial owner.
    /// 🔒️ Reports an active reserved slot that a nested producer may settle through a child recipient.
    pub fn is_reserved(&self)->bool{self.funding.reserved}
    pub fn reserve(&mut self)->Result<(),ValueError>{if self.funding.reserved||!self.close.is_empty()||self.owner.is_some()||self.funding.failure_allocation.is_some()||self.funding.failure_port.is_some()||self.funding.failure_ledger.is_some()||self.funding.failure_verdict.is_some()||self.funding.failure_protocol_refusal.is_some()||!self.funding.failure_birth.terminal_is_empty(){return Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot"))}self.funding.reserved=true;Ok(())}
    /// 🔓️ Settles the reserved slot with the owner its operation produced, if any.
    pub fn settle(&mut self,owner:Option<Box<dyn ErasedSnapshotRetirement>>){*self.owner=owner;self.funding.reserved=false;}
    /// ▶️ Runs one operation with the slot reserved and retains whatever original owner it returns.
    pub fn with_reserved_owner<T,E:From<ValueError>>(&mut self,operation:impl FnOnce()->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{self.reserve().map_err(E::from)?;let(result,owner)=operation();self.settle(owner);result}
 fn close_leaf_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,reserved:bool)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,ValueError>{let demand=if reserved{crate::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}else{self.failure_storage_release_demand()};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};if remaining<1||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if reserved&&!self.funding.reserved{return Ok(reply.complete(Err(ValueError::invariant("original return slot is not reserved").into())))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original recipient close exceeded its admitted work").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}let result=if reserved{self.funding.reserved=false;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}else{self.funding.failure_pool.release_one(effect.grant().maximum_release_bytes)};Ok(reply.complete(result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn pending_failure(&self)->bool{self.funding.failure_allocation.is_some()||self.funding.failure_port.is_some()||self.funding.failure_ledger.is_some()||self.funding.failure_verdict.is_some()||self.funding.failure_protocol_refusal.is_some()||!self.funding.failure_birth.terminal_is_empty()}
 fn record_leaf(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{let before=effect.performed_work();let result=match effect.performed(1){Ok(true)=>Ok(RetainedCloneStep::Progress(Default::default())),Ok(false)=>Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original recipient close exceeded its admitted work")),Err(error)=>Err(error)};crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-before})}
 pub(super) fn close_spare_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  let grant=effect.grant();let demand=self.failure_storage_release_demand();
  let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if remaining<1||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let recorded=Self::record_leaf(effect);if !matches!(&recorded,crate::RetainedWorkPoll::Complete(turn)if turn.result.is_ok()){return recorded}
  crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:self.funding.failure_pool.release_one(grant.maximum_release_bytes).map(RetainedCloneStep::Progress),work:1})
 }
 pub(super) fn close_reserved_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  let grant=effect.grant();let demand=crate::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};
  let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if remaining<1||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.funding.reserved{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original return slot is not reserved")),work:0})}
  let recorded=Self::record_leaf(effect);if !matches!(&recorded,crate::RetainedWorkPoll::Complete(turn)if turn.result.is_ok()){return recorded}
  self.funding.reserved=false;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1})
 }
}
impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
 fn close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,ValueError>{if self.pending_failure(){return Ok(reply.complete(Err(pending_failure().into())))}if !self.close.is_empty(){return Err(ValueError::invariant("recipient has an unresolved child close"))}if self.funding.reserved{return self.close_leaf_into(effect,reply,true)}if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_leaf_into(effect,reply,false)}self.close.close_into(&mut self.owner,body,effect,reply)}
 fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,ValueError>{if self.close.is_empty(){return Err(ValueError::invariant("recipient has no original child close to recover"))}self.close.recover_close_into(&mut self.owner,body,effect,reply)}
 fn close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,ValueError>{if original.retirement_recipient_identity()!=Some(self.funding.identity){return Ok(reply.complete(Err(ValueError::invariant("decoder cleanup changed its original recipient").into())))}if self.pending_failure(){return Ok(reply.complete(Err(pending_failure().into())))}if !self.close.is_empty(){return Err(ValueError::invariant("recipient has an unresolved child close"))}if self.funding.reserved{return self.close_leaf_into(effect,reply,true)}if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_leaf_into(effect,reply,false)}self.close.close_original_into(&mut self.owner,original,body,effect,reply)}
 fn recover_close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,ValueError>{if original.retirement_recipient_identity()!=Some(self.funding.identity){return Err(ValueError::invariant("decoder cleanup recovery changed its original recipient"))}if self.close.is_empty(){return Err(ValueError::invariant("recipient has no original child close to recover"))}self.close.recover_close_original_into(&mut self.owner,original,body,effect,reply)}

 fn requires_original_birth(&self)->bool{self.close.is_empty()&&!self.funding.reserved&&self.owner.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{
  if original.retirement_recipient_identity()!=Some(self.funding.identity){return refused(ValueError::invariant("decoder birth changed its original recipient"))}
  match self.owner.as_mut(){Some(owner)if self.close.is_empty()&&!self.funding.reserved&&!owner.terminal_is_empty()&&owner.requires_original_birth()=>owner.close_original_birth_step(original,grant),_=>refused(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"decoder has no selected original child birth"))}
 }
 fn visit_original(&mut self,identity:std::any::TypeId,visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{self.owner.as_mut().is_some_and(|owner|owner.visit_original(identity,visitor))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{
  if original.retirement_recipient_identity()!=Some(self.funding.identity){return refused(ValueError::invariant("decoder cleanup changed its original recipient"))}
  if !self.close.is_empty(){return refused(ValueError::invariant("recipient retains an original child close"))}if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  crate::close_factory_ticket_original(&mut self.owner,original,body,effect)
 }
 fn requires_original_control(&self)->bool{true}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if !self.close.is_empty(){return self.close.next_work(&self.owner,body)}if self.pending_failure(){return Err(pending_failure())}if self.funding.reserved||self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return Ok(1)}self.close.next_work(&self.owner,body)}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->semio_framework_value_retirement::OriginalRetirementPoll{
  if !self.close.is_empty(){return refused(ValueError::invariant("recipient retains an original child close"))}if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.owner,body,effect,control)
 }
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if !self.close.is_empty(){return self.close.next_demand(&self.owner,body)}
  if self.pending_failure(){return Err(pending_failure())}
  if self.funding.reserved{Ok(crate::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}else if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){Ok(self.failure_storage_release_demand())}else{self.owner.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))}
 }
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{
  if !self.close.is_empty(){return refused(ValueError::invariant("recipient retains an original child close"))}if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  crate::close_factory_ticket(&mut self.owner,body,effect)
 }
 fn terminal_is_empty(&self)->bool{!self.funding.reserved&&self.close.is_empty()&&self.owner.is_none()&&self.funding.failure_pool.is_empty()&&!self.pending_failure()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(|d|d.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.depth)}
}
fn pending_failure()->ValueError{ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original failure allocation requires recovery")}
fn refused(error:ValueError)->semio_framework_value_retirement::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
impl Drop for NativeDecodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);ManuallyDrop::drop(&mut self.close);}}}}

