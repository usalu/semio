//! ⚖️ Original canonical source comparison advances one funded scalar or frontier transition.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as Canonical};
use semio_framework_value::list::PagedList;

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Frame{ordinal:usize,index:usize,count:usize,phase:u8,object:bool,chunks:[usize;2],offsets:[usize;2]}
impl Frame{fn new(ordinal:usize)->Self{Self{ordinal,index:0,count:0,phase:0,object:false,chunks:[0;2],offsets:[0;2]}}}
fn node_text<'a>(node:Node<'a>)->Result<Text<'a>,ValueError>{match node{Node::String(value)=>Ok(Text::from(value)),Node::Text(value)=>Ok(value),_=>Err(refusal("original compared source text role changed"))}}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct SourceCompare{frames:PagedList<Frame,{usize::MAX}>,started:bool,closing:bool,result:Option<bool>,progress:RetainedCloneProgress}
impl SourceCompare{
 pub(super) fn new()->Self{Self{frames:PagedList::empty(),started:false,closing:false,result:None,progress:Default::default()}}
 pub(super) fn result(&self)->Option<bool>{self.result}
 pub(super) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(super) fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.closing{return Ok(if self.frames.is_empty(){RetirementDemand{release_bytes:self.frames.next_release_allocation_bytes().map_err(ValueError::from)?,depth:self.frames.next_release_depth_demand().map_err(ValueError::from)?,..Default::default()}}else{RetirementDemand{copy_bytes:size_of::<Frame>(),depth:self.frames.next_pop_depth_demand().map_err(ValueError::from)?,..Default::default()}})}
  let push=!self.started||self.frames.last().is_some_and(|frame|frame.phase==3);
  let birth=push&&!self.frames.has_reserved_slot();
  let path=if birth{self.frames.next_reserve_depth_demand()}else if push{self.frames.next_push_depth_demand()}else if self.frames.last().is_some_and(|frame|frame.phase==4){self.frames.next_pop_depth_demand()}else if !self.frames.is_empty(){self.frames.next_get_depth_demand(self.frames.len()-1)}else{Ok(1)}.map_err(ValueError::from)?;
  Ok(RetirementDemand{copy_bytes:if birth{0}else if push{size_of::<Frame>()}else{self.frames.len().saturating_mul(2*size_of::<usize>()).saturating_add(2*size_of::<Node>())},capacity_bytes:if birth{self.frames.next_allocation_bytes().map_err(ValueError::from)?}else{0},depth:(self.frames.len()+usize::from(push)).max(path).max(1),..Default::default()})
 }
 fn at<'a>(&self,mut source:&'a dyn Canonical)->Result<&'a dyn Canonical,ValueError>{for frame in self.frames.iter().skip(1){source=source.canonical_tree_child(frame.ordinal)?;}Ok(source)}
 fn text_pair(frame:&mut Frame,a:Text<'_>,b:Text<'_>)->Result<Option<bool>,ValueError>{
  if matches!((a,b),(Text::Contiguous(a),Text::Contiguous(b))if a.as_ptr()==b.as_ptr()&&a.len()==b.len())||matches!((a,b),(Text::Native(a),Text::Native(b))if std::ptr::eq(a,b)){return Ok(Some(true))}
  let a=a.next_byte(&mut frame.chunks[0],&mut frame.offsets[0]).map_err(|_|refusal("original source text chunk is absent"))?;
  let b=b.next_byte(&mut frame.chunks[1],&mut frame.offsets[1]).map_err(|_|refusal("original baseline text chunk is absent"))?;
  Ok(if a!=b{Some(false)}else if a.is_none(){Some(true)}else{None})
 }
 pub(super) fn step(&mut self,a:&dyn Canonical,b:&dyn Canonical,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.progress=Default::default();if self.closing||self.result.is_some(){return Ok(Default::default())}let d=self.demands()?;if !admitted(d,grant){return Ok(Default::default())}
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};self.progress=p;
  if (!self.started||self.frames.last().is_some_and(|frame|frame.phase==3))&&!self.frames.has_reserved_slot(){let step=self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::from(error.refusal()))?;p.copied_bytes=0;p.retained_capacity_bytes=step.allocated_bytes;self.progress=p;return Ok(p)}
  if !self.started{self.frames.push_reserved(Frame::new(0)).unwrap_or_else(|_|unreachable!());self.started=true;return Ok(p)}
  if self.frames.is_empty(){self.result=Some(true);return Ok(p)}
  let left=self.at(a)?;let right=self.at(b)?;let frame=self.frames.last_mut().unwrap();
  match frame.phase{
   0=>match(left.canonical_tree_node()?,right.canonical_tree_node()?){
    (Node::Array(a),Node::Array(b))|(Node::Object(a),Node::Object(b)) if a==b=>{frame.count=a;frame.object=matches!(left.canonical_tree_node()?,Node::Object(_));frame.phase=1;},
    (Node::String(_),Node::String(_))|(Node::Text(_),Node::Text(_))=>frame.phase=5,
    (Node::Null,Node::Null)=>frame.phase=4,
    (Node::Bool(a),Node::Bool(b)) if a==b=>frame.phase=4,
    (Node::I64(a),Node::I64(b))|(Node::I64Text(a),Node::I64Text(b)) if a==b=>frame.phase=4,
    (Node::U64(a),Node::U64(b))|(Node::U64Text(a),Node::U64Text(b)) if a==b=>frame.phase=4,
    (Node::I128(a),Node::I128(b)) if a==b=>frame.phase=4,
    (Node::U128(a),Node::U128(b)) if a==b=>frame.phase=4,
    (Node::F32(a),Node::F32(b))|(Node::F32HexWord(a),Node::F32HexWord(b)) if a.to_bits()==b.to_bits()=>frame.phase=4,
    (Node::F64(a),Node::F64(b)) if a.to_bits()==b.to_bits()=>frame.phase=4,
    _=>self.result=Some(false),
   },
   1=>{frame.chunks=[0;2];frame.offsets=[0;2];frame.phase=if frame.index==frame.count{4}else if frame.object{2}else{3};},
   2=>match Self::text_pair(frame,left.canonical_tree_key(frame.index)?,right.canonical_tree_key(frame.index)?)?{Some(false)=>self.result=Some(false),Some(true)=>frame.phase=3,None=>{}},
   3=>{let ordinal=frame.index;frame.index+=1;frame.phase=1;self.frames.push_reserved(Frame::new(ordinal)).unwrap_or_else(|_|unreachable!());},
   4=>{self.frames.pop();},
   5=>{match Self::text_pair(frame,node_text(left.canonical_tree_node()?)?,node_text(right.canonical_tree_node()?)?)?{Some(false)=>self.result=Some(false),Some(true)=>frame.phase=4,None=>{}}},
   _=>return Err(refusal("original source comparison phase is invalid")),
  }Ok(p)
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.frames.is_empty()&&self.frames.allocated_bytes()==0}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.progress=Default::default();if !self.closing{return Err(refusal("original source comparison close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.demands()?;if !admitted(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};if self.frames.is_empty(){let step=self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;p.released_bytes=step.released_allocation_bytes;}else{self.frames.pop();p.copied_bytes=d.copy_bytes;}self.progress=p;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
