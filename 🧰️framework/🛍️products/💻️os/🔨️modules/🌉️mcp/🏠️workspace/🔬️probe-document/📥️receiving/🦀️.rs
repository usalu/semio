//! 📥️ The probe's exact bare-JSON mutation keeps its parser, unique value and complete syntax cause through bounded native turns.
use super::ProbeMutation;
use semio_framework_pack_json::{JsonError,JsonGrammarCursor,JsonMemberPolicy};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,native_decoding::{NativeDecodeControl,NativeConstructionStepResult}};
use semio_framework_dsl_record::receiving::OwnedSlot;
use store::{OpBinaryReceiving,OpBinaryReceivingCursor};
use std::mem::{ManuallyDrop,size_of};
type Parser=JsonGrammarCursor<DslValue>;
/// 🧳️ Source bytes stay borrowed from the original immutable owner until all parsed and diagnostic owners return.
pub struct ProbeMutationReceiving{
 source:(usize,usize),range:(usize,usize),native:Option<(u64,Option<u64>)>,parser:OwnedSlot<Parser>,output:OwnedSlot<ProbeMutation>,syntax:OwnedSlot<JsonError>,ready:bool,failed:bool,closing:bool,closed:bool,
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
 fn close(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  let d=match self.close_demand(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if self.closed{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if !funds(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d.copy_bytes))),work:1}}
  if !self.output.terminal_is_empty(){let t=self.output.close_step(g);return RetainedWorkOutcome{result:t.result.map(RetainedCloneStep::Progress),work:t.work}}
  if !self.parser.terminal_is_empty(){let t=self.parser.close_step(g);return RetainedWorkOutcome{result:t.result.map(RetainedCloneStep::Progress),work:t.work}}
  self.closed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d.copy_bytes))),work:1}
 }
}
impl OpBinaryReceivingCursor<ProbeMutation> for ProbeMutationReceiving{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.closing{return self.close_demand(body)}if self.failed{return Err(invalid("probe mutation retains its failed original call"))}
  if self.ready{return Ok(Default::default())}if self.native.is_none(){return Ok(fixed(size_of::<Option<(u64,Option<u64>)>>()))}
  let mut demand=self.parser.value.as_ref().ok_or_else(||invalid("probe mutation lost its parser"))?.normal_step_demands(&source[self.range.0..self.range.1]).map_err(|error|ValueError::literal(error.kind(),"probe JSON demand refused"))?;
  demand.copy_bytes=demand.copy_bytes.checked_add(Self::metadata()).ok_or_else(||invalid("probe mutation metadata extent overflow"))?;Ok(demand)
 }
 fn step(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let d=match self.demands(source,body){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if self.ready||!funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let identity=(native.ledger_identity(),native.retirement_recipient_identity());
  if self.native.is_some_and(|bound|bound!=identity){return RetainedWorkOutcome{result:Err(invalid("probe mutation changed its original native recipient").into()),work:0}}
  let bound=self.native.is_none();let maximum_work=if bound{1}else{2};
  match native.prepare_work(maximum_work){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(Default::default()),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}}
  let turn=native.receive_construction_step_typed(d,|native,grant|{
   if bound{self.native=Some(identity);return NativeConstructionStepResult{result:Ok(()),progress:receipt(d.copy_bytes),work:1}}
   let extra=Self::metadata();if grant.maximum_copy_bytes<extra{return NativeConstructionStepResult{result:Ok(()),progress:Default::default(),work:0}}
   let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-extra,..grant};
   let parser=self.parser.value.as_mut().unwrap();let result=parser.step_source(&source[self.range.0..self.range.1],1,native,child,false);
   let mut progress=parser.normal_step_progress();let mut work=parser.last_turn_work();
   let result=match result{
    Ok(Some(value))=>{*self.output.value=Some(ProbeMutation::SetValue(value));self.ready=true;progress.copied_bytes+=extra;progress.copied_items=progress.copied_items.max(1);work+=1;Ok(())},
    Ok(None)=>Ok(()),
    Err(error)=>{let kind=error.kind();*self.syntax.value=Some(error);self.failed=true;progress.copied_bytes+=extra;progress.copied_items=progress.copied_items.max(1);work+=1;Err(ValueError::literal(kind,"probe mutation retains its complete original JSON cause"))},
   };NativeConstructionStepResult{result,progress,work}
  });
  let work_refusal=native.commit_work(turn.work).err();
  let mut failure=match turn.result{Some(Err(error))=>Some(OriginalReceivingPrimary::from(error)),_=>None};
  if let Some(error)=turn.receipt_refusal{let f=failure.get_or_insert_with(||OriginalReceivingPrimary::from(invalid("probe mutation construction receipt refused")));f.receipt_refusal=Some(error);}
  if let Some(error)=work_refusal{let f=failure.get_or_insert_with(||OriginalReceivingPrimary::from(invalid("probe mutation original work settlement refused")));f.work_refusal=Some(error);}
  if let Some(mut failure)=failure{failure.progress=turn.progress;RetainedWorkOutcome{result:Err(failure),work:turn.work}}else{RetainedWorkOutcome{result:Ok(turn.progress),work:turn.work}}
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
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{self.close(g)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.parser.terminal_is_empty()&&self.output.terminal_is_empty()&&self.syntax.terminal_is_empty()}
}
struct ProbeReceivingRetirement(ManuallyDrop<ProbeMutationReceiving>);
impl RetireOwned for ProbeMutationReceiving{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ProbeReceivingRetirement(ManuallyDrop::new(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ProbeReceivingRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for ProbeReceivingRetirement{
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{let turn=self.0.close(g);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
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
