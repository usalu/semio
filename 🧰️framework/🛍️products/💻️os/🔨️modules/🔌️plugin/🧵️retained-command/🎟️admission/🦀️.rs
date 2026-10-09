//! 🎟️ Raw command admission retains all enclosing source owners even when capacity is refused.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactRetainedAdmissionRefusal{CapacityIsZero,RawCapacityRejected}
impl ArtifactRetainedAdmissionRefusal{
 pub fn id(self)->&'static str{match self{Self::CapacityIsZero=>"capacity-is-zero",Self::RawCapacityRejected=>"raw-capacity-rejected"}}
 pub(super) fn detail(self)->&'static [u8]{match self{Self::CapacityIsZero=>b"retained command capacity is zero",Self::RawCapacityRejected=>b"retained command raw capacity was rejected"}}
}
pub(super) fn admit_raw(maximum_raw_bytes:usize,maximum_work_items:usize)->(Vec<u8>,Option<ArtifactRetainedAdmissionRefusal>){
 let mut raw=Vec::new();
 if maximum_raw_bytes==0||maximum_work_items==0{return(raw,Some(ArtifactRetainedAdmissionRefusal::CapacityIsZero));}
 if raw.try_reserve_exact(maximum_raw_bytes).is_err(){return(raw,Some(ArtifactRetainedAdmissionRefusal::RawCapacityRejected));}
 (raw,None)
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
