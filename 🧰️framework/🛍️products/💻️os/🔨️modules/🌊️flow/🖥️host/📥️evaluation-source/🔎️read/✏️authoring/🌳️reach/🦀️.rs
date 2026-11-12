//! 🌳️ Original raw-edge reachability keeps a funded numeric stack and edge visitation roster.
use super::compare::TextCompare;
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowSourceIdentity};
use semio_framework_job::{OriginalConstructionDemand,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::{RetainedAllocationEffect,OriginalRetirementPoll,RetirementPollExit};
use semio_os_flow_host_source::{close_with_effect,close_with_poll,poll_outcome};
use super::storage::Construction;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original edge reachability changed its borrowed source")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{visited:Vec<u8>,stack:Vec<usize>}
pub(super) struct ReachWork{construction:Construction,fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,compare:TextCompare,matched:Option<bool>,binding:Option<(FlowSourceIdentity,usize,usize,usize,usize,bool)>,origin:Option<(u64,u64)>,phase:u8,scan:usize,current:usize,output:Option<bool>,closing:bool}
impl ReachWork{
 pub(super) fn new()->Self{Self{construction:Default::default(),fields:ManuallyDrop::new(Fields{visited:Vec::new(),stack:Vec::new()}),retirement:Default::default(),compare:TextCompare::new(),matched:None,binding:None,origin:None,phase:0,scan:0,current:usize::MAX,output:None,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 fn query(&mut self,left:&str,right:&str,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{if self.matched.is_none(){self.matched=self.compare.step(left,right,cx)?;return Ok(None)}if self.compare.reset(cx)?{return Ok(self.matched.take())}Ok(None)}
 pub(super) fn next_demands(&self,source:&dyn FlowSnapshotSource,copy:usize)->Result<RetirementDemand,ValueError>{let _=copy;Ok(RetirementDemand{capacity_bytes:match self.phase{1=>source.synapses().len(),3=>(source.synapses().len()+1).checked_mul(size_of::<usize>()).ok_or_else(invalid)?,_=>0},..fixed(512)})}
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,from:&str,to:&str,exclude_pair:bool,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}let binding=(source.source_identity(),from.as_ptr()as usize,from.len(),to.as_ptr()as usize,to.len(),exclude_pair);let origin=(cx.operation().0,cx.generation().0);if self.binding.is_some_and(|b|b!=binding)||self.origin.is_some_and(|o|o!=origin){return Err(invalid().into())}
  if let Some(output)=self.output{return Ok(Some(output))}match self.phase{
   0=>{if self.binding.is_none(){if Self::paid(cx,size_of::<Self>())?{self.binding=Some(binding);self.origin=Some(origin)}return Ok(None)}if let Some(same)=self.query(from,to,cx)?{if same{self.output=Some(true)}else{self.phase=1}}},
   1|3=>{let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;self.construction.run(cx,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},||(||{let actual=if self.phase==1{self.fields.visited.try_reserve_exact(source.synapses().len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original reachability visited roster refused"))?;self.fields.visited.capacity()}else{self.fields.stack.try_reserve_exact(source.synapses().len()+1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original reachability stack refused"))?;self.fields.stack.capacity()*size_of::<usize>()};self.phase+=1;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})())?;},
   2=>if Self::paid(cx,1+size_of::<usize>())?{if self.fields.visited.len()==source.synapses().len(){self.phase=3}else{self.fields.visited.push(0)}},
   4=>if Self::paid(cx,size_of::<usize>())?{self.fields.stack.push(usize::MAX);self.phase=5},
   5=>if Self::paid(cx,3*size_of::<usize>())?{if let Some(vertex)=self.fields.stack.pop(){self.current=vertex;self.scan=0;self.phase=6}else{self.output=Some(false)}},
   6=>{if self.scan==source.synapses().len(){if Self::paid(cx,1)?{self.phase=5}return Ok(None)}if self.fields.visited[self.scan]!=0{if Self::paid(cx,size_of::<usize>())?{self.scan+=1}return Ok(None)}if Self::paid(cx,1)?{self.phase=if exclude_pair{7}else{9}}},
   7=>{let edge=source.synapses().get(self.scan).ok_or_else(invalid)?;if let Some(equal)=self.query(&edge.from,from,cx)?{if equal{self.phase=8}else{self.phase=9}}},
   8=>{let edge=source.synapses().get(self.scan).ok_or_else(invalid)?;if let Some(equal)=self.query(&edge.to,to,cx)?{if equal{self.scan+=1;self.phase=6}else{self.phase=9}}},
   9=>{let edge=source.synapses().get(self.scan).ok_or_else(invalid)?;let vertex=if self.current==usize::MAX{to}else{&source.synapses().get(self.current).ok_or_else(invalid)?.to};if let Some(equal)=self.query(&edge.from,vertex,cx)?{if equal{self.phase=10}else{self.scan+=1;self.phase=6}}},
   10=>{let edge=source.synapses().get(self.scan).ok_or_else(invalid)?;if let Some(equal)=self.query(&edge.to,from,cx)?{if equal{self.output=Some(true)}else{self.phase=11}}},
   11=>if Self::paid(cx,2*size_of::<usize>()+1)?{self.fields.visited[self.scan]=1;self.fields.stack.push(self.scan);self.scan+=1;self.phase=6},
   _=>return Err(invalid().into()),
  }Ok(self.output)
 }
 pub(super) fn begin_close(&mut self){self.closing=true}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.visited.capacity()!=0{return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,copy)}if self.fields.stack.capacity()!=0{return self.retirement.next_owner_demands::<Vec<usize>>(true,&None,copy)}Ok(if self.binding.is_some()||self.origin.is_some(){fixed(size_of::<Self>())}else{Default::default()})}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}
  macro_rules! vector{($field:ident)=>{if self.fields.$field.capacity()!=0{let mut owner=Some(std::mem::take(&mut self.fields.$field));let turn=self.retirement.begin_granted(&mut owner,effect);if let Some(owner)=owner{self.fields.$field=owner}return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  vector!(visited);vector!(stack);if self.binding.is_some()||self.origin.is_some(){let d=fixed(size_of::<Self>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;self.origin=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.visited.capacity()==0&&self.fields.stack.capacity()==0&&self.retirement.is_empty()&&self.binding.is_none()&&self.origin.is_none()}
}
impl Drop for ReachWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original reachability requires terminal stack closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
