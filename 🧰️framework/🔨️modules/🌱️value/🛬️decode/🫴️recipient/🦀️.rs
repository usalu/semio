//! 🫴️ Retains original decoder ownership until physical drain.
use crate::{ErasedSnapshotRetirement,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
static RECIPIENT_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);

/// 🪑️ Stack-resident recipient capacity is admitted before a decoder can create partial owners.
pub struct NativeDecodeRetirementRecipient{pub(super) owner:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,pub(super) reserved:bool,pub(super) identity:u64}
impl NativeDecodeRetirementRecipient{
    /// 🈳️ Creates the single empty caller-owned return slot without heap allocation.
    pub fn new()->Self{let identity=RECIPIENT_IDS.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|identity|identity.checked_add(1)).expect("native recipient identity space exhausted");Self{owner:ManuallyDrop::new(None),reserved:false,identity}}
    /// 👓️ Reports retained ownership without borrowing or transferring the actual owner.
    pub fn has_owner(&self)->bool{self.owner.is_some()}
    /// 🔒️ Reserves the empty slot before any construction can create a partial owner.
    /// 🔒️ Reports an active reserved slot that a nested producer may settle through a child recipient.
    pub fn is_reserved(&self)->bool{self.reserved}
    pub fn reserve(&mut self)->Result<(),ValueError>{if self.reserved||self.owner.is_some(){return Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot"))}self.reserved=true;Ok(())}
    /// 🔓️ Settles the reserved slot with the owner its operation produced, if any.
    pub fn settle(&mut self,owner:Option<Box<dyn ErasedSnapshotRetirement>>){*self.owner=owner;self.reserved=false;}
    /// 🪑️ Runs one operation with the slot reserved and retains whatever original owner it returns.
    pub fn with_reserved_owner<T,E:From<ValueError>>(&mut self,operation:impl FnOnce()->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{self.reserve().map_err(E::from)?;let(result,owner)=operation();self.settle(owner);result}
}
impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.terminal_is_empty()&&grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"decoder return slot close requires admitted depth"));}
        if self.reserved{self.reserved=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        crate::close_factory_ticket(&mut self.owner,grant)
    }
    fn terminal_is_empty(&self)->bool{!self.reserved&&self.owner.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.reserved{return Ok(0)}self.owner.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.copy_bytes))}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.reserved{return Ok(0)}self.owner.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,copy)?.capacity_bytes))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.reserved{return Ok(0)}self.owner.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.release_bytes))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if self.reserved{return Ok(1)}self.owner.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.depth))}
}
impl Drop for NativeDecodeRetirementRecipient{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder recipient retains returned physical ownership");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);}}}}

#[path="♻️retirement/🦀️.rs"]
mod retirement;
