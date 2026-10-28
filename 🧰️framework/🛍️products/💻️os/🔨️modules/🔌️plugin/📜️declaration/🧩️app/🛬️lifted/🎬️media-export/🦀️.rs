use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseLease;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportCompletion;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportCredit;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_interaction::retirement;

#[path="📖️read/🦀️.rs"]mod original_read;
pub use original_read::{media_original_read_capture_demand,media_original_read_matches};

/// 🎞️ Exact live-instance and operation authority for one resumable media export.
pub struct ArtifactMediaExportJobRequest<A: ArtifactApp> {
    pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub retained:RetainedCloneGrant,
    pub operation: semio_framework_job::Operation,
    pub tool_id: String,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub snapshot: std::sync::Arc<A::Snapshot>,
    pub snapshot_close: ArtifactSnapshotCloseLease<A::Snapshot>,
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub children: semio_framework_plugin_decl_views::ChildContentView,
    pub operation_context: semio_framework_plugin_decl_views::AppOperationContext,
    pub port: String,
    pub output_credit: ArtifactMediaExportCredit,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactMediaExportCompletion,
}

pub struct ArtifactMediaExportRequestRetirement<A:ArtifactApp>{
 original:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,semio_framework_plugin_decl_views::ChildContentView,semio_framework_plugin_decl_views::AppOperationContext)>>>,
 snapshot:std::mem::ManuallyDrop<Option<std::sync::Arc<A::Snapshot>>>,lease:std::mem::ManuallyDrop<Option<ArtifactSnapshotCloseLease<A::Snapshot>>>,
 outputs:std::mem::ManuallyDrop<Option<(ArtifactMediaExportCredit,ArtifactOutputChunks,ArtifactMediaExportCompletion)>>,outputs_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(ArtifactMediaExportCredit,ArtifactOutputChunks,ArtifactMediaExportCompletion)>>>,
 metadata:std::mem::ManuallyDrop<Option<(String,String,String)>>,metadata_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,String,String)>>>,
}

impl<A:ArtifactApp> ArtifactMediaExportRequestRetirement<A>{
 pub fn new(source:ArtifactMediaExportJobRequest<A>)->Self{Self{original:std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain((source.snapshot_read,source.children,source.operation_context)))),snapshot:std::mem::ManuallyDrop::new(Some(source.snapshot)),lease:std::mem::ManuallyDrop::new(Some(source.snapshot_close)),outputs:std::mem::ManuallyDrop::new(Some((source.output_credit,source.output_chunks,source.completion))),outputs_close:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(Some((source.tool_id,source.parent_document_id,source.port))),metadata_close:std::mem::ManuallyDrop::new(None)}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.snapshot.is_none()&&self.lease.is_none()&&self.outputs.is_none()&&self.outputs_close.is_none()&&self.metadata.is_none()&&self.metadata_close.is_none()}
 pub fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(one)}else{original.next_demand(body)}}if self.snapshot.is_some(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..one})}if let Some(lease)=self.lease.as_ref(){return Ok(if lease.terminal_is_empty(){one}else{lease.retirement_demands()})}
  if self.outputs.is_some()||self.outputs_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.outputs,&self.outputs_close,body)}if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.metadata,&self.metadata_close,body)}Ok(one)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
  let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||!semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))}let item=RetainedCloneProgress{copied_items:1,..Default::default()};
  if let Some(original)=self.original.as_mut(){if original.terminal_is_empty(){self.original.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}return original.step(grant)}
  if let Some(snapshot)=self.snapshot.as_ref(){if !self.lease.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained media request lost its paired original snapshot supplier"))}self.snapshot.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item}))}
  if let Some(lease)=self.lease.as_mut(){if lease.terminal_is_empty(){self.lease.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}return lease.close_step(grant)}
  if self.outputs.is_some()||self.outputs_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.outputs,&mut self.outputs_close,grant)}if self.metadata.is_some()||self.metadata_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.metadata,&mut self.metadata_close,grant)}Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))
 }
}

impl<A:ArtifactApp> Drop for ArtifactMediaExportRequestRetirement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retained media request requires its original paid close")}}
