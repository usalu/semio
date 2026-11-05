//! 🧵️ A Flow authoring command retains its original source binding, domain output and exact child closure.
use super::{FlowAuthoringRequest,FlowAuthoringOutcome,FlowAuthoringSerials};
use super::{remove::RemoveWork,add::AddWork,insert::InsertWork,connect::ConnectWork,reorganize::ReorganizeWork};
use super::super::*;
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowSourceIdentity,FlowMutation};
use semio_framework_job::{OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
use super::storage::{storage_demands,storage_ready,storage_close_demands,storage_close_step};
use semio_framework_value_native_storage::NativeOwnedStorage;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Flow authoring changed its original source, catalogue, request or operation")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
enum Child{Remove(NativeOwnedStorage<RemoveWork>),Add(NativeOwnedStorage<AddWork>),Insert(NativeOwnedStorage<InsertWork>),Connect(NativeOwnedStorage<ConnectWork>),Reorganize(NativeOwnedStorage<ReorganizeWork>)}
impl Child{
 fn new(request:FlowAuthoringRequest<'_>)->Self{match request{FlowAuthoringRequest::RemoveWidget{..}|FlowAuthoringRequest::Disconnect{..}=>Self::Remove(NativeOwnedStorage::empty()),FlowAuthoringRequest::AddWidget{..}=>Self::Add(NativeOwnedStorage::empty()),FlowAuthoringRequest::InsertPort{..}=>Self::Insert(NativeOwnedStorage::empty()),FlowAuthoringRequest::ConnectPorts{..}=>Self::Connect(NativeOwnedStorage::empty()),FlowAuthoringRequest::Reorganize{..}=>Self::Reorganize(NativeOwnedStorage::empty())}}
 fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>,copy:usize)->Result<RetirementDemand,ValueError>{match(self,request){
  (Self::Remove(slot),FlowAuthoringRequest::RemoveWidget{id}|FlowAuthoringRequest::Disconnect{id})=>storage_demands(slot,|w|w.next_demands(source,id,copy)),
  (Self::Add(slot),FlowAuthoringRequest::AddWidget{descriptor,..})=>storage_demands(slot,|w|w.next_demands(descriptor,infos,copy)),
  (Self::Insert(slot),FlowAuthoringRequest::InsertPort{id,input,..})=>storage_demands(slot,|w|w.next_demands(source,infos,id,input,copy)),
  (Self::Connect(slot),FlowAuthoringRequest::ConnectPorts{from,from_port,to,to_port})=>storage_demands(slot,|w|w.next_demands(source,infos,from,from_port,to,to_port,copy)),
  (Self::Reorganize(slot),FlowAuthoringRequest::Reorganize{options})=>storage_demands(slot,|w|w.next_demands(source,infos,options,copy)),
  _=>Err(invalid()),
 }}
 fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>,serials:&mut FlowAuthoringSerials,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{match(self,request){
  (Self::Remove(slot),FlowAuthoringRequest::RemoveWidget{id})=>{if !storage_ready(slot,cx,||RemoveWork::new(true))?{return Ok(None)}slot.get_mut().ok_or_else(invalid)?.step(source,id,cx)},
  (Self::Remove(slot),FlowAuthoringRequest::Disconnect{id})=>{if !storage_ready(slot,cx,||RemoveWork::new(false))?{return Ok(None)}slot.get_mut().ok_or_else(invalid)?.step(source,id,cx)},
  (Self::Add(slot),FlowAuthoringRequest::AddWidget{descriptor,x,y})=>{if !storage_ready(slot,cx,AddWork::new)?{return Ok(None)}let work=slot.get_mut().ok_or_else(invalid)?;let result=work.step(source,infos,descriptor,x,y,cx)?;if let Some(serial)=work.serial(){serials.widget=serial}Ok(result)},
  (Self::Insert(slot),FlowAuthoringRequest::InsertPort{id,input,index})=>{if !storage_ready(slot,cx,InsertWork::new)?{return Ok(None)}slot.get_mut().ok_or_else(invalid)?.step(source,infos,id,input,index,cx)},
  (Self::Connect(slot),FlowAuthoringRequest::ConnectPorts{from,from_port,to,to_port})=>{if !storage_ready(slot,cx,ConnectWork::new)?{return Ok(None)}slot.get_mut().ok_or_else(invalid)?.step(source,infos,&mut serials.synapse,from,from_port,to,to_port,cx)},
  (Self::Reorganize(slot),FlowAuthoringRequest::Reorganize{options})=>{if !storage_ready(slot,cx,ReorganizeWork::new)?{return Ok(None)}slot.get_mut().ok_or_else(invalid)?.step(source,infos,options,cx)},
  _=>Err(invalid().into()),
 }}
 fn begin_close(&mut self){macro_rules! close{($slot:ident)=>{if let Some(work)=$slot.get_mut(){work.begin_close()}}}match self{Self::Remove(s)=>close!(s),Self::Add(s)=>close!(s),Self::Insert(s)=>close!(s),Self::Connect(s)=>close!(s),Self::Reorganize(s)=>close!(s)}}
 fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{macro_rules! quote{($slot:ident)=>{storage_close_demands($slot,|work|work.close_demands(copy),|work|work.terminal_is_empty())}}match self{Self::Remove(s)=>quote!(s),Self::Add(s)=>quote!(s),Self::Insert(s)=>quote!(s),Self::Connect(s)=>quote!(s),Self::Reorganize(s)=>quote!(s)}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{macro_rules! close{($slot:ident)=>{storage_close_step($slot,grant,|work,g|work.close_step(g),|work|work.terminal_is_empty())}}match self{Self::Remove(s)=>close!(s),Self::Add(s)=>close!(s),Self::Insert(s)=>close!(s),Self::Connect(s)=>close!(s),Self::Reorganize(s)=>close!(s)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Remove(s)=>s.terminal_is_empty(),Self::Add(s)=>s.terminal_is_empty(),Self::Insert(s)=>s.terminal_is_empty(),Self::Connect(s)=>s.terminal_is_empty(),Self::Reorganize(s)=>s.terminal_is_empty()}}
}
/// 🎮️ All six operations share one original admission and preserve their complete output through close.
pub struct FlowAuthoringWork{child:Option<Child>,result:ManuallyDrop<Option<Result<Vec<FlowMutation>,FlowCoreError>>>,output:ManuallyDrop<Option<FlowAuthoringOutcome>>,retirement:RetainedCloneClose,serials:FlowAuthoringSerials,binding:Option<(FlowSourceIdentity,usize,[u64;20])>,origin:Option<(u64,u64)>,phase:u8,closing:bool,progress:RetainedCloneProgress,last_work:u64}
impl FlowAuthoringWork{
 /// 🌱️ Retains only inline state until the original caller admits the first event.
 pub fn new(serials:FlowAuthoringSerials)->Self{Self{child:None,result:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),serials,binding:None,origin:None,phase:0,closing:false,progress:Default::default(),last_work:0}}
 fn validate(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>)->Result<(),ValueError>{let binding=(source.source_identity(),infos as*const _ as usize,request.identity());if self.binding.is_some_and(|b|b!=binding){return Err(invalid())}Ok(())}
 fn paid(cx:&mut StepContext<'_>,d:RetirementDemand)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 /// 🪙️ Quotes the next event from the same borrowed scene, catalogue and command fields.
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source,infos,request)?;if self.closing{return self.next_close_demands(copy)}Ok(match self.phase{
   0=>fixed(size_of::<Child>()+size_of::<Option<(FlowSourceIdentity,usize,[u64;20])>>()+size_of::<Option<(u64,u64)>>()),
   1=>return self.child.as_ref().ok_or_else(invalid)?.next_demands(source,infos,request,copy),
   2=>{let child=self.child.as_ref().ok_or_else(invalid)?;if child.terminal_is_empty(){fixed(size_of::<Option<Child>>())}else{return child.close_demands(copy)}},
   3=>fixed(size_of::<FlowAuthoringOutcome>()+size_of::<Option<Result<Vec<FlowMutation>,FlowCoreError>>>()),
   4=>fixed(size_of::<Option<FlowAuthoringOutcome>>()),
   _=>Default::default(),
  })
 }
 /// ⏱️ Pumps individually funded events using only the original context's remaining opportunity.
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>,cx:&mut StepContext<'_>)->Result<Option<FlowAuthoringOutcome>,OriginalTurnFailure>{
  let start=cx.retained_progress();let fuel=cx.fuel_remaining();let result=(||{for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.step_once(source,infos,request,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(None)})();let end=cx.retained_progress();self.progress=RetainedCloneProgress{copied_items:end.copied_items-start.copied_items,copied_bytes:end.copied_bytes-start.copied_bytes,retained_capacity_bytes:end.retained_capacity_bytes-start.retained_capacity_bytes,released_bytes:end.released_bytes-start.released_bytes};self.last_work=fuel-cx.fuel_remaining();result
 }
 fn step_once(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,request:FlowAuthoringRequest<'_>,cx:&mut StepContext<'_>)->Result<Option<FlowAuthoringOutcome>,OriginalTurnFailure>{
  self.validate(source,infos,request)?;if self.closing{return Err(invalid().into())}let origin=(cx.operation().0,cx.generation().0);if self.origin.is_some_and(|o|o!=origin){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  match self.phase{
   0=>{let d=self.next_demands(source,infos,request,cx.retained_grant().maximum_copy_bytes)?;if Self::paid(cx,d)?{self.binding=Some((source.source_identity(),infos as*const _ as usize,request.identity()));self.origin=Some(origin);self.child=Some(Child::new(request));self.phase=1}},
   1=>{if let Some(result)=self.child.as_mut().ok_or_else(invalid)?.step(source,infos,request,&mut self.serials,cx)?{*self.result=Some(result);self.child.as_mut().unwrap().begin_close();self.phase=2}},
   2=>{let child=self.child.as_mut().ok_or_else(invalid)?;if !child.terminal_is_empty(){let d=child.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|child.close_step(g))?;return Ok(None)}if Self::paid(cx,fixed(size_of::<Option<Child>>()))?{self.child=None;self.phase=3}},
   3=>if Self::paid(cx,self.next_demands(source,infos,request,cx.retained_grant().maximum_copy_bytes)?)?{*self.output=Some(FlowAuthoringOutcome{result:self.result.take().ok_or_else(invalid)?,serials:self.serials});self.phase=4},
   4=>if Self::paid(cx,fixed(size_of::<Option<FlowAuthoringOutcome>>()))?{self.phase=5;return Ok(self.output.take())},
   5=>{},_=>return Err(invalid().into()),
  }Ok(None)
 }
 /// 📥️ Restores the same completed output when its original caller could not accept custody.
 pub fn restore_output(&mut self,outcome:FlowAuthoringOutcome){assert_eq!(self.phase,5);assert!(self.output.is_none());*self.output=Some(outcome);self.phase=4}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.last_work}
 /// 🛑️ Keeps every original child, partial leaf and completed result for explicit closure.
 pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<Option<Child>>()))}else{child.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.result.is_some(){return self.retirement.next_owner_demands::<Result<Vec<FlowMutation>,FlowCoreError>>(true,&None,copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<FlowAuthoringOutcome>(true,&None,copy)}Ok(if self.binding.is_some()||self.origin.is_some(){fixed(size_of::<Option<(FlowSourceIdentity,usize,[u64;20])>>()+size_of::<Option<(u64,u64)>>())}else{Default::default()})}
 /// 🧹️ Returns the exact child work as well as its physical progress on both result branches.
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.last_work=0;let outcome=self.close_once(g);self.last_work=outcome.work;self.progress=outcome.result.as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());outcome
 }
 fn close_once(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){return child.close_step(g)}let d=fixed(size_of::<Option<Child>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.child=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  macro_rules! owner{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(result);owner!(output);if self.binding.is_some()||self.origin.is_some(){let d=fixed(size_of::<Option<(FlowSourceIdentity,usize,[u64;20])>>()+size_of::<Option<(u64,u64)>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;self.origin=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.child.is_none()&&self.result.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()&&self.origin.is_none()}
}
impl Drop for FlowAuthoringWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"Flow authoring must close its original source and output custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.result);ManuallyDrop::drop(&mut self.output)}}}}
