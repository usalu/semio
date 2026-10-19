//! 🏭️ Segmented output owners are quoted before their funded construction callback allocates.
use super::*;
impl ArtifactOutputChunks {
 /// 📏️ Quotes the original Arc header and fixed native chunk slots without allocating.
 pub fn original_birth_demands(maximum:usize)->Result<RetirementDemand,ValueError>{
  if maximum==0||maximum>Self::MAXIMUM_TOTAL_BYTES{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original segmented output requires its declared nonzero byte bound"))}
  let slots=maximum.div_ceil(Self::CHUNK_BYTES);
  let capacity_bytes=slots.checked_mul(size_of::<Vec<u8>>()).and_then(|bytes|bytes.checked_add(semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>())).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original output slot birth footprint overflowed"))?;
  Ok(RetirementDemand{copy_bytes:size_of::<Self>(),capacity_bytes,depth:1,..Default::default()})
 }
 /// 🎟️ A scoped original constructor authorizes every owner before its actual backing is allocated.
 pub fn from_original_construction(maximum:usize,scope:&mut semio_framework::action_bus::OriginalToolConstructionScope<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{
  scope.admit_owner(Self::original_birth_demands(maximum)?)?;
  Self::construct_original(maximum)
 }
 pub(super) fn construct_original(maximum:usize)->Result<(Self,RetainedCloneProgress),ValueError>{
  let slots=maximum.div_ceil(Self::CHUNK_BYTES);let mut chunks=std::collections::VecDeque::new();
  chunks.try_reserve_exact(slots).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original output slots could not retain their declared capacity"))?;
  let capacity_bytes=chunks.capacity()*size_of::<Vec<u8>>()+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>();
  let owner=Self{inner:std::sync::Arc::new(ArtifactOutputChunksInner{state:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactOutputChunksState{chunks:ArtifactFixedQueue{items:chunks,capacity:slots,allocation_admitted:true}}),bytes:std::sync::atomic::AtomicUsize::new(0),bytes_remaining:std::sync::atomic::AtomicUsize::new(0),remaining:std::sync::atomic::AtomicUsize::new(0),slots,maximum,sealed:std::sync::atomic::AtomicBool::new(false)})};
  Ok((owner,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),retained_capacity_bytes:capacity_bytes,..Default::default()}))
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
