//! 📖️ The installed ToolRun receiver borrows the exact paid Job pages until descriptor acknowledgement.
use semio_framework_os_kernel as protocol;
pub(crate) struct OriginalToolRunPayload<'a>(pub(crate) &'a semio_framework_job_payload::RetainedJobPayload);
impl protocol::codec::RetainedWireByteSource for OriginalToolRunPayload<'_>{
 fn original_source_identity(&self)->protocol::codec::OriginalWireSourceIdentity{let(address,extent,authority)=self.0.original_wire_identity();protocol::codec::OriginalWireSourceIdentity{address,extent,authority}}
 fn byte_at(&self,index:usize)->Option<u8>{self.0.original_byte_at(index)}
}
