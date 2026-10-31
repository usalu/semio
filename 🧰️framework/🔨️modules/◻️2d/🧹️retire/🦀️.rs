//! 🧹️ Shared counters admit work before private geometry ownership changes.
#[derive(Clone,Copy,Debug)]
pub struct WorkRetirementProgress{pub phase:&'static str,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Default)]
pub struct WorkRetirementCounter{work:u64,done:bool}
impl WorkRetirementCounter{
 pub fn advance(&mut self,grant:usize,mut step:impl FnMut()->bool)->Result<WorkRetirementProgress,&'static str>{
  if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid retirement work grant");}
  for _ in 0..grant{if self.done{break;}self.done=step();self.work+=1;}Ok(WorkRetirementProgress{phase:if self.done{"complete"}else{"closing"},work:self.work,done:self.done})
 }
}

/// 🎮️ Retains typed ownership inline and closes only under the caller's explicit physical grant.
#[macro_export]
macro_rules! physical_work_retirement {
 ($retired:ident,$job:ty) => {
  pub struct $retired {#[doc(hidden)] pub owner:semio_framework_value::retirement::controlled::ControlledRetirement<$job>}
  impl $retired {
   pub fn new(job:$job)->Self {Self{owner:semio_framework_value::retirement::controlled::ControlledRetirement::retain(job)}}
   pub fn terminal_is_empty(&self)->bool {self.owner.terminal_is_empty()}
   pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.owner.step(grant)}
   pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand()}
   pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_capacity_byte_demand(copy)}
   pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_demand()}
   pub fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_depth_demand()}
  }
  semio_framework_value::artifact_retire_struct!($retired{owner});
 };
}
