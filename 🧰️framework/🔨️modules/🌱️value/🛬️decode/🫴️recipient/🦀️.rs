//! 🫴️ Retains original decoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,RetainedWorkOutcome,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
#[path="♻️retirement/🦀️.rs"]
mod retirement;

/// 🪑️ Stack-resident recipient capacity is admitted before a decoder can create partial owners.
pub struct NativeDecodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) funding:NativeDecodeRetirementFunding}
/// 🧾️ Keeps the actual recipient currencies separate from its physical owner slot.
pub(super) struct NativeDecodeRetirementFunding{pub(super) reserved:bool,pub(super) identity:u64,pub(super) failure_pool:semio_framework_value_control_contract::OriginalFailureStoragePool,pub(super) failure_allocation:Option<crate::NativeAllocationTransaction>,pub(super) failure_port:Option<Option<crate::NativeAllocationIdentity>>,pub(super) failure_ledger:Option<crate::NativeAllocationIdentity>,pub(super) failure_verdict:Option<Result<(),crate::NativeAllocationRefusal>>,pub(super) failure_protocol_refusal:Option<ValueError>,pub(super) failure_birth:semio_framework_value_control_contract::OriginalFailureStorageBirth}
impl NativeDecodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),funding:NativeDecodeRetirementFunding{reserved:false,identity,failure_pool:Default::default(),failure_allocation:None,failure_port:None,failure_ledger:None,failure_verdict:None,failure_protocol_refusal:None,failure_birth:Default::default()}}}
    /// 🪪️ Identifies the original retained recipient.
    pub fn identity(&self)->u64{self.funding.identity}
    /// 🧺️ Reports whether this original recipient holds a prepaid collision node.
    pub fn has_failure_storage(&self)->bool{!self.funding.failure_pool.is_empty()}
    /// 📏️ Quotes transfer of an original protocol refusal that exceeded Birth custody.
    pub fn failure_protocol_refusal_demand(&self)->crate::RetirementDemand{if self.funding.failure_protocol_refusal.is_none(){return Default::default()}crate::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<ValueError>>(),depth:1,..Default::default()}}
    /// 📏️ Quotes this recipient’s selected spare-node release without borrowing a control.
    pub fn failure_storage_release_demand(&self)->crate::RetirementDemand{semio_framework_value_retirement::original_failure_storage_release_demand(&self.funding.failure_pool)}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{!self.terminal_is_empty()}
    /// 🚧️ Reports a returned owner, reserved slot or unsettled failure birth that still needs closing, excluding idle spare failure custody.
    pub fn has_pending_close(&self)->bool{self.funding.reserved||self.owner.is_some()||self.pending_failure()}
    /// 🔒️ Reserves the empty slot before any construction can create a partial owner.
    /// 🔒️ Reports an active reserved slot that a nested producer may settle through a child recipient.
    pub fn is_reserved(&self)->bool{self.funding.reserved}
    pub fn reserve(&mut self)->Result<(),ValueError>{if self.funding.reserved||self.owner.is_some()||self.funding.failure_allocation.is_some()||self.funding.failure_port.is_some()||self.funding.failure_ledger.is_some()||self.funding.failure_verdict.is_some()||self.funding.failure_protocol_refusal.is_some()||!self.funding.failure_birth.terminal_is_empty(){return Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot"))}self.funding.reserved=true;Ok(())}
    /// 🔓️ Settles the reserved slot with the owner its operation produced, if any.
    pub fn settle(&mut self,owner:Option<Box<dyn ErasedSnapshotRetirement>>){*self.owner=owner;self.funding.reserved=false;}
    /// ▶️ Runs one operation with the slot reserved and retains whatever original owner it returns.
    pub fn with_reserved_owner<T,E:From<ValueError>>(&mut self,operation:impl FnOnce()->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{self.reserve().map_err(E::from)?;let(result,owner)=operation();self.settle(owner);result}
 fn pending_failure(&self)->bool{self.funding.failure_allocation.is_some()||self.funding.failure_port.is_some()||self.funding.failure_ledger.is_some()||self.funding.failure_verdict.is_some()||self.funding.failure_protocol_refusal.is_some()||!self.funding.failure_birth.terminal_is_empty()}
 fn record_leaf(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  match effect.performed(1){Ok(true)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:1}),Ok(false)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original recipient close exceeded its admitted work")),work:1}),Err(error)=>crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:1})}
 }
 pub(super) fn close_spare_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  let grant=effect.grant();let demand=self.failure_storage_release_demand();
  if effect.maximum_work_units()<1||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Pending{work:0}}
  let recorded=Self::record_leaf(effect);if !matches!(&recorded,crate::RetainedWorkPoll::Complete(turn)if turn.result.is_ok()){return recorded}
  crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:self.funding.failure_pool.release_one(grant.maximum_release_bytes).map(RetainedCloneStep::Progress),work:1})
 }
 pub(super) fn close_reserved_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{
  let grant=effect.grant();let demand=crate::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};
  if effect.maximum_work_units()<1||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Pending{work:0}}
  if !self.funding.reserved{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original return slot is not reserved")),work:0})}
  let recorded=Self::record_leaf(effect);if !matches!(&recorded,crate::RetainedWorkPoll::Complete(turn)if turn.result.is_ok()){return recorded}
  self.funding.reserved=false;crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1})
 }
}
impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
 fn requires_original_birth(&self)->bool{!self.funding.reserved&&self.owner.as_ref().is_some_and(|owner|owner.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{
  if original.retirement_recipient_identity()!=Some(self.funding.identity){return refused(ValueError::invariant("decoder birth changed its original recipient"))}
  match self.owner.as_mut(){Some(owner)if !self.funding.reserved=>owner.close_original_birth_step(original,grant),_=>refused(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"decoder has no selected original child birth"))}
 }
 fn visit_original(&mut self,identity:std::any::TypeId,visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{self.owner.as_mut().is_some_and(|owner|owner.visit_original(identity,visitor))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{
  if original.retirement_recipient_identity()!=Some(self.funding.identity){return refused(ValueError::invariant("decoder cleanup changed its original recipient"))}
  if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  crate::close_factory_ticket_original(&mut self.owner,original,effect)
 }
 fn requires_original_control(&self)->bool{true}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.pending_failure(){return Err(pending_failure())}if self.funding.reserved||self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return Ok(1)}match self.owner.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->semio_framework_value_retirement::OriginalRetirementPoll{
  if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.owner,effect,control)
 }
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if self.pending_failure(){return Err(pending_failure())}
  if self.funding.reserved{Ok(crate::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}else if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){Ok(self.failure_storage_release_demand())}else{self.owner.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))}
 }
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{
  if self.pending_failure(){return refused(pending_failure())}
  if self.funding.reserved{return self.close_reserved_step(effect).map_err(Into::into)}
  if self.owner.is_none()&&!self.funding.failure_pool.is_empty(){return self.close_spare_step(effect).map_err(Into::into)}
  crate::close_factory_ticket(&mut self.owner,effect)
 }
 fn terminal_is_empty(&self)->bool{!self.funding.reserved&&self.owner.is_none()&&self.funding.failure_pool.is_empty()&&!self.pending_failure()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(|d|d.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.depth)}
}
fn pending_failure()->ValueError{ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original failure allocation requires recovery")}
fn refused(error:ValueError)->semio_framework_value_retirement::OriginalRetirementPoll{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
impl Drop for NativeDecodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}

