//! 🎟️ Raw command admission retains all enclosing source owners even when capacity is refused.
/// 🧾️ A static, allocation-free canonical Fault wire (`origin`, `code`, `severity`, `message`, `scope`, `retainedProgress`, `retryable`) a retained command settles its own refusals with, so the host decodes the code instead of an undecodable plain text page; see [`semio_framework_diagnostic::try_decode_fault_bytes`].
macro_rules! static_fault{($code:literal,$message:literal)=>{concat!(r#"{"origin":"framework","code":""#,$code,r#"","severity":"error","message":""#,$message,r#"","scope":{},"retainedProgress":{"copiedItems":0,"copiedBytes":0,"retainedCapacityBytes":0,"releasedBytes":0},"retryable":false}"#).as_bytes()}}
pub(crate) use static_fault;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactRetainedAdmissionRefusal{CapacityIsZero,RawCapacityRejected}
impl ArtifactRetainedAdmissionRefusal{
 pub fn id(self)->&'static str{match self{Self::CapacityIsZero=>"capacity-is-zero",Self::RawCapacityRejected=>"raw-capacity-rejected"}}
 pub fn detail(self)->&'static [u8]{match self{Self::CapacityIsZero=>static_fault!("interactive-job.command-capacity-zero","retained command capacity is zero"),Self::RawCapacityRejected=>static_fault!("interactive-job.command-raw-capacity-rejected","retained command raw capacity was rejected")}}
}
/// 🎟️ Refuses a command whose declared ceilings are zero before any raw bytes are owned.
pub(super) fn admit_limits(maximum_raw_bytes:usize,maximum_work_items:usize)->Option<ArtifactRetainedAdmissionRefusal>{
 (maximum_raw_bytes==0||maximum_work_items==0).then_some(ArtifactRetainedAdmissionRefusal::CapacityIsZero)
}
/// 📏️ Reserves exactly the wire bytes the command declares, never its ceiling: the buffer is released by one funded turn, so a ceiling-sized reservation (16 MiB for a snapshot edit) could never be closed under the 1 MiB release grant.
pub(super) fn admit_raw(declared_bytes:usize,maximum_raw_bytes:usize)->(Vec<u8>,Option<ArtifactRetainedAdmissionRefusal>){
 let mut raw=Vec::new();
 if declared_bytes>maximum_raw_bytes||raw.try_reserve_exact(declared_bytes).is_err(){return(raw,Some(ArtifactRetainedAdmissionRefusal::RawCapacityRejected));}
 (raw,None)
}
#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
