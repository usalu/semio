//! 📨️ Original pending evaluations retain their session origins and yield one canonical dispatch at a time.
use crate as flow;
use crate::{close_with_effect,poll_outcome};
use crate::invocation_publication::{FlowInvocationOriginWork,FlowInvocationDispatchWork,FlowInvocationOriginLease,FlowInvocationDispatch};
use semio_framework_artifact_flow_flow::{FlowSnapshotSource,FlowSourceIdentity,neural};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,InteractiveJobCloseStep as Close,close_original_retained_step,receive_original_retained_work};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
use std::collections::VecDeque;
#[path="🔎️origin.rs"]
mod origin;
pub use origin::{FlowOriginLookupWork,FlowOriginLookupOutcome};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow evaluation changed its source, caller or publication custody")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(fixed(bytes)){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(progress(bytes))?;Ok(true)}
fn retire_with_effect(cx:&mut StepContext<'_>,d:RetirementDemand,body:impl FnOnce(RetainedCloneGrant,&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->Result<Close,OriginalTurnFailure>{cx.with_original_retirement(|cx|crate::close_with_effect(cx,d,body))}
fn retire(cx:&mut StepContext<'_>,d:RetirementDemand,body:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome)->Result<Close,OriginalTurnFailure>{cx.with_original_retirement(|cx|close_original_retained_step(cx,d,body))}
fn close_metadata(cx:&mut StepContext<'_>,bytes:usize,body:impl FnOnce())->Result<RetainedCloneProgress,OriginalTurnFailure>{let d=fixed(bytes);retire(cx,d,|g|{if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}body();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(bytes))),work:1}}).map(|step|step.progress())}
#[derive(RetireOwned)]
struct Fields{pending:VecDeque<neural::PendingExtensionEval>,current:Option<neural::PendingExtensionEval>,origin:Option<FlowInvocationOriginLease>,alias:Option<FlowInvocationOriginLease>,invocation:Option<FlowInvocationDispatch>}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:FlowSourceIdentity,session:usize,window:usize,window_length:usize,kind:usize,kind_length:usize}
#[derive(RetireOwned)]
pub enum FlowPendingDispatchStep{Invocation(FlowInvocationDispatch),Complete}
pub struct FlowPendingDispatchWork{fields:ManuallyDrop<Option<Fields>>,original:OriginalSourceBinding,binding:Option<Binding>,origin:Option<FlowInvocationOriginWork>,lookup:Option<FlowOriginLookupWork>,dispatch:Option<FlowInvocationDispatchWork>,retirement:RetainedCloneClose,generation:u64,count:usize,phase:u8,matched:bool,resume:bool,completed:bool,closing:bool}
impl FlowPendingDispatchWork{
 pub fn new(pending:VecDeque<neural::PendingExtensionEval>,generation:u64,cx:&StepContext<'_>)->Self{Self{count:pending.len(),fields:ManuallyDrop::new(Some(Fields{pending,current:None,origin:None,alias:None,invocation:None})),original:OriginalSourceBinding::borrow(cx),binding:None,origin:None,lookup:None,dispatch:None,retirement:Default::default(),generation,phase:6,matched:false,resume:false,completed:false,closing:false}}
 pub fn count(&self)->usize{self.count}
 fn identity(source:&dyn FlowSnapshotSource,session:&flow::FlowEvalSession,window:&str,kind:&str)->Binding{Binding{source:source.source_identity(),session:session as*const _ as usize,window:window.as_ptr()as usize,window_length:window.len(),kind:kind.as_ptr()as usize,kind_length:kind.len()}}
 fn validate(&self,source:&dyn FlowSnapshotSource,session:&flow::FlowEvalSession,window:&str,kind:&str)->Result<(),ValueError>{if self.closing||self.completed||self.binding.is_some_and(|binding|binding!=Self::identity(source,session,window,kind)){Err(invalid())}else{Ok(())}}
 pub fn next_demands(&self,source:&dyn FlowSnapshotSource,session:&flow::FlowEvalSession,window:&str,kind:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source,session,window,kind)?;if self.binding.is_none(){return Ok(fixed(size_of::<Option<Binding>>()))}
  let fields=self.fields.as_ref().ok_or_else(invalid)?;
  match self.phase{
   6=>Ok(fixed(if fields.pending.is_empty(){1}else{2*size_of::<neural::PendingExtensionEval>()+size_of::<FlowOriginLookupWork>()})),
   60=>self.lookup.as_ref().ok_or_else(invalid)?.next_demands(session,window,copy),
   61=>{let lookup=self.lookup.as_ref().ok_or_else(invalid)?;if lookup.terminal_is_empty(){Ok(fixed(size_of::<FlowOriginLookupWork>()))}else{lookup.close_demands(copy)}},
   64=>if !self.retirement.is_empty(){self.retirement.demands(copy)}else if fields.current.is_some(){self.retirement.next_owner_demands::<neural::PendingExtensionEval>(true,&None,copy)}else{Ok(fixed(1))},
   70=>Ok(fixed(size_of::<FlowInvocationOriginWork>()+2*size_of::<neural::PendingExtensionEval>())),
   7=>self.origin.as_ref().ok_or_else(invalid)?.next_demands(source,window,kind,copy),
   8=>{let origin=self.origin.as_ref().ok_or_else(invalid)?;if origin.terminal_is_empty(){Ok(fixed(size_of::<FlowInvocationOriginWork>()))}else{origin.next_close_demands(copy)}},
   9=>if session.invocation_origin_reserve_pending(){session.next_invocation_origin_reserve_demands(copy)}else{Ok(fixed(1))},
   10|11=>Ok(fixed(2*size_of::<Option<FlowInvocationOriginLease>>())),
   12=>Ok(fixed(size_of::<FlowInvocationDispatchWork>())),
   13=>self.dispatch.as_ref().ok_or_else(invalid)?.next_demands(&**fields.origin.as_ref().ok_or_else(invalid)?,copy),
   14=>{let dispatch=self.dispatch.as_ref().ok_or_else(invalid)?;if dispatch.terminal_is_empty(){Ok(fixed(size_of::<FlowInvocationDispatchWork>()))}else{dispatch.next_close_demands(copy)}},
   15=>if fields.origin.is_some(){self.retirement.next_owner_demands::<FlowInvocationOriginLease>(true,&None,copy)}else if !self.retirement.is_empty(){self.retirement.demands(copy)}else{Ok(fixed(2*size_of::<FlowInvocationDispatch>()))},
   16=>Ok(fixed(2*size_of::<FlowPendingDispatchStep>())),
   _=>Err(invalid()),
  }
 }
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,session:&mut flow::FlowEvalSession,window:&str,kind:&str,cx:&mut StepContext<'_>)->Result<Option<FlowPendingDispatchStep>,OriginalTurnFailure>{
  self.validate(source,session,window,kind)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  for _ in 0..512{let before=(cx.fuel_remaining(),cx.retained_progress());if let Some(output)=self.once(source,session,window,kind,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.fuel_remaining(),cx.retained_progress()){break}}Ok(None)
 }
 fn once(&mut self,source:&dyn FlowSnapshotSource,session:&mut flow::FlowEvalSession,window:&str,kind:&str,cx:&mut StepContext<'_>)->Result<Option<FlowPendingDispatchStep>,OriginalTurnFailure>{
  if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
  if self.binding.is_none(){if paid(cx,size_of::<Option<Binding>>())?{self.binding=Some(Self::identity(source,session,window,kind))}return Ok(None)}
  match self.phase{
   6=>{if self.fields.as_ref().ok_or_else(invalid)?.pending.is_empty(){if paid(cx,1)?{self.phase=16}return Ok(None)}if paid(cx,2*size_of::<neural::PendingExtensionEval>()+size_of::<FlowOriginLookupWork>())?{let fields=self.fields.as_mut().ok_or_else(invalid)?;fields.current=fields.pending.pop_front();self.lookup=Some(FlowOriginLookupWork::new(fields.current.as_ref().ok_or_else(invalid)?.node_hash));self.matched=false;self.resume=false;self.phase=60}},
   60=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if let Some(output)=lookup.step(session,window,cx)?{self.matched=output.source.is_some();self.resume=output.resume;self.fields.as_mut().ok_or_else(invalid)?.origin=output.source;lookup.begin_close();self.phase=61}},
   61=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if lookup.terminal_is_empty(){if paid(cx,size_of::<FlowOriginLookupWork>())?{self.lookup=None;self.phase=if self.matched{64}else{70}}}else{cx.with_original_retirement(|cx|lookup.close_step(cx))?;}},
   64=>{if !self.retirement.is_empty(){let d=self.retirement.demands(cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect)))?;return Ok(None)}let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.current.is_some(){let d=self.retirement.next_owner_demands::<neural::PendingExtensionEval>(true,&None,cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|{let turn=self.retirement.begin_granted(&mut fields.current,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(None)}if paid(cx,1)?{self.phase=12}},
   70=>if paid(cx,size_of::<FlowInvocationOriginWork>()+2*size_of::<neural::PendingExtensionEval>())?{let pending=self.fields.as_mut().ok_or_else(invalid)?.current.take().ok_or_else(invalid)?;self.origin=Some(FlowInvocationOriginWork::new(pending,self.generation));self.phase=7},
   7=>{let origin=self.origin.as_mut().ok_or_else(invalid)?;if let Some(output)=origin.step(source,window,kind,cx)?{self.fields.as_mut().ok_or_else(invalid)?.origin=Some(output);origin.begin_close();self.phase=8}},
   8=>{let origin=self.origin.as_mut().ok_or_else(invalid)?;if origin.terminal_is_empty(){if paid(cx,size_of::<FlowInvocationOriginWork>())?{self.origin=None;self.phase=9}}else{let d=origin.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|{let result=origin.close_step(g,effect);RetainedWorkOutcome{result,work:u128::from(origin.last_turn_work())}})?;}},
   9=>{if !session.invocation_origin_reserve_pending(){if paid(cx,1)?{self.phase=10}}else{let d=session.next_invocation_origin_reserve_demands(cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|session.reserve_invocation_origin_original_step(effect))?else{return Ok(None)};let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));turn.result?;}},
   10=>if paid(cx,2*size_of::<Option<FlowInvocationOriginLease>>())?{let fields=self.fields.as_mut().ok_or_else(invalid)?;fields.alias=Some(fields.origin.as_ref().ok_or_else(invalid)?.clone());self.phase=11},
   11=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;let d=fixed(2*size_of::<Option<FlowInvocationOriginLease>>());if !cx.retained_work_can_enter(d){return Ok(None)}let origin=fields.alias.take().ok_or_else(invalid)?;match session.retain_invocation_origin_step(origin,cx.retained_grant()){Ok(p)=>{cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..p})?;self.phase=12},Err((error,origin))=>{fields.alias=Some(origin);return Err(OriginalTurnFailure::debit_original(error,cx))}}},
   12=>if paid(cx,size_of::<FlowInvocationDispatchWork>())?{self.dispatch=Some(FlowInvocationDispatchWork::new(self.resume));self.phase=13},
   13=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;let dispatch=self.dispatch.as_mut().ok_or_else(invalid)?;if let Some(output)=dispatch.step(&**fields.origin.as_ref().ok_or_else(invalid)?,cx)?{fields.invocation=Some(output);dispatch.begin_close();self.phase=14}},
   14=>{let dispatch=self.dispatch.as_mut().ok_or_else(invalid)?;if dispatch.terminal_is_empty(){if paid(cx,size_of::<FlowInvocationDispatchWork>())?{self.dispatch=None;self.phase=15}}else{let d=dispatch.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|{let result=dispatch.close_step(g,effect);RetainedWorkOutcome{result,work:u128::from(dispatch.last_turn_work())}})?;}},
   15=>{let fields=self.fields.as_mut().ok_or_else(invalid)?;if fields.origin.is_some(){let d=self.retirement.next_owner_demands::<FlowInvocationOriginLease>(true,&None,cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|{let turn=self.retirement.begin_granted(&mut fields.origin,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(None)}if !self.retirement.is_empty(){let d=self.retirement.demands(cx.retained_grant().maximum_copy_bytes)?;retire_with_effect(cx,d,|g,effect|poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect)))?;return Ok(None)}if paid(cx,2*size_of::<FlowInvocationDispatch>())?{let output=fields.invocation.take().ok_or_else(invalid)?;self.phase=6;return Ok(Some(FlowPendingDispatchStep::Invocation(output)))}},
   16=>if paid(cx,2*size_of::<FlowPendingDispatchStep>())?{self.completed=true;return Ok(Some(FlowPendingDispatchStep::Complete))},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:FlowInvocationDispatch){assert!(!self.closing&&!self.completed&&self.phase==6);let fields=self.fields.as_mut().expect("original pending dispatch fields");assert!(fields.invocation.is_none());fields.invocation=Some(output);self.phase=15}
 pub fn begin_close(&mut self){self.closing=true;if let Some(work)=self.lookup.as_mut(){work.begin_close()}if let Some(work)=self.origin.as_mut(){work.begin_close()}if let Some(work)=self.dispatch.as_mut(){work.begin_close()}}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(work)=self.lookup.as_ref(){return if work.terminal_is_empty(){Ok(fixed(size_of::<FlowOriginLookupWork>()))}else{work.close_demands(copy)}}
  if let Some(work)=self.dispatch.as_ref(){return if work.terminal_is_empty(){Ok(fixed(size_of::<FlowInvocationDispatchWork>()))}else{work.next_close_demands(copy)}}
  if let Some(work)=self.origin.as_ref(){return if work.terminal_is_empty(){Ok(fixed(size_of::<FlowInvocationOriginWork>()))}else{work.next_close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,copy)}
  Ok(fixed(size_of::<Option<Binding>>()))
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{
  if !self.closing||!self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if let Some(work)=self.lookup.as_mut(){if !work.terminal_is_empty(){return cx.with_original_retirement(|cx|work.close_step(cx)).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}let p=close_metadata(cx,size_of::<FlowOriginLookupWork>(),||self.lookup=None)?;return Ok(if p==Default::default(){Close::Blocked}else{Close::Pending{progress:p}})}
  if let Some(work)=self.dispatch.as_mut(){if work.terminal_is_empty(){let p=close_metadata(cx,size_of::<FlowInvocationDispatchWork>(),||self.dispatch=None)?;return Ok(if p==Default::default(){Close::Blocked}else{Close::Pending{progress:p}})}let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=retire_with_effect(cx,d,|g,effect|{let result=work.close_step(g,effect);RetainedWorkOutcome{result,work:u128::from(work.last_turn_work())}})?;return Ok(Close::Pending{progress:step.progress()})}
  if let Some(work)=self.origin.as_mut(){if work.terminal_is_empty(){let p=close_metadata(cx,size_of::<FlowInvocationOriginWork>(),||self.origin=None)?;return Ok(if p==Default::default(){Close::Blocked}else{Close::Pending{progress:p}})}let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=retire_with_effect(cx,d,|g,effect|{let result=work.close_step(g,effect);RetainedWorkOutcome{result,work:u128::from(work.last_turn_work())}})?;return Ok(Close::Pending{progress:step.progress()})}
  let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
  if !self.retirement.is_empty()||self.fields.is_some(){let step=retire_with_effect(cx,d,|g,effect|{if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}let turn=self.retirement.begin_granted(&mut self.fields,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(Close::Pending{progress:step.progress()})}
  if self.binding.is_some(){let p=close_metadata(cx,d.copy_bytes,||self.binding=None)?;return Ok(if p==Default::default(){Close::Blocked}else{Close::Complete{progress:p}})}
  Ok(Close::Complete{progress:Default::default()})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.lookup.is_none()&&self.dispatch.is_none()&&self.origin.is_none()&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for FlowPendingDispatchWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original pending dispatch must close its real queue, children and output");unsafe{ManuallyDrop::drop(&mut self.fields)}}}
