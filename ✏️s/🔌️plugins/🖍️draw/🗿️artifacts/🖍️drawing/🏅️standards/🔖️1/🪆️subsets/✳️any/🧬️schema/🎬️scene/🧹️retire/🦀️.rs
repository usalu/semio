//! 🧹️ Physically retires the original typed scene plan under admitted byte and item grants.
use crate::schema::scene_preparation::{DocumentScenePlan,DocumentSceneError};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
#[derive(Clone,Copy,Debug)]
pub struct ScenePlanCloseProgress{pub phase:&'static str,pub owners:u64,pub work:u64,pub done:bool}
/// 🧺️ Keeps every original allocation until its physical release is funded.
pub struct ScenePlanCloseJob{owner:ControlledRetirement<DocumentScenePlan>,work:u64}
impl ScenePlanCloseJob{
 pub fn new(plan:DocumentScenePlan)->Self{Self{owner:ControlledRetirement::new(plan).unwrap_or_else(|(error,_)|panic!("scene plan retirement authority refused: {error}")),work:0}}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=self.owner.step(grant)?;self.work+=step.progress().copied_items as u64;Ok(step)}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 pub fn advance(&mut self,items:usize)->Result<ScenePlanCloseProgress,DocumentSceneError>{
  let invalid=|error:ValueError|DocumentSceneError::Invalid(error.to_string());
  if items==0||items as u128>9_007_199_254_740_991{return Err(DocumentSceneError::Invalid("Invalid scene retirement work grant".into()));}
  for _ in 0..items{
   if self.terminal_is_empty(){break;}
   let copy=self.next_copy_byte_demand().map_err(invalid)?;let release=self.next_release_byte_demand().map_err(invalid)?;
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(if copy==0{release}else{copy}).map_err(invalid)?,maximum_release_bytes:release,maximum_depth:self.next_depth_demand().map_err(invalid)?};
   if self.close_step(grant).map_err(invalid)?.progress()==Default::default(){break;}
  }
  let done=self.terminal_is_empty();Ok(ScenePlanCloseProgress{phase:if done{"complete"}else{"closing"},owners:self.work,work:self.work,done})
 }
}
impl RetireOwned for ScenePlanCloseJob{
 fn retirement(self)->Box<dyn RetirementCursor>{self.owner.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.owner.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
