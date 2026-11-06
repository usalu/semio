//! 🧳️ Reserved requests retain every input lane until the same original caller funds its retirement.
use super::ArtifactReservedToolJobRequest;
use crate::ArtifactApp;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,sequence,sequence_birth_bytes,deferred,deferred_birth_bytes_for};
use semio_framework_plugin_original_job_ownership::OriginalRequestLease;
impl<A:ArtifactApp> RetireOwned for ArtifactReservedToolJobRequest<A>{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,history,children,raw_wire,input,completion,..}=self;sequence(vec![deferred(controller_id),deferred(tool_id),deferred(payload_schema_id),deferred(parent_document_id),deferred(authoring_seed),deferred(OriginalRequestLease(snapshot)),deferred(snapshot_read),deferred(OriginalRequestLease(config)),deferred(OriginalRequestLease(history)),deferred(children),deferred(raw_wire),deferred(input),deferred(completion)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::deferred_birth_bytes;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.controller_id),deferred_birth_bytes_for(&self.tool_id),deferred_birth_bytes_for(&self.payload_schema_id),deferred_birth_bytes_for(&self.parent_document_id),deferred_birth_bytes_for(&self.authoring_seed),deferred_birth_bytes::<OriginalRequestLease<A::Snapshot>>(),deferred_birth_bytes_for(&self.snapshot_read),deferred_birth_bytes::<OriginalRequestLease<A::Config>>(),deferred_birth_bytes::<OriginalRequestLease<semio_plugin_decl_views::history_view::HistoryView>>(),deferred_birth_bytes_for(&self.children),deferred_birth_bytes_for(&self.raw_wire),deferred_birth_bytes_for(&self.input),deferred_birth_bytes_for(&self.completion)])}
 fn controlled_retirement_supported()->bool{<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}
