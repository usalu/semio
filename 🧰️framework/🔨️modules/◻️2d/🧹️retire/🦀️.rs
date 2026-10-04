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
