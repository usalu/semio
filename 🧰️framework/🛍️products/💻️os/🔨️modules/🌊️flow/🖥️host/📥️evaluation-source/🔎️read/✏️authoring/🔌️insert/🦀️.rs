//! 🔌️ Original variadic insertion retains the widget and changed synapse fields as funded leaves.
use super::super::*;
use super::{text::FundedText,compare::TextCompare,unsigned::UnsignedWork,widget::{WidgetWork,WidgetPlan},synapse::SynapseWork};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,Widget,FlowMutation,SynapseSpec,ChangeWidget,ChangeSynapse,widget_id_for};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
use super::storage::{storage_demands,storage_ready,storage_close_demands,storage_close_step};
use semio_framework_value_native_storage::NativeOwnedStorage;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original port insertion changed its retained frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{id:Option<String>,wire_id:Option<String>,widget:Option<Widget>,wire:Option<SynapseSpec>,output:Option<Vec<FlowMutation>>,error:Option<FlowCoreError>}
pub(super) struct InsertWork{fields:ManuallyDrop<Fields>,text:Option<FundedText>,widget:NativeOwnedStorage<WidgetWork>,synapse:Option<SynapseWork>,retirement:RetainedCloneClose,compare:TextCompare,number:UnsignedWork,phase:u8,scan:usize,original:usize,count:usize,at:usize,changed:usize,matched:Option<bool>,compared:bool,pending_old:usize,parsed:Option<Option<u64>>,digits:[u8;20],digits_len:usize,closing:bool}
impl InsertWork{
 pub(super) fn new()->Self{Self{fields:ManuallyDrop::new(Fields{id:None,wire_id:None,widget:None,wire:None,output:None,error:None}),text:None,widget:NativeOwnedStorage::empty(),synapse:None,retirement:Default::default(),compare:TextCompare::new(),number:UnsignedWork::new(),phase:0,scan:0,original:0,count:0,at:0,changed:0,matched:None,compared:false,pending_old:0,parsed:None,digits:[0;20],digits_len:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
 fn original<'a>(&self,source:&'a dyn FlowSnapshotSource)->Result<&'a Widget,ValueError>{source.widgets().get(self.original).ok_or_else(invalid)}
 fn info<'a>(&self,source:&dyn FlowSnapshotSource,infos:&'a HistoryFoldIndex<String,OperatorInfo>)->Option<&'a OperatorInfo>{match self.original(source).ok()?{Widget::Neuron{neuron_kind,..}=>infos.get(neuron_kind),_=>None}}
 fn plan<'a>(&self,source:&'a dyn FlowSnapshotSource,input:bool)->Result<WidgetPlan<'a>,ValueError>{Ok(WidgetPlan::Insert{widget:self.original(source)?,input,count:self.count+1})}
 fn wire_fields<'a>(&'a self,source:&'a dyn FlowSnapshotSource,input:bool)->Result<[&'a str;5],ValueError>{let wire=source.synapses().get(self.scan).ok_or_else(invalid)?;let number=std::str::from_utf8(&self.digits[..self.digits_len]).map_err(|_|invalid())?;Ok([&wire.id,&wire.from,&wire.to,if input{&wire.from_port}else{number},if input{number}else{&wire.to_port}])}
 fn copy_id(&mut self,id:&str,wire:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let output=if wire{&mut self.fields.wire_id}else{&mut self.fields.id};if output.is_some()&&self.text.is_none(){return Ok(true)}
  if self.text.is_none(){if Self::paid(cx,size_of::<FundedText>())?{self.text=Some(FundedText::new())}return Ok(false)}
  let text=self.text.as_mut().unwrap();if output.is_none(){if text.ready(){text.take_into(output,cx)?;if output.is_some(){text.begin_close()}}else{text.advance(id,cx)?;}return Ok(false)}
  if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;return Ok(false)}if Self::paid(cx,size_of::<Option<FundedText>>())?{self.text=None;return Ok(true)}Ok(false)
 }
 pub(super) fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,id:&str,input:bool,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.phase==3{return storage_demands(&self.widget,|_|Ok(fixed(1)))}
  if self.widget.reserved(){return if self.phase==4{storage_demands(&self.widget,|work|work.next_demands(id,self.plan(source,input)?,self.info(source,infos),copy))}else{storage_close_demands(&self.widget,|work|work.close_demands(copy),|work|work.terminal_is_empty())}}
  if let Some(synapse)=self.synapse.as_ref(){return if self.phase==10{synapse.next_demands(self.wire_fields(source,input)?,copy)}else if synapse.terminal_is_empty(){Ok(fixed(size_of::<Option<SynapseWork>>()))}else{synapse.close_demands(copy)}}
  if let Some(text)=self.text.as_ref(){let wire=self.phase==12;let present=if wire{self.fields.wire_id.is_some()}else{self.fields.id.is_some()};return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else if present{text.close_demands(copy)}else{text.next_demands(if wire{&source.synapses().get(self.scan).ok_or_else(invalid)?.id}else{id},copy)}}
  Ok(if self.phase==7{RetirementDemand{capacity_bytes:(self.changed+1).checked_mul(size_of::<FlowMutation>()).ok_or_else(invalid)?,..fixed(128)}}else{fixed(size_of::<NativeOwnedStorage<WidgetWork>>().max(size_of::<SynapseWork>()).max(size_of::<FlowMutation>()+size_of::<String>()).max(256))})
 }
 fn matches(&mut self,source:&dyn FlowSnapshotSource,id:&str,input:bool,cx:&mut StepContext<'_>)->Result<Option<Option<usize>>,OriginalTurnFailure>{
  let wire=source.synapses().get(self.scan).ok_or_else(invalid)?;if self.matched.is_none(){self.matched=self.compare.step(if input{&wire.to}else{&wire.from},id,cx)?;return Ok(None)}if !self.compared{if !self.compare.reset(cx)?{return Ok(None)}self.compared=true;}
  if !self.matched.unwrap(){self.matched=None;self.compared=false;return Ok(Some(None))}
  if self.parsed.is_none(){self.parsed=self.number.step(if input{&wire.to_port}else{&wire.from_port},usize::MAX as u64,cx)?;return Ok(None)}
  if !self.number.reset(cx)?{return Ok(None)}let parsed=self.parsed.take().flatten().map(|v|v as usize).filter(|v|*v>=self.at);self.matched=None;self.compared=false;Ok(Some(parsed))
 }
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,id:&str,input:bool,index:usize,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}match self.phase{
   0=>if self.copy_id(id,false,cx)?{self.phase=1},
   1=>{let Some(widget)=source.widgets().get(self.scan)else{if Self::paid(cx,size_of::<FlowCoreError>())?{self.fields.error=Some(FlowCoreError::UnknownNeuronWidget(self.fields.id.take().ok_or_else(invalid)?));self.phase=20}return Ok(None)};
    if self.matched.is_none(){self.matched=self.compare.step(widget_id_for(widget),id,cx)?;return Ok(None)}if !self.compare.reset(cx)?{return Ok(None)}if self.matched.take().unwrap(){self.original=self.scan;self.phase=2}else{self.scan+=1}},
   2=>{if !Self::paid(cx,size_of::<FlowCoreError>()+3*size_of::<usize>())?{return Ok(None)}
    let Widget::Neuron{input_ports,output_ports,..}=self.original(source)? else{self.fields.error=Some(FlowCoreError::UnknownNeuronWidget(self.fields.id.take().ok_or_else(invalid)?));self.phase=20;return Ok(None)};
    let info=self.info(source,infos);let spec=if input{info.and_then(|i|i.variadic_input.as_ref())}else{info.and_then(|i|i.variadic_output.as_ref())};
    let Some(spec)=spec else{let id=self.fields.id.take().ok_or_else(invalid)?;self.fields.error=Some(if input{FlowCoreError::NotVariadicInput(id)}else{FlowCoreError::NotVariadicOutput(id)});self.phase=20;return Ok(None)};
    let explicit=if input{input_ports}else{output_ports};let count=if explicit.is_empty(){spec.min}else{explicit.len()};if spec.max.is_some_and(|max|count>=max){let id=self.fields.id.take().ok_or_else(invalid)?;self.fields.error=Some(if input{FlowCoreError::MaxInputPortsReached(id)}else{FlowCoreError::MaxOutputPortsReached(id)});self.phase=20}else{self.count=count;self.at=index.min(count);self.phase=3}},
   3=>if storage_ready(&mut self.widget,cx,WidgetWork::new)?&&Self::paid(cx,1)?{self.phase=4},
   4=>{let plan=self.plan(source,input)?;let info=self.info(source,infos);if let Some(widget)=self.widget.get_mut().unwrap().step(id,plan,info,cx)?{self.fields.widget=Some(widget);self.widget.get_mut().unwrap().begin_close();self.phase=5}},
   5=>{if !self.widget.terminal_is_empty(){let d=storage_close_demands(&self.widget,|work|work.close_demands(cx.retained_grant().maximum_copy_bytes),|work|work.terminal_is_empty())?;close_original_retained_step(cx,d,|grant|storage_close_step(&mut self.widget,grant,|work,g|work.close_step(g),|work|work.terminal_is_empty()))?;return Ok(None)}if Self::paid(cx,1)?{self.scan=0;self.phase=6}},
   6=>{if self.scan==source.synapses().len(){if Self::paid(cx,size_of::<u8>())?{self.phase=7}return Ok(None)}if let Some(changed)=self.matches(source,id,input,cx)?{self.changed+=usize::from(changed.is_some());self.scan+=1}},
   7=>{let d=self.next_demands(source,infos,id,input,cx.retained_grant().maximum_copy_bytes)?;let mut accepted=None;admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{let result=(||{let mut output=Vec::new();output.try_reserve_exact(self.changed+1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original port insertion mutation roster refused"))?;let actual=output.capacity()*size_of::<FlowMutation>();self.fields.output=Some(output);self.scan=0;self.phase=8;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})();RetainedWorkOutcome{result,work:1}})?;},
   8=>{if self.scan==source.synapses().len(){if Self::paid(cx,size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.output.as_mut().ok_or_else(invalid)?.push(FlowMutation::ChangeWidget(ChangeWidget{id:fields.id.take().ok_or_else(invalid)?,widget:fields.widget.take().ok_or_else(invalid)?}));self.phase=19}return Ok(None)}if let Some(parsed)=self.matches(source,id,input,cx)?{if let Some(old)=parsed{self.pending_old=old;self.phase=22}else{self.scan+=1}}},
   22=>if Self::paid(cx,size_of::<[u8;20]>()+size_of::<usize>())?{let mut value=self.pending_old.checked_add(1).ok_or_else(||ValueError::invariant("original variadic port index overflow"))?;let mut reversed=[0;20];let mut n=0;loop{reversed[n]=b'0'+(value%10)as u8;n+=1;value/=10;if value==0{break}}for i in 0..n{self.digits[i]=reversed[n-i-1]}self.digits_len=n;self.phase=9},
   9=>if Self::paid(cx,size_of::<SynapseWork>())?{self.synapse=Some(SynapseWork::new());self.phase=10},
   10=>{let wire=source.synapses().get(self.scan).ok_or_else(invalid)?;let number=std::str::from_utf8(&self.digits[..self.digits_len]).map_err(|_|invalid())?;let fields=[wire.id.as_str(),wire.from.as_str(),wire.to.as_str(),if input{wire.from_port.as_str()}else{number},if input{number}else{wire.to_port.as_str()}];if let Some(wire)=self.synapse.as_mut().unwrap().step(fields,cx)?{self.fields.wire=Some(wire);self.synapse.as_mut().unwrap().begin_close();self.phase=11}},
   11=>{let wire=self.synapse.as_mut().unwrap();if !wire.terminal_is_empty(){let d=wire.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|wire.close_step(g))?;return Ok(None)}if Self::paid(cx,size_of::<Option<SynapseWork>>())?{self.synapse=None;self.phase=12}},
   12=>{if self.copy_id(&source.synapses().get(self.scan).ok_or_else(invalid)?.id,true,cx)?{self.phase=13}},
   13=>if Self::paid(cx,size_of::<FlowMutation>())?{let fields=&mut*self.fields;fields.output.as_mut().ok_or_else(invalid)?.push(FlowMutation::ChangeSynapse(ChangeSynapse{id:fields.wire_id.take().ok_or_else(invalid)?,synapse:fields.wire.take().ok_or_else(invalid)?}));self.scan+=1;self.phase=8},
   19=>if Self::paid(cx,size_of::<Option<Vec<FlowMutation>>>())?{self.phase=21;return Ok(Some(Ok(self.fields.output.take().ok_or_else(invalid)?)))},
   20=>if Self::paid(cx,size_of::<Option<FlowCoreError>>())?{self.phase=21;return Ok(Some(Err(self.fields.error.take().ok_or_else(invalid)?)))},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<Vec<FlowMutation>,FlowCoreError>){assert_eq!(self.phase,21);match result{Ok(output)=>{self.fields.output=Some(output);self.phase=19},Err(error)=>{self.fields.error=Some(error);self.phase=20}}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(widget)=self.widget.get_mut(){widget.begin_close()}if let Some(wire)=self.synapse.as_mut(){wire.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<Option<$ty>>()))}else{child.close_demands(copy)}}}}
  child!(text,FundedText);if !self.widget.terminal_is_empty(){return storage_close_demands(&self.widget,|work|work.close_demands(copy),|work|work.terminal_is_empty())}child!(synapse,SynapseWork);if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(id,String);owner!(wire_id,String);owner!(widget,Widget);owner!(wire,SynapseSpec);owner!(output,Vec<FlowMutation>);owner!(error,FlowCoreError);Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_mut(){if child.terminal_is_empty(){let d=fixed(size_of::<Option<$ty>>());if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$field=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}return child.close_step(g)}}}
  child!(text,FundedText);if !self.widget.terminal_is_empty(){return storage_close_step(&mut self.widget,g,|work,g|work.close_step(g),|work|work.terminal_is_empty())}child!(synapse,SynapseWork);if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(id);owner!(wire_id);owner!(widget);owner!(wire);owner!(output);owner!(error);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.widget.terminal_is_empty()&&self.synapse.is_none()&&self.retirement.is_empty()&&self.fields.id.is_none()&&self.fields.wire_id.is_none()&&self.fields.widget.is_none()&&self.fields.wire.is_none()&&self.fields.output.is_none()&&self.fields.error.is_none()}
}
impl Drop for InsertWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original port insertion requires terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
