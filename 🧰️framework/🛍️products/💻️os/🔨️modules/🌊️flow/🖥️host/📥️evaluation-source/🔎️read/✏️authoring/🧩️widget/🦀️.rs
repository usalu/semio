//! 🧩️ Original widget fields become owned authoring output through independently funded events.
use super::super::*;
use super::{text::FundedText,value::ValueCopy};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_artifact_flow_flow::{Widget,WidgetDescriptor,widget_id_for};
use semio_framework_value_ordered::ordered::OrderedSet;
use neural_engine::{Dictionary,OperatorInfo,BudgetedOwnedEntry,Value as NeuralValue};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original widget authoring changed its retained fields")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[derive(Clone,Copy)]
pub(super) enum WidgetPlan<'a>{New(&'a WidgetDescriptor),Insert{widget:&'a Widget,input:bool,count:usize}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{id:Option<String>,text:Option<String>,aux:Option<String>,key:Option<String>,word:Option<String>,params:Option<Dictionary>,value:Option<NeuralValue>,inputs:Vec<String>,outputs:Vec<String>,output:Option<Widget>}
pub(super) struct WidgetWork{fields:ManuallyDrop<Fields>,text:Option<FundedText>,value:Option<ValueCopy>,entry:Option<BudgetedOwnedEntry>,retirement:RetainedCloneClose,phase:u8,param:usize,port:usize,side:bool,number:[u8;32],number_len:usize,closing:bool}
impl WidgetWork{
 pub(super) fn new()->Self{Self{fields:ManuallyDrop::new(Fields{id:None,text:None,aux:None,key:None,word:None,params:Some(Dictionary::new()),value:None,inputs:Vec::new(),outputs:Vec::new(),output:None}),text:None,value:None,entry:None,retirement:Default::default(),phase:0,param:0,port:0,side:true,number:[0;32],number_len:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=fixed(bytes);if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(progress(d))?;cx.consume_fuel(1);Ok(true)}
 fn copied(fields:&Fields,index:u8)->bool{match index{0=>fields.id.is_some(),1=>fields.text.is_some(),2=>fields.aux.is_some(),3=>fields.key.is_some(),_=>fields.word.is_some()}}
 fn copy_fields(fields:&mut Fields,cursor:&mut Option<FundedText>,source:&str,index:u8,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if Self::copied(fields,index)&&cursor.is_none(){return Ok(true)}
  if cursor.is_none(){if Self::paid(cx,size_of::<FundedText>())?{*cursor=Some(FundedText::new());}return Ok(false)}
  if !Self::copied(fields,index){let text=cursor.as_mut().unwrap();if text.ready(){let output=match index{0=>&mut fields.id,1=>&mut fields.text,2=>&mut fields.aux,3=>&mut fields.key,_=>&mut fields.word};text.take_into(output,cx)?;if output.is_some(){text.begin_close();}}else{text.advance(source,cx)?;}return Ok(false)}
  let text=cursor.as_mut().unwrap();if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|text.close_step(grant))?;return Ok(false)}
  if Self::paid(cx,size_of::<Option<FundedText>>())?{*cursor=None;return Ok(true)}Ok(false)
 }
 fn copy(&mut self,source:&str,index:u8,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Self::copy_fields(&mut self.fields,&mut self.text,source,index,cx)}
 fn text_for<'a>(plan:WidgetPlan<'a>,aux:bool)->Option<&'a str>{match plan{
  WidgetPlan::Insert{widget:Widget::Neuron{neuron_kind,..},..}=>if aux{None}else{Some(neuron_kind)},
  WidgetPlan::Insert{..}=>None,
  WidgetPlan::New(descriptor)=>match descriptor{
   WidgetDescriptor::Neuron{neuron_kind,..}=>if aux{None}else{Some(neuron_kind)},
   WidgetDescriptor::InputSlider{label,..}=>if aux{None}else{Some(label)},
   WidgetDescriptor::InputNote{text,..}=>if aux{None}else{Some(text.as_deref().unwrap_or(""))},
   WidgetDescriptor::OutputAction{action,..}=>if aux{None}else{Some(if action.is_empty(){"log"}else{action})},
   WidgetDescriptor::OutputExport{format,..}=>if aux{None}else{Some(if format.is_empty(){"svg"}else{format})},
   WidgetDescriptor::Variable{name,schema,..}=>Some(if aux{schema.as_deref().filter(|text|!text.trim().is_empty()).unwrap_or("dictionary")}else{name.as_deref().filter(|text|!text.trim().is_empty()).unwrap_or("value")}),
   _=>None,
  }
 }}
 fn port_count(plan:WidgetPlan<'_>,info:Option<&OperatorInfo>,input:bool)->usize{match plan{
  WidgetPlan::Insert{widget:Widget::Neuron{input_ports,output_ports,..},input:changed,count}=>if input==changed{count}else{if input{input_ports.len()}else{output_ports.len()}},
  WidgetPlan::New(WidgetDescriptor::Neuron{..})=>if input{if let Some(spec)=info.and_then(|info|info.variadic_input.as_ref()){spec.min}else{info.filter(|info|info.inputs.first().is_some_and(|port|port.name!="*")).map_or(0,|info|info.inputs.len())}}else{info.and_then(|info|info.variadic_output.as_ref()).map_or(0,|spec|spec.min)},
  _=>0,
 }}
 fn numeric(plan:WidgetPlan<'_>,info:Option<&OperatorInfo>,input:bool)->bool{match plan{WidgetPlan::Insert{input:changed,..}=>input==changed,WidgetPlan::New(WidgetDescriptor::Neuron{..})=>info.is_some_and(|info|if input{info.variadic_input.is_some()}else{info.variadic_output.is_some()}),_=>false}}
 fn number(&mut self,value:usize){let mut digits=[0;20];let mut value=value;let mut len=0;loop{digits[len]=b'0'+(value%10)as u8;len+=1;value/=10;if value==0{break}}for index in 0..len{self.number[index]=digits[len-index-1]}self.number_len=len}
 pub(super) fn next_demands(&self,id:&str,plan:WidgetPlan<'_>,info:Option<&OperatorInfo>,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(entry)=self.entry.as_ref(){return if self.phase==7{entry.next_demands(copy)}else if entry.terminal_is_empty(){Ok(fixed(size_of::<Option<BudgetedOwnedEntry>>()))}else{entry.next_close_demands(copy)}}
  if let Some(value)=self.value.as_ref(){return if self.fields.value.is_some(){if value.terminal_is_empty(){Ok(fixed(size_of::<Option<ValueCopy>>()))}else{value.close_demands(copy)}}else{value.next_demands(info.and_then(|i|i.inputs.get(self.param)).and_then(|p|p.default.as_ref()).ok_or_else(invalid)?,copy)}}
  if let Some(text)=self.text.as_ref(){if text.terminal_is_empty(){return Ok(fixed(size_of::<Option<FundedText>>()))}let slot=match self.phase{0=>0,1=>1,2=>2,4=>3,_=>4};if Self::copied(&self.fields,slot){return text.close_demands(copy)}}
  let mut d=fixed(256.max(size_of::<Widget>()).max(size_of::<BudgetedOwnedEntry>()));if self.phase==20{d.capacity_bytes=Self::port_count(plan,info,self.side).checked_mul(size_of::<String>()).ok_or_else(invalid)?}
  if let Some(text)=self.text.as_ref(){let source=match self.phase{0=>Some(id),1|2=>Self::text_for(plan,self.phase==2),4=>info.and_then(|i|i.inputs.get(self.param)).map(|p|p.name.as_str()),21=>match plan{WidgetPlan::Insert{widget:Widget::Neuron{input_ports,output_ports,..},..}=>if self.side{input_ports.get(self.port)}else{output_ports.get(self.port)}.map(String::as_str),_=>info.and_then(|i|i.inputs.get(self.port)).map(|p|p.name.as_str())},22=>Some(std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?),_=>None};if let Some(source)=source{return text.next_demands(source,copy)}}
  if self.phase==7{return self.entry.as_ref().ok_or_else(invalid)?.next_demands(copy)}Ok(d)
 }
 pub(super) fn step(&mut self,id:&str,plan:WidgetPlan<'_>,info:Option<&OperatorInfo>,cx:&mut StepContext<'_>)->Result<Option<Widget>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}
  match self.phase{
   0=>if self.copy(id,0,cx)?{self.phase=1},
   1|2=>{let aux=self.phase==2;if let Some(source)=Self::text_for(plan,aux){if self.copy(source,if aux{2}else{1},cx)?{self.phase+=1;}}else if Self::paid(cx,size_of::<u8>())?{self.phase+=1}},
   3=>match plan{WidgetPlan::Insert{widget:Widget::Neuron{params,..},..}=>{if Self::paid(cx,size_of::<Dictionary>())?{let previous=self.fields.params.replace(params.clone()).unwrap();assert!(previous.is_empty());drop(previous);self.phase=20;}},WidgetPlan::Insert{..}=>return Err(invalid().into()),WidgetPlan::New(WidgetDescriptor::Neuron{..})=>{if Self::paid(cx,size_of::<u8>())?{self.phase=4}},_=>{if Self::paid(cx,size_of::<u8>())?{self.phase=20}}},
   4=>{if let Some(channel)=info.and_then(|info|info.inputs.get(self.param)){if channel.default.is_some(){if self.copy(&channel.name,3,cx)?{self.phase=5}}else if Self::paid(cx,size_of::<usize>())?{self.param+=1;}}else if Self::paid(cx,size_of::<u8>())?{self.phase=20}},
   5=>{let source=info.and_then(|info|info.inputs.get(self.param)).and_then(|channel|channel.default.as_ref()).ok_or_else(invalid)?;
    if self.value.is_none(){if Self::paid(cx,size_of::<ValueCopy>())?{self.value=Some(ValueCopy::new());}return Ok(None)}
    if self.fields.value.is_none(){if let Some(value)=self.value.as_mut().unwrap().step(source,cx)?{self.fields.value=Some(value);self.value.as_mut().unwrap().begin_close();}return Ok(None)}
    let value=self.value.as_mut().unwrap();if !value.terminal_is_empty(){let d=value.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|value.close_step(grant))?;return Ok(None)}
    if Self::paid(cx,size_of::<Option<ValueCopy>>())?{self.value=None;self.phase=6}
   },
   6=>{if Self::paid(cx,size_of::<BudgetedOwnedEntry>())?{self.entry=Some(BudgetedOwnedEntry::new(self.fields.params.take().ok_or_else(invalid)?,self.fields.key.take().ok_or_else(invalid)?,self.fields.value.take().ok_or_else(invalid)?));self.phase=7;}},
   7=>{let entry=self.entry.as_mut().unwrap();let d=entry.next_demands(cx.retained_grant().maximum_copy_bytes)?;if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(None)}let result=entry.step(cx.retained_grant());let p=entry.step_progress();cx.consume_fuel(entry.last_turn_work());match result{Ok(params)=>{if let Some(params)=params{self.fields.params=Some(params);entry.begin_close();self.phase=8}cx.consume_retained(p)?;},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}},
   8=>{let entry=self.entry.as_mut().unwrap();if !entry.terminal_is_empty(){let d=entry.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|{let result=entry.close_step(grant);RetainedWorkOutcome{result,work:entry.last_turn_work()}})?;return Ok(None)}if Self::paid(cx,size_of::<Option<BudgetedOwnedEntry>>()+size_of::<usize>())?{self.entry=None;self.param+=1;self.phase=4}},
   20=>{let count=Self::port_count(plan,info,self.side);let d=RetirementDemand{capacity_bytes:count.checked_mul(size_of::<String>()).ok_or_else(invalid)?,..fixed(size_of::<Vec<String>>())};let mut accepted=None;admit_original_construction(cx,&mut accepted,original(d),|_|{let result=(||{let rows=if self.side{&mut self.fields.inputs}else{&mut self.fields.outputs};rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original widget port roster allocation refused"))?;let actual=rows.capacity()*size_of::<String>();self.port=0;self.phase=21;Ok(((),RetainedCloneProgress{retained_capacity_bytes:actual,..progress(d)}))})();RetainedWorkOutcome{result,work:1}})?;},
   21=>{let count=Self::port_count(plan,info,self.side);if self.port==count{if Self::paid(cx,size_of::<u8>()+size_of::<bool>())?{if self.side{self.side=false;self.phase=20}else{self.phase=30}}return Ok(None)}
    if Self::numeric(plan,info,self.side){if Self::paid(cx,size_of::<[u8;32]>()+size_of::<usize>())?{self.number(self.port);self.phase=22;}return Ok(None)}
    let source=match plan{WidgetPlan::Insert{widget:Widget::Neuron{input_ports,output_ports,..},..}=>if self.side{input_ports.get(self.port)}else{output_ports.get(self.port)}.map(String::as_str),WidgetPlan::New(WidgetDescriptor::Neuron{..})=>info.and_then(|info|info.inputs.get(self.port)).map(|channel|channel.name.as_str()),_=>None}.ok_or_else(invalid)?;
    if self.copy(source,4,cx)?{self.phase=23}
   },
   22=>{let source=std::str::from_utf8(&self.number[..self.number_len]).map_err(|_|invalid())?;if Self::copy_fields(&mut self.fields,&mut self.text,source,4,cx)?{self.phase=23}},
   23=>{if Self::paid(cx,size_of::<String>()+size_of::<usize>())?{let word=self.fields.word.take().ok_or_else(invalid)?;if self.side{self.fields.inputs.push(word)}else{self.fields.outputs.push(word)}self.port+=1;self.phase=21}},
   30=>{if Self::paid(cx,size_of::<Widget>())?{let fields=&mut*self.fields;let id=fields.id.take().ok_or_else(invalid)?;let widget=match plan{
    WidgetPlan::Insert{widget:Widget::Neuron{preview,..},..}=>Widget::Neuron{id,neuron_kind:fields.text.take().ok_or_else(invalid)?,params:fields.params.take().ok_or_else(invalid)?,input_ports:std::mem::take(&mut fields.inputs),output_ports:std::mem::take(&mut fields.outputs),preview:*preview},
    WidgetPlan::New(descriptor)=>match descriptor{
     WidgetDescriptor::Neuron{..}=>Widget::Neuron{id,neuron_kind:fields.text.take().ok_or_else(invalid)?,params:fields.params.take().ok_or_else(invalid)?,input_ports:std::mem::take(&mut fields.inputs),output_ports:std::mem::take(&mut fields.outputs),preview:true},
     WidgetDescriptor::InputSlider{value,min,max,step,..}=>{let(value,min,max,step)=semio_framework_artifact_flow_flow::resolve_input_slider_fields(*value,*min,*max,*step);Widget::InputSlider{id,label:fields.text.take().ok_or_else(invalid)?,value,min,max,step}},
     WidgetDescriptor::InputNote{..}=>Widget::InputNote{id,text:fields.text.take().ok_or_else(invalid)?},
     WidgetDescriptor::InputImage{..}=>Widget::InputImage{id,src:String::new()},
     WidgetDescriptor::OutputPreview{..}=>Widget::OutputPreview{id,preview:Dictionary::new(),expanded:OrderedSet::new()},
     WidgetDescriptor::OutputAction{..}=>Widget::OutputAction{id,action:fields.text.take().ok_or_else(invalid)?},
     WidgetDescriptor::OutputExport{..}=>Widget::OutputExport{id,format:fields.text.take().ok_or_else(invalid)?},
     WidgetDescriptor::Variable{..}=>Widget::Variable{id,name:fields.text.take().ok_or_else(invalid)?,schema:fields.aux.take().ok_or_else(invalid)?},
    },_=>return Err(invalid().into()),
   };fields.output=Some(widget);self.phase=31;}},
   31=>if Self::paid(cx,size_of::<Option<Widget>>())?{self.phase=32;return Ok(self.fields.output.take())},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,widget:Widget){assert_eq!(self.phase,32);self.fields.output=Some(widget);self.phase=31}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(value)=self.value.as_mut(){value.begin_close()}if let Some(entry)=self.entry.as_mut(){entry.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if let Some(value)=self.value.as_ref(){return if value.terminal_is_empty(){Ok(fixed(size_of::<Option<ValueCopy>>()))}else{value.close_demands(copy)}}if let Some(entry)=self.entry.as_ref(){return if entry.terminal_is_empty(){Ok(fixed(size_of::<Option<BudgetedOwnedEntry>>()))}else{entry.next_close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owned{($field:ident,$ty:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owned!(id,String);owned!(text,String);owned!(aux,String);owned!(key,String);owned!(word,String);owned!(params,Dictionary);owned!(value,NeuralValue);owned!(output,Widget);
  if self.fields.inputs.capacity()!=0||self.fields.outputs.capacity()!=0{return self.retirement.next_owner_demands::<Vec<String>>(true,&None,copy)}Ok(Default::default())}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! cursor{($field:ident,$ty:ty)=>{if let Some(cursor)=self.$field.as_mut(){if cursor.terminal_is_empty(){let d=fixed(size_of::<Option<$ty>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$field=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(d))),work:1}}return cursor.close_step(grant)}}}
  cursor!(text,FundedText);cursor!(value,ValueCopy);
  if let Some(entry)=self.entry.as_mut(){if entry.terminal_is_empty(){let d=fixed(size_of::<Option<BudgetedOwnedEntry>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.entry=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(d))),work:1}}let result=entry.close_step(grant);return RetainedWorkOutcome{result,work:entry.last_turn_work()}}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(id);owner!(text);owner!(aux);owner!(key);owner!(word);owner!(params);owner!(value);owner!(output);
  if self.fields.inputs.capacity()!=0||self.fields.outputs.capacity()!=0{let field=if self.fields.inputs.capacity()!=0{&mut self.fields.inputs}else{&mut self.fields.outputs};let mut values=Some(std::mem::take(field));let turn=self.retirement.begin_granted(&mut values,grant);if let Some(values)=values{*field=values}return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.text.is_none()&&self.value.is_none()&&self.entry.is_none()&&self.retirement.is_empty()&&self.fields.id.is_none()&&self.fields.text.is_none()&&self.fields.aux.is_none()&&self.fields.key.is_none()&&self.fields.word.is_none()&&self.fields.params.is_none()&&self.fields.value.is_none()&&self.fields.inputs.capacity()==0&&self.fields.outputs.capacity()==0&&self.fields.output.is_none()}
}
impl Drop for WidgetWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original authoring widget fields require explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
