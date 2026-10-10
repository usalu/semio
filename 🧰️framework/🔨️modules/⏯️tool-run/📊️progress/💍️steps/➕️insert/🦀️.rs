//! 💍️ Owns original ring insertion, displaced leaves and independently funded comparison and physical storage.
use super::{ToolRunStepRing,ToolRunStep,ToolRunStepArg,TOOL_RUN_STEP_RING_CAPACITY,TOOL_RUN_STEP_ARGS_MAX};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,ordered_map::{BoundedOrdGrant,BoundedOrdProgress}},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
use std::mem::{ManuallyDrop,size_of};

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunStepRingInsertGrant{pub retirement:RetainedCloneGrant,pub comparison:BoundedOrdGrant,pub maximum_moved_items:usize,pub maximum_moved_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunStepRingInsertDemand{pub retirement:RetirementDemand,pub compared_items:usize,pub compared_bytes:usize,pub moved_items:usize,pub moved_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunStepRingInsertProgress{pub retirement:RetainedCloneProgress,pub comparison:BoundedOrdProgress,pub moved_items:usize,pub moved_bytes:usize}
impl ToolRunStepRingInsertProgress{pub fn fits(self,grant:ToolRunStepRingInsertGrant)->bool{self.retirement.fits(grant.retirement)&&self.comparison.fits(grant.comparison)&&self.moved_items<=grant.maximum_moved_items&&self.moved_bytes<=grant.maximum_moved_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct State{ring:Option<ToolRunStepRing>,input:Option<ToolRunStep>,displaced:Option<ToolRunStep>,close:Option<ControlledRetirement<ToolRunStep>>,index:usize,phase:u8,equal:bool,cancelled:bool}
pub struct ToolRunStepRingInsert{state:ManuallyDrop<State>,transferred:bool}
impl ToolRunStepRingInsert{
 pub fn new(ring:ToolRunStepRing,input:ToolRunStep)->Self{Self{state:ManuallyDrop::new(State{ring:Some(ring),input:Some(input),displaced:None,close:None,index:0,phase:0,equal:true,cancelled:false}),transferred:false}}
 pub fn complete(&self)->bool{self.state.phase==6&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;}
 pub fn next_demands(&self)->Result<ToolRunStepRingInsertDemand,ValueError>{
  let state=&*self.state;if state.cancelled||state.phase>=6{return Ok(Default::default());}let ring=state.ring.as_ref().unwrap();if ring.len()>TOOL_RUN_STEP_RING_CAPACITY||state.input.as_ref().is_some_and(|step|step.args.len()>TOOL_RUN_STEP_ARGS_MAX){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original step ring exceeds schema bounds"));}
  let mut demand=ToolRunStepRingInsertDemand{retirement:RetirementDemand{depth:1,..Default::default()},..Default::default()};match state.phase{
   0=>if ring.newest().is_some(){demand.compared_items=1;demand.compared_bytes=match state.index{0=>size_of::<super::ToolRunStepKind>(),1|2=>size_of::<u16>(),3=>size_of::<Option<u64>>(),_=>size_of::<usize>()};},
   1=>if state.index<state.input.as_ref().unwrap().args.len(){demand.compared_items=1;demand.compared_bytes=size_of::<ToolRunStepArg>();},
   2=>demand.retirement.copy_bytes=if state.equal{size_of::<ToolRunStep>()+size_of::<u64>()+size_of::<u32>()}else if ring.len()==TOOL_RUN_STEP_RING_CAPACITY{size_of::<ToolRunStep>()}else{0},
   3=>if let Some(close)=state.close.as_ref(){if !close.terminal_is_empty(){let copy=close.next_copy_byte_demand()?;demand.retirement=RetirementDemand{copy_bytes:copy,capacity_bytes:close.next_capacity_byte_demand(copy)?,release_bytes:close.next_release_byte_demand()?,depth:close.next_depth_demand()?};}}else{demand.retirement.copy_bytes=size_of::<ToolRunStep>();},
   4=>if ring.steps.len()==ring.steps.capacity(){demand.retirement.capacity_bytes=(ring.len()+1)*size_of::<ToolRunStep>();demand.retirement.release_bytes=ring.steps.capacity()*size_of::<ToolRunStep>();demand.moved_items=ring.len();demand.moved_bytes=ring.len()*size_of::<ToolRunStep>();},
   5=>demand.retirement.copy_bytes=size_of::<ToolRunStep>(),
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"step ring insertion phase is invalid")),
  }Ok(demand)
 }
 pub fn advance(&mut self,grant:ToolRunStepRingInsertGrant)->Result<ToolRunStepRingInsertProgress,ValueError>{
  if self.state.cancelled||self.state.phase>=6||grant.retirement.maximum_items==0{return Ok(Default::default());}let demand=self.next_demands()?;let quote=demand.retirement;if quote.copy_bytes>grant.retirement.maximum_copy_bytes||quote.capacity_bytes>grant.retirement.maximum_capacity_bytes||quote.release_bytes>grant.retirement.maximum_release_bytes||quote.depth>grant.retirement.maximum_depth||demand.compared_items>grant.comparison.maximum_items||demand.compared_bytes>grant.comparison.maximum_bytes||demand.moved_items>grant.maximum_moved_items||demand.moved_bytes>grant.maximum_moved_bytes{return Ok(Default::default());}
  let state=&mut *self.state;let ring=state.ring.as_mut().unwrap();let mut receipt=ToolRunStepRingInsertProgress{retirement:RetainedCloneProgress{copied_items:1,copied_bytes:quote.copy_bytes,..Default::default()},..Default::default()};match state.phase{
   0=>if let Some(newest)=ring.newest(){let input=state.input.as_ref().unwrap();state.equal&=match state.index{0=>newest.kind==input.kind,1=>newest.stage==input.stage,2=>newest.reason==input.reason,3=>newest.subject==input.subject,_=>newest.args.len()==input.args.len()};state.index+=1;receipt.comparison=BoundedOrdProgress{compared_items:demand.compared_items,compared_bytes:demand.compared_bytes};if state.index==5{state.index=0;state.phase=if state.equal{1}else{2};}}else{state.equal=false;state.phase=4;},
   1=>{let input=state.input.as_ref().unwrap();if state.index==input.args.len(){state.phase=2;}else{state.equal=ring.newest().unwrap().args[state.index]==input.args[state.index];state.index+=1;receipt.comparison=BoundedOrdProgress{compared_items:demand.compared_items,compared_bytes:demand.compared_bytes};if !state.equal{state.phase=2;}}},
   2=>if state.equal{let input=state.input.take().unwrap();let newest=ring.steps.back_mut().unwrap();newest.sequence=input.sequence;newest.repeat=newest.repeat.saturating_add(input.repeat);state.displaced=Some(input);state.phase=3;}else if ring.len()==TOOL_RUN_STEP_RING_CAPACITY{state.displaced=ring.steps.pop_front();state.phase=3;}else{state.phase=4;},
   3=>if let Some(close)=state.close.as_mut(){if close.terminal_is_empty(){state.close=None;state.phase=if state.input.is_some(){4}else{6};}else{return Ok(ToolRunStepRingInsertProgress{retirement:close.step(grant.retirement)?.progress(),..Default::default()});}}else{let original=state.displaced.take().unwrap();match ControlledRetirement::new(original){Ok(close)=>state.close=Some(close),Err((error,original))=>{state.displaced=Some(original);return Err(error);}}},
   4=>{if ring.steps.len()==ring.steps.capacity(){let old=ring.steps.capacity()*size_of::<ToolRunStep>();ring.steps.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"step ring original backing growth failed"))?;receipt.retirement.retained_capacity_bytes=ring.steps.capacity()*size_of::<ToolRunStep>();receipt.retirement.released_bytes=old;receipt.moved_items=demand.moved_items;receipt.moved_bytes=demand.moved_bytes;}state.phase=5;},
   5=>{ring.steps.push_back(state.input.take().unwrap());state.phase=6;},
   _=>unreachable!(),
  }Ok(receipt)
 }
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(ToolRunStepRing,RetainedCloneProgress)>,ValueError>{if !self.complete()||self.state.ring.is_none()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ToolRunStepRing>()||grant.maximum_depth==0{return Ok(None);}self.state.phase=7;Ok(Some((self.state.ring.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolRunStepRing>(),..Default::default()})))}
}
impl RetireOwned for ToolRunStepRingInsert{fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl Drop for ToolRunStepRingInsert{fn drop(&mut self){if !self.transferred&&!std::thread::panicking(){panic!("original step ring insertion requires controlled retirement")}}}
