//! 🔌️ Borrowed widget ports preserve declared identities and schemas without presentation allocation.
use super::super::*;
use super::compare::TextCompare;
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,Widget};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
#[derive(Clone,Copy)]
pub(super) enum PortTypes<'a>{Empty,Single(&'a str),Channels(&'a[String])}
impl<'a> PortTypes<'a>{
 pub(super) fn raw(self,index:usize)->Option<&'a str>{match self{Self::Empty=>None,Self::Single(value)=>(index==0).then_some(value),Self::Channels(values)=>values.get(index).map(String::as_str)}}
}
#[derive(Clone,Copy)]
pub(super) enum PortName<'a>{Text(&'a str),Number(usize)}
#[derive(Clone,Copy)]
pub(super) struct Port<'a>{pub(super) name:PortName<'a>,pub(super) types:PortTypes<'a>}
pub(super) fn port_at<'a>(widget:&'a Widget,infos:&'a HistoryFoldIndex<String,OperatorInfo>,input:bool,index:usize)->(bool,Option<Port<'a>>){
 let text=|name,types|Port{name:PortName::Text(name),types};
 let single=|name,types|if index==0{(false,Some(text(name,types)))}else{(true,None)};
 match widget{
  Widget::Neuron{neuron_kind,input_ports,output_ports,..}=>{
   let info=infos.get(neuron_kind);let variadic=info.and_then(|info|if input{info.variadic_input.as_ref()}else{info.variadic_output.as_ref()});
   if let Some(spec)=variadic{let names=if input{input_ports}else{output_ports};let count=if names.is_empty(){spec.min}else{names.len()};if index>=count{return(true,None)}
    let name=if names.is_empty(){PortName::Number(index)}else{PortName::Text(&names[index])};let types=if input{PortTypes::Empty}else{info.and_then(|info|info.outputs.first()).map_or(PortTypes::Empty,|p|PortTypes::Channels(&p.value_types))};return(false,Some(Port{name,types}))}
   if input{
    if let Some(info)=info.filter(|i|i.inputs.first().is_some_and(|p|p.name!="*")){return if let Some(p)=info.inputs.get(index){(false,Some(text(&p.name,PortTypes::Channels(&p.value_types))))}else{(true,None)}}
    if let Some(name)=input_ports.get(index){let types=info.and_then(|i|i.inputs.iter().find(|p|p.name==*name)).map_or(PortTypes::Empty,|p|PortTypes::Channels(&p.value_types));return(false,Some(text(name,types)))}(true,None)
   }else{if let Some(p)=info.and_then(|i|i.outputs.get(index)){(false,Some(text(&p.name,PortTypes::Channels(&p.value_types))))}else{(true,None)}}
  },
  Widget::InputSlider{..}=>if input{(true,None)}else{single("number",PortTypes::Single(neural_engine::VALUE_TYPE_NUMBER))},
  Widget::InputNote{..}=>if input{(true,None)}else{single("text",PortTypes::Single(neural_engine::VALUE_TYPE_TEXT))},
  Widget::InputImage{..}=>if input{(true,None)}else{single("image",PortTypes::Empty)},
  Widget::Variable{name,schema,..}=>single(name,PortTypes::Single(schema)),
  Widget::OutputPreview{..}|Widget::OutputAction{..}|Widget::OutputExport{..}=>if input{single("",PortTypes::Empty)}else{(true,None)},
  Widget::Cluster{tree,..}=>{let Some(neuron)=tree.neurons.get(index)else{return(true,None)};let kind=if input{neural_engine::INPUT_KIND}else{neural_engine::OUTPUT_KIND};if neuron.kind!=kind{return(false,None)}let name=neuron.params.get("channel").and_then(|v|v.as_atom()).and_then(|v|v.as_str()).unwrap_or(&neuron.id);(false,Some(text(name,PortTypes::Empty)))},
 }
}
pub(super) struct PortProbe{compare:TextCompare,index:usize,found:Option<usize>,has:bool,matched:Option<bool>,number:[u8;20],length:usize,formatted:bool,done:bool}
impl PortProbe{
 pub(super) fn new()->Self{Self{compare:TextCompare::new(),index:0,found:None,has:false,matched:None,number:[0;20],length:0,formatted:false,done:false}}
 pub(super) fn has(&self)->bool{self.has}
 pub(super) fn declared(&self)->bool{self.found.is_some()}
 pub(super) fn types<'a>(&self,widget:&'a Widget,infos:&'a HistoryFoldIndex<String,OperatorInfo>,input:bool)->PortTypes<'a>{self.found.and_then(|i|port_at(widget,infos,input,i).1).map_or(PortTypes::Empty,|p|p.types)}
 fn paid(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()};if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub(super) fn step(&mut self,widget:Option<&Widget>,infos:&HistoryFoldIndex<String,OperatorInfo>,input:bool,requested:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.done{return Ok(true)}let Some(widget)=widget else{if Self::paid(cx)?{self.done=true}return Ok(self.done)};
  let(end,port)=port_at(widget,infos,input,self.index);if end{if Self::paid(cx)?{self.done=true}return Ok(self.done)}let Some(port)=port else{if Self::paid(cx)?{self.index+=1}return Ok(false)};
  if !self.formatted{if !Self::paid(cx)?{return Ok(false)}self.has=true;if let PortName::Number(mut value)=port.name{let mut digits=[0;20];let mut n=0;loop{digits[n]=b'0'+(value%10)as u8;n+=1;value/=10;if value==0{break}}for i in 0..n{self.number[i]=digits[n-i-1]}self.length=n}self.formatted=true;return Ok(false)}
  let name=match port.name{PortName::Text(text)=>text,PortName::Number(_)=>std::str::from_utf8(&self.number[..self.length]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original numeric port text is invalid"))?};
  if self.matched.is_none(){self.matched=self.compare.step(name,requested,cx)?;return Ok(false)}if !self.compare.reset(cx)?{return Ok(false)}if self.matched.take().unwrap(){self.found=Some(self.index);self.done=true}else{self.index+=1;self.formatted=false}Ok(self.done)
 }
}
