//! 📸️ Original captured metadata/read leaves and unresolved source graphs retain separate custody.
use semio_framework::ViewModel;
use crate::app::{ArtifactApp,ArtifactContextRoot,ArtifactOwnedToolJobContext,ChildContentView,GestureCapture,WindowConfigSnapshot,WindowTransientSnapshot};
use semio_framework_os_kernel as store;
use semio_framework_value::{DslValue,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetirementCursor,RetirementStep,RetireOwned,controlled::ControlledRetirement}};
use std::{mem::ManuallyDrop,sync::Arc};

#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct ContextToolRunMetadata{pub tool_id:String,pub identity:semio_framework_tool_run::ToolRunIdentity,pub state:semio_framework_tool_run::ToolRunState,pub progress:semio_framework_tool_run::ToolRunProgress}
pub(crate) struct ContextToolRunSources{pub entities:Arc<std::collections::BTreeSet<u64>>,pub payload:Option<Arc<[u8]>>}
#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct ContextOwnedFields<D:Send+Sync+'static,T:Send+Sync+'static,M:Send+'static>{
 pub gesture:GestureCapture<M>,pub view_state:Option<ViewModel>,pub tool_run:Option<ContextToolRunMetadata>,pub draft:ArtifactContextRoot<D>,pub transient:ArtifactContextRoot<T>,pub presence:Option<store::ErasedSnapshotRead>,pub window_config:Option<WindowConfigSnapshot>,pub window_transient:Option<WindowTransientSnapshot>,pub provisional:Vec<DslValue>,pub refused_reads:[Option<store::ErasedSnapshotRead>;2],pub transient_source_fault:Option<String>,
}
pub(crate) struct ContextOriginalResidual<A:ArtifactApp>{pub children:Arc<ChildContentView>,pub peers:Option<Arc<store::PresencePeersRoot<A::Presence>>>,pub tool_run:Option<ContextToolRunSources>,pub draft_source_fault:Option<store::VcsError>}
pub(crate) struct ContextRetirementParts<A:ArtifactApp>{pub metadata:ContextOwnedFields<A::Draft,A::Transient,A::Mutation>,pub residual:ContextOriginalResidual<A>}

struct Cursor<A:ArtifactApp>{metadata:ControlledRetirement<ContextOwnedFields<A::Draft,A::Transient,A::Mutation>>,residual:ManuallyDrop<Option<ContextOriginalResidual<A>>>}
impl<A:ArtifactApp> Cursor<A>{fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.metadata.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.metadata.next_copy_byte_demand()?,capacity_bytes:self.metadata.next_capacity_byte_demand(body)?,release_bytes:self.metadata.next_release_byte_demand()?,depth:self.metadata.next_depth_demand()?});}if self.residual.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original context source graph still requires its concrete issuer"));}Ok(Default::default())}}
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobContext<A>{
 fn retirement(self)->Box<dyn RetirementCursor>{let parts=self.into_retirement_parts();Box::new(Cursor{metadata:ControlledRetirement::new(parts.metadata).unwrap_or_else(|_|panic!("captured owned context fields refused genuine typed custody")),residual:ManuallyDrop::new(Some(parts.residual))})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<A>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<A:ArtifactApp> RetirementCursor for Cursor<A>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}if grant.maximum_items==0{return RetirementStep::Progress(Default::default());}if self.metadata.terminal_is_empty(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original context source graph remains retained"));}match self.metadata.step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.metadata.terminal_is_empty()&&self.residual.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<A:ArtifactApp> Drop for Cursor<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"captured original context retains unresolved source graph");}}
