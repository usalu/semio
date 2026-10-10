//! 🎟️ Caller-funded retained work has independent currencies and exact cumulative receipts.
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},RetirementDemand,ValueError,ValueRefusalKind};
pub const NO_RETAINED_WORK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct RetainedWorkBudget{items:usize,copy:usize,capacity:usize,release:usize,depth:usize,used_items:usize,used_copy:usize,used_capacity:usize,used_release:usize}
impl RetainedWorkBudget{
 pub const fn new(grant:RetainedCloneGrant)->Self{Self{items:grant.maximum_items,copy:grant.maximum_copy_bytes,capacity:grant.maximum_capacity_bytes,release:grant.maximum_release_bytes,depth:grant.maximum_depth,used_items:0,used_copy:0,used_capacity:0,used_release:0}}
 pub const fn remaining(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.items-self.used_items,maximum_copy_bytes:self.copy-self.used_copy,maximum_capacity_bytes:self.capacity-self.used_capacity,maximum_release_bytes:self.release-self.used_release,maximum_depth:self.depth}}
 pub fn can_enter(&self,demand:RetirementDemand)->bool{let grant=self.remaining();grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn charge(&mut self,receipt:RetainedCloneProgress)->Result<(),ValueError>{if !receipt.fits(self.remaining()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained work receipt exceeds original funding"));}self.used_items+=receipt.copied_items;self.used_copy+=receipt.copied_bytes;self.used_capacity+=receipt.retained_capacity_bytes;self.used_release+=receipt.released_bytes;Ok(())}
 pub const fn receipt(&self)->RetainedCloneProgress{RetainedCloneProgress{copied_items:self.used_items,copied_bytes:self.used_copy,retained_capacity_bytes:self.used_capacity,released_bytes:self.used_release}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
