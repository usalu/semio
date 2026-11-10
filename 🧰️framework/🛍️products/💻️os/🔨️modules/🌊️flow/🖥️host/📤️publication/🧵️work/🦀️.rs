//! 🧵️ Original evaluation results publish without constructing a second mutable host.
use super::*;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use super::{work::FlowEvaluationCompletion,display::FlowDisplayCursor,paint::FlowPaintCursor};
use semio_framework_value_ordered::ordered::SharedOwner;
use std::mem::ManuallyDrop;
use semio_framework_artifact_flow_flow::FlowSourceIdentity;
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Completion{snapshot:Option<TreeSnapshot>,channels:Option<EvalChannels>,remaining:Vec<String>,pending_extensions:Vec<neural::PendingExtensionEval>,generation:u64}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Sources{completion:Option<Completion>,display:Option<FlowDisplayCursor>,paint:Option<FlowPaintCursor>,live:Option<String>,painted:Option<String>,converged:Option<String>,converged_bytes:Vec<u8>,shared_live:Option<SharedOwner<String>>,shared_snapshot:Option<SharedOwner<TreeSnapshot>>,shared_channels:Option<SharedOwner<EvalChannels>>}
/// 📦️ The funded original roots cross into their actual publication target together.
pub struct FlowPublicationPayload{pub live:SharedOwner<String>,pub painted:String,pub converged:Option<String>,pub snapshot:Option<SharedOwner<TreeSnapshot>>,pub channels:Option<SharedOwner<EvalChannels>>,pub generation:u64}
/// 🎯️ A first-party target lends its exact old roots and receives only preborn original output.
pub trait FlowPublicationTarget{
 type Displaced:RetireOwned;
 fn publication_identity(&self)->([usize;8],u64);
 fn converged_json(&self)->&str;
 fn publish_original(&mut self,payload:FlowPublicationPayload,grant:RetainedCloneGrant)->Result<(Self::Displaced,RetainedCloneProgress),(ValueError,FlowPublicationPayload)>;
}
/// 🧵️ Retains the genuine completion and every displaced session root until paid closure.
pub struct FlowWorkPublication<T:FlowPublicationTarget>{sources:ManuallyDrop<Option<Sources>>,retirement:Option<ControlledRetirement<Sources>>,displaced:Option<ControlledRetirement<T::Displaced>>,binding:FlowSourceIdentity,origin:(u64,u64),session_binding:Option<([usize;8],u64)>,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Work publication identity changed")}
impl<T:FlowPublicationTarget> FlowWorkPublication<T>{
 /// 🌱️ Moves the exact original completion into an allocation-free frontier.
 pub fn new(completion:FlowEvaluationCompletion,source:&dyn FlowSnapshotSource,operation:u64,generation:u64)->Self{let FlowEvaluationCompletion{snapshot,channels,remaining,pending_extensions,generation:registry_generation}=completion;Self{sources:ManuallyDrop::new(Some(Sources{completion:Some(Completion{snapshot:Some(snapshot),channels:Some(channels),remaining,pending_extensions,generation:registry_generation}),display:Some(FlowDisplayCursor::new()),paint:Some(FlowPaintCursor::new()),live:None,painted:None,converged:None,converged_bytes:Vec::new(),shared_live:None,shared_snapshot:None,shared_channels:None})),retirement:None,displaced:None,binding:source.source_identity(),origin:(operation,generation),session_binding:None,phase:0,closing:false,progress:Default::default(),work:0}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn is_complete(&self)->bool{self.phase==255&&self.displaced.is_none()}
 pub fn remaining(&self)->&[String]{self.sources.as_ref().and_then(|s|s.completion.as_ref()).map_or(&[],|c|c.remaining.as_slice())}
 pub fn pending_extensions(&self)->&[neural::PendingExtensionEval]{self.sources.as_ref().and_then(|s|s.completion.as_ref()).map_or(&[],|c|c.pending_extensions.as_slice())}
 pub fn channels(&self)->Option<&EvalChannels>{self.sources.as_ref()?.completion.as_ref().and_then(|c|c.channels.as_ref())}
 /// 📡️ Moves the genuine parked wave only after a separately admitted ownership event.
 pub fn take_pending(&mut self,grant:RetainedCloneGrant)->Result<(Vec<neural::PendingExtensionEval>,RetainedCloneProgress),ValueError>{if !self.is_complete()||self.closing{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Work publication must complete before pending transfer"))}if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original pending wave transfer requires an admitted event"))}Ok((std::mem::take(&mut self.sources.as_mut().unwrap().completion.as_mut().unwrap().pending_extensions),RetainedCloneProgress{copied_items:1,..Default::default()}))}
 fn check(&self,source:&dyn FlowSnapshotSource)->Result<(),ValueError>{if self.binding!=source.source_identity(){return Err(invalid())}Ok(())}
 /// 🪙️ Quotes the next actual display, shared header, text copy or displaced-owner event.
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,session:&T,copy:usize)->Result<RetirementDemand,ValueError>{
  self.check(source)?;if self.closing{return self.next_close_demands(copy)}if self.phase!=255&&self.session_binding.is_some_and(|binding|binding!=session.publication_identity()){return Err(invalid())}if let Some(owner)=self.displaced.as_ref(){return owner.selected_demand(copy)}let s=self.sources.as_ref().unwrap();let c=s.completion.as_ref().unwrap();let mut d=RetirementDemand{depth:1,..Default::default()};match self.phase{
   0=>return s.display.as_ref().unwrap().demands(source,c.channels.as_ref().unwrap(),infos),
   1=>d=SharedOwner::<String>::source_admission_demand()?,
   2=>return s.paint.as_ref().unwrap().demands(source,c.channels.as_ref().unwrap(),infos,if c.remaining.is_empty(){""}else{session.converged_json()}),
   3 if c.remaining.is_empty()=>d=SharedOwner::<TreeSnapshot>::source_admission_demand()?,
   4 if c.remaining.is_empty()=>d=SharedOwner::<EvalChannels>::source_admission_demand()?,
   5 if c.remaining.is_empty()=>d.capacity_bytes=s.shared_live.as_ref().unwrap().get().len(),
   6 if c.remaining.is_empty()=>d.copy_bytes=s.shared_live.as_ref().unwrap().get().len().saturating_sub(s.converged_bytes.len()).min(copy),
   _=>{}
  }Ok(d)
 }
 /// ▶️ Reconciles exactly one original event with the initiating operation's existing wallet.
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,session:&mut T,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let mut progress=RetainedCloneProgress::default();let mut work=0u64;let mut result=Ok(false);
  for _ in 0..512{let before=cx.retained_progress();result=self.step_once(source,infos,session,cx);progress=progress.checked_add(self.progress)?;work=work.checked_add(self.work).ok_or_else(invalid)?;if !matches!(result,Ok(false))||cx.should_yield()||cx.is_cancelled()||cx.retained_progress()==before{break}}
  self.progress=progress;self.work=work;match result{Err(mut failure)=>{failure.effects=progress;Err(failure)},answer=>answer}
 }
 fn step_once(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,session:&mut T,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.progress=Default::default();self.work=0;self.check(source)?;if self.origin!=(cx.operation().0,cx.generation().0){return Err(invalid().into())}if cx.is_cancelled(){self.begin_close();return Ok(false)}if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();let d=self.next_demands(source,infos,session,grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(false)}
  if !self.closing&&self.session_binding.is_none(){self.session_binding=Some(session.publication_identity())}let outcome=cx.with_original_effect(grant,|_,effect|if self.closing{match self.close_ladder(grant.maximum_copy_bytes,effect){Ok(_)|Err(RetirementPollExit::Pending(_))=>Ok(false),Err(RetirementPollExit::Failed(failure))=>Err(failure.into_value_error())}}else{self.advance(source,infos,session,effect)}).map_err(OriginalTurnFailure::from)?;let Some((result,performed))=outcome else{return Ok(false)};if performed==0&&(self.progress!=Default::default()||result.is_err()){cx.consume_fuel(self.work.max(1))}match result{Ok(done)=>{cx.consume_retained(self.progress)?;Ok(done)},Err(error)=>Err(OriginalTurnFailure::debit_original(error.with_retained_progress(self.progress),cx))}
 }
 fn advance(&mut self,source:&dyn FlowSnapshotSource,infos:&HistoryFoldIndex<String,OperatorInfo>,session:&mut T,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let grant=effect.grant();if let Some(owner)=self.displaced.as_mut(){let poll=owner.step(grant.maximum_copy_bytes,effect);self.progress=owner.step_progress();if let RetainedWorkPoll::Complete(turn)=poll{turn.result.map_err(|failure|failure.into_value_error())?;}else{return Ok(false)}if owner.terminal_is_empty(){self.displaced=None}return Ok(self.is_complete())}if self.is_complete(){return Ok(true)}
  let s=self.sources.as_mut().unwrap();let c=s.completion.as_mut().unwrap();self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.work=1;match self.phase{
   0=>{let result=s.display.as_mut().unwrap().step(source,c.channels.as_ref().unwrap(),infos,grant);self.progress=result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());self.work=u64::from(self.progress!=Default::default()||result.is_err());result?;if s.display.as_ref().unwrap().complete(){s.live=s.display.as_mut().unwrap().take_text();self.phase=1}},
   1=>{let mut input=s.live.take();let admitted=SharedOwner::<String>::admit_from(&mut input,&mut s.shared_live,effect);s.live=input;match admitted.result{Ok(step)=>{self.progress=step.progress();if s.shared_live.is_some(){self.phase=2}},Err(error)=>{self.progress=error.retained_progress();return Err(error)}}},
   2=>{let result=s.paint.as_mut().unwrap().step(source,c.channels.as_ref().unwrap(),infos,if c.remaining.is_empty(){""}else{session.converged_json()},effect);self.progress=s.paint.as_ref().unwrap().step_progress();self.work=s.paint.as_ref().unwrap().last_turn_work();if let Some(text)=result?{s.painted=Some(text);self.phase=3}},
   3=>{if c.remaining.is_empty()&&s.shared_snapshot.is_none(){let mut input=c.snapshot.take();let admitted=SharedOwner::<TreeSnapshot>::admit_from(&mut input,&mut s.shared_snapshot,effect);c.snapshot=input;match admitted.result{Ok(step)=>self.progress=step.progress(),Err(error)=>{self.progress=error.retained_progress();return Err(error)}}if s.shared_snapshot.is_none(){return Ok(false)}}self.phase=4},
   4=>{if c.remaining.is_empty()&&s.shared_channels.is_none(){let mut input=c.channels.take();let admitted=SharedOwner::<EvalChannels>::admit_from(&mut input,&mut s.shared_channels,effect);c.channels=input;match admitted.result{Ok(step)=>self.progress=step.progress(),Err(error)=>{self.progress=error.retained_progress();return Err(error)}}if s.shared_channels.is_none(){return Ok(false)}}self.phase=5},
   5=>{if c.remaining.is_empty(){let len=s.shared_live.as_ref().unwrap().get().len();s.converged_bytes.try_reserve_exact(len).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original converged text birth refused"))?;self.progress.retained_capacity_bytes=s.converged_bytes.capacity();}self.phase=6},
   6=>{if c.remaining.is_empty(){let text=s.shared_live.as_ref().unwrap().get().as_bytes();let begin=s.converged_bytes.len();let end=(begin+grant.maximum_copy_bytes.min(text.len()-begin)).min(text.len());s.converged_bytes.extend_from_slice(&text[begin..end]);self.progress.copied_bytes=end-begin;if end!=text.len(){return Ok(false)}s.converged=Some(unsafe{String::from_utf8_unchecked(std::mem::take(&mut s.converged_bytes))});}self.phase=7},
   7=>{let payload=FlowPublicationPayload{live:s.shared_live.take().unwrap(),painted:s.painted.take().unwrap(),converged:s.converged.take(),snapshot:s.shared_snapshot.take(),channels:s.shared_channels.take(),generation:c.generation};match session.publish_original(payload,grant){Ok((original,progress))=>{self.progress=progress;self.displaced=Some(ControlledRetirement::retain(original));self.phase=255},Err((error,payload))=>{s.shared_live=Some(payload.live);s.painted=Some(payload.painted);s.converged=payload.converged;s.shared_snapshot=payload.snapshot;s.shared_channels=payload.channels;self.progress=error.retained_progress();return Err(error)}}},
   _=>return Err(invalid())
  }Ok(false)
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.displaced.as_ref(){return owner.selected_demand(copy)}if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.sources.is_some()),..Default::default()})}
 pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.displaced.as_ref(){return owner.next_computational_work(body)}if let Some(owner)=self.retirement.as_ref(){return owner.next_computational_work(body)}Ok(u64::from(self.sources.is_some()))}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_ladder(body,effect);crate::settle_close(effect,result,&mut self.progress)}
 fn close_ladder(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  self.progress=Default::default();self.work=0;let grant=effect.grant();if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.displaced.as_mut(){let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.displaced=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.retirement.is_none(){if self.sources.is_none(){return Ok(RetainedCloneStep::Complete(self.progress))}if grant.maximum_depth==0{return Err(invalid().into())}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Work publication handoff exceeds its work ceiling").into())}let sources=self.sources.take().ok_or_else(invalid)?;self.retirement=Some(ControlledRetirement::retain(sources));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.work=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  let owner=self.retirement.as_mut().unwrap();let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.sources.is_none()&&self.retirement.is_none()&&self.displaced.is_none()}
}
impl<T:FlowPublicationTarget> Drop for FlowWorkPublication<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Work publication requires paid closure")}}

