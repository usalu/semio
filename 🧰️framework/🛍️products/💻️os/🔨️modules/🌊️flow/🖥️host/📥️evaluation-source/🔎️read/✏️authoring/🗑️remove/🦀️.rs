//! 🗑️ Bounded deletion authoring retains original text and output leaves through cancellation.
use super::super::*;
use semio_framework_artifact_flow_flow::{FlowMutation,RemoveSynapse,RemoveWidget,ChangeLayout,FlowLayoutEntry,widget_id_for};
use super::{compare::TextCompare,text::FundedText};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original deletion authoring changed its funded frontier")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{output:Option<Vec<FlowMutation>>,id:Option<String>,entries:Option<Vec<FlowLayoutEntry>>,error:Option<FlowCoreError>}
pub(super) struct RemoveWork{fields:ManuallyDrop<Fields>,text:Option<FundedText>,compare:TextCompare,retirement:RetainedCloneClose,remove_widget:bool,phase:u8,scan:usize,count:usize,matched:Option<bool>,layout:bool,closing:bool}
impl RemoveWork{
 pub(super) fn new(remove_widget:bool)->Self{Self{fields:ManuallyDrop::new(Fields{output:None,id:None,entries:None,error:None}),text:None,compare:TextCompare::new(),retirement:Default::default(),remove_widget,phase:0,scan:0,count:0,matched:None,layout:false,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);Ok(true)}
 fn copy_id(&mut self,id:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.fields.id.is_some()&&self.text.is_none(){return Ok(true)}
  if self.text.is_none(){if !Self::paid(cx,size_of::<FundedText>())?{return Ok(false)}self.text=Some(FundedText::new());return Ok(false)}
  let text=self.text.as_mut().unwrap();
  if self.fields.id.is_none(){if text.ready(){text.take_into(&mut self.fields.id,cx)?;if self.fields.id.is_some(){text.begin_close();}}else{text.advance(id,cx)?;}return Ok(false)}
  if !text.terminal_is_empty(){let demand=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|text.close_step(grant))?;return Ok(false)}
  if !Self::paid(cx,size_of::<Option<FundedText>>())?{return Ok(false)}self.text=None;Ok(true)
 }
 pub(super) fn next_demands(&self,source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,id:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else if self.fields.id.is_some(){text.close_demands(copy)}else{text.next_demands(if self.phase==3&&self.remove_widget{&source.synapses().get(self.scan).ok_or_else(invalid)?.id}else{id},copy)}}
  Ok(RetirementDemand{capacity_bytes:match self.phase{2=>self.count.checked_mul(size_of::<FlowMutation>()).ok_or_else(invalid)?,6=>size_of::<FlowLayoutEntry>(),_=>0},..fixed(1024)})
 }
 pub(super) fn step(&mut self,source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,id:&str,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}
  match self.phase{
   0=>{let candidate=if self.remove_widget{source.widgets().get(self.scan).map(widget_id_for)}else{source.synapses().get(self.scan).map(|wire|wire.id.as_str())};
    let Some(candidate)=candidate else{if Self::paid(cx,size_of::<u8>())?{self.phase=10}return Ok(None)};
    if self.matched.is_none(){self.matched=self.compare.step(candidate,id,cx)?;return Ok(None)}
    if !self.compare.reset(cx)?{return Ok(None)}if self.matched.take().unwrap(){self.phase=if self.remove_widget{1}else{2};self.scan=0;self.count=usize::from(!self.remove_widget);}else{self.scan+=1;}
   },
   1=>{let Some(wire)=source.synapses().get(self.scan)else{if Self::paid(cx,size_of::<u8>())?{self.layout=source.layout().contains_key(id);self.count+=1+usize::from(self.layout);self.phase=2;self.scan=0}return Ok(None)};
    let candidate=if self.matched==Some(false){wire.to.as_str()}else{wire.from.as_str()};
    let result=self.compare.step(candidate,id,cx)?;if let Some(result)=result{if !self.compare.reset(cx)?{return Ok(None)}if result{self.count+=1;self.scan+=1;self.matched=None}else if self.matched==Some(false){self.scan+=1;self.matched=None}else{self.matched=Some(false)}}
   },
   2=>{let capacity=self.count.checked_mul(size_of::<FlowMutation>()).ok_or_else(invalid)?;let d=RetirementDemand{capacity_bytes:capacity,..fixed(size_of::<Option<Vec<FlowMutation>>>())};let mut accepted=None;
    admit_original_construction(cx,&mut accepted,original(d),|_|{let result=(||{let mut output=Vec::new();output.try_reserve_exact(self.count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original deletion leaf roster allocation refused"))?;let actual=output.capacity()*size_of::<FlowMutation>();self.fields.output=Some(output);self.phase=3;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..progress(d)}))})();RetainedWorkOutcome{result,work:1}})?;
   },
   3=>{if !self.remove_widget{if self.copy_id(id,cx)?{self.phase=4;}return Ok(None)}
    let Some(wire)=source.synapses().get(self.scan)else{if Self::paid(cx,size_of::<u8>())?{self.phase=if self.layout{5}else{8};self.matched=None}return Ok(None)};
    if self.matched==Some(true){if self.copy_id(&wire.id,cx)?{self.phase=4;}return Ok(None)}
    let candidate=if self.matched==Some(false){wire.to.as_str()}else{wire.from.as_str()};if let Some(result)=self.compare.step(candidate,id,cx)?{if !self.compare.reset(cx)?{return Ok(None)}if result{self.matched=Some(true)}else if self.matched==Some(false){self.scan+=1;self.matched=None}else{self.matched=Some(false)}}
   },
   4=>{if Self::paid(cx,size_of::<FlowMutation>()+size_of::<Option<String>>())?{let value=FlowMutation::RemoveSynapse(RemoveSynapse{id:self.fields.id.take().ok_or_else(invalid)?});self.fields.output.as_mut().ok_or_else(invalid)?.push(value);if self.remove_widget{self.scan+=1;self.matched=None;self.phase=3}else{self.phase=9}}}
   5=>{if self.copy_id(id,cx)?{self.phase=6;}}
   6=>{let d=RetirementDemand{capacity_bytes:size_of::<FlowLayoutEntry>(),..fixed(size_of::<Option<Vec<FlowLayoutEntry>>>())};let mut accepted=None;admit_original_construction(cx,&mut accepted,original(d),|_|{let result=(||{let mut entries=Vec::new();entries.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original deletion layout row allocation refused"))?;self.fields.entries=Some(entries);self.phase=7;Ok(((),progress(d)))})();RetainedWorkOutcome{result,work:1}})?;}
   7=>{if Self::paid(cx,size_of::<FlowLayoutEntry>()+size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.entries.as_mut().unwrap().push(FlowLayoutEntry{id:fields.id.take().ok_or_else(invalid)?,layout:None});let entries=fields.entries.take().unwrap();fields.output.as_mut().unwrap().push(FlowMutation::ChangeLayout(ChangeLayout{entries}));self.phase=8;}}
   8=>{if self.copy_id(id,cx)?&&Self::paid(cx,size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.output.as_mut().unwrap().push(FlowMutation::RemoveWidget(RemoveWidget{id:fields.id.take().ok_or_else(invalid)?}));self.phase=9;}}
   9=>{if Self::paid(cx,size_of::<Option<Vec<FlowMutation>>>())?{self.phase=12;return Ok(Some(Ok(self.fields.output.take().ok_or_else(invalid)?)))}}
   10=>{if self.copy_id(id,cx)?{if Self::paid(cx,size_of::<FlowCoreError>())?{let id=self.fields.id.take().ok_or_else(invalid)?;self.fields.error=Some(if self.remove_widget{FlowCoreError::UnknownWidget(id)}else{FlowCoreError::UnknownSynapse(id)});self.phase=11;}}}
   11=>{if Self::paid(cx,size_of::<Option<FlowCoreError>>())?{self.phase=12;return Ok(Some(Err(self.fields.error.take().ok_or_else(invalid)?)))}}
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<Vec<FlowMutation>,FlowCoreError>){assert_eq!(self.phase,12);match result{Ok(output)=>{self.fields.output=Some(output);self.phase=9},Err(error)=>{self.fields.error=Some(error);self.phase=11}}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.output.is_some(){return self.retirement.next_owner_demands::<Vec<FlowMutation>>(true,&None,copy)}if self.fields.id.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}if self.fields.entries.is_some(){return self.retirement.next_owner_demands::<Vec<FlowLayoutEntry>>(true,&None,copy)}if self.fields.error.is_some(){return self.retirement.next_owner_demands::<FlowCoreError>(true,&None,copy)}Ok(Default::default())}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(text)=self.text.as_mut(){if text.terminal_is_empty(){let d=fixed(size_of::<Option<FundedText>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.text=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(d))),work:1}}return text.close_step(grant)}
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(grant.maximum_copy_bytes,effect))}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,effect);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(output);owner!(id);owner!(entries);owner!(error);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.retirement.is_empty()&&self.fields.output.is_none()&&self.fields.id.is_none()&&self.fields.entries.is_none()&&self.fields.error.is_none()}
}
impl Drop for RemoveWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original deletion authoring must close its text and leaf custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
