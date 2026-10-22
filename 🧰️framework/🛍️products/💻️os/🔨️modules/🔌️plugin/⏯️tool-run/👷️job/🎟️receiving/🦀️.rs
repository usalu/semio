//! 📖️ The installed ToolRun receiver borrows the exact paid Job pages until descriptor acknowledgement.
pub(crate) struct OriginalToolRunPayload<'a>(pub(crate) &'a semio_framework_job_payload::RetainedJobPayload);
impl replication::codec::RetainedWireByteSource for OriginalToolRunPayload<'_>{
 fn original_source_identity(&self)->replication::codec::OriginalWireSourceIdentity{let(address,extent,authority)=self.0.original_wire_identity();replication::codec::OriginalWireSourceIdentity{address,extent,authority}}
 fn byte_at(&self,index:usize)->Option<u8>{self.0.original_byte_at(index)}
}
