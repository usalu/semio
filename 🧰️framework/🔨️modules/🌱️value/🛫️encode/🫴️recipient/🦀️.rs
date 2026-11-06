//! 🫴️ One explicitly installed caller slot retains returned encoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,RetainedWorkOutcome,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);

/// 🪑️ Stack-resident recipient capacity is admitted before a encoder can create partial owners.
pub struct NativeEncodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) reserved:bool,pub(super) identity:u64}
impl NativeEncodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),reserved:false,identity}}
    /// 🪪️ Identifies the exact caller-owned retirement port.
    pub fn identity(&self)->u64{self.identity}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{self.owner.is_some()}
}
impl ErasedSnapshotRetirement for NativeEncodeRetirementRecipient{
 fn visit_original(&mut self,identity:std::any::TypeId,visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{self.owner.as_mut().is_some_and(|owner|owner.visit_original(identity,visitor))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if self.reserved{Ok(crate::RetirementDemand{depth:1,..Default::default()})}else{self.owner.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))}
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
  if !self.reserved{return crate::close_factory_ticket(&mut self.owner,grant)}
  if grant.maximum_items==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if grant.maximum_depth==0{return RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"native return slot close requires admitted depth").into()),work:0}}
  self.reserved=false;
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 fn terminal_is_empty(&self)->bool{!self.reserved&&self.owner.is_none()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(|d|d.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(|d|d.depth)}
}
impl Drop for NativeEncodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"encoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}
