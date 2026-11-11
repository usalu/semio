//! 🧬️ Lift file: 13 item(s), about 38 lines, of `🧵️retained-command/🧬️context/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (mod original_fields;, Source, ArtifactOwnedContextHandle, impl ArtifactOwnedContextHandle, impl Deref for ArtifactOwnedContextHandle, impl AsRef for ArtifactOwnedContextHandle, ...); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use crate::ArtifactApp;
use crate::tool_contract::ArtifactOwnedToolJobContext;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement},shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
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
 pub fn admit(original:ArtifactOwnedToolJobContext<A>,effect:&mut RetainedAllocationEffect<'_>)->(Self,RetainedCloneProgress){
  let(mut input,mut output)=(Some(original),None);
  let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect);
  let(source,progress)=match(turn.result,output){(Ok(step),Some(source))=>(Source::Admitted(source),step.progress()),(result,_)=>{let original=input.take().expect("refused original context admission retains its input");(Source::Refused{original,fault:result.err().unwrap_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original context admission was unfunded"))},Default::default())}};
  (Self{source:ManuallyDrop::new(Some(source))},progress)
 }
 pub fn get(&self)->&ArtifactOwnedToolJobContext<A>{match self.source.as_ref().expect("original context already transferred"){Source::Admitted(source)=>source.get(),Source::Refused{original,..}=>original}}
 pub fn admission_fault(&self)->Option<&ValueError>{match self.source.as_ref()?{Source::Admitted(_)=>None,Source::Refused{fault,..}=>Some(fault)}}
 pub fn source_issuers_ready(&self)->bool{self.admission_fault().is_none()&&self.get().source_issuers_ready()}
 pub fn try_duplicate(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{match self.source.as_ref(){Some(Source::Admitted(source))=>source.try_duplicate(effect).result.map(|(source,progress)|(Self{source:ManuallyDrop::new(Some(Source::Admitted(source)))},progress)),_=>Err(unadmitted())}}
}
impl<A:ArtifactApp> std::ops::Deref for ArtifactOwnedContextHandle<A>{type Target=ArtifactOwnedToolJobContext<A>;fn deref(&self)->&Self::Target{self.get()}}
impl<A:ArtifactApp> AsRef<ArtifactOwnedToolJobContext<A>> for ArtifactOwnedContextHandle<A>{fn as_ref(&self)->&ArtifactOwnedToolJobContext<A>{self.get()}}
impl<A:ArtifactApp> Drop for ArtifactOwnedContextHandle<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"captured original context requires granted issuer handback");}}

fn unadmitted()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured context retains original child, peer, tool-run and gesture issuers")}
pub struct Cursor<A:ArtifactApp>{source:ArtifactOwnedContextHandle<A>,owner:Option<ControlledRetirement<SealedShared<ArtifactOwnedToolJobContext<A>>>>}
impl<A:ArtifactApp> Cursor<A>{fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.source.source.as_ref(){Some(Source::Refused{..})=>Err(unadmitted()),Some(Source::Admitted(_))=>Ok(RetirementDemand{depth:1,..Default::default()}),None=>self.owner.as_ref().map_or(Ok(Default::default()),|owner|owner.next_demand(body))}}}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedContextHandle<A>{type RetirementCursor=Cursor<A>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{source:self,owner:None}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<A>>())}fn controlled_retirement_supported()->bool{true}}
impl<A:ArtifactApp> Cursor<A>{
 fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();let empty=RetainedCloneProgress::default();
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
  match self.source.source.as_ref(){Some(Source::Refused{..})=>return Err(unadmitted().into()),Some(Source::Admitted(_))=>{if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original context handback requires admitted depth").into())}let Some(Source::Admitted(source))=self.source.source.take()else{unreachable!()};self.owner=Some(ControlledRetirement::new(source).unwrap_or_else(|_|panic!("sealed original context retirement refused")));RetirementPollExit::pay(effect,1)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))},None=>{}}
  let Some(owner)=self.owner.as_mut()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"captured context owner is absent").into())};
  let step=RetirementPollExit::resume(owner.step(body,&mut effect.nested(grant)))?;if owner.terminal_is_empty(){self.owner.take();}
  Ok(RetainedCloneStep::Progress(step.progress()))
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{let result=self.step(body,effect);crate::completion_retirement::cursor_turn(effect,result)}
}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 semio_framework_value::native_retirement_reply!();
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self.owner.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.next_computational_work(body),_=>Ok(1)}}
 fn terminal_is_empty(&self)->bool{self.source.source.is_none()&&self.owner.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original context payload remains with its concrete issuers");}}
