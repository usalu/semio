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

/// 🎟️ Advances physical work only with explicitly assembled demand grants.
pub fn advance_physical_work<T:semio_framework_value::retirement::RetireOwned>(owner:&mut semio_framework_value::retirement::controlled::ControlledRetirement<T>,work:&mut u64,items:usize)->Result<WorkRetirementProgress,semio_framework_value::ValueError>{
 if items==0||items as u128>9_007_199_254_740_991{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"Retirement work grant must be positive"));}
 for _ in 0..items{if owner.terminal_is_empty(){break;}let minimum=owner.next_copy_byte_demand()?;let copy=if minimum==0{0}else{minimum.max(4096)};let release=owner.next_release_byte_demand()?;let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release})?,maximum_release_bytes:release,maximum_depth:owner.next_depth_demand()?};let step=owner.step(grant)?;let(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress))=step;*work+=progress.copied_items as u64;if progress==Default::default(){break;}}
 Ok(WorkRetirementProgress{phase:if owner.terminal_is_empty(){"complete"}else{"closing"},work:*work,done:owner.terminal_is_empty()})
}

/// 🎮️ Binds a physical typed owner to explicitly admitted close grants.
#[macro_export]
macro_rules! physical_work_retirement {
 ($retired:ident,$job:ty,$error:ty,$invalid:expr) => {
  pub struct $retired {owner:semio_framework_value::retirement::controlled::ControlledRetirement<$job>,work:u64}
  impl $retired {
   pub fn new(job:$job)->Self {Self{owner:semio_framework_value::retirement::controlled::ControlledRetirement::new(job).unwrap_or_else(|(error,_)|panic!("physical raster owner refused: {error}")),work:0}}
   pub fn terminal_is_empty(&self)->bool {self.owner.terminal_is_empty()}
   pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{self.owner.step(grant)}
   pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand()}
   pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_capacity_byte_demand(copy)}
   pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_demand()}
   pub fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_depth_demand()}
   pub fn advance(&mut self,items:usize)->Result<$crate::retirement::WorkRetirementProgress,$error>{
    if items==0||items as u128>9_007_199_254_740_991{return Err(($invalid)("Retirement work grant must be positive"));}
    for _ in 0..items {if self.terminal_is_empty(){break;}let minimum=self.next_copy_byte_demand().map_err(|error|($invalid)(&error.to_string()))?;let copy=if minimum==0{0}else{minimum.max(4096)};
     let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(if copy>0{copy}else{self.next_release_byte_demand().map_err(|error|($invalid)(&error.to_string()))?}).map_err(|error|($invalid)(&error.to_string()))?,maximum_release_bytes:self.next_release_byte_demand().map_err(|error|($invalid)(&error.to_string()))?,maximum_depth:self.next_depth_demand().map_err(|error|($invalid)(&error.to_string()))?};
     let step=self.close_step(grant).map_err(|error|($invalid)(&error.to_string()))?;let progress=match step{semio_framework_value::retained_clone::RetainedCloneStep::Progress(p)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(p)=>p};self.work+=progress.copied_items as u64;if progress==Default::default(){break;}
    }
    Ok($crate::retirement::WorkRetirementProgress{phase:if self.terminal_is_empty(){"complete"}else{"closing"},work:self.work,done:self.terminal_is_empty()})
   }
  }
  semio_framework_value::artifact_retire_struct!($retired{owner,work});
 };
}
