//! 🧯️ Original typed conflicts and causal envelopes keep all native fields through granted closure.
use crate::{Conflict,MutationEnvelope};
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneStep}};

struct OriginalProtocolOwner<T:RetireOwned>(ControlledRetirement<T>);
impl<T:RetireOwned> OriginalProtocolOwner<T> {
    fn new(original:T)->Self {Self(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("protocol family declares exact controlled retirement")))}
    fn original(&self)->Option<&T> {self.0.original()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.0.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.0.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.0.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.0.next_depth_demand()}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=self.0.step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.0.terminal_is_empty(),"original typed protocol owner")}
}
enum ProtocolOwner {Conflict(OriginalProtocolOwner<Conflict>),Envelope(OriginalProtocolOwner<MutationEnvelope>)}
/// 🔒️ Inline native owner retains the original fatal conflict or causal envelope until exact cleanup.
pub struct ProtocolConflictRetirement {owner:ProtocolOwner}
impl ProtocolConflictRetirement {
    pub fn new(original:Conflict)->Self {Self{owner:ProtocolOwner::Conflict(OriginalProtocolOwner::new(original))}}
    pub fn causal_envelope(original:MutationEnvelope)->Self {Self{owner:ProtocolOwner::Envelope(OriginalProtocolOwner::new(original))}}
}
impl ErasedSnapshotRetirement for ProtocolConflictRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match &mut self.owner{ProtocolOwner::Conflict(owner)=>owner.close_step(grant),ProtocolOwner::Envelope(owner)=>owner.close_step(grant)}}
    fn terminal_is_empty(&self)->bool{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.terminal_is_empty(),ProtocolOwner::Envelope(owner)=>owner.terminal_is_empty()}}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.next_copy_byte_demand(),ProtocolOwner::Envelope(owner)=>owner.next_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.next_capacity_byte_demand(copy),ProtocolOwner::Envelope(owner)=>owner.next_capacity_byte_demand(copy)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.next_release_byte_demand(),ProtocolOwner::Envelope(owner)=>owner.next_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.next_depth_demand(),ProtocolOwner::Envelope(owner)=>owner.next_depth_demand()}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
