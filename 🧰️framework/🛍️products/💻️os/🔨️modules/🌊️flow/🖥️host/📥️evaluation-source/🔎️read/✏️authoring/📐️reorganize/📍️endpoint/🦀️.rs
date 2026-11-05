//! 📍️ Original handle keys and node fallbacks resolve through paid virtual-label binary searches.
use super::{Handle,fixed,paid,invalid};
use super::order::{Label,TextOrder};
use super::super::ports::{port_at,PortName};
use super::super::super::{HistoryFoldIndex,OperatorInfo};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,widget_id_for};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::RetireOwned;
use std::mem::size_of;
#[derive(RetireOwned)]
struct FindWork{lex:TextOrder,left:usize,right:usize,middle:usize,pending:i8,phase:u8,found:Option<usize>,started:bool}
impl FindWork{
 fn new()->Self{Self{lex:TextOrder::new(),left:0,right:0,middle:0,pending:0,phase:0,found:None,started:false}}
 fn step<'a>(&mut self,length:usize,query:Label<'a>,label:impl Fn(usize)->Label<'a>,cx:&mut StepContext<'_>)->Result<Option<Option<usize>>,OriginalTurnFailure>{
  if !self.started{if paid(cx,fixed(size_of::<Self>()))?{self.right=length;self.started=true}return Ok(None)}
  match self.phase{
   0=>{if self.left>=self.right{return Ok(Some(self.found))}if paid(cx,fixed(size_of::<Self>()))?{self.middle=self.left+(self.right-self.left)/2;self.phase=1}},
   1=>{if self.lex.result().is_none(){self.lex.step(label(self.middle),query,cx)?;return Ok(None)}if paid(cx,fixed(size_of::<i8>()+size_of::<u8>()))?{self.pending=self.lex.result().unwrap();self.phase=2}},
   2=>{if self.lex.reset(cx)?{self.phase=3}},
   3=>{if paid(cx,fixed(size_of::<Self>()))?{if self.pending==0{self.found=Some(self.middle)}if self.pending<=0{self.left=self.middle+1}else{self.right=self.middle}self.phase=0}},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
}
pub(super) fn handle_label<'a>(source:&'a dyn FlowSnapshotSource,infos:&'a HistoryFoldIndex<String,OperatorInfo>,nodes:&[usize],handle:Handle)->Label<'a>{
 let widget=&source.widgets()[nodes[handle.node]];let port=port_at(widget,infos,handle.input,handle.port).1.expect("original layout handle remains declared");
 Label{head:widget_id_for(widget),tail:Some(port.name)}
}
#[derive(RetireOwned)]
pub(super) struct EndpointWork{search:FindWork,phase:u8,widget:Option<usize>,port:Option<usize>,scan:usize,base:usize,target:Option<usize>,done:bool}
impl EndpointWork{
 pub(super) fn new()->Self{Self{search:FindWork::new(),phase:0,widget:None,port:None,scan:0,base:0,target:None,done:false}}
 fn query<'a>(&self,source:&'a dyn FlowSnapshotSource,infos:&'a HistoryFoldIndex<String,OperatorInfo>,raw:&'a str,port:&'a str,input:bool)->Label<'a>{
  let name=match(self.widget,self.port){(Some(widget),Some(index))=>port_at(&source.widgets()[widget],infos,input,index).1.expect("original resolved port remains declared").name,_=>PortName::Text(port)};Label{head:raw,tail:Some(name)}
 }
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,raw:&str,port:&str,input:bool,nodes:&[usize],handles:&[Handle],handle_order:&[usize],cx:&mut StepContext<'_>)->Result<Option<Option<usize>>,OriginalTurnFailure>{
  if self.done{return Ok(Some(self.target))}
  match self.phase{
   0=>{if paid(cx,fixed(size_of::<Self>()))?{self.phase=if port.is_empty(){1}else{3}}},
   1=>{if let Some(found)=self.search.step(nodes.len(),Label::text(raw),|index|Label::text(widget_id_for(&source.widgets()[nodes[index]])),cx)?{if paid(cx,fixed(size_of::<Self>()))?{self.widget=found.map(|index|nodes[index]);self.phase=if self.widget.is_some(){2}else{3};}}},
   2=>{if paid(cx,fixed(size_of::<Self>()))?{let(end,found)=port_at(&source.widgets()[self.widget.unwrap()],infos,input,self.scan);if found.is_some(){self.port=Some(self.scan);self.phase=3}else if end{self.phase=3}else{self.scan+=1}}},
   3=>{if paid(cx,fixed(size_of::<Self>()))?{self.search=FindWork::new();self.phase=4}},
   4=>{let query=self.query(source,infos,raw,port,input);if let Some(found)=self.search.step(handle_order.len(),query,|index|handle_label(source,infos,nodes,handles[handle_order[index]]),cx)?{if paid(cx,fixed(size_of::<Self>()))?{if let Some(index)=found{self.target=Some(handles[handle_order[index]].node);self.done=true}else{self.phase=5}}}},
   5=>{if paid(cx,fixed(size_of::<Self>()))?{self.search=FindWork::new();self.phase=6}},
   6=>{let query=self.query(source,infos,raw,port,input);if let Some(found)=self.search.step(nodes.len(),query,|index|Label::text(widget_id_for(&source.widgets()[nodes[index]])),cx)?{if paid(cx,fixed(size_of::<Self>()))?{if found.is_some(){self.target=found;self.done=true}else{self.scan=0;self.phase=7}}}},
   7=>{let count=raw.len().saturating_sub(self.scan).min(64);if paid(cx,fixed(size_of::<Self>()+count))?{let end=self.scan+count;let delimiter=raw.as_bytes()[self.scan..end].iter().position(|byte|*byte==b'@');if let Some(offset)=delimiter{self.base=self.scan+offset;self.phase=8}else if end==raw.len(){self.base=end;self.phase=8}else{self.scan=end}}},
   8=>{if paid(cx,fixed(size_of::<Self>()))?{self.search=FindWork::new();self.phase=9}},
   9=>{if let Some(found)=self.search.step(nodes.len(),Label::text(&raw[..self.base]),|index|Label::text(widget_id_for(&source.widgets()[nodes[index]])),cx)?{if paid(cx,fixed(size_of::<Self>()))?{self.target=found;self.done=true}}},
   _=>return Err(invalid().into()),
  }Ok(self.done.then_some(self.target))
 }
}
