//! 🎟️ Raw command admission retains all enclosing source owners even when capacity is refused.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactRetainedAdmissionRefusal{CapacityIsZero,RawCapacityRejected}
impl ArtifactRetainedAdmissionRefusal{
 pub fn id(self)->&'static str{match self{Self::CapacityIsZero=>"capacity-is-zero",Self::RawCapacityRejected=>"raw-capacity-rejected"}}
 pub fn detail(self)->&'static [u8]{match self{Self::CapacityIsZero=>b"retained command capacity is zero",Self::RawCapacityRejected=>b"retained command raw capacity was rejected"}}
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
