//! 🧵️ Unmounted paid parent factory binds the exact host child read after complete field construction.
use crate::RewritingSnapshot;
use store::ArtifactChildRead;
use semio_framework_plugin::ChildContentView;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_value::{DecodedValue,NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot,STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};

pub(crate) const WORKING_CHILD_SLOT:&str="workingGraph";

fn retire_parent(value:RewritingSnapshot){
 <RewritingSnapshot as semio_framework_dsl_record::DslField>::retire_decoded(value);
}

pub(crate) fn capture(parent:&RewritingSnapshot,children:&ChildContentView)->Result<ArtifactChildRead<SemioGraphSnapshot>,ValueError>{
 let handle=&parent.working_graph.content;
 let read=children.capture_read::<SemioGraphSnapshot>(WORKING_CHILD_SLOT,&handle.target.artifact_id,&handle.target.dialect)?;
 read.check_identity(WORKING_CHILD_SLOT,&handle.target)?;
 if read.snapshot()?.schema!=STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Rewriting captured child has a different Semio graph document schema"))}
 Ok(read)
}

pub(crate) fn owned_parent(parent:&RewritingSnapshot,read:ArtifactChildRead<SemioGraphSnapshot>,control:&mut SqliteSnapshotControl<'_>)->Result<RewritingSnapshot,ValueError>{
 read.check_identity(WORKING_CHILD_SLOT,&parent.working_graph.content.target)?;
 let record=control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint,allocation|{
  let mut callback=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(progress.completed,progress.total);
  let mut native_allocation=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|allocation(request.bytes);let mut native=NativeEncodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
  let result=parent.__dsl_to_record_controlled(&mut native);
  (result,native.owned_bytes())
 })??;
 let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(record);
 let output=control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint,allocation|{
  let mut callback=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(progress.completed,progress.total);
  let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
  let result=RewritingSnapshot::__dsl_from_record_controlled(record.as_record(),&mut native);
  (result,native.owned_bytes())
 })??;
 let mut output=DecodedValue::new(output,retire_parent);
 output.get_mut().working_graph.content.bind_read(WORKING_CHILD_SLOT,read)?;
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;
 Ok(output.take())
}
