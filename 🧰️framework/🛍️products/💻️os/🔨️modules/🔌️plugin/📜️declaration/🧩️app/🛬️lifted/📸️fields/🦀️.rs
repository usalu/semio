//! 📸️ Lift file: 12 item(s), about 59 lines, of `🧵️retained-command/🧬️context/📸️fields/🦀️.rs` that belong to crate `decl-app` (WP2).
//!
//! WP3 owns the source file and cuts the items here (ContextToolRunMetadata, ContextToolRunSources, ContextOwnedFields, ContextOriginalResidual, ContextRetirementParts, ContextResidualLeases, ...); WP2 wires this file with one `mod` line in the crate root. Until `decl-app` exists, the source module re-exports the items with `pub use`.
use semio_framework::ViewModel;
use crate::ArtifactApp;
use store::SnapshotReadProjection as ArtifactContextRoot;
use semio_framework_plugin_decl_views::gesture_contract::GestureCapture;
use crate::tool_contract::ArtifactOwnedToolJobContext;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_framework_os_kernel as store;
use semio_framework_value::{DslValue,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement,arc_custody::SharedControlledRetirement}};
use std::{mem::ManuallyDrop,sync::Arc};

#[derive(semio_framework_value::RetireOwned)]
pub struct ContextToolRunMetadata{pub tool_id:String,pub identity:semio_framework_tool_run::ToolRunIdentity,pub state:semio_framework_tool_run::ToolRunState,pub progress:semio_framework_tool_run::ToolRunProgress}
pub struct ContextToolRunSources{pub entities:Arc<std::collections::BTreeSet<u64>>,pub payload:Option<Arc<Vec<u8>>>}
#[derive(semio_framework_value::RetireOwned)]
pub struct ContextOwnedFields<D:Send+Sync+'static,T:Send+Sync+'static,M:Send+'static>{
 pub gesture:GestureCapture<M>,pub view_state:Option<ViewModel>,pub tool_run:Option<ContextToolRunMetadata>,pub draft:ArtifactContextRoot<D>,pub transient:ArtifactContextRoot<T>,pub presence:Option<store::os_store::snapshot_read::ErasedSnapshotRead>,pub window_config:Option<WindowConfigSnapshot>,pub window_transient:Option<WindowTransientSnapshot>,pub provisional:Vec<DslValue>,pub refused_reads:[Option<store::os_store::snapshot_read::ErasedSnapshotRead>;2],pub transient_source_fault:Option<String>,pub draft_source_fault:Option<String>,
}
pub struct ContextOriginalResidual<A:ArtifactApp>{pub children:Arc<ChildContentView>,pub peers:Option<Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,pub peer_factory:Option<Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,pub tool_run:Option<ContextToolRunSources>}
pub struct ContextRetirementParts<A:ArtifactApp>{pub metadata:ContextOwnedFields<A::Draft,A::Transient,A::Mutation>,pub residual:ContextOriginalResidual<A>}

/// 🔗️ The aliases a captured context borrows from roots its issuers still own: each retires as a lease, and only a last owner pays the deep release.
#[derive(semio_framework_value::RetireOwned)]
struct ContextResidualLeases{children:SharedControlledRetirement<ChildContentView>,entities:Option<SharedControlledRetirement<std::collections::BTreeSet<u64>>>,payload:Option<SharedControlledRetirement<Vec<u8>>>}

struct Cursor<A:ArtifactApp>{metadata:ControlledRetirement<ContextOwnedFields<A::Draft,A::Transient,A::Mutation>>,residual:ManuallyDrop<Option<ContextOriginalResidual<A>>>,leases:Option<ControlledRetirement<ContextResidualLeases>>,peers:Option<store::os_store::presence::PresencePeersRetirement<A::Presence>>}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original context lease depth overflow"))?;Ok(demand)}
impl<A:ArtifactApp> Cursor<A>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.metadata.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.metadata.next_copy_byte_demand()?,capacity_bytes:self.metadata.next_capacity_byte_demand(body)?,release_bytes:self.metadata.next_release_byte_demand()?,depth:self.metadata.next_depth_demand()?});}
  if self.residual.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(leases)=self.leases.as_ref(){return Ok(RetirementDemand{copy_bytes:leases.next_copy_byte_demand()?,capacity_bytes:leases.next_capacity_byte_demand(body)?,release_bytes:leases.next_release_byte_demand()?,depth:leases.next_depth_demand()?});}
  if let Some(peers)=self.peers.as_ref(){return nested(peers.retirement_demands(body)?);}
  Ok(Default::default())
 }
}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobContext<A>{
 fn retirement(self)->Box<dyn RetirementCursor>{let parts=self.into_retirement_parts();Box::new(Cursor{metadata:ControlledRetirement::new(parts.metadata).unwrap_or_else(|_|panic!("captured owned context fields refused genuine typed custody")),residual:ManuallyDrop::new(Some(parts.residual)),leases:None,peers:None})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<A>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete;}
  if grant.maximum_items==0{return RetirementStep::Progress(Default::default());}
  if !self.metadata.terminal_is_empty(){return match self.metadata.step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)};}
  if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"original context leases require admitted depth"));}
  if let Some(residual)=self.residual.take(){
   let ContextOriginalResidual{children,peers,peer_factory,tool_run}=residual;
   let(entities,payload)=match tool_run{Some(ContextToolRunSources{entities,payload})=>(Some(SharedControlledRetirement::lease(entities)),payload.map(SharedControlledRetirement::lease)),None=>(None,None)};
   self.peers=peers.map(|root|store::os_store::presence::PresencePeersRetirement::of_alias(root,peer_factory));
   return match ControlledRetirement::new(ContextResidualLeases{children:SharedControlledRetirement::lease(children),entities,payload}){Ok(owner)=>{self.leases=Some(owner);RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}Err((error,_))=>RetirementStep::Failure(error)};
  }
  if let Some(leases)=self.leases.as_mut(){
   let step=leases.step(grant);
   if leases.terminal_is_empty(){self.leases=None;}
   return match step{Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)};
  }
  let Some(peers)=self.peers.as_mut()else{return RetirementStep::Complete};
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  let step=peers.close_step(child);
  if peers.terminal_is_empty(){self.peers=None;}
  match step{Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}
 }
 fn terminal_is_empty(&self)->bool{self.metadata.terminal_is_empty()&&self.residual.is_none()&&self.leases.is_none()&&self.peers.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"captured original context retains unresolved source graph");}}
