//! 🧳️ Pending original requests retain all nineteen owned sources through paid cancellation and terminal closure.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_plugin_original_job_ownership::OriginalRequestLease;
impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobRequest<A>{
 type RetirementCursor=semio_framework_value::retirement_fields_type!(ArtifactOriginalToolCommand<A::Command>,Option<semio_framework::action_bus::RetainedToolWireInput>,Option<semio_framework::action_bus::RetainedToolWireInput>,String,String,String,String,String,OriginalRequestLease<A::Snapshot>,store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,OriginalRequestLease<A::Config>,Option<WindowConfigSnapshot>,OriginalRequestLease<HistoryView>,OriginalRequestLease<protocol::InteractionState>,OriginalRequestLease<InteractionHoverState>,ArtifactOwnedContextHandle<A>,ArtifactInstanceOperationOwnerHandle,ArtifactOutputChunks,ArtifactToolCompletion<A>);
 fn retirement_seed(self)->Self::RetirementCursor{
  let ArtifactOwnedToolJobRequest{command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,window_config,history,interaction_state,interaction_hover,context,instance_operation_owner,output_chunks,completion,..}=self;
  semio_framework_value::retirement_fields_seed!(command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,OriginalRequestLease(snapshot),snapshot_read,OriginalRequestLease(config),window_config,OriginalRequestLease(history),OriginalRequestLease(interaction_state),OriginalRequestLease(interaction_hover),context,instance_operation_owner,output_chunks,completion)
 }
 fn controlled_retirement_supported()->bool{<A::Command as RetireOwned>::controlled_retirement_supported()&&<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}
