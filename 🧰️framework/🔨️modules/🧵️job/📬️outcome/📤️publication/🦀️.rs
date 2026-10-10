//! 📤️ Original source identity and physical pages remain inside one artifact-neutral publication.
use super::*;
use semio_framework_value::RetirementDemand;

/// 🚦️ Describes a semantic publication without an artifact-specific serializer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum JobPublicationKind{Preview,Checkpoint{applied_progress:u64},Fault}
impl JobPublicationKind{
 fn stream(self)->JobPayloadStream{match self{Self::Preview=>JobPayloadStream::Preview,Self::Checkpoint{..}=>JobPayloadStream::CheckpointState,Self::Fault=>JobPayloadStream::Fault}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalPublicationSource{address:usize,extent:usize,operation:OperationId,generation:Generation,kind:JobPublicationKind}

/// 🧳️ Enclosing admission owns this inline storage; each turn funds one original frontier.
pub struct RetainedJobPublication{builder:RetainedPayloadBuilder,stream:JobPayloadStream,binding:Option<OriginalPublicationSource>,cursor:usize,delivered:bool,closing:bool}
impl RetainedJobPublication{
 pub fn new()->Self{Self{builder:RetainedPayloadBuilder::new(JobPayloadStream::Preview),stream:JobPayloadStream::Preview,binding:None,cursor:0,delivered:false,closing:false}}
 fn original(kind:JobPublicationKind,source:&[u8],cx:&StepContext<'_>)->OriginalPublicationSource{OriginalPublicationSource{address:source.as_ptr()as usize,extent:source.len(),operation:cx.operation(),generation:cx.generation(),kind}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"publication requires its same original source extent kind and operation until physical closure")}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.builder.terminal_is_empty()&&!self.closing}
 pub fn published(&self)->Option<&RetainedJobPayload>{self.builder.published()}
 /// 📏️ Quotes the next actual source frontier without copying, allocating or changing custody.
 pub fn advance_demands(&self,kind:JobPublicationKind,source:&[u8],cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
  let binding=Self::original(kind,source,cx);if self.binding.is_some_and(|original|original!=binding)||self.closing{return Err(Self::invalid())}
  if self.delivered{return Ok(Default::default())}
  if self.binding.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  let mut demand=if self.stream!=kind.stream(){RetirementDemand{depth:1,..Default::default()}}else if !self.builder.is_initialized(){self.builder.initialization_demand()}else if self.cursor<source.len(){self.builder.append_demand(cx.retained_grant())?}else{RetirementDemand{depth:1,..Default::default()}};
  demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)
 }
 /// ✍️ Advances one paid source frontier and lends the original sealed payload on a separate turn.
 pub fn advance_from_source<'a>(&'a mut self,kind:JobPublicationKind,source:&[u8],cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
  let demand=self.advance_demands(kind,source,cx)?;if self.delivered||!Self::permits(cx.retained_grant(),demand){return Ok(None)}
  if self.binding.is_none(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.binding=Some(Self::original(kind,source,cx));return Ok(None)}
  if self.stream!=kind.stream(){let step=self.builder.select_stream(kind.stream(),cx.retained_grant())?;cx.consume_retained(step.progress())?;self.stream=kind.stream();return Ok(None)}
  if !self.builder.is_initialized(){self.builder.advance_initialization(cx)?;return Ok(None)}
  if self.cursor<source.len(){self.builder.append_original(cx,source,&mut self.cursor)?;return Ok(None)}
  if self.builder.published().is_none(){self.builder.seal(cx)?;return Ok(None)}
  let payload=self.builder.published().ok_or_else(Self::invalid)?;
  let original=match kind{JobPublicationKind::Preview=>JobOutcomeBorrow::admit_preview(cx,payload)?,JobPublicationKind::Checkpoint{applied_progress}=>JobOutcomeBorrow::admit_checkpoint(cx,payload,applied_progress)?,JobPublicationKind::Fault=>JobOutcomeBorrow::admit_fault(cx,payload)?};
  if original.is_some(){self.delivered=true}Ok(original)
 }
 /// 🤝️ Resolves the immutable descriptor against this same producer header and semantic kind.
 pub fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
  let binding=self.binding.filter(|_|self.delivered&&!self.closing).ok_or_else(Self::invalid)?;let payload=self.builder.published().ok_or_else(Self::invalid)?;
  match binding.kind{JobPublicationKind::Preview=>descriptor.preview(payload),JobPublicationKind::Checkpoint{applied_progress}=>{if descriptor.kind()!=(JobOutcomeKind::CheckpointReady{applied_progress}){return Err(Self::invalid())}descriptor.checkpoint(payload)},JobPublicationKind::Fault=>descriptor.fault(payload)}
 }
 /// 📏️ Quotes close admission, actual child backing, and final metadata removal separately.
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.builder.terminal_is_empty(){let mut demand=self.builder.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;return Ok(demand)}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ♻️ The caller ends its descriptor loan before original pages and publication metadata close.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.retirement_demands()?;if !Self::permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if !self.builder.terminal_is_empty(){let step=self.builder.close_step_granted(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  self.binding=None;self.cursor=0;self.delivered=false;self.closing=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl Default for RetainedJobPublication{fn default()->Self{Self::new()}}
impl Drop for RetainedJobPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"publication reached Drop before original physical retirement");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
