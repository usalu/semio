//! 🪢️ A qualified native batch transfers its exact original vector with its issued child command authority.
use super::*;
impl ChildEmitPreparation{
 /// 📏️ Quotes only the new preparation, address strings and original vector headers.
 pub fn owned_qualified_batch_birth_demand<M>(slot_bytes:usize,child_bytes:usize)->Result<RetirementDemand,ValueError>{
  let text=slot_bytes.checked_add(child_bytes).and_then(|size|size.checked_add("child.empty".len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"qualified child batch address extent overflow"))?;
  let capacity_bytes=text.checked_add(size_of::<TypedChildEmitPreparation<M>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"qualified child batch preparation extent overflow"))?;
  let copy_bytes=text.checked_add(2*size_of::<Vec<M>>()).and_then(|size|size.checked_add(2*size_of::<Self>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"qualified child batch transfer extent overflow"))?;
  Ok(RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
 }
 /// 🔏️ Insufficient grants and absent authority leave the exact original batch in its caller's custody.
 pub fn admit_owned_batch_qualified<S,M>(slot:&str,child_id:&str,operations:&mut Option<Vec<M>>,authority:&mut Option<OriginalChildEditorCommandAuthority>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>
 where M:protocol::SemanticMutation<S>+::protocol::OpBinary+RetireOwned{
  let demand=Self::owned_qualified_batch_birth_demand::<M>(slot.len(),child_id.len())?;
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(None)}
  if operations.as_ref().is_none_or(Vec::is_empty)||authority.as_ref().and_then(OriginalChildEditorCommandAuthority::stamp).is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"qualified child batch requires its original nonempty vector and issued authority"))}
  let mut preparation=Self::of_owned::<S,M>(slot.to_owned(),child_id.to_owned(),operations.take().unwrap());preparation.authority=authority.take();
  Ok(Some((preparation,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))
 }
}
