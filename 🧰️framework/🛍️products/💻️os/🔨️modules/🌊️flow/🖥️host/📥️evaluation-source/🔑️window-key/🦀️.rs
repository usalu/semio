//! 🔑️ A window key hashes the original UTF-8 word in bounded blocks under its actual caller.
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::size_of;
use std::hash::{Hash,Hasher};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedWorkOutcome,retained_clone::retained_clone_grant_admits};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FlowEvalWindowKey(u64);
impl FlowEvalWindowKey{pub fn cold(window:&str)->Self{let mut hash=std::collections::hash_map::DefaultHasher::new();window.hash(&mut hash);Self(hash.finish())}pub fn fingerprint(self)->u64{self.0}}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:OriginalSourceBinding,pointer:usize,length:usize}
pub struct FlowEvalWindowKeyAdvance{pub key:Option<FlowEvalWindowKey>,pub progress:RetainedCloneProgress}
pub struct FlowEvalWindowKeyWork{hash:std::collections::hash_map::DefaultHasher,binding:Option<Binding>,offset:usize,phase:u8,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original window key changed its caller, text or retained output")}
impl FlowEvalWindowKeyWork{
 pub fn new()->Self{Self{hash:Default::default(),binding:None,offset:0,phase:0,closing:false}}
 fn binding(window:&str,cx:&StepContext<'_>)->Binding{Binding{source:OriginalSourceBinding::borrow(cx),pointer:window.as_ptr()as usize,length:window.len()}}
 pub fn next_demands(&self,window:&str)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>3{return Err(invalid())}if self.binding.is_some_and(|b|b.pointer!=window.as_ptr()as usize||b.length!=window.len()){return Err(invalid())}let bytes=2*size_of::<Self>();Ok(RetirementDemand{copy_bytes:bytes+if self.phase==1{(window.len()-self.offset).min(64)}else{0},depth:1,..Default::default()})}
 pub fn advance(&mut self,window:&str,cx:&StepContext<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<FlowEvalWindowKeyAdvance>{
  let empty=RetainedCloneProgress::default();let result=(||{let d=self.next_demands(window)?;if !retained_clone_grant_admits(g,d){return Ok(FlowEvalWindowKeyAdvance{key:None,progress:empty})}let binding=Self::binding(window,cx);if self.binding.is_some_and(|original|original!=binding){return Err(invalid())}let mut key=None;match self.phase{0=>{self.binding=Some(binding);self.phase=1},1=>{let end=(self.offset+64).min(window.len());self.hash.write(&window.as_bytes()[self.offset..end]);self.offset=end;if end==window.len(){self.phase=2}},2=>{self.hash.write_u8(0xff);self.phase=3},3=>{key=Some(FlowEvalWindowKey(self.hash.finish()));self.phase=4},_=>return Err(invalid())}Ok(FlowEvalWindowKeyAdvance{key,progress:RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..empty}})})();let work=result.as_ref().map_or(0,|step|u64::from(step.progress.copied_items!=0));RetainedWorkOutcome{result,work}
 }
 pub fn step(&mut self,window:&str,cx:&mut StepContext<'_>)->Result<Option<FlowEvalWindowKey>,OriginalTurnFailure>{for _ in 0..512{if cx.should_yield()||cx.is_cancelled(){return Ok(None)}let d=self.next_demands(window)?;if !cx.retained_work_can_enter(d){return Ok(None)}let turn=self.advance(window,cx,cx.retained_grant());cx.consume_fuel(turn.work);let advance=turn.result?;if let Err(error)=cx.consume_retained(advance.progress){if advance.key.is_some(){self.phase=3}return Err(error.into())}if advance.key.is_some(){return Ok(advance.key)}if advance.progress==Default::default(){break}}Ok(None)}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:usize::from(!self.terminal_is_empty()),..Default::default()}}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{use semio_framework_job::InteractiveJobCloseStep as Close;if !self.closing||self.binding.is_some_and(|binding|!binding.source.matches(cx)){return Err(invalid().into())}if self.terminal_is_empty(){return Ok(Close::Complete{progress:Default::default()})}let d=self.close_demands();if cx.fuel_remaining()==0||cx.retirement_should_yield()||!cx.retained_work_can_enter(d){return Ok(Close::Blocked)}self.hash=Default::default();self.binding=None;self.offset=0;self.phase=255;let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};cx.consume_fuel(1);cx.consume_retained(p)?;Ok(Close::Complete{progress:p})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.phase==255&&self.binding.is_none()}
}
impl Drop for FlowEvalWindowKeyWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original window key must close its original caller binding")}}
