//! 🧹️ Physically retires the original typed scene plan under admitted byte and item grants.
use crate::preparation::DocumentScenePlan;
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
#[derive(Clone,Copy,Debug)]
pub struct ScenePlanCloseProgress{pub phase:&'static str,pub owners:u64,pub work:u64,pub done:bool}
/// 🧺️ Keeps every original allocation until its physical release is funded.
pub struct ScenePlanCloseJob{owner:ControlledRetirement<DocumentScenePlan>,work:u64}
impl ScenePlanCloseJob{
 pub fn new(plan:DocumentScenePlan)->Self{Self{owner:ControlledRetirement::retain(plan),work:0}}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=self.owner.step(grant)?;self.work+=step.progress().copied_items as u64;Ok(step)}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}

}
impl RetireOwned for ScenePlanCloseJob{
 fn retirement(self)->Box<dyn RetirementCursor>{self.owner.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.owner.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
