//! 📊️ Clones schema-bounded original progress one leaf at a time under caller ownership grants.
use super::{ToolRunProgress,ToolRunCounter,ToolRunStep,ToolRunStepArg,ToolRunStepRing,TOOL_RUN_COUNTERS_MAX,TOOL_RUN_STEP_RING_CAPACITY,TOOL_RUN_STEP_ARGS_MAX};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource},retirement::{RetireOwned,RetirementCursor}};
use std::mem::{ManuallyDrop,size_of};

#[derive(semio_framework_value::RetireOwned)]
struct State<A:RetireOwned+Sync>{source:Option<RetainedCloneSource<ToolRunProgress,A>>,counters:Vec<ToolRunCounter>,steps:Vec<ToolRunStep>,step:Option<ToolRunStep>,output:Option<ToolRunProgress>,index:usize,arg:usize,phase:u8,cancelled:bool}
pub struct ToolRunProgressClone<A:RetireOwned+Sync=ToolRunProgress>{state:ManuallyDrop<State<A>>,transferred:bool}
impl<A:RetireOwned+Sync> ToolRunProgressClone<A>{
 pub fn new(source:RetainedCloneSource<ToolRunProgress,A>)->Self{Self{state:ManuallyDrop::new(State{source:Some(source),counters:Vec::new(),steps:Vec::new(),step:None,output:None,index:0,arg:0,phase:0,cancelled:false}),transferred:false}}
 pub fn original(&self)->&ToolRunProgress{self.state.source.as_ref().unwrap().borrow().get()}
 pub fn complete(&self)->bool{self.state.phase==6&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;}
 pub fn next_demands(&self)->Result<RetirementDemand,ValueError>{let state=&*self.state;if state.cancelled||state.phase>=6{return Ok(Default::default());}let source=self.original();if source.counters.len()>TOOL_RUN_COUNTERS_MAX||source.steps.len()>TOOL_RUN_STEP_RING_CAPACITY{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original progress exceeds its schema-bounded leaves"));}Ok(match state.phase{
  0=>RetirementDemand{capacity_bytes:source.counters.len()*size_of::<ToolRunCounter>(),depth:1,..Default::default()},
  1=>RetirementDemand{copy_bytes:if state.index<source.counters.len(){size_of::<ToolRunCounter>()}else{0},depth:1,..Default::default()},
  2=>RetirementDemand{capacity_bytes:source.steps.len()*size_of::<ToolRunStep>(),depth:1,..Default::default()},
  3=>{if let Some(step)=source.steps.steps.get(state.index){if step.args.len()>TOOL_RUN_STEP_ARGS_MAX{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original progress step arguments exceed the schema limit"));}RetirementDemand{copy_bytes:size_of::<ToolRunStep>(),capacity_bytes:step.args.len()*size_of::<ToolRunStepArg>(),depth:1,..Default::default()}}else{RetirementDemand{depth:1,..Default::default()}}},
  4=>RetirementDemand{copy_bytes:if state.arg<source.steps.steps[state.index].args.len(){size_of::<ToolRunStepArg>()}else{size_of::<ToolRunStep>()},depth:1,..Default::default()},
  5=>RetirementDemand{copy_bytes:size_of::<ToolRunProgress>(),depth:1,..Default::default()},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original progress clone phase is invalid")),
 })}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.state.cancelled||self.state.phase>=6||grant.maximum_items==0{return Ok(Default::default());}let quote=self.next_demands()?;if grant.maximum_copy_bytes<quote.copy_bytes||grant.maximum_capacity_bytes<quote.capacity_bytes||grant.maximum_release_bytes<quote.release_bytes||grant.maximum_depth<quote.depth{return Ok(Default::default());}
  let state=&mut *self.state;let source=state.source.as_ref().unwrap().borrow().get();let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};match state.phase{
   0=>{state.counters.try_reserve_exact(source.counters.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"progress counter backing allocation failed"))?;receipt.retained_capacity_bytes=state.counters.capacity()*size_of::<ToolRunCounter>();state.phase=1;},
   1=>{if state.index==source.counters.len(){state.index=0;state.phase=2;}else{state.counters.push(source.counters[state.index]);state.index+=1;receipt.copied_bytes=size_of::<ToolRunCounter>();}},
   2=>{state.steps.try_reserve_exact(source.steps.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"progress step backing allocation failed"))?;receipt.retained_capacity_bytes=state.steps.capacity()*size_of::<ToolRunStep>();state.phase=3;},
   3=>{if state.index==source.steps.len(){state.phase=5;}else{let original=&source.steps.steps[state.index];let mut args=Vec::new();args.try_reserve_exact(original.args.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"progress argument backing allocation failed"))?;receipt.retained_capacity_bytes=args.capacity()*size_of::<ToolRunStepArg>();state.step=Some(ToolRunStep{sequence:original.sequence,kind:original.kind,stage:original.stage,reason:original.reason,subject:original.subject,repeat:original.repeat,args});state.arg=0;state.phase=4;receipt.copied_bytes=size_of::<ToolRunStep>();}},
   4=>{let original=&source.steps.steps[state.index];if state.arg<original.args.len(){state.step.as_mut().unwrap().args.push(original.args[state.arg]);state.arg+=1;receipt.copied_bytes=size_of::<ToolRunStepArg>();}else{state.steps.push(state.step.take().unwrap());state.index+=1;state.phase=3;receipt.copied_bytes=size_of::<ToolRunStep>();}},
   5=>{state.output=Some(ToolRunProgress{identity:source.identity,sequence:source.sequence,state:source.state,stage:source.stage,completed:source.completed,total:source.total,counters:std::mem::take(&mut state.counters),units_per_second:source.units_per_second,conflicts:source.conflicts,steps:ToolRunStepRing{steps:std::mem::take(&mut state.steps).into()}});state.phase=6;receipt.copied_bytes=size_of::<ToolRunProgress>();},
   _=>unreachable!(),
  }Ok(receipt)
 }
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(ToolRunProgress,RetainedCloneProgress)>,ValueError>{if !self.complete()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ToolRunProgress>()||grant.maximum_depth==0{return Ok(None);}let output=self.state.output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"completed progress clone lost its private output"))?;self.state.phase=7;Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolRunProgress>(),..Default::default()})))}
}
impl<A:RetireOwned+Sync> RetireOwned for ToolRunProgressClone<A>{fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl<A:RetireOwned+Sync> Drop for ToolRunProgressClone<A>{fn drop(&mut self){if !self.transferred&&!std::thread::panicking(){panic!("original progress clone requires controlled retirement")}}}
