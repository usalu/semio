//! 🔣️ Original History origin JSON retains its native grammar while borrowing the same source under complete caller policy.
use super::{RetainedHistoryOriginProjection,permits,metadata};
use semio_framework_pack_json::{JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy,JsonError};
use semio_framework_value::{DslValue,NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;
use ::protocol::MutationOrigin;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedHistoryOriginJsonStep{Blocked,Progress,Ready,Fault,Closed}
pub struct RetainedHistoryOriginJsonDecode{
 source:(usize,usize),range:(usize,usize),allocation_limit:usize,grammar:ManuallyDrop<Option<JsonGrammarCursor<DslValue>>>,parsed:ManuallyDrop<Option<DslValue>>,
 projection:ManuallyDrop<Option<RetainedHistoryOriginProjection>>,failure:ManuallyDrop<Option<JsonError>>,grammar_complete:bool,closing:bool,closed:bool,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
}
impl RetainedHistoryOriginJsonDecode{
 pub fn admit(source:&[u8],range:(usize,usize),limits:JsonReadLimits,g:RetainedCloneGrant)->Result<Option<Self>,ValueError>{
  if range.0>range.1||range.1>source.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original origin JSON range is invalid"))}
  if g.maximum_items==0||g.maximum_depth<2{return Ok(None)}
  let cursor=JsonSourceCursor::<_,DslValue>::new(&source[range.0..range.1],JsonMemberPolicy::Reject,limits)?;let(_,grammar)=cursor.into_grammar();
  Ok(Some(Self{source:(source.as_ptr()as usize,source.len()),range,allocation_limit:limits.maximum_allocation_bytes,grammar:ManuallyDrop::new(Some(grammar)),parsed:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),grammar_complete:false,closing:false,closed:false,receipt:Some((g,metadata()))}))
 }
 fn validate(&self,source:&[u8],control:&NativeDecodeControl<'_>)->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len())||control.maximum_bytes()!=self.allocation_limit{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"origin JSON changed its original source or cumulative receiving limit"))}Ok(())}
 pub fn source_identity(&self)->(usize,usize){self.source}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 fn publish(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.receipt=Some((g,p));if !p.fits(g){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"origin JSON exceeded original parent authority").with_retained_progress(p))}Ok(())}
 pub fn failure(&self)->Option<&JsonError>{self.failure.as_ref()}
 pub fn grammar_is_receivable(&self)->bool{self.grammar.is_some()&&(self.grammar_complete||self.closing||self.failure.is_some())}
 pub fn demands(&self,source:&[u8],control:&NativeDecodeControl<'_>,body:usize)->Result<RetirementDemand,JsonError>{
  self.validate(source,control)?;if self.receipt.is_some()||self.closed{return Ok(Default::default())}
  let mut d=if !self.closing&&self.failure.is_none()&&!self.grammar_complete{self.grammar.as_ref().unwrap().normal_step_demands(&source[self.range.0..self.range.1])?}
  else if let Some(projection)=self.projection.as_ref(){projection.demands(body)?}
  else{RetirementDemand{depth:1,..Default::default()}};
  d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"origin JSON parent depth overflow"))?;Ok(d)
 }
 pub fn step(&mut self,source:&[u8],control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedHistoryOriginJsonStep,ValueError>{
  self.validate(source,control)?;if self.receipt.is_some()||self.closing||self.closed{return Ok(RetainedHistoryOriginJsonStep::Blocked)}
  if self.failure.is_some(){return Ok(RetainedHistoryOriginJsonStep::Fault)}
  if g.maximum_items==0||g.maximum_depth<2{return Ok(RetainedHistoryOriginJsonStep::Blocked)}
  let d=match self.demands(source,control,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>{*self.failure=Some(error);self.publish(g,metadata())?;return Ok(RetainedHistoryOriginJsonStep::Fault)}};if !permits(g,d){return Ok(RetainedHistoryOriginJsonStep::Blocked)}
  let child=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};
  if !self.grammar_complete{
   let known_utf8=false;let grammar=self.grammar.as_mut().unwrap();let result=control.scoped_maximum(self.allocation_limit,|control|grammar.step_source(&source[self.range.0..self.range.1],1,control,child,known_utf8));let p=grammar.normal_step_progress();
   let status=match result{Ok(Some(value))=>{*self.parsed=Some(value);self.grammar_complete=true;RetainedHistoryOriginJsonStep::Progress},Ok(None)=>RetainedHistoryOriginJsonStep::Progress,Err(error)=>{*self.failure=Some(error);RetainedHistoryOriginJsonStep::Fault}};self.publish(g,p)?;return Ok(status)
  }
  if self.grammar.is_some(){return Ok(RetainedHistoryOriginJsonStep::Blocked)}
  if self.parsed.is_some(){
   let Some(mut projection)=RetainedHistoryOriginProjection::admit_original(&mut self.parsed,child)?else{return Ok(RetainedHistoryOriginJsonStep::Blocked)};
   let(issued,p)=projection.take_receipt().unwrap();if issued!=child{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"origin projection changed its original admission grant"))}*self.projection=Some(projection);self.publish(g,p)?;return Ok(RetainedHistoryOriginJsonStep::Progress)
  }
  let Some(projection)=self.projection.as_mut()else{return Ok(RetainedHistoryOriginJsonStep::Blocked)};
  if projection.is_ready(){return Ok(RetainedHistoryOriginJsonStep::Ready)}
  control.checkpoint()?;control.charge(d.capacity_bytes)?;let result=projection.step(child);let receipt=projection.take_receipt();let p=receipt.map_or_else(||result.as_ref().err().map_or(Default::default(),ValueError::retained_progress),|(_,p)|p);self.publish(g,p)?;
  result.map(|advanced|if self.projection.as_ref().is_some_and(RetainedHistoryOriginProjection::is_ready){RetainedHistoryOriginJsonStep::Ready}else if advanced{RetainedHistoryOriginJsonStep::Progress}else{RetainedHistoryOriginJsonStep::Blocked})
 }
 pub fn take_grammar_into(&mut self,recipient:&mut Option<JsonGrammarCursor<DslValue>>,g:RetainedCloneGrant)->Result<bool,ValueError>{if !self.grammar_is_receivable()||self.receipt.is_some()||recipient.is_some()||g.maximum_items==0||g.maximum_depth<2{return Ok(false)}*recipient=self.grammar.take();self.publish(g,metadata())?;Ok(true)}
 pub fn take_failure_into(&mut self,recipient:&mut Option<JsonError>,g:RetainedCloneGrant)->Result<bool,ValueError>{if self.receipt.is_some()||recipient.is_some()||self.failure.is_none()||g.maximum_items==0||g.maximum_depth<2{return Ok(false)}*recipient=self.failure.take();self.closing=true;self.publish(g,metadata())?;Ok(true)}
 pub fn take_partial_value_into(&mut self,recipient:&mut Option<DslValue>,g:RetainedCloneGrant)->Result<bool,ValueError>{if !self.closing||self.receipt.is_some()||recipient.is_some()||self.parsed.is_none()||g.maximum_items==0||g.maximum_depth<2{return Ok(false)}*recipient=self.parsed.take();self.publish(g,metadata())?;Ok(true)}
 pub fn take_projection_into(&mut self,recipient:&mut Option<RetainedHistoryOriginProjection>,g:RetainedCloneGrant)->Result<bool,ValueError>{if !self.closing||self.receipt.is_some()||recipient.is_some()||self.projection.is_none()||g.maximum_items==0||g.maximum_depth<2{return Ok(false)}*recipient=self.projection.take();self.publish(g,metadata())?;Ok(true)}
 pub fn take_ready_into(&mut self,recipient:&mut Option<MutationOrigin>,g:RetainedCloneGrant)->Result<bool,ValueError>{
  if self.receipt.is_some()||self.closing||self.grammar.is_some()||recipient.is_some()||!self.projection.as_ref().is_some_and(RetainedHistoryOriginProjection::is_ready)||g.maximum_items==0||g.maximum_depth<2{return Ok(false)}
  let child=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};let projection=self.projection.as_mut().unwrap();let moved=projection.take_ready_into(recipient,child)?;if !moved{return Ok(false)}let(issued,p)=projection.take_receipt().unwrap();if issued!=child{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"origin ready transfer changed original grant"))}self.closing=true;self.publish(g,p)?;Ok(true)
 }
 pub fn cancel(&mut self){self.closing=true;if let Some(projection)=self.projection.as_mut(){projection.cancel()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedHistoryOriginJsonStep,ValueError>{
  if self.receipt.is_some()||!self.closing||self.closed||self.grammar.is_some()||self.parsed.is_some()||self.failure.is_some(){return Ok(RetainedHistoryOriginJsonStep::Blocked)}
  if let Some(projection)=self.projection.as_mut(){
   if g.maximum_items==0||g.maximum_depth<2{return Ok(RetainedHistoryOriginJsonStep::Blocked)}
   if projection.terminal_is_empty(){drop(self.projection.take());self.publish(g,metadata())?;return Ok(RetainedHistoryOriginJsonStep::Progress)}
   let child=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};let closed=projection.close_step(child)?;if !closed{return Ok(RetainedHistoryOriginJsonStep::Blocked)}let(_,p)=projection.take_receipt().unwrap();self.publish(g,p)?;return Ok(RetainedHistoryOriginJsonStep::Progress)
  }
  if g.maximum_items==0||g.maximum_depth==0{return Ok(RetainedHistoryOriginJsonStep::Blocked)}self.closed=true;self.publish(g,metadata())?;Ok(RetainedHistoryOriginJsonStep::Closed)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.grammar.is_none()&&self.parsed.is_none()&&self.projection.is_none()&&self.failure.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedHistoryOriginJsonDecode{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original origin JSON abandoned a native grammar or receiving owner");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.grammar);ManuallyDrop::drop(&mut self.parsed);ManuallyDrop::drop(&mut self.projection);ManuallyDrop::drop(&mut self.failure)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
