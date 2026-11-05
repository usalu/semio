//! 🔗️ Original connection admission walks borrowed endpoints and edges before funding mutation leaves.
use super::super::*;
use super::{compare::TextCompare,text::FundedText,ports::{PortProbe,PortTypes},types::{TypeCompatibility,TypeTextWork,TextInput},synapse::SynapseWork,reach::ReachWork};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,Widget,FlowMutation,SynapseSpec,AddSynapse,RemoveSynapse,widget_id_for};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original connection changed its retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{id:Option<String>,leaf_id:Option<String>,wire:Option<SynapseSpec>,output:Option<Vec<FlowMutation>>,error:Option<FlowCoreError>,error_text:[Option<String>;4],flags:Vec<u8>}
pub(super) struct ConnectWork{fields:ManuallyDrop<Fields>,text:Option<FundedText>,typed:Option<TypeTextWork>,synapse:Option<SynapseWork>,reach:Option<ReachWork>,retirement:RetainedCloneClose,compare:TextCompare,slot:usize,matched:Option<bool>,query_result:Option<bool>,from_probe:PortProbe,to_probe:PortProbe,compatibility:TypeCompatibility,from_index:Option<usize>,to_index:Option<usize>,phase:u8,scan:usize,count:usize,fault:u8,error_slot:usize,number:[u8;21],number_len:usize,closing:bool}
impl ConnectWork{
 pub(super) fn new()->Self{Self{fields:ManuallyDrop::new(Fields{id:None,leaf_id:None,wire:None,output:None,error:None,error_text:[None,None,None,None],flags:Vec::new()}),text:None,typed:None,synapse:None,reach:None,retirement:Default::default(),compare:TextCompare::new(),slot:0,matched:None,query_result:None,from_probe:PortProbe::new(),to_probe:PortProbe::new(),compatibility:TypeCompatibility::new(),from_index:None,to_index:None,phase:0,scan:0,count:0,fault:0,error_slot:0,number:[0;21],number_len:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 fn fail(&mut self,fault:u8){self.fault=fault;self.phase=40}
 fn query_fields(compare:&mut TextCompare,slot:&mut usize,matched:&mut Option<bool>,query_result:&mut Option<bool>,left:[&str;4],right:[&str;4],len:usize,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  if let Some(result)=*query_result{if Self::paid(cx,3*size_of::<usize>())?{*slot=0;*query_result=None;return Ok(Some(result))}return Ok(None)}
  if matched.is_none(){*matched=compare.step(left[*slot],right[*slot],cx)?;return Ok(None)}
  if !compare.reset(cx)?{return Ok(None)}if !matched.take().unwrap(){*query_result=Some(false)}else{*slot+=1;if *slot==len{*query_result=Some(true)}}Ok(None)
 }
 fn widget<'a>(source:&'a dyn FlowSnapshotSource,index:Option<usize>)->Option<&'a Widget>{index.and_then(|i|source.widgets().get(i))}
 fn types<'a>(&self,source:&'a dyn FlowSnapshotSource,infos:&'a HistoryFoldIndex<String,OperatorInfo>,input:bool)->PortTypes<'a>{if input{Self::widget(source,self.to_index).map_or(PortTypes::Empty,|w|self.to_probe.types(w,infos,true))}else{Self::widget(source,self.from_index).map_or(PortTypes::Empty,|w|self.from_probe.types(w,infos,false))}}
 fn type_input<'a>(&self,source:&'a dyn FlowSnapshotSource,infos:&'a HistoryFoldIndex<String,OperatorInfo>,from:&'a str,from_port:&'a str,to:&'a str,to_port:&'a str)->TextInput<'a>{match self.error_slot{0=>TextInput::Endpoint(from,from_port),1=>TextInput::Types(self.types(source,infos,false)),2=>TextInput::Endpoint(to,to_port),_=>TextInput::Types(self.types(source,infos,true))}}
 fn error_id<'a>(&self,from:&'a str,from_port:&'a str,to:&'a str,to_port:&'a str)->Option<&'a str>{match self.fault{1=>Some(from),2=>Some(to),3=>Some(from_port),4=>Some(to_port),_=>None}}
 fn copy(fields:&mut Fields,text:&mut Option<FundedText>,source:&str,id:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let output=if id{&mut fields.id}else{&mut fields.leaf_id};if output.is_some()&&text.is_none(){return Ok(true)}
  if text.is_none(){if Self::paid(cx,size_of::<FundedText>())?{*text=Some(FundedText::new())}return Ok(false)}
  let cursor=text.as_mut().unwrap();if output.is_none(){if cursor.ready(){cursor.take_into(output,cx)?;if output.is_some(){cursor.begin_close()}}else{cursor.advance(source,cx)?;}return Ok(false)}
  if !cursor.terminal_is_empty(){let d=cursor.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|cursor.close_step(g))?;return Ok(false)}
  if Self::paid(cx,size_of::<Option<FundedText>>())?{*text=None;return Ok(true)}Ok(false)
 }
 pub(super) fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,from:&str,from_port:&str,to:&str,to_port:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(reach)=self.reach.as_ref(){return if self.phase==6{reach.next_demands(source,copy)}else if reach.terminal_is_empty(){Ok(fixed(size_of::<Option<ReachWork>>()))}else{reach.close_demands(copy)}}
  if let Some(text)=self.text.as_ref(){let present=if self.phase==23{self.fields.id.is_some()}else{self.fields.leaf_id.is_some()};if text.terminal_is_empty(){return Ok(fixed(size_of::<Option<FundedText>>()))}if present{return text.close_demands(copy)}let input=match self.phase{23=>std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?,24=>&source.synapses().get(self.scan).ok_or_else(invalid)?.id,40=>self.error_id(from,from_port,to,to_port).ok_or_else(invalid)?,_=>return Err(invalid())};return text.next_demands(input,copy)}
  if let Some(typed)=self.typed.as_ref(){return if self.phase==41{typed.next_demands(copy)}else if typed.terminal_is_empty(){Ok(fixed(size_of::<Option<TypeTextWork>>()))}else{typed.close_demands(copy)}}
  if let Some(synapse)=self.synapse.as_ref(){return if self.phase==28{synapse.next_demands([self.fields.id.as_deref().ok_or_else(invalid)?,from,to,from_port,to_port],copy)}else if synapse.terminal_is_empty(){Ok(fixed(size_of::<Option<SynapseWork>>()))}else{synapse.close_demands(copy)}}
  let _=infos;let capacity_bytes=match self.phase{13=>source.synapses().len(),20=>(self.count+1).checked_mul(size_of::<FlowMutation>()).ok_or_else(invalid)?,_=>0};Ok(RetirementDemand{capacity_bytes,..fixed(1024.max(size_of::<SynapseWork>()).max(size_of::<TypeTextWork>()))})
 }
 fn reserve(&mut self,cx:&mut StepContext<'_>,d:RetirementDemand,next:u8)->Result<(),OriginalTurnFailure>{
  let mut accepted=None;admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{let result=(||{let actual=match self.phase{
   13=>{self.fields.flags.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original connection reachability roster refused"))?;self.fields.flags.capacity()},
   20=>{let mut output=Vec::new();output.try_reserve_exact(self.count+1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original connection mutation roster refused"))?;let bytes=output.capacity()*size_of::<FlowMutation>();self.fields.output=Some(output);bytes},
   _=>return Err(invalid()),
  };self.scan=0;self.phase=next;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})();RetainedWorkOutcome{result,work:1}})?;Ok(())
 }
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,serial:&mut u64,from:&str,from_port:&str,to:&str,to_port:&str,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}match self.phase{
   0=>if let Some(equal)=Self::query_fields(&mut self.compare,&mut self.slot,&mut self.matched,&mut self.query_result,[from,"","",""],[to,"","",""],1,cx)?{if equal{self.fail(5)}else{self.phase=1;self.scan=0}},
   1|3=>{let from_side=self.phase==1;let target=if from_side{from}else{to};if let Some(widget)=source.widgets().get(self.scan){if let Some(equal)=Self::query_fields(&mut self.compare,&mut self.slot,&mut self.matched,&mut self.query_result,[widget_id_for(widget),"","",""],[target,"","",""],1,cx)?{if equal{if from_side{self.from_index=Some(self.scan)}else{self.to_index=Some(self.scan)}self.phase+=1}else{self.scan+=1}}}else if Self::paid(cx,2*size_of::<usize>())?{self.phase+=1}},
   2=>if self.from_probe.step(Self::widget(source,self.from_index),infos,false,from_port,cx)?{if self.from_probe.has(){self.scan=0;self.phase=3}else{self.fail(1)}},
   4=>if self.to_probe.step(Self::widget(source,self.to_index),infos,true,to_port,cx)?{if !self.to_probe.has(){self.fail(2)}else if !self.from_probe.declared(){self.fail(3)}else if !self.to_probe.declared(){self.fail(4)}else{self.phase=5}},
   5=>if Self::paid(cx,size_of::<ReachWork>())?{self.reach=Some(ReachWork::new());self.phase=6},
   6=>if let Some(cycle)=self.reach.as_mut().unwrap().step(source,from,to,false,cx)?{if cycle{self.fault=6}self.reach.as_mut().unwrap().begin_close();self.phase=7},
   7=>{let reach=self.reach.as_mut().unwrap();if !reach.terminal_is_empty(){let d=reach.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|reach.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<ReachWork>>())?{self.reach=None;self.phase=if self.fault==6{40}else{13}}},
   13=>{let d=self.next_demands(source,infos,from,from_port,to,to_port,cx.retained_grant().maximum_copy_bytes)?;self.reserve(cx,d,15)?;},
   15=>if Self::paid(cx,size_of::<usize>()+1)?{if self.fields.flags.len()==source.synapses().len(){self.scan=0;self.phase=16}else{self.fields.flags.push(0)}},
   16=>{let Some(wire)=source.synapses().get(self.scan)else{if Self::paid(cx,1)?{self.phase=18}return Ok(None)};if let Some(equal)=Self::query_fields(&mut self.compare,&mut self.slot,&mut self.matched,&mut self.query_result,[&wire.from,&wire.from_port,&wire.to,&wire.to_port],[from,from_port,to,to_port],4,cx)?{if equal{self.fail(7)}else{self.phase=17}}},
   17=>{let wire=source.synapses().get(self.scan).ok_or_else(invalid)?;if let Some(equal)=Self::query_fields(&mut self.compare,&mut self.slot,&mut self.matched,&mut self.query_result,[&wire.to,&wire.to_port,"",""],[to,to_port,"",""],2,cx)?{if equal{self.fields.flags[self.scan]=1;self.count+=1}self.scan+=1;self.phase=16}},
   18=>{let a=self.types(source,infos,false);let b=self.types(source,infos,true);if let Some(compatible)=self.compatibility.step(a,b,cx)?{if compatible{self.phase=20}else{self.fail(8)}}},
   20=>{let d=self.next_demands(source,infos,from,from_port,to,to_port,cx.retained_grant().maximum_copy_bytes)?;self.reserve(cx,d,21)?;},
   21=>if Self::paid(cx,size_of::<[u8;21]>()+size_of::<u64>()+size_of::<usize>())?{*serial=serial.wrapping_add(1);let mut value=*serial;let mut digits=[0;20];let mut n=0;loop{digits[n]=b'0'+(value%10)as u8;n+=1;value/=10;if value==0{break}}self.number[0]=b's';for i in 0..n{self.number[i+1]=digits[n-i-1]}self.number_len=n+1;self.scan=0;self.phase=22},
   22=>{let Some(wire)=source.synapses().get(self.scan)else{if Self::paid(cx,1)?{self.phase=23}return Ok(None)};if self.fields.flags[self.scan]!=0{if Self::paid(cx,size_of::<usize>())?{self.scan+=1}return Ok(None)}let number=std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?;if let Some(equal)=Self::query_fields(&mut self.compare,&mut self.slot,&mut self.matched,&mut self.query_result,[&wire.id,"","",""],[number,"","",""],1,cx)?{if equal{self.phase=21}else{self.scan+=1}}},
   23=>{let number=std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?;if Self::copy(&mut self.fields,&mut self.text,number,true,cx)?{self.scan=0;self.phase=24}},
   24=>{let Some(wire)=source.synapses().get(self.scan)else{if Self::paid(cx,1)?{self.phase=27}return Ok(None)};if self.fields.flags[self.scan]==0{if Self::paid(cx,size_of::<usize>())?{self.scan+=1}return Ok(None)}if Self::copy(&mut self.fields,&mut self.text,&wire.id,false,cx)?{self.phase=25}},
   25=>if Self::paid(cx,size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.output.as_mut().ok_or_else(invalid)?.push(FlowMutation::RemoveSynapse(RemoveSynapse{id:fields.leaf_id.take().ok_or_else(invalid)?}));self.scan+=1;self.phase=24},
   27=>if Self::paid(cx,size_of::<SynapseWork>())?{self.synapse=Some(SynapseWork::new());self.phase=28},
   28=>{let fields=[self.fields.id.as_deref().ok_or_else(invalid)?,from,to,from_port,to_port];if let Some(wire)=self.synapse.as_mut().unwrap().step(fields,cx)?{self.fields.wire=Some(wire);self.synapse.as_mut().unwrap().begin_close();self.phase=29}},
   29=>{let wire=self.synapse.as_mut().unwrap();if !wire.terminal_is_empty(){let d=wire.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|wire.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<SynapseWork>>())?{self.synapse=None;self.phase=30}},
   30=>if Self::paid(cx,size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.output.as_mut().ok_or_else(invalid)?.push(FlowMutation::AddSynapse(AddSynapse{index:u32::try_from(source.synapses().len()-self.count).unwrap_or(u32::MAX),synapse:fields.wire.take().ok_or_else(invalid)?}));self.phase=31},
   31=>if Self::paid(cx,size_of::<Option<Vec<FlowMutation>>>())?{self.phase=43;return Ok(Some(Ok(self.fields.output.take().ok_or_else(invalid)?)))},
   40=>{if self.fault==8{if Self::paid(cx,size_of::<TypeTextWork>())?{self.typed=Some(TypeTextWork::new());self.phase=41}return Ok(None)}if let Some(id)=self.error_id(from,from_port,to,to_port){if !Self::copy(&mut self.fields,&mut self.text,id,false,cx)?{return Ok(None)}}if Self::paid(cx,size_of::<FlowCoreError>())?{self.fields.error=Some(match self.fault{1=>FlowCoreError::NoOutputPort(self.fields.leaf_id.take().ok_or_else(invalid)?),2=>FlowCoreError::NoInputPort(self.fields.leaf_id.take().ok_or_else(invalid)?),3=>FlowCoreError::UnknownOutputPort(self.fields.leaf_id.take().ok_or_else(invalid)?),4=>FlowCoreError::UnknownInputPort(self.fields.leaf_id.take().ok_or_else(invalid)?),5=>FlowCoreError::SelfConnection,6=>FlowCoreError::CycleWouldBeCreated,7=>FlowCoreError::ConnectionAlreadyExists,_=>return Err(invalid().into())});self.phase=42}},
   41=>{let input=self.type_input(source,infos,from,from_port,to,to_port);if let Some(value)=self.typed.as_mut().unwrap().step(input,cx)?{self.fields.error_text[self.error_slot]=Some(value);self.typed.as_mut().unwrap().begin_close();self.phase=44}},
   44=>{let text=self.typed.as_mut().unwrap();if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<TypeTextWork>>()+size_of::<usize>())?{self.typed=None;self.error_slot+=1;self.phase=if self.error_slot==4{45}else{40}}},
   45=>if Self::paid(cx,size_of::<FlowCoreError>())?{self.fields.error=Some(FlowCoreError::IncompatiblePortTypes{source:self.fields.error_text[0].take().ok_or_else(invalid)?,source_type:self.fields.error_text[1].take().ok_or_else(invalid)?,target:self.fields.error_text[2].take().ok_or_else(invalid)?,target_type:self.fields.error_text[3].take().ok_or_else(invalid)?});self.phase=42},
   42=>if Self::paid(cx,size_of::<Option<FlowCoreError>>())?{self.phase=43;return Ok(Some(Err(self.fields.error.take().ok_or_else(invalid)?)))},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<Vec<FlowMutation>,FlowCoreError>){assert_eq!(self.phase,43);match result{Ok(output)=>{self.fields.output=Some(output);self.phase=31},Err(error)=>{self.fields.error=Some(error);self.phase=42}}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(text)=self.typed.as_mut(){text.begin_close()}if let Some(wire)=self.synapse.as_mut(){wire.begin_close()}if let Some(reach)=self.reach.as_mut(){reach.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<Option<$ty>>()))}else{child.close_demands(copy)}}}}
  child!(reach,ReachWork);child!(text,FundedText);child!(typed,TypeTextWork);child!(synapse,SynapseWork);if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(id,String);owner!(leaf_id,String);owner!(wire,SynapseSpec);owner!(output,Vec<FlowMutation>);owner!(error,FlowCoreError);
  if self.fields.error_text.iter().any(Option::is_some){return self.retirement.next_owner_demands::<String>(true,&None,copy)}if self.fields.flags.capacity()!=0{return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,copy)}Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_mut(){if child.terminal_is_empty(){let d=fixed(size_of::<Option<$ty>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$field=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}return child.close_step(g)}}}
  child!(reach,ReachWork);child!(text,FundedText);child!(typed,TypeTextWork);child!(synapse,SynapseWork);if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(id);owner!(leaf_id);owner!(wire);owner!(output);owner!(error);
  if let Some(field)=self.fields.error_text.iter_mut().find(|s|s.is_some()){let turn=self.retirement.begin_granted(field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}
  macro_rules! vector{($field:ident)=>{if self.fields.$field.capacity()!=0{let mut owner=Some(std::mem::take(&mut self.fields.$field));let turn=self.retirement.begin_granted(&mut owner,g);if let Some(owner)=owner{self.fields.$field=owner}return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  vector!(flags);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.reach.is_none()&&self.text.is_none()&&self.typed.is_none()&&self.synapse.is_none()&&self.retirement.is_empty()&&self.fields.id.is_none()&&self.fields.leaf_id.is_none()&&self.fields.wire.is_none()&&self.fields.output.is_none()&&self.fields.error.is_none()&&self.fields.error_text.iter().all(Option::is_none)&&self.fields.flags.capacity()==0}
}
impl Drop for ConnectWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original connection requires terminal child and output custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
