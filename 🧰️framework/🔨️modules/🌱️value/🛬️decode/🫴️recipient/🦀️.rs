//! 🫴️ Retains original decoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,RetainedWorkOutcome,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
#[path="♻️retirement/🦀️.rs"]
mod retirement;

/// 🪑️ Stack-resident recipient capacity is admitted before a decoder can create partial owners.
pub struct NativeDecodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) reserved:bool,pub(super) identity:u64,pub(super) failure_pool:semio_framework_value_control_contract::OriginalFailureStoragePool}
impl NativeDecodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),reserved:false,identity,failure_pool:Default::default()}}
    /// 🪪️ Identifies the original retained recipient.
    pub fn identity(&self)->u64{self.identity}
    /// 🧺️ Reports whether this original recipient holds a prepaid collision node.
    pub fn has_failure_storage(&self)->bool{!self.failure_pool.is_empty()}
    /// 📏️ Quotes this recipient’s selected spare-node release without borrowing a control.
    pub fn failure_storage_release_demand(&self)->crate::RetirementDemand{semio_framework_value_retirement::original_failure_storage_release_demand(&self.failure_pool)}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{self.owner.is_some()}
    /// 🔒️ Reserves the empty slot before any construction can create a partial owner.
    /// 🔒️ Reports an active reserved slot that a nested producer may settle through a child recipient.
    pub fn is_reserved(&self)->bool{self.reserved}
    pub fn reserve(&mut self)->Result<(),ValueError>{if self.reserved||self.owner.is_some(){return Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot"))}self.reserved=true;Ok(())}
    /// 🔓️ Settles the reserved slot with the owner its operation produced, if any.
    pub fn settle(&mut self,owner:Option<Box<dyn ErasedSnapshotRetirement>>){*self.owner=owner;self.reserved=false;}
    /// ▶️ Runs one operation with the slot reserved and retains whatever original owner it returns.
    pub fn with_reserved_owner<T,E:From<ValueError>>(&mut self,operation:impl FnOnce()->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{self.reserve().map_err(E::from)?;let(result,owner)=operation();self.settle(owner);result}
 pub(super) fn close_spare_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<semio_framework_value_control_contract::OriginalFailureStorage::birth_bytes(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  match self.failure_pool.release_one(grant.maximum_release_bytes){Ok(progress)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work:1},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}
 }
 pub(super) fn close_reserved_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if grant.maximum_items==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if grant.maximum_depth==0{return RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"native return slot close requires admitted depth")),work:0}}
  self.reserved=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
}
impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
 fn visit_original(&mut self,identity:std::any::TypeId,visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{self.owner.as_mut().is_some_and(|owner|owner.visit_original(identity,visitor))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
  use semio_framework_value_retirement::{OriginalRetirementFailure as Failure,settle_original_retirement_work as settle};
  if original.retirement_recipient_identity()!=Some(self.identity){return RetainedWorkOutcome{result:Err(Failure::from_primary(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"decoder cleanup changed its original recipient"))),work:0}}
  if !self.reserved&&self.owner.is_none()&&!self.failure_pool.is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<semio_framework_value_control_contract::OriginalFailureStorage::birth_bytes(){return settle(self.close_spare_step(grant),Ok(()))}match original.prepare_work(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}let turn=self.close_spare_step(grant);let result=original.commit_work(turn.work);return settle(turn,result)}
  if self.reserved{if grant.maximum_items==0||grant.maximum_depth==0{return settle(self.close_reserved_step(grant),Ok(()))}match original.prepare_work(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}let turn=self.close_reserved_step(grant);let result=original.commit_work(turn.work);return settle(turn,result)}
  crate::close_factory_ticket_original(&mut self.owner,original,grant)
 }
 fn requires_original_control(&self)->bool{true}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.reserved||self.owner.is_none()&&!self.failure_pool.is_empty(){return Ok(1)}match self.owner.as_ref(){Some(owner)if owner.terminal_is_empty()=>Ok(1),Some(owner)=>owner.next_computational_work(body),None=>Ok(0)}}
 fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->semio_framework_value_retirement::OriginalRetirementOutcome{
  use semio_framework_value_retirement::{OriginalRetirementFailure as Failure,settle_original_retirement_work as settle};
  if !self.reserved&&self.owner.is_none()&&!self.failure_pool.is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<semio_framework_value_control_contract::OriginalFailureStorage::birth_bytes(){return settle(self.close_spare_step(grant),Ok(()))}match control.prepare_work(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}let turn=self.close_spare_step(grant);let result=control.commit_work(turn.work);return settle(turn,result)}
  if self.reserved{if grant.maximum_items==0||grant.maximum_depth==0{return settle(self.close_reserved_step(grant),Ok(()))}match control.prepare_work(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}let turn=self.close_reserved_step(grant);let result=control.commit_work(turn.work);return settle(turn,result)}
  semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.owner,grant,control)
 }
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if self.reserved{Ok(crate::RetirementDemand{depth:1,..Default::default()})}else if self.owner.is_none()&&!self.failure_pool.is_empty(){Ok(crate::RetirementDemand{release_bytes:semio_framework_value_control_contract::OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()})}else{self.owner.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))}
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
  if !self.reserved&&self.owner.is_none()&&!self.failure_pool.is_empty(){return semio_framework_value_retirement::settle_original_retirement_work(self.close_spare_step(grant),Ok(()))}
  if !self.reserved{return crate::close_factory_ticket(&mut self.owner,grant)}
  semio_framework_value_retirement::settle_original_retirement_work(self.close_reserved_step(grant),Ok(()))
 }
 fn terminal_is_empty(&self)->bool{!self.reserved&&self.owner.is_none()&&self.failure_pool.is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(|d|d.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.depth)}
}
impl Drop for NativeDecodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}

