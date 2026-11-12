//! 🪪️ An original descriptor lends its explicit identity or funds a collision-tested generated identity.
use super::super::*;
use super::{text::FundedText,compare::TextCompare,unsigned::UnsignedWork};
use semio_framework_job::{OriginalConstructionDemand,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_artifact_flow_flow::{widget_id_for,WidgetDescriptor};
use super::storage::Construction;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original widget identity changed its retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
pub(super) struct IdentityWork{construction:Construction,output:ManuallyDrop<Option<String>>,text:Option<FundedText>,compare:TextCompare,numeric:UnsignedWork,parsed:Option<Option<u64>>,stage:u8,trim_start:usize,trim_end:usize,retirement:RetainedCloneClose,phase:u8,offset:usize,scan:usize,serial:u64,number:[u8;20],number_len:usize,explicit:bool,collision:Option<bool>,closing:bool}
impl IdentityWork{
 pub(super) fn new()->Self{Self{construction:Default::default(),output:ManuallyDrop::new(None),text:None,compare:TextCompare::new(),numeric:UnsignedWork::new(),parsed:None,stage:0,trim_start:0,trim_end:0,retirement:Default::default(),phase:0,offset:0,scan:0,serial:2,number:[0;20],number_len:0,explicit:false,collision:None,closing:false}}
 fn parts(descriptor:&WidgetDescriptor)->(Option<&str>,&str){match descriptor{
  WidgetDescriptor::Neuron{id,neuron_kind,..}=>(id.as_deref(),neuron_kind),
  WidgetDescriptor::InputSlider{id,..}=>(id.as_deref(),"slider"),
  WidgetDescriptor::InputNote{id,..}=>(id.as_deref(),"note"),
  WidgetDescriptor::InputImage{id,..}=>(id.as_deref(),"image"),
  WidgetDescriptor::Variable{id,..}=>(id.as_deref(),"variable"),
  WidgetDescriptor::OutputPreview{id,..}=>(id.as_deref(),"preview"),
  WidgetDescriptor::OutputAction{id,..}=>(id.as_deref(),"action"),
  WidgetDescriptor::OutputExport{id,..}=>(id.as_deref(),"export"),
 }}
 fn explicit_source<'a>(&self,descriptor:&'a WidgetDescriptor)->Result<&'a str,ValueError>{Self::parts(descriptor).0.and_then(|s|s.get(self.trim_start..self.trim_end)).ok_or_else(invalid)}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 fn digits(&mut self){let mut value=self.serial;let mut reversed=[0;20];let mut n=0;loop{reversed[n]=b'0'+(value%10)as u8;n+=1;value/=10;if value==0{break}}for i in 0..n{self.number[i]=reversed[n-i-1]}self.number_len=n}
 pub(super) fn next_demands(&self,descriptor:&WidgetDescriptor,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else if self.output.is_some(){text.close_demands(copy)}else{text.next_demands(self.explicit_source(descriptor)?,copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  let (_,prefix)=Self::parts(descriptor);Ok(if self.phase==2{RetirementDemand{capacity_bytes:prefix.len().checked_add(21).ok_or_else(invalid)?,..fixed(256)}}else{fixed(256)})
 }
 pub(super) fn step(&mut self,source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,descriptor:&WidgetDescriptor,cx:&mut StepContext<'_>)->Result<Option<(Result<String,FlowCoreError>,Option<u64>)>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}let(explicit,prefix)=Self::parts(descriptor);
  match self.phase{
   0=>{if Self::paid(cx,size_of::<FundedText>()+3*size_of::<usize>())?{if let Some(explicit)=explicit{self.trim_start=0;self.trim_end=explicit.len();self.phase=50}else{self.phase=2}}},
   50=>{let source=explicit.ok_or_else(invalid)?;let mut next=self.trim_start;let mut bytes=0;let mut found=false;while next<self.trim_end&&bytes<64{let ch=source[next..self.trim_end].chars().next().ok_or_else(invalid)?;bytes+=ch.len_utf8();if !ch.is_whitespace(){found=true;break}next+=ch.len_utf8()}if Self::paid(cx,bytes+2*size_of::<usize>())?{self.trim_start=next;if next==self.trim_end{self.phase=52}else if found{self.phase=51}}},
   51=>{let source=explicit.ok_or_else(invalid)?;let mut next=self.trim_end;let mut bytes=0;let mut found=false;while next>self.trim_start&&bytes<64{let ch=source[self.trim_start..next].chars().next_back().ok_or_else(invalid)?;bytes+=ch.len_utf8();if !ch.is_whitespace(){found=true;break}next-=ch.len_utf8()}if Self::paid(cx,bytes+2*size_of::<usize>())?{self.trim_end=next;if next==self.trim_start||found{self.phase=52}}},
   52=>if Self::paid(cx,size_of::<FundedText>()+size_of::<bool>())?{self.explicit=self.trim_start<self.trim_end;if self.explicit{self.text=Some(FundedText::new());self.phase=1}else{self.phase=2}},
   1=>{let text=self.text.as_mut().ok_or_else(invalid)?;if self.output.is_none(){if text.ready(){text.take_into(&mut self.output,cx)?;if self.output.is_some(){text.begin_close()}}else{text.advance(Self::parts(descriptor).0.and_then(|s|s.get(self.trim_start..self.trim_end)).ok_or_else(invalid)?,cx)?;}return Ok(None)}
    if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_poll(cx,d,|body,effect|text.close_step(body,effect))?;return Ok(None)}
    if Self::paid(cx,size_of::<Option<FundedText>>())?{self.text=None;self.phase=5}},
   2=>{let d=self.next_demands(descriptor,cx.retained_grant().maximum_copy_bytes)?;if self.construction.run(cx,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},||(||{let mut id=String::new();id.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original widget identity allocation refused"))?;let actual=id.capacity();*self.output=Some(id);Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})())?{self.offset=0;self.digits();self.phase=3}},
   3=>{let mut end=(self.offset+64).min(prefix.len());while !prefix.is_char_boundary(end){end-=1}let count=end-self.offset;if Self::paid(cx,count+size_of::<usize>())?{let id=self.output.as_mut().ok_or_else(invalid)?;for ch in prefix[self.offset..end].chars(){id.push(if ch=='.'{'_'}else{ch})}self.offset=end;if end==prefix.len(){self.phase=4}}},
   4=>{if Self::paid(cx,self.number_len+1+size_of::<u8>())?{self.output.as_mut().ok_or_else(invalid)?.push('_');self.output.as_mut().ok_or_else(invalid)?.push_str(std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?);self.scan=0;self.phase=5}},
   5=>{if let Some(widget)=source.widgets().get(self.scan){
    let candidate=widget_id_for(widget);let output=self.output.as_ref().ok_or_else(invalid)?;
    if self.explicit{if self.collision.is_none(){self.collision=self.compare.step(candidate,output,cx)?;return Ok(None)}if !self.compare.reset(cx)?{return Ok(None)}if self.collision.take().unwrap(){self.phase=8}else{self.scan+=1}}
    else if self.stage==0{let prefix_len=output.len().checked_sub(self.number_len).ok_or_else(invalid)?;let candidate_prefix=candidate.get(..prefix_len).unwrap_or("");if self.collision.is_none(){self.collision=self.compare.step(candidate_prefix,&output[..prefix_len],cx)?;return Ok(None)}if !self.compare.reset(cx)?{return Ok(None)}if self.collision.take().unwrap(){self.stage=1}else{self.scan+=1}}
    else{let prefix_len=output.len().checked_sub(self.number_len).ok_or_else(invalid)?;let suffix=candidate.get(prefix_len..).ok_or_else(invalid)?;if self.parsed.is_none(){self.parsed=self.numeric.step(suffix,u64::MAX,cx)?;return Ok(None)}if !self.numeric.reset(cx)?{return Ok(None)}if self.parsed.take().flatten()==Some(self.serial){self.phase=6}else{self.scan+=1;self.stage=0}}
   }else if Self::paid(cx,size_of::<u8>())?{self.phase=7}},
   6=>{if !self.retirement.is_empty(){let d=self.retirement.demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect)))?;return Ok(None)}if self.output.is_some(){let d=self.retirement.next_owner_demands::<String>(true,&None,cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|{let turn=self.retirement.begin_granted(&mut self.output,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(None)}if Self::paid(cx,size_of::<u64>())?{self.serial=self.serial.checked_add(1).ok_or_else(||ValueError::invariant("finite original widget identifiers exhausted u64"))?;self.stage=0;self.phase=2}},
   7|8=>{if Self::paid(cx,size_of::<Option<String>>())?{let id=self.output.take().ok_or_else(invalid)?;let result=if self.phase==8{Err(FlowCoreError::WidgetIdExists(id))}else{Ok(id)};self.phase=9;return Ok(Some((result,if self.explicit{None}else{Some(self.serial)})))}},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<String,FlowCoreError>){assert_eq!(self.phase,9);match result{Ok(id)=>{*self.output=Some(id);self.phase=7},Err(FlowCoreError::WidgetIdExists(id))=>{*self.output=Some(id);self.phase=8},_=>unreachable!()}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}self.retirement.next_owner_demands::<String>(self.output.is_some(),&None,copy)}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(text)=self.text.as_mut(){if text.terminal_is_empty(){let d=fixed(size_of::<Option<FundedText>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.text=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}return poll_outcome(text.close_step(g.maximum_copy_bytes,effect))}
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}let turn=self.retirement.begin_granted(&mut self.output,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for IdentityWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original widget identity requires terminal closure")}}
