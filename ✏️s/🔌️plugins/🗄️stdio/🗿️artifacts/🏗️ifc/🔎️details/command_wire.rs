//! 📨️ Original native command bytes become the shared edit event through retained field ownership.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonError};
use std::mem::size_of;
#[derive(Clone,Copy,Default,PartialEq,Eq)]
enum Phase{#[default]Init,Parse,Outer,Inner,Operation,Fields,Construct,Ready}
#[derive(Default)]
pub(super) struct CommandWire{
 source:Option<(usize,usize)>,parser:Option<JsonGrammarCursor<DslValue>>,root:Option<DslValue>,inner:Option<DslValue>,body:Option<DslValue>,fields:[Option<DslValue>;5],pub(super) output:Option<editing::SnapshotEditEvent>,error:Option<JsonError>,retirement:RetainedCloneClose,phase:Phase,next:usize,operation:Option<u8>,closing:bool,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn object(value:&Option<DslValue>)->Result<&[(String,DslValue)],ValueError>{match value{Some(DslValue::Object(entries))=>Ok(entries),_=>Err(invalid("IFC command requires its closed object"))}}
fn child(value:&mut Option<DslValue>,index:usize)->DslValue{let Some(DslValue::Object(entries))=value else{unreachable!()};std::mem::replace(&mut entries[index].1,DslValue::Null)}
fn text(value:Option<DslValue>)->String{let Some(DslValue::String(value))=value else{unreachable!()};value}
fn parser_metadata()->usize{2*size_of::<DslValue>()+size_of::<JsonError>()+size_of::<Phase>()}
impl CommandWire{
 pub fn retains_source(&self)->bool{self.source.is_some()}
 pub fn position(&self)->usize{self.parser.as_ref().map_or(0,JsonGrammarCursor::position)}
 pub fn ready(&self)->bool{self.phase==Phase::Ready&&self.output.is_some()&&!self.closing}
 pub fn demands(&self,wire:&[u8])->Result<RetirementDemand,ValueError>{
  if self.phase==Phase::Parse{let mut demand=self.parser.as_ref().ok_or_else(||invalid("IFC original command parser absent"))?.normal_step_demands(wire).map_err(|error|invalid(match error.kind(){ValueRefusalKind::DepthLimit=>"IFC command parser depth exceeded",_=>"IFC command parser frontier refused"}))?;demand.copy_bytes=demand.copy_bytes.checked_add(parser_metadata()).ok_or_else(||invalid("IFC command parser metadata extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("IFC command parser wrapper depth overflow"))?;return Ok(demand);}
  let copy=match self.phase{Phase::Init=>2*size_of::<JsonGrammarCursor<DslValue>>()+size_of::<Option<(usize,usize)>>(),Phase::Construct=>2*size_of::<editing::SnapshotEditEvent>(),_=>2*size_of::<DslValue>()+3*size_of::<usize>()};
  Ok(RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})
 }
 pub fn advance(&mut self,wire:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"IFC original command receiving cancelled").into());}
  if self.closing{return Err(invalid("IFC original command receiver closing").into());}
  let identity=(wire.as_ptr()as usize,wire.len());if self.source.is_some_and(|original|original!=identity){return Err(invalid("IFC original command wire owner changed").into());}
  let demand=self.demands(wire)?;let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false);}
  if self.phase==Phase::Parse{
   let parser=self.parser.as_mut().unwrap();let mut progress=RetainedCloneProgress::default();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parser_metadata(),maximum_depth:grant.maximum_depth-1,..grant};
   let result=cx.with_native_receiving::<_,ValueError>(|control|{let result=parser.step_source(wire,1,control,child,false);progress=parser.normal_step_progress();progress.copied_bytes+=parser_metadata();match result{Ok(Some(value))=>{self.root=Some(value);self.phase=Phase::Outer;Ok(())},Ok(None)=>Ok(()),Err(error)=>{let kind=error.kind();self.error=Some(error);Err(ValueError::literal(kind,"IFC original command JSON refused"))}}});
   cx.consume_receiving_retained(grant,progress)?;cx.consume_fuel(progress.copied_items.max(1)as u64);result?;return Ok(false);
  }
  match self.phase{
   Phase::Init=>{if wire.len()>editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES{return Err(invalid("IFC original command exceeds its raw extent").into());}self.source=Some(identity);self.parser=Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));self.phase=Phase::Parse;},
   Phase::Outer=>{let entries=object(&self.root)?;if entries.len()!=1||entries[0].0!="EditSnapshot"{return Err(invalid("IFC command requires only its EditSnapshot wrapper").into());}self.inner=Some(child(&mut self.root,0));self.phase=Phase::Inner;},
   Phase::Inner=>{let entries=object(&self.inner)?;if entries.len()!=1||entries[0].0!="event"{return Err(invalid("IFC command wrapper requires only its event field").into());}self.body=Some(child(&mut self.inner,0));self.next=0;self.phase=Phase::Operation;},
   Phase::Operation=>{let entries=object(&self.body)?;if entries.len()>4{return Err(invalid("IFC original edit event has excess fields").into());}if self.next==entries.len(){if self.operation.is_none(){return Err(invalid("IFC original event operation absent").into());}self.next=0;self.phase=Phase::Fields;}else{let(key,value)=&entries[self.next];if key=="operation"{if self.operation.is_some(){return Err(invalid("IFC original event has duplicate operation").into());}self.operation=Some(match value.as_str(){Some("setValue")=>0,Some("insertValue")=>1,Some("removeValue")=>2,Some("moveValue")=>3,Some("renameKey")=>4,Some("replaceSource")=>5,_=>return Err(invalid("IFC original event operation unknown").into())});}self.next+=1;}},
   Phase::Fields=>{let entries=object(&self.body)?;if self.next==entries.len(){self.phase=Phase::Construct;}else{let(key,value)=&entries[self.next];if key!="operation"{let slot=match key.as_str(){"path"=>0,"value"=>1,"from"=>2,"key"=>3,"source"=>4,_=>return Err(invalid("IFC original edit event has unknown fields").into())};if self.fields[slot].is_some()||slot!=1&&!matches!(value,DslValue::String(_)){return Err(invalid("IFC original event field has invalid type or duplicate").into());}self.fields[slot]=Some(child(&mut self.body,self.next));}self.next+=1;}},
   Phase::Construct=>{let mask=self.fields.iter().enumerate().fold(0u8,|mask,(index,value)|mask|if value.is_some(){1<<index}else{0});let operation=self.operation.unwrap();let expected=match operation{0|1=>3,2=>1,3=>5,4=>9,5=>16,_=>unreachable!()};if mask!=expected{return Err(invalid("IFC original event fields do not match its operation").into());}self.output=Some(match operation{0=>editing::SnapshotEditEvent::SetValue{path:text(self.fields[0].take()),value:self.fields[1].take().unwrap()},1=>editing::SnapshotEditEvent::InsertValue{path:text(self.fields[0].take()),value:self.fields[1].take().unwrap()},2=>editing::SnapshotEditEvent::RemoveValue{path:text(self.fields[0].take())},3=>editing::SnapshotEditEvent::MoveValue{from:text(self.fields[2].take()),path:text(self.fields[0].take())},4=>editing::SnapshotEditEvent::RenameKey{path:text(self.fields[0].take()),key:text(self.fields[3].take())},5=>editing::SnapshotEditEvent::ReplaceSource{source:text(self.fields[4].take())},_=>unreachable!()});self.phase=Phase::Ready;},
   Phase::Ready=>return Ok(self.ready()),Phase::Parse=>unreachable!(),
  }
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.parser.is_none()&&self.root.is_none()&&self.inner.is_none()&&self.body.is_none()&&self.fields.iter().all(Option::is_none)&&self.output.is_none()&&self.error.is_none()&&self.retirement.is_empty()}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,4096);}
  if self.output.is_some(){return self.retirement.next_owner_demands::<editing::SnapshotEditEvent>(true,&None,4096);}
  if self.fields.iter().any(Option::is_some)||self.body.is_some()||self.inner.is_some()||self.root.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,4096);}
  if self.parser.is_some(){return self.retirement.next_owner_demands::<JsonGrammarCursor<DslValue>>(true,&None,4096);}
  if self.error.is_some(){return self.retirement.next_owner_demands::<JsonError>(true,&None,4096);}
  Ok(RetirementDemand{copy_bytes:size_of::<Option<(usize,usize)>>(),depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid("IFC original command close not started"));}if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,self.close_demands()?){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g);}
  if let Some(step)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(step);}
  for value in &mut self.fields{if let Some(step)=self.retirement.begin_granted(value,g)?{return Ok(step);}}
  for value in [&mut self.body,&mut self.inner,&mut self.root]{if let Some(step)=self.retirement.begin_granted(value,g)?{return Ok(step);}}
  if let Some(step)=self.retirement.begin_granted(&mut self.parser,g)?{return Ok(step);}if let Some(step)=self.retirement.begin_granted(&mut self.error,g)?{return Ok(step);}
  self.source=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<(usize,usize)>>(),..Default::default()}))
 }
}
#[cfg(test)]#[path="command_tests.rs"]mod tests;
