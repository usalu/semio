//! 🏭️ Segmented output owners are quoted before their funded construction callback allocates.
use super::*;
impl ArtifactOutputChunks {
 /// 📏️ Quotes the original Arc header and bounded page table without allocating.
 pub fn original_birth_demands(maximum:usize)->Result<RetirementDemand,ValueError>{
  if maximum==0||maximum>Self::MAXIMUM_TOTAL_BYTES{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original segmented output requires its declared nonzero byte bound"))}
  let slots=maximum.div_ceil(Self::CHUNK_BYTES);
  let table=ArtifactOutputPages::table_bytes(slots)+ArtifactOutputPages::inline_slot_bytes(slots);
  let capacity_bytes=table+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>();
  Ok(RetirementDemand{copy_bytes:size_of::<Self>()+table,capacity_bytes,depth:1,..Default::default()})
 }
 /// 🎟️ A scoped original constructor authorizes every owner before its actual backing is allocated.
 pub fn from_original_construction(maximum:usize,scope:&mut semio_framework::action_bus::OriginalToolConstructionScope<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{
  scope.admit_owner(Self::original_birth_demands(maximum)?)?;
  Self::construct_original(maximum)
 }
 pub(super) fn construct_original(maximum:usize)->Result<(Self,RetainedCloneProgress),ValueError>{
  let slots=maximum.div_ceil(Self::CHUNK_BYTES);
  let mut chunks=ArtifactOutputPages::try_new(slots).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original output page table allocation refused"))?;
  let mut table=chunks.table_capacity_bytes();
  if ArtifactOutputPages::inline_slot_bytes(slots)!=0 {
   table+=chunks.reserve_tail().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original output first page allocation refused"))?;
  }
  let capacity_bytes=table+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>();
  let owner=Self::with_chunk_queue(maximum,slots,chunks);
  Ok((owner,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>()+table,retained_capacity_bytes:capacity_bytes,..Default::default()}))
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
