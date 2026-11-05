//! 📐️ Reorganization retains original graph labels, exact numeric geometry and funded mutation custody.
use super::super::*;
use super::{text::FundedText,reach::ReachWork,ports::port_at};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowMutation,FlowLayoutEntry,ChangeLayout,WidgetLayout,widget_id_for};
use semio_os_flow_host_core::authoring_layout::{LayeredLayoutCursor,LayeredLayoutNode,LayeredLayoutEdge};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[path="🔠️order/🦀️.rs"]mod order;
#[path="📍️endpoint/🦀️.rs"]mod endpoint;
use order::{IndexOrder,TextOrder,Label};
use endpoint::{EndpointWork,handle_label};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original reorganization changed its graph or funded frontier")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn paid(cx:&mut StepContext<'_>,d:RetirementDemand)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(true)}
fn quote<T>(count:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{capacity_bytes:count.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..fixed(size_of::<Option<Vec<T>>>())})}
fn reserve<T>(slot:&mut Option<Vec<T>>,count:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if slot.is_some(){return Ok(true)}let d=quote::<T>(count)?;admit_original_construction(cx,slot,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{
  let result=(||{let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original reorganization storage allocation refused"))?;let actual=output.capacity().checked_mul(size_of::<T>()).ok_or_else(invalid)?;Ok((output,RetainedCloneProgress{retained_capacity_bytes:actual,..receipt(d)}))})();RetainedWorkOutcome{result,work:1}
 })
}
#[derive(Clone,Copy,RetireOwned)]
struct Handle{node:usize,port:usize,input:bool}
#[derive(RetireOwned)]
struct Fields{order:Option<Vec<usize>>,scratch:Option<Vec<usize>>,reverse:Option<Vec<usize>>,handles:Option<Vec<Handle>>,handle_order:Option<Vec<usize>>,handle_scratch:Option<Vec<usize>>,nodes:Option<Vec<LayeredLayoutNode>>,edges:Option<Vec<LayeredLayoutEdge>>,entries:Option<Vec<FlowLayoutEntry>>,output:Option<Vec<FlowMutation>>,id:Option<String>,error:Option<FlowCoreError>,sort:Option<IndexOrder>,lex:Option<TextOrder>,endpoint:Option<EndpointWork>,kernel:Option<LayeredLayoutCursor>}
pub(super) struct ReorganizeWork{fields:ManuallyDrop<Fields>,reach:Option<ReachWork>,text:Option<FundedText>,retirement:RetainedCloneClose,phase:u8,scan:usize,kept:usize,handles:usize,input:bool,port:usize,blocked:bool,from:Option<usize>,to:Option<usize>,a:usize,b:usize,selection:Option<(bool,bool)>,count:usize,closing:bool}
impl ReorganizeWork{
 pub(super) fn new()->Self{Self{fields:ManuallyDrop::new(Fields{order:None,scratch:None,reverse:None,handles:None,handle_order:None,handle_scratch:None,nodes:None,edges:None,entries:None,output:None,id:None,error:None,sort:None,lex:Some(TextOrder::new()),endpoint:None,kernel:None}),reach:None,text:None,retirement:Default::default(),phase:0,scan:0,kept:0,handles:0,input:true,port:0,blocked:false,from:None,to:None,a:0,b:0,selection:None,count:0,closing:false}}
 pub(super) fn next_demands(&self,source:&dyn FlowSnapshotSource,_infos:&HistoryFoldIndex<String,OperatorInfo>,_options:&DagLayoutOptions,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(copy)}Ok(match self.phase{
   0|1=>quote::<usize>(source.widgets().len())?,7 if self.fields.order.as_ref().is_some_and(|rows|rows.len()==self.kept)&&self.fields.reverse.is_none()=>quote::<usize>(source.widgets().len())?,
   11=>quote::<Handle>(self.handles)?,12|13=>quote::<usize>(self.handles)?,17=>quote::<LayeredLayoutNode>(self.kept.checked_add(1).ok_or_else(invalid)?)?,19=>quote::<LayeredLayoutEdge>(source.synapses().len())?,
   21=>return self.reach.as_ref().ok_or_else(invalid)?.next_demands(source,copy),
   22=>{let reach=self.reach.as_ref().ok_or_else(invalid)?;if !reach.terminal_is_empty(){return reach.close_demands(copy)}fixed(size_of::<Option<ReachWork>>())},
   31=>quote::<FlowLayoutEntry>(self.count)?,35=>{if let Some(text)=self.text.as_ref(){if self.fields.id.is_none(){return text.next_demands(self.selected(source)?.0,copy)}if !text.terminal_is_empty(){return text.close_demands(copy)}}fixed(size_of::<Self>()+size_of::<FundedText>())},
   37=>quote::<FlowMutation>(usize::from(self.count>0))?,
   _=>fixed(size_of::<Self>().max(LayeredLayoutCursor::maximum_step_copy_bytes()).max(size_of::<IndexOrder>()+256)),
  })
 }
 fn port_step(&mut self,end:bool){if end{self.port=0;if self.input{self.input=false}else{self.input=true;self.scan+=1}}else{self.port+=1}}
 fn reset_ports(&mut self){self.scan=0;self.port=0;self.input=true}
 fn selected<'a>(&self,source:&'a dyn FlowSnapshotSource)->Result<(&'a str,WidgetLayout),ValueError>{
  let(node,_)=self.selection.ok_or_else(invalid)?;if node{let index=*self.fields.order.as_ref().and_then(|rows|rows.get(self.a)).ok_or_else(invalid)?;let(x,y)=self.fields.nodes.as_ref().and_then(|rows|rows.get(self.a)).ok_or_else(invalid)?.position();Ok((widget_id_for(&source.widgets()[index]),WidgetLayout{x,y}))}else{let(id,layout)=source.layout().entry_at_rank(self.b).ok_or_else(invalid)?;Ok((id,WidgetLayout{x:layout.x,y:layout.y}))}
 }
 fn select(&mut self,source:&dyn FlowSnapshotSource,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Fields{order,lex,..}=&mut*self.fields;let nodes=order.as_ref().ok_or_else(invalid)?;if self.a>=nodes.len()&&self.b>=source.layout().len(){return Ok(true)}
  if self.a>=nodes.len()||self.b>=source.layout().len(){if paid(cx,fixed(size_of::<Option<(bool,bool)>>()))?{self.selection=Some((self.b>=source.layout().len(),self.a>=nodes.len()))}return Ok(false)}
  let lex=lex.as_mut().ok_or_else(invalid)?;if lex.result().is_none(){lex.step(Label::text(widget_id_for(&source.widgets()[nodes[self.a]])),Label::text(source.layout().entry_at_rank(self.b).ok_or_else(invalid)?.0),cx)?;return Ok(false)}
  if paid(cx,fixed(size_of::<Option<(bool,bool)>>()))?{let result=lex.result().unwrap();self.selection=Some((result<=0,result>=0))}Ok(false)
 }
 fn copy_id(&mut self,source:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.fields.id.is_some()&&self.text.is_none(){return Ok(true)}
  if self.text.is_none(){if paid(cx,fixed(size_of::<Option<FundedText>>()))?{self.text=Some(FundedText::new())}return Ok(false)}
  let text=self.text.as_mut().unwrap();if self.fields.id.is_none(){if text.ready(){text.take_into(&mut self.fields.id,cx)?;if self.fields.id.is_some(){text.begin_close()}}else{text.advance(source,cx)?;}return Ok(false)}
  if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|text.close_step(grant))?;return Ok(false)}
  if !paid(cx,fixed(size_of::<Option<FundedText>>()))?{return Ok(false)}self.text=None;Ok(true)
 }
 pub(super) fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,options:&DagLayoutOptions,cx:&mut StepContext<'_>)->Result<Option<Result<Vec<FlowMutation>,FlowCoreError>>,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}
  match self.phase{
   0=>if reserve(&mut self.fields.order,source.widgets().len(),cx)?{self.phase=1},
   1=>if reserve(&mut self.fields.scratch,source.widgets().len(),cx)?{self.phase=2},
   2=>{if self.scan<source.widgets().len(){if paid(cx,fixed(2*size_of::<usize>()+size_of::<Self>()))?{let index=self.scan;self.fields.order.as_mut().unwrap().push(index);self.fields.scratch.as_mut().unwrap().push(0);self.scan+=1}}else if paid(cx,fixed(size_of::<Self>()))?{self.phase=3}},
   3=>if paid(cx,fixed(size_of::<IndexOrder>()+size_of::<Self>()))?{self.fields.sort=Some(IndexOrder::new());self.phase=4},
   4=>{let Fields{order,scratch,sort,..}=&mut*self.fields;if sort.as_mut().unwrap().step(order.as_mut().unwrap(),scratch.as_mut().unwrap(),|index|Label::text(widget_id_for(&source.widgets()[index])),cx)?{self.phase=5;self.scan=0}},
   5=>{if self.scan>=self.fields.order.as_ref().unwrap().len(){if paid(cx,fixed(size_of::<Self>()))?{self.phase=7}return Ok(None)}if self.kept==0{if paid(cx,fixed(size_of::<Self>()+size_of::<usize>()))?{self.kept=1;self.scan+=1}return Ok(None)}
    let Fields{order,lex,..}=&mut*self.fields;let rows=order.as_ref().unwrap();let left=widget_id_for(&source.widgets()[rows[self.kept-1]]);let right=widget_id_for(&source.widgets()[rows[self.scan]]);let lex=lex.as_mut().unwrap();if lex.result().is_none(){lex.step(Label::text(left),Label::text(right),cx)?;}else if paid(cx,fixed(size_of::<Self>()))?{self.blocked=lex.result()==Some(0);self.phase=6}
   },
   6=>{if self.fields.lex.as_mut().unwrap().reset(cx)?{if !self.blocked{let value=self.fields.order.as_ref().unwrap()[self.scan];self.fields.order.as_mut().unwrap()[self.kept]=value;self.kept+=1}self.scan+=1;self.phase=5}},
   7=>{if self.fields.order.as_ref().unwrap().len()>self.kept{if paid(cx,fixed(size_of::<usize>()+size_of::<Vec<usize>>()))?{self.fields.order.as_mut().unwrap().pop();}}else if reserve(&mut self.fields.reverse,source.widgets().len(),cx)?{self.scan=0;self.phase=8}},
   8=>{if self.scan<source.widgets().len(){if paid(cx,fixed(size_of::<usize>()+size_of::<Self>()))?{self.fields.reverse.as_mut().unwrap().push(usize::MAX);self.scan+=1}}else if paid(cx,fixed(size_of::<Self>()))?{self.scan=0;self.phase=9}},
   9=>{if self.scan<self.kept{if paid(cx,fixed(size_of::<usize>()+size_of::<Self>()))?{let original=self.fields.order.as_ref().unwrap()[self.scan];self.fields.reverse.as_mut().unwrap()[original]=self.scan;self.scan+=1}}else if paid(cx,fixed(size_of::<Self>()))?{self.reset_ports();self.phase=10}},
   10=>{if self.scan>=source.widgets().len(){if paid(cx,fixed(size_of::<Self>()))?{self.phase=11}return Ok(None)}if paid(cx,fixed(size_of::<Self>()+size_of::<Handle>()))?{if self.fields.reverse.as_ref().unwrap()[self.scan]==usize::MAX{self.scan+=1}else{let(end,port)=port_at(&source.widgets()[self.scan],infos,self.input,self.port);if port.is_some(){self.handles=self.handles.checked_add(1).ok_or_else(invalid)?}self.port_step(end)}}},
   11=>if reserve(&mut self.fields.handles,self.handles,cx)?{self.phase=12},
   12=>if reserve(&mut self.fields.handle_order,self.handles,cx)?{self.phase=13},
   13=>if reserve(&mut self.fields.handle_scratch,self.handles,cx)?{self.reset_ports();self.phase=14},
   14=>{if self.scan>=source.widgets().len(){if paid(cx,fixed(size_of::<Self>()))?{self.phase=15}return Ok(None)}if paid(cx,fixed(size_of::<Self>()+size_of::<Handle>()+2*size_of::<usize>()))?{let node=self.fields.reverse.as_ref().unwrap()[self.scan];if node==usize::MAX{self.scan+=1}else{let(end,port)=port_at(&source.widgets()[self.scan],infos,self.input,self.port);if port.is_some(){let index=self.fields.handles.as_ref().unwrap().len();self.fields.handles.as_mut().unwrap().push(Handle{node,port:self.port,input:self.input});self.fields.handle_order.as_mut().unwrap().push(index);self.fields.handle_scratch.as_mut().unwrap().push(0);}self.port_step(end)}}},
   15=>if paid(cx,fixed(size_of::<IndexOrder>()+size_of::<Self>()))?{self.fields.sort=Some(IndexOrder::new());self.phase=16},
   16=>{let Fields{order,handles,handle_order,handle_scratch,sort,..}=&mut*self.fields;let ids=order.as_ref().unwrap();let rows=handles.as_ref().unwrap();if sort.as_mut().unwrap().step(handle_order.as_mut().unwrap(),handle_scratch.as_mut().unwrap(),|index|handle_label(source,infos,ids,rows[index]),cx)?{self.phase=17}},
   17=>if reserve(&mut self.fields.nodes,self.kept.checked_add(1).ok_or_else(invalid)?,cx)?{self.scan=0;self.phase=18},
   18=>{if self.scan<=self.kept{if paid(cx,fixed(size_of::<LayeredLayoutNode>()+size_of::<Self>()))?{self.fields.nodes.as_mut().unwrap().push(LayeredLayoutNode::new());self.scan+=1}}else if paid(cx,fixed(size_of::<Self>()))?{self.phase=19}},
   19=>if reserve(&mut self.fields.edges,source.synapses().len(),cx)?{self.scan=0;self.phase=20},
   20=>{if self.scan>=source.synapses().len(){if paid(cx,fixed(size_of::<LayeredLayoutCursor>()+size_of::<Self>()))?{self.fields.kernel=Some(LayeredLayoutCursor::new());self.phase=27}return Ok(None)}if paid(cx,fixed(size_of::<Option<ReachWork>>()+size_of::<Self>()))?{self.reach=Some(ReachWork::new());self.phase=21}},
   21=>{let wire=&source.synapses()[self.scan];if let Some(blocked)=self.reach.as_mut().unwrap().step(source,&wire.from,&wire.to,true,cx)?{self.blocked=blocked;self.reach.as_mut().unwrap().begin_close();self.phase=22}},
   22=>{let reach=self.reach.as_mut().unwrap();if !reach.terminal_is_empty(){let demand=reach.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|reach.close_step(grant))?;return Ok(None)}if paid(cx,fixed(size_of::<Option<ReachWork>>()+size_of::<Self>()+size_of::<EndpointWork>()))?{self.reach=None;if self.blocked{self.scan+=1;self.phase=20}else{self.fields.endpoint=Some(EndpointWork::new());self.phase=23}}},
   23=>{let wire=&source.synapses()[self.scan];let Fields{order,handles,handle_order,endpoint,..}=&mut*self.fields;if let Some(found)=endpoint.as_mut().unwrap().step(source,infos,&wire.from,&wire.from_port,false,order.as_ref().unwrap(),handles.as_ref().unwrap(),handle_order.as_ref().unwrap(),cx)?{self.from=found;self.phase=if found.is_some(){24}else{26}}},
   24=>if paid(cx,fixed(size_of::<EndpointWork>()+size_of::<Self>()))?{self.fields.endpoint=Some(EndpointWork::new());self.phase=25},
   25=>{let wire=&source.synapses()[self.scan];let Fields{order,handles,handle_order,endpoint,..}=&mut*self.fields;if let Some(found)=endpoint.as_mut().unwrap().step(source,infos,&wire.to,&wire.to_port,true,order.as_ref().unwrap(),handles.as_ref().unwrap(),handle_order.as_ref().unwrap(),cx)?{self.to=found;self.phase=26}},
   26=>if paid(cx,fixed(size_of::<EndpointWork>()+size_of::<LayeredLayoutEdge>()+size_of::<Self>()))?{if let(Some(source),Some(target))=(self.from,self.to){if source!=target{self.fields.edges.as_mut().unwrap().push(LayeredLayoutEdge{source,target})}}self.fields.endpoint=None;self.from=None;self.to=None;self.scan+=1;self.phase=20},
   27=>{if paid(cx,fixed(LayeredLayoutCursor::maximum_step_copy_bytes()))?{let Fields{kernel,nodes,edges,..}=&mut*self.fields;if kernel.as_mut().unwrap().step(nodes.as_mut().unwrap(),edges.as_ref().unwrap(),options).map_err(|_|invalid())?{self.a=0;self.b=0;self.count=0;self.phase=29}}},
   29=>{if self.select(source,cx)?{if paid(cx,fixed(size_of::<Self>()))?{self.phase=31}}else if self.selection.is_some(){self.phase=30}},
   30=>if self.fields.lex.as_mut().unwrap().reset(cx)?{let(a,b)=self.selection.take().unwrap();self.a+=usize::from(a);self.b+=usize::from(b);self.count=self.count.checked_add(1).ok_or_else(invalid)?;self.phase=29},
   31=>if reserve(&mut self.fields.entries,self.count,cx)?{self.a=0;self.b=0;self.phase=33},
   33=>{if self.select(source,cx)?{if paid(cx,fixed(size_of::<Self>()))?{self.phase=37}}else if self.selection.is_some(){self.phase=34}},
   34=>if self.fields.lex.as_mut().unwrap().reset(cx)?{self.phase=35},
   35=>{let id=self.selected(source)?.0;if self.copy_id(id,cx)?{self.phase=36}},
   36=>if paid(cx,fixed(size_of::<FlowLayoutEntry>()+size_of::<Self>()))?{let layout=self.selected(source)?.1;let id=self.fields.id.take().ok_or_else(invalid)?;self.fields.entries.as_mut().unwrap().push(FlowLayoutEntry{id,layout:Some(layout)});let(a,b)=self.selection.take().unwrap();self.a+=usize::from(a);self.b+=usize::from(b);self.phase=33},
   37=>if reserve(&mut self.fields.output,usize::from(self.count>0),cx)?{self.phase=38},
   38=>if paid(cx,fixed(size_of::<FlowMutation>()+size_of::<Option<Vec<FlowLayoutEntry>>>()+size_of::<Self>()))?{let entries=self.fields.entries.take().unwrap();if !entries.is_empty(){self.fields.output.as_mut().unwrap().push(FlowMutation::ChangeLayout(ChangeLayout{entries}))}self.phase=39},
   39=>if paid(cx,fixed(size_of::<Option<Vec<FlowMutation>>>()+size_of::<Self>()))?{self.phase=40;return Ok(Some(Ok(self.fields.output.take().ok_or_else(invalid)?)))},
   41=>if paid(cx,fixed(size_of::<Option<FlowCoreError>>()+size_of::<Self>()))?{self.phase=40;return Ok(Some(Err(self.fields.error.take().ok_or_else(invalid)?)))},
   40=>{},_=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,result:Result<Vec<FlowMutation>,FlowCoreError>){assert_eq!(self.phase,40);match result{Ok(output)=>{self.fields.output=Some(output);self.phase=39},Err(error)=>{self.fields.error=Some(error);self.phase=41}}}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(reach)=self.reach.as_mut(){reach.begin_close()}if let Some(text)=self.text.as_mut(){text.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(reach)=self.reach.as_ref(){return if reach.terminal_is_empty(){Ok(fixed(size_of::<Option<ReachWork>>()))}else{reach.close_demands(copy)}}if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(fixed(size_of::<Option<FundedText>>()))}else{text.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$type:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$type>(true,&None,copy)}}}
  owner!(order,Vec<usize>);owner!(scratch,Vec<usize>);owner!(reverse,Vec<usize>);owner!(handles,Vec<Handle>);owner!(handle_order,Vec<usize>);owner!(handle_scratch,Vec<usize>);owner!(nodes,Vec<LayeredLayoutNode>);owner!(edges,Vec<LayeredLayoutEdge>);owner!(entries,Vec<FlowLayoutEntry>);owner!(output,Vec<FlowMutation>);owner!(id,String);owner!(error,FlowCoreError);owner!(sort,IndexOrder);owner!(lex,TextOrder);owner!(endpoint,EndpointWork);owner!(kernel,LayeredLayoutCursor);Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(reach)=self.reach.as_mut(){if !reach.terminal_is_empty(){return reach.close_step(grant)}let d=fixed(size_of::<Option<ReachWork>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.reach=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}
  if let Some(text)=self.text.as_mut(){if !text.terminal_is_empty(){return text.close_step(grant)}let d=fixed(size_of::<Option<FundedText>>());if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.text=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(order);owner!(scratch);owner!(reverse);owner!(handles);owner!(handle_order);owner!(handle_scratch);owner!(nodes);owner!(edges);owner!(entries);owner!(output);owner!(id);owner!(error);owner!(sort);owner!(lex);owner!(endpoint);owner!(kernel);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{let f=&self.fields;self.closing&&self.reach.is_none()&&self.text.is_none()&&self.retirement.is_empty()&&f.order.is_none()&&f.scratch.is_none()&&f.reverse.is_none()&&f.handles.is_none()&&f.handle_order.is_none()&&f.handle_scratch.is_none()&&f.nodes.is_none()&&f.edges.is_none()&&f.entries.is_none()&&f.output.is_none()&&f.id.is_none()&&f.error.is_none()&&f.sort.is_none()&&f.lex.is_none()&&f.endpoint.is_none()&&f.kernel.is_none()}
}
impl Drop for ReorganizeWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original reorganization must close its exact numeric and mutation custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
