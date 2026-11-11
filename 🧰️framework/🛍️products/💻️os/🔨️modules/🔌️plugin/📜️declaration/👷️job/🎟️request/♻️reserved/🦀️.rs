//! 🧳️ Reserved requests retain every input lane until the same original caller funds its retirement.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_plugin_original_job_ownership::OriginalRequestLease;

type Fields<A>=(String,String,String,String,String,OriginalRequestLease<<A as ArtifactApp>::Snapshot>,store::os_store::snapshot_read::SnapshotRead<<A as ArtifactApp>::Snapshot>,OriginalRequestLease<<A as ArtifactApp>::Config>,OriginalRequestLease<HistoryView>,ChildContentView,Vec<u8>,(Option<ArtifactReservedToolInput>,ArtifactToolCompletion<A>));

impl<A:ArtifactApp> RetireOwned for ArtifactReservedToolJobRequest<A>{
 type RetirementCursor=<Fields<A> as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,snapshot,snapshot_read,config,history,children,raw_wire,input,completion,..}=self;(controller_id,tool_id,payload_schema_id,parent_document_id,authoring_seed,OriginalRequestLease(snapshot),snapshot_read,OriginalRequestLease(config),OriginalRequestLease(history),children,raw_wire,(input,completion)).retirement_seed()}
 fn controlled_retirement_supported()->bool{<A::Snapshot as RetireOwned>::controlled_retirement_supported()&&<A::Config as RetireOwned>::controlled_retirement_supported()}
}
