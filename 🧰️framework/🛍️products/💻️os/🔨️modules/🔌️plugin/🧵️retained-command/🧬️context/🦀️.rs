//! 🎒️ Captured command contexts retain their original sealed header and concrete payload issuers.
use crate::app::{ArtifactApp,ArtifactOwnedToolJobContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::ManuallyDrop;
#[path="📸️fields/🦀️.rs"]
mod original_fields;
pub(crate) use original_fields::{ContextOwnedFields,ContextOriginalResidual,ContextRetirementParts,ContextToolRunMetadata,ContextToolRunSources};

enum Source<A:ArtifactApp>{Admitted(SealedShared<ArtifactOwnedToolJobContext<A>>),Refused{original:ArtifactOwnedToolJobContext<A>,fault:ValueError}}

/// 🔐️ No raw Arc, Weak or infallible alias cloning escapes the original context issuer.
pub struct ArtifactOwnedContextHandle<A:ArtifactApp>{source:ManuallyDrop<Option<Source<A>>>}
impl<A:ArtifactApp> ArtifactOwnedContextHandle<A>{
 pub fn admit(original:ArtifactOwnedToolJobContext<A>,grant:RetainedCloneGrant)->(Self,RetainedCloneProgress){
  let issuer=SharedIssuer::owned();
  let(source,progress)=match SealedShared::admit(original,issuer,grant){Ok((source,progress))=>(Source::Admitted(source),progress),Err((fault,original))=>(Source::Refused{original,fault},Default::default())};
  (Self{source:ManuallyDrop::new(Some(source))},progress)
 }
 pub fn get(&self)->&ArtifactOwnedToolJobContext<A>{match self.source.as_ref().expect("original context already transferred"){Source::Admitted(source)=>source.get(),Source::Refused{original,..}=>original}}
 pub fn admission_fault(&self)->Option<&ValueError>{match self.source.as_ref()?{Source::Admitted(_)=>None,Source::Refused{fault,..}=>Some(fault)}}
 pub fn source_issuers_ready(&self)->bool{self.admission_fault().is_none()&&self.get().source_issuers_ready()}
 pub fn try_duplicate(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{match self.source.as_ref(){Some(Source::Admitted(source))=>source.try_duplicate(grant).map(|(source,progress)|(Self{source:ManuallyDrop::new(Some(Source::Admitted(source)))},progress)),_=>Err(unadmitted())}}
}
impl<A:ArtifactApp> std::ops::Deref for ArtifactOwnedContextHandle<A>{type Target=ArtifactOwnedToolJobContext<A>;fn deref(&self)->&Self::Target{self.get()}}
impl<A:ArtifactApp> AsRef<ArtifactOwnedToolJobContext<A>> for ArtifactOwnedContextHandle<A>{fn as_ref(&self)->&ArtifactOwnedToolJobContext<A>{self.get()}}
impl<A:ArtifactApp> Drop for ArtifactOwnedContextHandle<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"captured original context requires granted issuer handback");}}

fn unadmitted()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured context retains original child, peer, tool-run and gesture issuers")}
struct Cursor<A:ArtifactApp>{source:ArtifactOwnedContextHandle<A>,owner:Option<ControlledRetirement<SealedShared<ArtifactOwnedToolJobContext<A>>>>}
impl<A:ArtifactApp> Cursor<A>{fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.source.source.as_ref(){Some(Source::Refused{..})=>Err(unadmitted()),Some(Source::Admitted(_))=>Ok(RetirementDemand{depth:1,..Default::default()}),None=>self.owner.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))}}}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedContextHandle<A>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{source:self,owner:None})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<A>>())}fn controlled_retirement_supported()->bool{true}}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete;}
  if grant.maximum_items==0{return RetirementStep::Progress(Default::default());}
  match self.source.source.as_ref(){Some(Source::Refused{..})=>return RetirementStep::Failure(unadmitted()),Some(Source::Admitted(_))=>{if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"original context handback requires admitted depth"));}let Some(Source::Admitted(source))=self.source.source.take()else{unreachable!()};self.owner=Some(ControlledRetirement::new(source).unwrap_or_else(|_|panic!("sealed original context retirement refused")));return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});},None=>{}}
  let owner=self.owner.as_mut().unwrap();match owner.step(grant){Ok(step)=>{let progress=step.progress();if owner.terminal_is_empty(){self.owner.take();}RetirementStep::Progress(progress)},Err(error)=>RetirementStep::Failure(error)}
 }
 fn terminal_is_empty(&self)->bool{self.source.source.is_none()&&self.owner.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original context payload remains with its concrete issuers");}}
