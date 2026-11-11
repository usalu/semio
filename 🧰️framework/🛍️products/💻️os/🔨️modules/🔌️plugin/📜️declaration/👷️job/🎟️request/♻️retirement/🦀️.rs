//! 🧳️ Pending original requests retain all nineteen owned sources through paid cancellation and terminal closure.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_plugin_original_job_ownership::OriginalRequestLease;
type Leading<A>=(ArtifactOriginalToolCommand<<A as ArtifactApp>::Command>,Option<semio_framework::action_bus::RetainedToolWireInput>,Option<semio_framework::action_bus::RetainedToolWireInput>,String,String,String,String,String,OriginalRequestLease<<A as ArtifactApp>::Snapshot>,store::os_store::snapshot_read::SnapshotRead<<A as ArtifactApp>::Snapshot>);
type Trailing<A>=(OriginalRequestLease<<A as ArtifactApp>::Config>,Option<WindowConfigSnapshot>,OriginalRequestLease<HistoryView>,OriginalRequestLease<protocol::InteractionState>,OriginalRequestLease<InteractionHoverState>,ArtifactOwnedContextHandle<A>,ArtifactInstanceOperationOwnerHandle,ArtifactOutputChunks,ArtifactToolCompletion<A>);
type Fields<A>=(Leading<A>,Trailing<A>);

impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobRequest<A>{
 type RetirementCursor=<Fields<A> as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{
  let ArtifactOwnedToolJobRequest{command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,window_config,history,interaction_state,interaction_hover,context,instance_operation_owner,output_chunks,completion,..}=self;
  let leading=(command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,OriginalRequestLease(snapshot),snapshot_read);
  let trailing=(OriginalRequestLease(config),window_config,OriginalRequestLease(history),OriginalRequestLease(interaction_state),OriginalRequestLease(interaction_hover),context,instance_operation_owner,output_chunks,completion);
  (leading,trailing).retirement_seed()
 }
 fn controlled_retirement_supported()->bool{<A::Command as RetireOwned>::controlled_retirement_supported()&&<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}
