//! 📤️ Original source identity and physical pages remain inside one artifact-neutral publication.
use semio_framework_job::{OriginalSourceBinding, StepContext, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobOutcomeKind, OriginalTurnFailure, receive_original_retained_work, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_job_payload::{RetainedPayloadBuilder, JobPayloadStream, RetainedJobPayload};
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::RetirementDemand;

/// 🚦️ Describes a semantic publication without an artifact-specific serializer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum JobPublicationKind{Preview,Checkpoint{applied_progress:u64},Fault}
impl JobPublicationKind{
 fn stream(self)->JobPayloadStream{match self{Self::Preview=>JobPayloadStream::Preview,Self::Checkpoint{..}=>JobPayloadStream::CheckpointState,Self::Fault=>JobPayloadStream::Fault}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalPublicationSource{address:usize,extent:usize,caller:OriginalSourceBinding,kind:JobPublicationKind}

/// 🧳️ Enclosing admission owns this inline storage; each turn funds one original frontier.
pub struct RetainedJobPublication{builder:RetainedPayloadBuilder,stream:JobPayloadStream,binding:Option<OriginalPublicationSource>,cursor:usize,delivered:bool,closing:bool}
impl RetainedJobPublication{
 /// 📏️ Quotes only initialized empty publication metadata, leaving payload bindings untouched.
 pub fn empty_carrier_copy_bytes()->usize{RetainedPayloadBuilder::empty_carrier_copy_bytes()+size_of::<JobPayloadStream>()+size_of::<Option<OriginalPublicationSource>>()+size_of::<usize>()+2*size_of::<bool>()}
 /// 🖋️ Initializes an original enclosing destination without moving or zeroing payload bindings.
 /// # Safety
 /// The destination must be exclusive uninitialized storage whose original caller paid the empty metadata quote.
 pub unsafe fn initialize_empty_at(target:*mut Self){unsafe{RetainedPayloadBuilder::initialize_empty_at(std::ptr::addr_of_mut!((*target).builder),JobPayloadStream::Preview);std::ptr::addr_of_mut!((*target).stream).write(JobPayloadStream::Preview);std::ptr::addr_of_mut!((*target).binding).write(None);std::ptr::addr_of_mut!((*target).cursor).write(0);std::ptr::addr_of_mut!((*target).delivered).write(false);std::ptr::addr_of_mut!((*target).closing).write(false);}}

 pub fn new()->Self{Self{builder:RetainedPayloadBuilder::new(JobPayloadStream::Preview),stream:JobPayloadStream::Preview,binding:None,cursor:0,delivered:false,closing:false}}
 fn original(kind:JobPublicationKind,source:&[u8],cx:&StepContext<'_>)->OriginalPublicationSource{OriginalPublicationSource{address:source.as_ptr()as usize,extent:source.len(),caller:OriginalSourceBinding::borrow(cx),kind}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"publication requires its same original source extent kind and operation until physical closure")}
 fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.builder.terminal_is_empty()&&!self.closing}
 /// 📬️ Reports whether the original publication has lent its current outcome.
 pub fn is_delivered(&self)->bool{self.delivered}
 pub fn published(&self)->Option<&RetainedJobPayload>{self.builder.published()}
 /// ⏩️ Quotes the next actual source frontier without copying, allocating or changing custody.
 pub fn advance_demands(&self,kind:JobPublicationKind,source:&[u8],cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
  let binding=Self::original(kind,source,cx);if self.binding.is_some_and(|original|original!=binding)||self.closing{return Err(Self::invalid())}
  if self.delivered{return Ok(Default::default())}
  if self.binding.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  let mut demand=if self.stream!=kind.stream(){RetirementDemand{depth:1,..Default::default()}}else if !self.builder.is_initialized(){self.builder.initialization_demand()}else if self.cursor<source.len(){self.builder.append_demand(cx.retained_grant())?}else if self.builder.published().is_none(){RetainedPayloadBuilder::seal_demand()}else{RetirementDemand{depth:1,..Default::default()}};
  demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(demand)
 }
 /// ✍️ Advances one paid source frontier and lends the original sealed payload on a separate turn.
 pub fn advance_from_source<'a>(&'a mut self,kind:JobPublicationKind,source:&[u8],cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;let demand=self.advance_demands(kind,source,cx)?;if cx.is_cancelled()||cx.should_yield()||self.delivered||!Self::permits(cx.retained_grant(),demand){return Ok(None)}
  if self.binding.is_none(){self.binding=Some(Self::original(kind,source,cx));receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;return Ok(None)}
  if self.stream!=kind.stream(){let turn=self.builder.select_stream(kind.stream(),cx.retained_grant());let result=turn.result.map(|step|{if matches!(step,RetainedCloneStep::Complete(_)){self.stream=kind.stream();}step});receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;return Ok(None)}
  if !self.builder.is_initialized(){let turn=self.builder.advance_initialization(cx);receive_original_retained_work(cx,turn)?;return Ok(None)}
  if self.cursor<source.len(){let turn=self.builder.append_original(cx,source,&mut self.cursor);receive_original_retained_work(cx,turn)?;return Ok(None)}
  if self.builder.published().is_none(){let turn=self.builder.seal(cx);receive_original_retained_work(cx,turn)?;return Ok(None)}
  let payload=self.builder.published().ok_or_else(Self::invalid)?;
  let original=match kind{JobPublicationKind::Preview=>JobOutcomeBorrow::admit_preview(cx,payload)?,JobPublicationKind::Checkpoint{applied_progress}=>JobOutcomeBorrow::admit_checkpoint(cx,payload,applied_progress)?,JobPublicationKind::Fault=>JobOutcomeBorrow::admit_fault(cx,payload)?};
  if original.is_some(){self.delivered=true}Ok(original)
 }
 /// 🤝️ Resolves the immutable descriptor against this same producer header and semantic kind.
 pub fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
  let binding=self.binding.filter(|_|self.delivered&&!self.closing).ok_or_else(Self::invalid)?;let payload=self.builder.published().ok_or_else(Self::invalid)?;
  match binding.kind{JobPublicationKind::Preview=>descriptor.preview(payload),JobPublicationKind::Checkpoint{applied_progress}=>{if descriptor.kind()!=(JobOutcomeKind::CheckpointReady{applied_progress}){return Err(Self::invalid())}descriptor.checkpoint(payload)},JobPublicationKind::Fault=>descriptor.fault(payload)}
 }
 /// 📪️ Quotes close admission, actual child backing, and final metadata removal separately.
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.builder.terminal_is_empty(){let mut demand=self.builder.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(Self::invalid)?;return Ok(demand)}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the actual child close or one separate publication metadata turn.
 pub fn retirement_work(&self)->Result<u64,ValueError>{
  self.retirement_demands()?;
  if self.terminal_is_empty(){return Ok(0)}
  if self.closing&&!self.builder.terminal_is_empty(){return self.builder.retirement_work()}
  Ok(1)
 }
 /// ♻️ The caller ends its descriptor loan before original pages and publication metadata close.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !Self::permits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  if !self.builder.terminal_is_empty(){let turn=self.builder.close_step_granted(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
  self.binding=None;self.cursor=0;self.delivered=false;self.closing=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }

}
impl Default for RetainedJobPublication{fn default()->Self{Self::new()}}
impl Drop for RetainedJobPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"publication reached Drop before original physical retirement");}}
