//! 🫴️ One explicitly installed caller slot retains returned encoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);

/// 🪑️ Stack-resident recipient capacity is admitted before a encoder can create partial owners.
pub struct NativeEncodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) reserved:bool,pub(super) identity:u64}
impl NativeEncodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),reserved:false,identity}}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{self.owner.is_some()}
}
impl ErasedSnapshotRetirement for NativeEncodeRetirementRecipient{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.terminal_is_empty()&&grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"encoder return slot close requires admitted depth"));}
        if self.reserved{self.reserved=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        crate::close_factory_ticket(&mut self.owner,grant)
    }
    fn terminal_is_empty(&self)->bool{!self.reserved&&self.owner.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(copy))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(std::mem::size_of_val(owner.as_ref()))}else{owner.next_release_byte_demand()})}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(usize::from(self.reserved)),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_depth_demand()})}
}
impl Drop for NativeEncodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"encoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}
