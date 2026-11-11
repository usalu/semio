use semio_framework_value::RetirementDemand;
use semio_framework_value::{ValueError,RetainedAllocationEffect,RetainedWorkPoll,OriginalRetirementPoll};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::ArtifactApp;
use semio_plugin_ownership::store_lifecycle::ArtifactSnapshotCloseLease;
use semio_plugin_decl_media::ArtifactMediaExportCompletion;
use semio_plugin_decl_media::ArtifactMediaExportCredit;
use semio_plugin_decl_media::ArtifactOutputChunks;
use semio_plugin_interaction::retirement;

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
    pub children: semio_plugin_decl_views::ChildContentView,
    pub operation_context: semio_plugin_decl_views::AppOperationContext,
    pub port: String,
    pub output_credit: ArtifactMediaExportCredit,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactMediaExportCompletion,
}

pub struct ArtifactMediaExportRequestRetirement<A:ArtifactApp>{
 original:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,semio_plugin_decl_views::ChildContentView,semio_plugin_decl_views::AppOperationContext)>>>,
 snapshot:std::mem::ManuallyDrop<Option<std::sync::Arc<A::Snapshot>>>,lease:std::mem::ManuallyDrop<Option<ArtifactSnapshotCloseLease<A::Snapshot>>>,
 outputs:std::mem::ManuallyDrop<Option<(ArtifactMediaExportCredit,ArtifactOutputChunks,ArtifactMediaExportCompletion)>>,outputs_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(ArtifactMediaExportCredit,ArtifactOutputChunks,ArtifactMediaExportCompletion)>>>,
 metadata:std::mem::ManuallyDrop<Option<(String,String,String)>>,metadata_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<(String,String,String)>>>,
}

impl<A:ArtifactApp> ArtifactMediaExportRequestRetirement<A>{
 pub fn new(source:ArtifactMediaExportJobRequest<A>)->Self{Self{original:std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain((source.snapshot_read,source.children,source.operation_context)))),snapshot:std::mem::ManuallyDrop::new(Some(source.snapshot)),lease:std::mem::ManuallyDrop::new(Some(source.snapshot_close)),outputs:std::mem::ManuallyDrop::new(Some((source.output_credit,source.output_chunks,source.completion))),outputs_close:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(Some((source.tool_id,source.parent_document_id,source.port))),metadata_close:std::mem::ManuallyDrop::new(None)}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.snapshot.is_none()&&self.lease.is_none()&&self.outputs.is_none()&&self.outputs_close.is_none()&&self.metadata.is_none()&&self.metadata_close.is_none()}
 pub fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(one)}else{original.next_demand(body)}}if self.snapshot.is_some(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<std::sync::Arc<A::Snapshot>>(),..one})}if let Some(lease)=self.lease.as_ref(){return Ok(if lease.terminal_is_empty(){one}else{lease.retirement_demands()})}
  if self.outputs.is_some()||self.outputs_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.outputs,&self.outputs_close,body)}if self.metadata.is_some()||self.metadata_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.metadata,&self.metadata_close,body)}Ok(one)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,semio_framework_value::ValueError>{
  if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(1)}else{original.next_computational_work(body)}}
  if self.snapshot.is_some()||self.lease.is_some(){return Ok(1)}
  if self.outputs.is_some()||self.outputs_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(&self.outputs,&self.outputs_close,body)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(&self.metadata,&self.metadata_close,body)}Ok(0)
 }
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let body=grant.maximum_copy_bytes;
  let demand=match self.retirement_demands(body){Ok(demand)=>demand,Err(error)=>return settle(effect,Err(error.into()))};
  if grant.maximum_items==0||!semio_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  let item=RetainedCloneProgress{copied_items:1,..Default::default()};
  if let Some(original)=self.original.as_mut(){
   if !original.terminal_is_empty(){return original.step(body,effect)}
   if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return settle(effect,Err(error.into()))}
   self.original.take();return settle(effect,Ok(RetainedCloneStep::Progress(item)))
  }
  if let Some(snapshot)=self.snapshot.as_ref(){
   if !self.lease.as_ref().is_some_and(|lease|lease.can_release(snapshot)){return settle(effect,Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained media request lost its paired original snapshot supplier").into()))}
   if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return settle(effect,Err(error.into()))}
   self.snapshot.take();return settle(effect,Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item})))
  }
  if let Some(lease)=self.lease.as_mut(){
   if let Err(error)=semio_plugin_ownership::bounded_ownership::plugin_pay_unit(effect){return settle(effect,Err(error.into()))}
   if lease.terminal_is_empty(){self.lease.take();return settle(effect,Ok(RetainedCloneStep::Progress(item)))}
   let step=lease.close_step(grant);return settle(effect,step.map_err(Into::into))
  }
  if self.outputs.is_some()||self.outputs_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_original(&mut self.outputs,&mut self.outputs_close,effect)}
  if self.metadata.is_some()||self.metadata_close.is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_original(&mut self.metadata,&mut self.metadata_close,effect)}
  settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))
 }
}
fn settle(effect:&mut RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()})}

impl<A:ArtifactApp> Drop for ArtifactMediaExportRequestRetirement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retained media request requires its original paid close")}}
