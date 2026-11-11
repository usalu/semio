//! 📥️ The probe's exact bare-JSON mutation keeps its parser, unique value and complete syntax cause through bounded native turns.
use super::ProbeMutation;
use semio_framework_pack_json::{JsonError,JsonGrammarCursor,JsonMemberPolicy};
use semio_framework_value::{DslValue,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,OriginalEffectReply,OriginalEffectReplyComplete},retirement::{RetireOwned,RetirementCursor,RetirementStep,erased_cursor_turn}};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,native_decoding::NativeDecodeControl};
use semio_framework_dsl_record::receiving::OwnedSlot;
use store::{OpBinaryReceiving,OpBinaryReceivingCursor};
use std::mem::{ManuallyDrop,size_of};
type Parser=JsonGrammarCursor<DslValue>;
/// 🧳️ Source bytes stay borrowed from the original immutable owner until all parsed and diagnostic owners return.
pub struct ProbeMutationReceiving{
 source:(usize,usize),range:(usize,usize),native:Option<(NativeAllocationIdentity,Option<u64>)>,parser:OwnedSlot<Parser>,output:OwnedSlot<ProbeMutation>,syntax:OwnedSlot<JsonError>,ready:bool,failed:bool,closing:bool,closed:bool,
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn receipt(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
impl OpBinaryReceiving for ProbeMutation{
 type DomainFailure=JsonError;
 type Receiving=ProbeMutationReceiving;
 fn admit_receiving(source:&[u8],range:(usize,usize),g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self::Receiving,RetainedCloneProgress)>>{
  if range.0>range.1||range.1>source.len(){return RetainedWorkOutcome{result:Err(invalid("probe mutation escaped its original input")),work:0}}
  let bytes=2*size_of::<ProbeMutationReceiving>();if !funds(g,fixed(bytes)){return RetainedWorkOutcome{result:Ok(None),work:0}}
  let mut parser=OwnedSlot::default();*parser.value=Some(Parser::new(JsonMemberPolicy::Reject));
  RetainedWorkOutcome{result:Ok(Some((ProbeMutationReceiving{source:(source.as_ptr()as usize,source.len()),range,native:None,parser,output:OwnedSlot::default(),syntax:OwnedSlot::default(),ready:false,failed:false,closing:false,closed:false},receipt(bytes)))),work:1}
 }
}
impl ProbeMutationReceiving{
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len())||self.range.0>self.range.1||self.range.1>source.len(){return Err(invalid("probe mutation source identity changed"))}Ok(())}
 fn metadata()->usize{size_of::<Option<ProbeMutation>>()+size_of::<Option<JsonError>>()+2*size_of::<bool>()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}
  if self.syntax.value.is_some(){return Err(invalid("probe mutation retains its unreceived original JSON cause"))}
  if !self.closing{return Ok(fixed(size_of::<bool>()))}
  if !self.output.terminal_is_empty(){return self.output.close_demands(body)}
  if !self.parser.terminal_is_empty(){return self.parser.close_demands(body)}
  Ok(fixed(size_of::<bool>()))
 }
 fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let g=effect.grant();let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  let d=match self.close_demand(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return settle(effect,Err(error))};
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  if !funds(g,d){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  let stepped=|poll:RetainedWorkPoll<RetainedCloneProgress,OriginalRetirementFailure>|match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})};
  if let Err(error)=Self::perform(effect){return settle(effect,Err(error))}
  if !self.closing{self.closing=true;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(d.copy_bytes))))}
  if !self.output.terminal_is_empty(){return stepped(self.output.close_step(&mut effect.nested(g)))}
  if !self.parser.terminal_is_empty(){return stepped(self.parser.close_step(&mut effect.nested(g)))}
  self.closed=true;settle(effect,Ok(RetainedCloneStep::Complete(receipt(d.copy_bytes))))
 }
 fn step_action(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  let g=effect.grant();let d=self.demands_all(source,body)?;
  if self.ready||!funds(g,d){return Ok(Default::default())}
  let identity=(native.ledger_identity(),native.retirement_recipient_identity());
  if self.native.is_some_and(|bound|bound!=identity){return Err(invalid("probe mutation changed its original native recipient").into())}
  Self::perform(effect)?;
  if self.native.is_none(){self.native=Some(identity);return Ok(receipt(d.copy_bytes))}
  let extra=Self::metadata();if g.maximum_copy_bytes<extra{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,..g};
  let parser=self.parser.value.as_mut().ok_or_else(||invalid("probe mutation lost its parser"))?;
  let result=parser.step_source(&source[self.range.0..self.range.1],1,native,&mut effect.nested(child),false);
  let mut progress=parser.normal_step_progress();
  match result{
   Ok(Some(value))=>{*self.output.value=Some(ProbeMutation::SetValue(value));self.ready=true;progress.copied_bytes+=extra;progress.copied_items=progress.copied_items.max(1);Self::perform(effect)?;Ok(progress)},
   Ok(None)=>Ok(progress),
   Err(error)=>{let kind=error.kind();*self.syntax.value=Some(error);self.failed=true;progress.copied_bytes+=extra;progress.copied_items=progress.copied_items.max(1);Self::perform(effect)?;Err(ValueError::literal(kind,"probe mutation retains its complete original JSON cause").with_retained_progress(progress).into())}
  }
 }
 fn demands_all(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{<Self as OpBinaryReceivingCursor<ProbeMutation>>::demands(self,source,body)}
 fn perform(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"probe mutation work is not funded"))}}
}
impl OpBinaryReceivingCursor<ProbeMutation> for ProbeMutationReceiving{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.closing{return self.close_demand(body)}if self.failed{return Err(invalid("probe mutation retains its failed original call"))}
  if self.ready{return Ok(Default::default())}if self.native.is_none(){return Ok(fixed(size_of::<Option<(NativeAllocationIdentity,Option<u64>)>>()))}
  let mut demand=self.parser.value.as_ref().ok_or_else(||invalid("probe mutation lost its parser"))?.normal_step_demands(&source[self.range.0..self.range.1]).map_err(|error|ValueError::literal(error.kind(),"probe JSON demand refused"))?;
  demand.copy_bytes=demand.copy_bytes.checked_add(Self::metadata()).ok_or_else(||invalid("probe mutation metadata extent overflow"))?;Ok(demand)
 }
 fn step_in(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let result=self.step_action(source,native,body,effect);RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn domain_failure(&self)->Option<&JsonError>{self.syntax.value.as_ref()}
 fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{Ok(if self.syntax.value.is_some(){fixed(2*size_of::<Option<JsonError>>())}else{Default::default()})}
 fn domain_failure_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.syntax.value.is_some()))}
 fn take_domain_failure_into(&mut self,recipient:&mut Option<JsonError>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  if recipient.is_some(){return RetainedWorkOutcome{result:Err(invalid("probe JSON cause requires its empty original recipient")),work:0}}
  let d=match self.domain_failure_demand(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if self.syntax.value.is_none()||!funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  *recipient=self.syntax.value.take();RetainedWorkOutcome{result:Ok(receipt(d.copy_bytes)),work:1}
 }
 fn is_ready(&self)->bool{self.ready&&!self.failed&&!self.closing&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<ProbeMutation>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  let bytes=2*size_of::<Option<ProbeMutation>>()+size_of::<bool>();
  if recipient.is_some(){return RetainedWorkOutcome{result:Err(invalid("probe mutation requires its empty typed recipient")),work:0}}
  if !self.is_ready()||!funds(g,fixed(bytes)){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  *recipient=self.output.value.take();self.closing=true;RetainedWorkOutcome{result:Ok(receipt(bytes)),work:1}
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close(effect)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.parser.terminal_is_empty()&&self.output.terminal_is_empty()&&self.syntax.terminal_is_empty()}
}
/// ♻️ Retires the probe receiving's remaining owners through its paid close.
#[doc(hidden)]
pub struct ProbeReceivingRetirement(ManuallyDrop<ProbeMutationReceiving>);
impl RetireOwned for ProbeMutationReceiving{
 type RetirementCursor=ProbeReceivingRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ProbeReceivingRetirement(ManuallyDrop::new(self))}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for ProbeReceivingRetirement{
 fn close_into<'reply>(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(invalid("probe mutation close requires its original producer invocation"))}
  Ok(match erased_cursor_turn(self.0.close(effect)){RetirementStep::Failure(error,_)=>reply.complete(Err(error)),step=>reply.complete(Ok(step))})
 }
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|d|d.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl Drop for ProbeReceivingRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty(),"probe mutation retirement retains its original owners");if self.0.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.0)}}}}
impl Drop for ProbeMutationReceiving{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"probe mutation cursor retains its original parser value or syntax cause")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
