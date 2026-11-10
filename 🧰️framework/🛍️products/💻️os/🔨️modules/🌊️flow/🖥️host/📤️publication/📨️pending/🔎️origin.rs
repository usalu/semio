//! 🔎️ A retry pins its actual original session origin before comparing window text or constructing its wire.
use super::*;
use flow::FlowInvocationOriginLease;
#[derive(RetireOwned)]
struct Fields{candidate:Option<FlowInvocationOriginLease>,ordering:Option<flow::FlowTextOrderingWork>}
#[derive(RetireOwned)]
pub struct FlowOriginLookupOutcome{pub source:Option<FlowInvocationOriginLease>,pub resume:bool}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:OriginalSourceBinding,session:usize,word:usize,length:usize}
pub struct FlowOriginLookupWork{fields:ManuallyDrop<Option<Fields>>,retirement:RetainedCloneClose,binding:Option<Binding>,hash:u64,scan:usize,phase:u8,matched:bool,closing:bool}
impl FlowOriginLookupWork{
 pub fn new(hash:u64)->Self{Self{fields:ManuallyDrop::new(Some(Fields{candidate:None,ordering:None})),retirement:Default::default(),binding:None,hash,scan:0,phase:0,matched:false,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{paid(cx,bytes)}
 pub fn next_demands(&self,session:&flow::FlowEvalSession,word:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.binding.is_some_and(|prior|prior.session!=session as*const _ as usize||prior.word!=word.as_ptr()as usize||prior.length!=word.len()){return Err(invalid())}
  if self.binding.is_none(){return Ok(fixed(size_of::<Option<Binding>>()))}
  let fields=self.fields.as_ref().ok_or_else(invalid)?;
  match self.phase{
   0=>Ok(if session.invocation_origin_read_ready(){fixed(size_of::<Option<u64>>()+size_of::<usize>())}else{Default::default()}),
   1=>Ok(flow::FlowEvalSession::invocation_origin_alias_demands()),
   2=>{let candidate=fields.candidate.as_ref().ok_or_else(invalid)?;if candidate.node_hash!=self.hash{Ok(fixed(size_of::<bool>()))}else if let Some(ordering)=fields.ordering.as_ref(){ordering.next_demands(&candidate.window_id,word,copy)}else{Ok(fixed(size_of::<flow::FlowTextOrderingWork>()))}},
   3=>{let ordering=fields.ordering.as_ref().ok_or_else(invalid)?;Ok(if ordering.terminal_is_empty(){fixed(size_of::<Option<flow::FlowTextOrderingWork>>())}else{ordering.close_demands()})},
   4=>if self.matched{Ok(fixed(size_of::<bool>()))}else if !self.retirement.is_empty(){self.retirement.demands(copy)}else if fields.candidate.is_some(){self.retirement.next_owner_demands::<FlowInvocationOriginLease>(true,&None,copy)}else{Ok(fixed(size_of::<usize>()))},
   5|6=>Ok(fixed(2*size_of::<FlowOriginLookupOutcome>())),
   _=>Err(invalid()),
  }
 }
 pub fn step(&mut self,session:&flow::FlowEvalSession,word:&str,cx:&mut StepContext<'_>)->Result<Option<FlowOriginLookupOutcome>,OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(result)=self.once(session,word,cx)?{return Ok(Some(result))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(None)}
 fn once(&mut self,session:&flow::FlowEvalSession,word:&str,cx:&mut StepContext<'_>)->Result<Option<FlowOriginLookupOutcome>,OriginalTurnFailure>{
  let binding=Binding{source:OriginalSourceBinding::borrow(cx),session:session as*const _ as usize,word:word.as_ptr()as usize,length:word.len()};if self.closing||self.binding.is_some_and(|prior|prior!=binding){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}if self.binding.is_none(){if Self::paid(cx,size_of::<Option<Binding>>())?{self.binding=Some(binding)}return Ok(None)}
  match self.phase{
   0=>{if !session.invocation_origin_read_ready(){return Ok(None)}if Self::paid(cx,size_of::<Option<u64>>()+size_of::<usize>())?{match session.invocation_origin_at(self.scan).map(|source|source.node_hash){None=>self.phase=6,Some(hash)if hash==self.hash=>self.phase=1,Some(_)=>self.scan+=1}}},
   1=>{let d=flow::FlowEvalSession::invocation_origin_alias_demands();if !cx.retained_work_can_enter(d){return Ok(None)}let fields=self.fields.as_mut().ok_or_else(invalid)?;let turn=session.retain_invocation_origin_at(self.scan,&mut fields.candidate,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;if fields.candidate.is_some(){self.phase=2}},
   2=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.candidate.as_ref().ok_or_else(invalid)?.node_hash!=self.hash{if Self::paid(cx,size_of::<bool>())?{self.matched=false;self.phase=4}return Ok(None)}if fields.ordering.is_none(){if Self::paid(cx,size_of::<flow::FlowTextOrderingWork>())?{fields.ordering=Some(flow::FlowTextOrderingWork::new())}return Ok(None)}let ordering=fields.ordering.as_mut().ok_or_else(invalid)?;if let Some(result)=ordering.step(&fields.candidate.as_ref().ok_or_else(invalid)?.window_id,word,cx)?{self.matched=result==std::cmp::Ordering::Equal;ordering.begin_close();self.phase=3}},
   3=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;let ordering=fields.ordering.as_mut().ok_or_else(invalid)?;if ordering.terminal_is_empty(){if Self::paid(cx,size_of::<Option<flow::FlowTextOrderingWork>>())?{fields.ordering=None;self.phase=4}}else{let d=ordering.close_demands();close_original_retained_step(cx,d,|g|ordering.close_step(g))?;}},
   4=>{if self.matched{if Self::paid(cx,size_of::<bool>())?{self.phase=5}return Ok(None)}if !self.retirement.is_empty(){let d=self.retirement.demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect)))?;return Ok(None)}let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.candidate.is_some(){let d=self.retirement.next_owner_demands::<FlowInvocationOriginLease>(true,&None,cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|{let turn=self.retirement.begin_granted(&mut fields.candidate,effect);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(None)}if Self::paid(cx,size_of::<usize>())?{self.scan+=1;self.phase=0}},
   5=>{if Self::paid(cx,2*size_of::<FlowOriginLookupOutcome>())?{self.phase=7;return Ok(Some(FlowOriginLookupOutcome{source:self.fields.as_mut().ok_or_else(invalid)?.candidate.take(),resume:session.has_evaluation_progress(self.hash)}))}},
   6=>{if Self::paid(cx,2*size_of::<FlowOriginLookupOutcome>())?{self.phase=7;return Ok(Some(FlowOriginLookupOutcome{source:None,resume:false}))}},
   _=>return Err(invalid().into())
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:FlowOriginLookupOutcome){assert_eq!(self.phase,7);self.matched=output.source.is_some();self.fields.as_mut().expect("original lookup fields").candidate=output.source;self.phase=if self.matched{5}else{6}}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,copy)}Ok(fixed(size_of::<Option<Binding>>()))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{if !self.closing||self.binding.is_some_and(|binding|!binding.source.matches(cx)){return Err(invalid().into())}let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_with_effect(cx,d,|g,effect|{if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}if self.fields.is_some(){let turn=self.retirement.begin_granted(&mut self.fields,effect);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}if self.binding.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(d.copy_bytes))),work:1}})?;Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for FlowOriginLookupWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original invocation lookup must close its actual candidate and comparison");unsafe{ManuallyDrop::drop(&mut self.fields)}}}
