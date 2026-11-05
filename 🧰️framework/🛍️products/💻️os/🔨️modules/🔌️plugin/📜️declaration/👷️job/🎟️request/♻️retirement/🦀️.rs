//! 🧳️ Pending original requests retain all nineteen owned sources through paid cancellation and terminal closure.
use super::ArtifactOwnedToolJobRequest;
use crate::ArtifactApp;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,sequence,sequence_birth_bytes,deferred,deferred_birth_bytes,deferred_birth_bytes_for};
use std::sync::Arc;

use semio_framework_plugin_original_job_ownership::OriginalRequestLease;
fn lease_field_birth<T:RetireOwned+Sync>(_:&Arc<T>)->usize{deferred_birth_bytes::<OriginalRequestLease<T>>()}

impl<A:ArtifactApp> RetireOwned for ArtifactOwnedToolJobRequest<A>{
 fn retirement(self)->Box<dyn RetirementCursor>{
  let ArtifactOwnedToolJobRequest{command,input,checkpoint,controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,window_config,history,interaction_state,interaction_hover,context,instance_operation_owner,output_chunks,completion,..}=self;
  let snapshot=OriginalRequestLease(snapshot);let config=OriginalRequestLease(config);let history=OriginalRequestLease(history);let interaction_state=OriginalRequestLease(interaction_state);let interaction_hover=OriginalRequestLease(interaction_hover);
  sequence(vec![deferred(command),deferred(input),deferred(checkpoint),deferred(controller_id),deferred(tool_id),deferred(payload_schema_id),deferred(parent_document_id),deferred(authoring_seed),deferred(snapshot),deferred(snapshot_read),deferred(config),deferred(window_config),deferred(history),deferred(interaction_state),deferred(interaction_hover),deferred(context),deferred(instance_operation_owner),deferred(output_chunks),deferred(completion)])
 }
 fn retirement_birth_bytes(&self)->Option<usize>{
  sequence_birth_bytes(&[deferred_birth_bytes_for(&self.command),deferred_birth_bytes_for(&self.input),deferred_birth_bytes_for(&self.checkpoint),deferred_birth_bytes_for(&self.controller_id),deferred_birth_bytes_for(&self.tool_id),deferred_birth_bytes_for(&self.payload_schema_id),deferred_birth_bytes_for(&self.parent_document_id),deferred_birth_bytes_for(&self.authoring_seed),lease_field_birth(&self.snapshot),deferred_birth_bytes_for(&self.snapshot_read),lease_field_birth(&self.config),deferred_birth_bytes_for(&self.window_config),lease_field_birth(&self.history),lease_field_birth(&self.interaction_state),lease_field_birth(&self.interaction_hover),deferred_birth_bytes_for(&self.context),deferred_birth_bytes_for(&self.instance_operation_owner),deferred_birth_bytes_for(&self.output_chunks),deferred_birth_bytes_for(&self.completion)])
 }
 fn controlled_retirement_supported()->bool{<A::Command as RetireOwned>::controlled_retirement_supported()&&<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}


