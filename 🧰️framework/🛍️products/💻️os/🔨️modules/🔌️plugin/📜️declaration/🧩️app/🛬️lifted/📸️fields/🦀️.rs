//! 📸️ Lift file: 12 item(s), about 59 lines, of `🧵️retained-command/🧬️context/📸️fields/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (ContextToolRunMetadata, ContextToolRunSources, ContextOwnedFields, ContextOriginalResidual, ContextRetirementParts, ContextResidualLeases, ...); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use semio_framework::ViewModel;
use crate::ArtifactApp;
use store::SnapshotReadProjection as ArtifactContextRoot;
use semio_plugin_decl_views::gesture_contract::GestureCapture;
use crate::tool_contract::ArtifactOwnedToolJobContext;
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_plugin_window_config::WindowConfigSnapshot;
use semio_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_os_kernel as store;
use semio_framework_value::{DslValue,RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedCloneStep,RetirementPollExit,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement,arc_custody::{ArcLease,original::SharedOriginalStorage}}};
use std::{mem::ManuallyDrop,sync::Arc};

pub use semio_plugin_decl_emission::ViewLease as ContextViewLease;
#[derive(semio_framework_value::RetireOwned)]
pub struct ContextToolRunMetadata{pub tool_id:String,pub identity:semio_framework_tool_run::ToolRunIdentity,pub state:semio_framework_tool_run::ToolRunState,pub progress:semio_framework_tool_run::ToolRunProgress}
pub struct ContextToolRunSources{pub entities:SharedOriginalStorage<std::collections::BTreeSet<u64>>,pub payload:Option<SharedOriginalStorage<Vec<u8>>>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ContextOwnedFields<D:Send+Sync+'static,T:Send+Sync+'static,M:Send+'static>{
 pub gesture:GestureCapture<M>,pub view_state:Option<ContextViewLease>,pub tool_run:Option<ContextToolRunMetadata>,pub original_tool_run:Option<semio_framework_tool_run_presentation::ToolRunCapturedView>,pub draft:ArtifactContextRoot<D>,pub transient:ArtifactContextRoot<T>,pub presence:Option<store::os_store::snapshot_read::ErasedSnapshotRead>,pub window_config:Option<WindowConfigSnapshot>,pub window_transient:Option<WindowTransientSnapshot>,pub provisional:Vec<DslValue>,pub refused_reads:[Option<store::os_store::snapshot_read::ErasedSnapshotRead>;2],pub transient_source_fault:Option<String>,pub draft_source_fault:Option<String>,
}
pub struct ContextOriginalResidual<A:ArtifactApp>{pub children:Arc<ChildContentView>,pub peers:Option<Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,pub peer_factory:Option<Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,pub tool_run:Option<ContextToolRunSources>}
pub struct ContextRetirementParts<A:ArtifactApp>{pub metadata:ContextOwnedFields<A::Draft,A::Transient,A::Mutation>,pub residual:ContextOriginalResidual<A>}

/// 🔗️ The aliases a captured context borrows from roots its issuers still own: each retires as a lease, and only a last owner pays the deep release.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct ContextResidualLeases{children:ArcLease<ChildContentView>,entities:Option<SharedOriginalStorage<std::collections::BTreeSet<u64>>>,payload:Option<SharedOriginalStorage<Vec<u8>>>}

pub struct Cursor<A:ArtifactApp>{metadata:ControlledRetirement<ContextOwnedFields<A::Draft,A::Transient,A::Mutation>>,residual:ManuallyDrop<Option<ContextOriginalResidual<A>>>,leases:Option<ControlledRetirement<ContextResidualLeases>>,peers:Option<store::os_store::presence::PresencePeersRetirement<A::Presence>>}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original context lease depth overflow"))?;Ok(demand)}
impl<A:ArtifactApp> Cursor<A>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.metadata.terminal_is_empty(){return self.metadata.next_demand(body);}
  if self.residual.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(leases)=self.leases.as_ref(){return leases.next_demand(body);}
  if let Some(peers)=self.peers.as_ref(){return nested(peers.retirement_demands(body)?);}
  Ok(Default::default())
 }
}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobContext<A>{
 type RetirementCursor=Cursor<A>;
 fn retirement_seed(self)->Self::RetirementCursor{let parts=self.into_retirement_parts();Cursor{metadata:ControlledRetirement::new(parts.metadata).unwrap_or_else(|_|panic!("captured owned context fields refused genuine typed custody")),residual:ManuallyDrop::new(Some(parts.residual)),leases:None,peers:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<A>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<A:ArtifactApp> Cursor<A>{
 fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();let empty=RetainedCloneProgress::default();
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
  if !self.metadata.terminal_is_empty(){return RetirementPollExit::resume(self.metadata.step(body,&mut effect.nested(grant))).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original context leases require admitted depth").into())}
  if let Some(residual)=self.residual.take(){
   let ContextOriginalResidual{children,peers,peer_factory,tool_run}=residual;
   let(entities,payload)=match tool_run{Some(ContextToolRunSources{entities,payload})=>(Some(entities),payload),None=>(None,None)};
   self.peers=peers.map(|root|store::os_store::presence::PresencePeersRetirement::of_alias(root,peer_factory));
   return match ControlledRetirement::new(ContextResidualLeases{children:ArcLease::new(children),entities,payload}){Ok(owner)=>{self.leases=Some(owner);RetirementPollExit::pay(effect,1)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}Err((error,_))=>Err(error.into())};
  }
  if let Some(leases)=self.leases.as_mut(){
   let step=RetirementPollExit::resume(leases.step(body,&mut effect.nested(grant)));
   if leases.terminal_is_empty(){self.leases=None;}
   return step.map(|step|RetainedCloneStep::Progress(step.progress()));
  }
  let Some(peers)=self.peers.as_mut()else{return Ok(RetainedCloneStep::Complete(empty))};
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  let step=peers.close_step(&mut effect.nested(child));
  if peers.terminal_is_empty(){self.peers=None;}
  step.map(|step|RetainedCloneStep::Progress(step.progress()))
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{let result=self.step(body,effect);crate::completion_retirement::cursor_turn(effect,result)}
}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 semio_framework_value::native_retirement_reply!();
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if !self.metadata.terminal_is_empty(){return self.metadata.next_computational_work(body)}match self.leases.as_ref(){Some(leases)if self.residual.is_none()&&!leases.terminal_is_empty()=>leases.next_computational_work(body),_=>Ok(1)}}
 fn terminal_is_empty(&self)->bool{self.metadata.terminal_is_empty()&&self.residual.is_none()&&self.leases.is_none()&&self.peers.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"captured original context retains unresolved source graph");}}

#[cfg(test)]
mod tests{
 use super::*;
 fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:16}}
 fn drain<T:RetireOwned>(value:T){
  let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("controlled custody"));
  for _ in 0..64{if owner.terminal_is_empty(){return}owner.step(grant()).expect("retirement step");}
  assert!(owner.terminal_is_empty(),"retirement reaches its terminal within the bound");
 }
 /// 🧩️ LAW: closing a captured context's view lease succeeds while the host window still holds the view, releases exactly its own reference and never waits for sole ownership.
 #[test]
 fn closing_a_context_view_lease_succeeds_while_the_window_still_holds_the_view(){
  let window=Arc::new(ViewModel::new(semio_framework::Locale::En,semio_framework::Terminology::Native));
  drain(ContextViewLease::new(Arc::clone(&window)));
  assert_eq!(Arc::strong_count(&window),1,"the lease released exactly its own reference");
  drain(window);
 }
}
