//! 🧹️ Physically retires the original typed scene plan under admitted byte and item grants.
use crate::preparation::DocumentScenePlan;
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,ValueError,retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
#[derive(Clone,Copy,Debug)]
pub struct ScenePlanCloseProgress{pub phase:&'static str,pub owners:u64,pub work:u64,pub done:bool}
/// 🧺️ Keeps every original allocation until its physical release is funded.
pub struct ScenePlanCloseJob{owner:ControlledRetirement<DocumentScenePlan>}
impl ScenePlanCloseJob{
 pub fn new(plan:DocumentScenePlan)->Self{Self{owner:ControlledRetirement::retain(plan)}}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.owner.step(body,effect)}
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.next_demand(body)}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}

}
impl RetireOwned for ScenePlanCloseJob{
 type RetirementCursor=<ControlledRetirement<DocumentScenePlan> as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{self.owner.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.owner.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
