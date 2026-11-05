//! 🧵️ One original evaluation owns its completion, publication and pending wave through paid closure.
use super::{work::{FlowEvaluationWork,FlowEvaluationCompletion},publication::{FlowPublicationTarget,FlowWorkPublication},neural,FlowSnapshotSource,FlowSourceIdentity,HistoryFoldIndex,OperatorInfo};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalDomainReservation,admit_original_domain_failure,close_original_retained_step,InteractiveJobCloseStep as Close};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{caller:OriginalSourceBinding,source:FlowSourceIdentity,infos:usize,target:usize}
#[derive(RetireOwned)]
struct Fields{completion:Option<FlowEvaluationCompletion>,pending:Option<Vec<neural::PendingExtensionEval>>}
/// 📡️ The original pending wave is moved intact for its caller's origin and dispatch stages.
#[derive(RetireOwned)]
pub struct FlowEvaluationPrepared{pub pending:Vec<neural::PendingExtensionEval>,pub unfinished:bool}
impl FlowEvaluationPrepared{fn receive(pending:Vec<neural::PendingExtensionEval>,unfinished:bool)->Self{Self{pending,unfinished}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation pipeline changed its caller, source, catalogue or publication target")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
/// 🎯️ The caller keeps its genuine immutable source and stable publication target alive until this controller closes.
pub struct FlowEvaluationPipeline<T:FlowPublicationTarget>{evaluation:Option<FlowEvaluationWork>,publication:Option<FlowWorkPublication<T>>,failure:OriginalDomainReservation<neural::OperatorWorkFailure>,fields:ManuallyDrop<Option<Fields>>,retirement:RetainedCloneClose,binding:Option<Binding>,phase:u8,unfinished:bool,closing:bool}
impl<T:FlowPublicationTarget> FlowEvaluationPipeline<T>{
 /// 🌱️ Receives an already funded Work and its actual preborn registry and catalogue leases.
 pub fn new(evaluation:FlowEvaluationWork)->Self{Self{evaluation:Some(evaluation),publication:None,failure:OriginalDomainReservation::new(),fields:ManuallyDrop::new(Some(Fields{completion:None,pending:None})),retirement:Default::default(),binding:None,phase:0,unfinished:false,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize,closing:bool)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||(!closing&&cx.is_cancelled())||!cx.retained_work_can_enter(fixed(bytes)){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(progress(bytes))?;Ok(true)}
 /// 📏️ Quotes the next actual binding, failure frame, Work, publication or ownership transfer.
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,target:&T,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(copy)}
  if self.binding.is_some_and(|binding|binding.source!=source.source_identity()||binding.infos!=infos as*const _ as usize||binding.target!=target as*const T as usize){return Err(invalid())}
  if self.binding.is_none(){return Ok(fixed(size_of::<Option<Binding>>()))}
  if self.failure.pending(){return Ok(fixed(0))}
  match self.phase{
   0=>Ok(if self.failure.ready(){fixed(1)}else{semio_framework_job::original_domain_failure_demand::<neural::OperatorWorkFailure>()}),
   1=>self.evaluation.as_ref().ok_or_else(invalid)?.next_demands(source,copy),
   2=>{let work=self.evaluation.as_ref().ok_or_else(invalid)?;if work.terminal_is_empty(){Ok(fixed(size_of::<FlowEvaluationWork>()+size_of::<FlowWorkPublication<T>>()))}else{work.next_close_demands(copy)}},
   3=>self.publication.as_ref().ok_or_else(invalid)?.next_demands(source,infos,target,copy),
   4=>Ok(fixed(2*size_of::<Vec<neural::PendingExtensionEval>>())),
   5=>{let publication=self.publication.as_ref().ok_or_else(invalid)?;if publication.terminal_is_empty(){Ok(fixed(size_of::<FlowWorkPublication<T>>()))}else{publication.next_close_demands(copy)}},
   6=>Ok(fixed(2*size_of::<FlowEvaluationPrepared>())),
   _=>Err(invalid())
  }
 }
 /// ▶️ Advances the same original caller without deriving another catalogue, host or source.
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,target:&mut T,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationPrepared>,OriginalTurnFailure>{
  for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(output)=self.once(source,infos,target,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(None)
 }
 fn once(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,target:&mut T,cx:&mut StepContext<'_>)->Result<Option<FlowEvaluationPrepared>,OriginalTurnFailure>{
  let binding=Binding{caller:OriginalSourceBinding::borrow(cx),source:source.source_identity(),infos:infos as*const _ as usize,target:target as*const T as usize};
  if self.closing||self.binding.is_some_and(|prior|prior!=binding){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  if self.binding.is_none(){if Self::paid(cx,size_of::<Option<Binding>>(),false)?{self.binding=Some(binding)}return Ok(None)}
  if self.failure.pending(){if let Some(failure)=admit_original_domain_failure(&mut self.failure,cx)?{return Err(failure)}return Ok(None)}
  match self.phase{
   0=>if !self.failure.ready(){self.failure.prepare(cx)?;}else if Self::paid(cx,1,false)?{self.phase=1},
   1=>{let evaluation=self.evaluation.as_mut().ok_or_else(invalid)?;let completion=&mut self.fields.as_mut().ok_or_else(invalid)?.completion;let ready=self.failure.with_body(cx,|same,domain|{match evaluation.step(source,same){Ok(Some(output))=>{*completion=Some(output);Ok(true)},Ok(None)=>Ok(false),Err(error)=>{*domain=Some(error);Ok(false)}}})?;if ready==Some(true){evaluation.begin_close();self.phase=2}},
   2=>{let evaluation=self.evaluation.as_mut().ok_or_else(invalid)?;if evaluation.terminal_is_empty(){if Self::paid(cx,size_of::<FlowEvaluationWork>()+size_of::<FlowWorkPublication<T>>(),false)?{self.publication=Some(FlowWorkPublication::new(self.fields.as_mut().ok_or_else(invalid)?.completion.take().ok_or_else(invalid)?,source,cx.operation().0,cx.generation().0));self.evaluation=None;self.phase=3}}else{let d=evaluation.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|{let result=evaluation.close_step(g);RetainedWorkOutcome{result,work:evaluation.last_turn_work()}})?;}},
   3=>{let publication=self.publication.as_mut().ok_or_else(invalid)?;if publication.step(source,infos,target,cx)?{self.unfinished=!publication.remaining().is_empty();self.phase=4}},
   4=>{let d=fixed(2*size_of::<Vec<neural::PendingExtensionEval>>());if !cx.retained_work_can_enter(d){return Ok(None)}let publication=self.publication.as_mut().ok_or_else(invalid)?;let(pending,p)=publication.take_pending(cx.retained_grant())?;self.fields.as_mut().ok_or_else(invalid)?.pending=Some(pending);publication.begin_close();self.phase=5;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..p})?},
   5=>{let publication=self.publication.as_mut().ok_or_else(invalid)?;if publication.terminal_is_empty(){if Self::paid(cx,size_of::<FlowWorkPublication<T>>(),false)?{self.publication=None;self.phase=6}}else{let d=publication.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|{let result=publication.close_step(g);RetainedWorkOutcome{result,work:publication.last_turn_work()}})?;}},
   6=>if Self::paid(cx,2*size_of::<FlowEvaluationPrepared>(),false)?{self.phase=7;return Ok(Some(FlowEvaluationPrepared::receive(self.fields.as_mut().ok_or_else(invalid)?.pending.take().ok_or_else(invalid)?,self.unfinished)))},
   _=>return Err(invalid().into())
  }Ok(None)
 }
 /// ↩️ Returns a handoff its downstream admitted owner could not receive.
 pub fn restore_output(&mut self,output:FlowEvaluationPrepared){assert_eq!(self.phase,7);self.fields.as_mut().expect("original evaluation pipeline fields").pending=Some(output.pending);self.unfinished=output.unfinished;self.phase=6}
 /// 🛑️ Stops normal evaluation while retaining every partially accepted owned field.
 pub fn begin_close(&mut self){self.closing=true;if let Some(evaluation)=self.evaluation.as_mut(){evaluation.begin_close()}if let Some(publication)=self.publication.as_mut(){publication.begin_close()}}
 /// 📏️ Quotes the actual next Work, publication, refusal or owned-field retirement.
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(evaluation)=self.evaluation.as_ref(){return if evaluation.terminal_is_empty(){Ok(fixed(size_of::<usize>()))}else{evaluation.next_close_demands(copy)}}if let Some(publication)=self.publication.as_ref(){return if publication.terminal_is_empty(){Ok(fixed(size_of::<usize>()))}else{publication.next_close_demands(copy)}}if !self.failure.terminal_is_empty(){return self.failure.retirement_demands(copy)}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,copy)}Ok(fixed(size_of::<Option<Binding>>()))}
 /// 🧹️ The same physical original caller pays every child's full result and actual work.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{
  if !self.closing||self.binding.is_some_and(|binding|!binding.caller.matches(cx)){return Err(invalid().into())}
  if let Some(evaluation)=self.evaluation.as_mut(){if evaluation.terminal_is_empty(){if !Self::paid(cx,size_of::<usize>(),true)?{return Ok(Close::Blocked)}self.evaluation=None;return Ok(Close::Pending{progress:progress(size_of::<usize>())})}let d=evaluation.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let result=evaluation.close_step(g);RetainedWorkOutcome{result,work:evaluation.last_turn_work()}})?;return Ok(Close::Pending{progress:step.progress()})}
  if let Some(publication)=self.publication.as_mut(){if publication.terminal_is_empty(){if !Self::paid(cx,size_of::<usize>(),true)?{return Ok(Close::Blocked)}self.publication=None;return Ok(Close::Pending{progress:progress(size_of::<usize>())})}let d=publication.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let result=publication.close_step(g);RetainedWorkOutcome{result,work:publication.last_turn_work()}})?;return Ok(Close::Pending{progress:step.progress()})}
  let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{if !self.failure.terminal_is_empty(){return self.failure.close_step(g)}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.fields.is_some(){let turn=self.retirement.begin_granted(&mut self.fields,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}if self.binding.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(d.copy_bytes))),work:1}})?;Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})
 }
 /// 🪦️ Terminal ownership is unconditional, including cancellation and refusal.
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.evaluation.is_none()&&self.publication.is_none()&&self.failure.terminal_is_empty()&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<T:FlowPublicationTarget> Drop for FlowEvaluationPipeline<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original evaluation pipeline must close every accepted source result, refusal and publication");unsafe{ManuallyDrop::drop(&mut self.fields)}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
