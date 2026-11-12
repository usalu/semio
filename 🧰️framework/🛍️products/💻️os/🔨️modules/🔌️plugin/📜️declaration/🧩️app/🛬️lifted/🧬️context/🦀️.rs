//! 🧬️ Lift file: 13 item(s), about 38 lines, of `🧵️retained-command/🧬️context/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (mod original_fields;, Source, ArtifactOwnedContextHandle, impl ArtifactOwnedContextHandle, impl Deref for ArtifactOwnedContextHandle, impl AsRef for ArtifactOwnedContextHandle, ...); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use crate::ArtifactApp;
use crate::tool_contract::ArtifactOwnedToolJobContext;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement},shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;
#[path="../📸️fields/🦀️.rs"]
mod original_fields;
pub use original_fields::{ContextViewLease,ContextOwnedFields,ContextOriginalResidual,ContextRetirementParts,ContextToolRunMetadata,ContextToolRunSources};

#[path="../../../👷️job/🎯️completion-and-owned-context/📥️provisional/🦀️.rs"]
mod original_provisional;

enum Source<A:ArtifactApp>{Admitted(SealedShared<ArtifactOwnedToolJobContext<A>>),Refused{original:ArtifactOwnedToolJobContext<A>,fault:ValueError}}

/// 🔐️ No raw Arc, Weak or infallible alias cloning escapes the original context issuer.
pub struct ArtifactOwnedContextHandle<A:ArtifactApp>{source:ManuallyDrop<Option<Source<A>>>}
impl<A:ArtifactApp> ArtifactOwnedContextHandle<A>{
 pub fn admit(original:ArtifactOwnedToolJobContext<A>,grant:RetainedCloneGrant,cx:&mut StepContext<'_>)->(Self,RetainedCloneProgress){
  let mut input=Some(original);let mut output=None;
  let outcome=match cx.with_original_effect(grant,|_,effect|SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect)){Ok(Some((turn,_)))=>turn.result.map(|step|step.progress()),Ok(None)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original context admission is not funded")),Err(error)=>Err(error)};
  let(source,progress)=match(output,outcome){(Some(source),outcome)=>(Source::Admitted(source),outcome.unwrap_or_default()),(None,outcome)=>(Source::Refused{original:input.take().expect("refused context admission retains its original"),fault:outcome.err().unwrap_or_else(unadmitted)},Default::default())};
  (Self{source:ManuallyDrop::new(Some(source))},progress)
 }
 /// 🚫️ A refused admission surrenders its untouched original and returns the refusal, so the handle never drops holding a payload and the denied turn charges nothing.
 pub fn admitted(mut self)->Result<Self,ValueError>{match self.source.as_ref(){Some(Source::Refused{..})=>match self.source.take(){Some(Source::Refused{fault,..})=>Err(fault),_=>unreachable!()},_=>Ok(self)}}
 pub fn get(&self)->&ArtifactOwnedToolJobContext<A>{match self.source.as_ref().expect("original context already transferred"){Source::Admitted(source)=>source.get(),Source::Refused{original,..}=>original}}
 pub fn admission_fault(&self)->Option<&ValueError>{match self.source.as_ref()?{Source::Admitted(_)=>None,Source::Refused{fault,..}=>Some(fault)}}
 pub fn source_issuers_ready(&self)->bool{self.admission_fault().is_none()&&self.get().source_issuers_ready()}
 pub fn try_duplicate(&self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  match self.source.as_ref(){
   Some(Source::Admitted(source))=>{let turn=source.try_duplicate(effect);RetainedWorkOutcome{result:turn.result.map(|(source,progress)|(Self{source:ManuallyDrop::new(Some(Source::Admitted(source)))},progress)),work:turn.work}},
   _=>RetainedWorkOutcome{result:Err(unadmitted()),work:0}
  }
 }
}
impl<A:ArtifactApp> std::ops::Deref for ArtifactOwnedContextHandle<A>{type Target=ArtifactOwnedToolJobContext<A>;fn deref(&self)->&Self::Target{self.get()}}
impl<A:ArtifactApp> AsRef<ArtifactOwnedToolJobContext<A>> for ArtifactOwnedContextHandle<A>{fn as_ref(&self)->&ArtifactOwnedToolJobContext<A>{self.get()}}
impl<A:ArtifactApp> Drop for ArtifactOwnedContextHandle<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"captured original context requires granted issuer handback");}}

fn unadmitted()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured context retains original child, peer, tool-run and gesture issuers")}
pub struct Cursor<A:ArtifactApp>{source:ArtifactOwnedContextHandle<A>,owner:Option<ControlledRetirement<SealedShared<ArtifactOwnedToolJobContext<A>>>>}
impl<A:ArtifactApp> Cursor<A>{fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.source.source.as_ref(){Some(Source::Refused{..})=>Err(unadmitted()),Some(Source::Admitted(_))=>Ok(RetirementDemand{depth:1,..Default::default()}),None=>self.owner.as_ref().map_or(Ok(Default::default()),|owner|owner.next_demand(body))}}}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedContextHandle<A>{type RetirementCursor=Cursor<A>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{source:self,owner:None}}fn controlled_retirement_supported()->bool{true}}
impl<A:ArtifactApp> Cursor<A>{
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();
  if self.terminal_is_empty(){return RetirementStep::Complete(0);}
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
  match self.source.source.as_ref(){Some(Source::Refused{..})=>return RetirementStep::Failure(unadmitted().into(),0),Some(Source::Admitted(_))=>{if grant.maximum_depth==0{return RetirementStep::BudgetExhausted;}if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return RetirementStep::Failure(error.into(),effect.performed_work())}let Some(Source::Admitted(source))=self.source.source.take()else{unreachable!()};self.owner=Some(ControlledRetirement::retain(source));return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},effect.performed_work());},None=>{}}
  let Some(owner)=self.owner.as_mut()else{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"captured context owner is absent").into(),0)};
  let step=match owner.step(body,effect){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}};
  if owner.terminal_is_empty(){self.owner.take();}
  step
 }
}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}

 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self.owner.as_ref(){Some(owner)=>owner.next_computational_work(body),None=>Ok(u64::from(!self.terminal_is_empty()))}}
 fn terminal_is_empty(&self)->bool{self.source.source.is_none()&&self.owner.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original context payload remains with its concrete issuers");}}
