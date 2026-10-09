//! 🎛️ Pure retained host preparation exchanges admitted numeric owners after complete projection.
use super::*;
use crate::infinite::board::io::text::dag_input::retained::DagInputFacts;
use semio_framework_value::{NativeDecodeControl,ValueError,numeric_scratch::NumericInsertCursor,retirement::*};

pub struct DagInputApplication{facts:Option<DagInputFacts>,selection:Selection,active:Option<NodeId>,stale:NumericSet<NodeId>,statuses:NumericIndex<NodeId,DagNodeEvalStatusKind>,ports:NumericSet<(NodeId,usize)>,field:usize,row:usize,port:usize,identity:IdentityCursor,node:Option<NodeId>,phase:u8,channels:bool,id_insert:Option<NumericInsertCursor<u64,()>>,status_insert:Option<NumericInsertCursor<u64,DagNodeEvalStatusKind>>,port_insert:Option<NumericInsertCursor<(u64,usize),()>>,complete:bool,progress:semio_framework_value::retained_clone::RetainedCloneProgress}
pub struct DagInputDisplaced{selection:Option<(Selection,Selection,Selection)>,chrome:Option<(Option<NodeId>,NumericSet<NodeId>,NumericIndex<NodeId,DagNodeEvalStatusKind>,NumericSet<(NodeId,usize)>)>,progress:Option<(Option<NodeId>,NumericSet<NodeId>)>}
impl DagInputDisplaced{pub fn empty()->Self{Self{selection:None,chrome:None,progress:None}}}
#[derive(Default)]
struct IdentityCursor{candidate:usize,byte:usize}
struct Identity<'a>{first:&'a str,second:Option<&'a str>}
impl Identity<'_>{fn len(&self)->usize{self.first.len()+self.second.map_or(0,|text|text.len()+1)}fn byte(&self,index:usize)->Option<u8>{if index<self.first.len(){self.first.as_bytes().get(index).copied()}else if index==self.first.len()&&self.second.is_some(){Some(b'@')}else{self.second?.as_bytes().get(index-self.first.len()-1).copied()}}}
enum IdentityStep{Progress,Resolved(u64),Missing}
impl IdentityCursor{
 fn step(&mut self,host:&DagHost,target:Identity<'_>,domain:usize,node:Option<u64>)->IdentityStep{
  let count=match domain{0=>host.host_snapshot.nodes.len(),1=>host.host_snapshot.edges.len(),2=>host.handle_key_map.len(),_=>node.and_then(|id|host.host_snapshot.nodes.get((id-1)as usize)).map_or(0,|node|node.inputs().len())};
  if self.candidate==count{return IdentityStep::Missing}
  let id=match domain{0=>self.candidate as u64+1,1=>host.edge_engine_ids.get(self.candidate).and_then(|id|*id).unwrap_or(0),2=>self.candidate as u64+10,_=>self.candidate as u64};
  let candidate=match domain{0=>host.host_snapshot.nodes.get(self.candidate).filter(|_|host.node_id_map.get(&id)==Some(&self.candidate)).map(|node|node.id.as_str()),1=>host.host_snapshot.edges.get(self.candidate).filter(|_|id!=0).map(|edge|edge.id.as_str()),2=>host.handle_key_map.get(&id).filter(|_|host.engine.handles.contains_key(&id)).map(String::as_str),_=>node.and_then(|id|host.host_snapshot.nodes.get((id-1)as usize)).and_then(|node|node.inputs().get(self.candidate)).map(|port|port.id.as_str())};
  let Some(candidate)=candidate else{self.candidate+=1;self.byte=0;return IdentityStep::Progress};
  if candidate.len()!=target.len()||candidate.as_bytes().get(self.byte).copied()!=target.byte(self.byte){self.candidate+=1;self.byte=0;return IdentityStep::Progress}
  if self.byte==target.len(){return IdentityStep::Resolved(id)}self.byte+=1;IdentityStep::Progress
 }
}
impl DagInputApplication{
 pub fn new(facts:DagInputFacts)->Self{Self{facts:Some(facts),selection:Selection::default(),active:None,stale:NumericSet::new(),statuses:NumericIndex::new(),ports:NumericSet::new(),field:0,row:0,port:0,identity:IdentityCursor::default(),node:None,phase:0,channels:false,id_insert:None,status_insert:None,port_insert:None,complete:false,progress:Default::default()}}
 pub fn normal_step_progress(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.progress}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_capacity_byte_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_capacity_byte_demand(&self.statuses),6=>self.ports.insert_capacity_byte_demand(self.port_insert.as_ref().unwrap()),_=>Ok(0)}}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_depth_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_depth_demand(&self.statuses),6=>self.ports.insert_depth_demand(self.port_insert.as_ref().unwrap()),_=>Ok(1)}}
 pub fn step(&mut self,host:&DagHost,units:usize,control:&mut NativeDecodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<bool,ValueError>{
  self.progress=Default::default();if units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}control.checkpoint()?;if self.complete{return Ok(true)}if self.next_capacity_byte_demand()?>grant.maximum_capacity_bytes||self.next_depth_demand()?>grant.maximum_depth{return Ok(false)}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;let before=control.owned_bytes();let result=self.advance(host,control);self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;result?;control.step()?;Ok(self.complete)
 }
 fn next_row(&mut self){self.row+=1;self.phase=0;self.identity=IdentityCursor::default();self.node=None;self.port=0;}
 fn advance(&mut self,host:&DagHost,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  let facts=self.facts.as_ref().unwrap();
  if self.phase==0{
   let present=match facts{
    DagInputFacts::Progress(facts)=>if self.field==0{if facts.active.is_none(){self.field=1;return Ok(())}true}else{self.row<facts.stale.len()},
    DagInputFacts::Statuses(facts)=>self.row<facts.len(),
    DagInputFacts::Selection(facts)=>{let rows=match self.field{0=>&facts.nodes,1=>&facts.edges,_=>&facts.handles};if self.row==rows.len(){self.field+=1;self.row=0;if self.field==3{self.complete=true;}return Ok(())}true},
    DagInputFacts::Channels(facts)=>{self.channels=host.draw_lod_for_frame().uses_channel_row_pick();self.row<facts.len()},
   };if !present{self.complete=true;return Ok(())}self.phase=1;return Ok(())
  }
  if self.phase==1{
   let(target,domain)=match facts{
    DagInputFacts::Progress(facts)=>(Identity{first:if self.field==0{facts.active.as_deref().unwrap()}else{facts.stale[self.row].as_str()},second:None},0),
    DagInputFacts::Statuses(facts)=>(Identity{first:facts.entry_at_rank(self.row).unwrap().0,second:None},0),
    DagInputFacts::Selection(facts)=>(Identity{first:match self.field{0=>&facts.nodes[self.row],1=>&facts.edges[self.row],_=>&facts.handles[self.row]},second:None},self.field),
    DagInputFacts::Channels(facts)=>(Identity{first:&facts[self.row].widget_id,second:self.channels.then_some(facts[self.row].port.as_str())},if self.channels{2}else{0}),
   };
   match self.identity.step(host,target,domain,None){IdentityStep::Progress=>{},IdentityStep::Missing=>{if matches!(facts,DagInputFacts::Progress(_))&&self.field==0{self.field=1;self.identity=IdentityCursor::default();self.phase=0;}else{self.next_row()}},IdentityStep::Resolved(id)=>{
    self.node=Some(id);self.identity=IdentityCursor::default();self.phase=2;
   }}return Ok(())
  }
  if self.phase==2{
   let id=self.node.unwrap();match facts{
    DagInputFacts::Progress(_)=>{if self.field==0{self.active=Some(id);self.field=1;self.phase=0;self.identity=IdentityCursor::default();return Ok(())}if self.active==Some(id){self.next_row();return Ok(())}self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
    DagInputFacts::Statuses(facts)=>{let status=facts.entry_at_rank(self.row).unwrap().1;let kind=match status{DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,DagNodeEvaluationStatus::Queued{}=>DagNodeEvalStatusKind::Queued,DagNodeEvaluationStatus::Computing{}=>{self.active=Some(id);DagNodeEvalStatusKind::Computing},DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,DagNodeEvaluationStatus::Blocked{..}=>DagNodeEvalStatusKind::Blocked};self.status_insert=Some(NumericInsertCursor::new(id,kind));self.phase=4;},
    DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>{self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
   }return Ok(())
  }
  if self.phase==3{
   let output=match facts{DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&mut self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&mut self.selection.node_ids,1=>&mut self.selection.edge_ids,_=>&mut self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&mut self.selection.handle_ids}else{&mut self.selection.node_ids}};
   if output.insert_step(self.id_insert.as_mut().unwrap(),1,control)?{self.id_insert.take();self.next_row();}return Ok(())
  }
  if self.phase==4{
   if self.status_insert.as_mut().unwrap().step(&mut self.statuses,1,control)?{self.status_insert.take();let DagInputFacts::Statuses(facts)=facts else{unreachable!()};match facts.entry_at_rank(self.row).unwrap().1{DagNodeEvaluationStatus::Queued{}=>{self.id_insert=Some(NumericInsertCursor::new(self.node.unwrap(),()));self.phase=3},DagNodeEvaluationStatus::Blocked{..}=>self.phase=5,_=>self.next_row()}}return Ok(())
  }
  if self.phase==5{
   let DagInputFacts::Statuses(facts)=facts else{unreachable!()};let DagNodeEvaluationStatus::Blocked{ports}=facts.entry_at_rank(self.row).unwrap().1 else{unreachable!()};if self.port==ports.len(){self.next_row();return Ok(())}
   match self.identity.step(host,Identity{first:&ports[self.port],second:None},3,self.node){IdentityStep::Progress=>{},IdentityStep::Missing=>{self.port+=1;self.identity=IdentityCursor::default()},IdentityStep::Resolved(port)=>{self.port_insert=Some(NumericInsertCursor::new((self.node.unwrap(),port as usize),()));self.phase=6;}}return Ok(())
  }
  if self.ports.insert_step(self.port_insert.as_mut().unwrap(),1,control)?{self.port_insert.take();self.identity.candidate+=1;self.identity.byte=0;self.phase=5;}Ok(())
 }
 pub fn commit(&mut self,host:&mut DagHost)->DagInputDisplaced{
  assert!(self.complete);let mut displaced=DagInputDisplaced{selection:None,chrome:None,progress:None};
  match self.facts.as_ref().unwrap(){
   DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>displaced.selection=Some((std::mem::replace(&mut host.engine.selection,std::mem::take(&mut self.selection)),std::mem::take(&mut host.engine.preselect),std::mem::take(&mut host.engine.preselect_removed))),
   DagInputFacts::Progress(_)=>displaced.progress=Some((std::mem::replace(&mut host.computing_active,self.active),std::mem::replace(&mut host.computing_stale,std::mem::take(&mut self.stale)))),
   DagInputFacts::Statuses(_)=>displaced.chrome=Some((std::mem::replace(&mut host.computing_active,self.active),std::mem::replace(&mut host.computing_stale,std::mem::take(&mut self.stale)),std::mem::replace(&mut host.node_eval_status,std::mem::take(&mut self.statuses)),std::mem::replace(&mut host.unresolved_input_ports,std::mem::take(&mut self.ports)))),
  }displaced
 }
}
semio_framework_value::artifact_retire_struct!(DagInputDisplaced{selection,chrome,progress});
semio_framework_value::artifact_retire_struct!(IdentityCursor{candidate,byte});
semio_framework_value::artifact_retire_struct!(DagInputApplication{facts,selection,active,stale,statuses,ports,field,row,port,identity,node,phase,channels,id_insert,status_insert,port_insert,complete,progress});
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
