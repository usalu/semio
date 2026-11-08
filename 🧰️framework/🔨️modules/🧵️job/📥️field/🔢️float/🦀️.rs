//! 🔢️ The initiating caller lends its actual field to the single bounded native float cursor.
use crate::{OriginalTurnFailure,StepContext};
use semio_framework_value::{OriginalFloatWord,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original numeric field changed its initiating operation")}
/// 🧮️ Value owns interpretation; this owner keeps the real original context and field binding.
pub struct OriginalFloatFieldWork{raw:OriginalFloatWord,origin:Option<(u64,u64,usize,usize)>,closing:bool,progress:RetainedCloneProgress,work:u128}
impl OriginalFloatFieldWork{
 pub fn new()->Self{Self{raw:OriginalFloatWord::new(),origin:None,closing:false,progress:Default::default(),work:0}}
 pub fn next_demands(&self,source:&str,copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(copy)}Ok(if self.origin.is_none(){RetirementDemand{copy_bytes:size_of::<Option<(u64,u64,usize,usize)>>(),depth:1,..Default::default()}}else{self.raw.next_demands(source)})}
 fn record(&mut self,p:RetainedCloneProgress,work:u128){self.progress=self.progress.checked_add(p).expect("bounded original float turn progress");self.work+=work}
 fn once(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<Option<Option<f64>>,OriginalTurnFailure>{
  let origin=(cx.operation().0,cx.generation().0,source.as_ptr()as usize,source.len());if self.closing||self.origin.is_some_and(|o|o!=origin){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}
  if self.origin.is_none(){self.origin=Some(origin);let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};self.record(p,1);cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(None)}
  if cx.fuel_remaining()<self.raw.next_work_demand(){return Ok(None)}
  let turn=self.raw.advance(source,cx.retained_grant());cx.consume_work(turn.work);match turn.result{Ok(output)=>{let p=self.raw.step_progress();self.record(p,turn.work);if let Err(error)=cx.consume_retained(p){if let Some(output)=output{self.raw.restore_output(output)}return Err(error.into())}Ok(output)},Err(error)=>{self.record(error.retained_progress(),turn.work);Err(OriginalTurnFailure::debit_original(error,cx))}}
 }
 pub fn step(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<Option<Option<f64>>,OriginalTurnFailure>{self.progress=Default::default();self.work=0;for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.once(source,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(None)}
 pub fn bytes_read(&self)->usize{self.raw.offset()}
 #[cfg(any(test,feature="testkit"))]
 pub fn test_phase(&self)->u8{self.raw.phase()}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u128{self.work}
 pub fn restore_output(&mut self,output:Option<f64>){self.raw.restore_output(output)}
 pub fn begin_close(&mut self){self.closing=true;self.raw.begin_close()}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.raw.terminal_is_empty(){return self.raw.close_demands(copy)}Ok(if self.origin.is_some(){RetirementDemand{copy_bytes:size_of::<Option<(u64,u64,usize,usize)>>(),depth:1,..Default::default()}}else{Default::default()})}
 pub fn close_step(&mut self,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let g=effect.grant();self.progress=Default::default();self.work=0;if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.raw.terminal_is_empty(){let turn=self.raw.close_step(effect);self.work=turn.work;self.progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};return turn}if self.origin.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let d=RetirementDemand{copy_bytes:size_of::<Option<(u64,u64,usize,usize)>>(),depth:1,..Default::default()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.origin=None;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};self.work=1;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:1}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.origin.is_none()&&self.raw.terminal_is_empty()}
}
impl Drop for OriginalFloatFieldWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original numeric field read must close actual source and output")}}
#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;
