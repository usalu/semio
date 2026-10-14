//! 🔗️ One paid transfer lends the actual Store read retained by the original command owner.
use std::cell::RefCell;
use super::{AppOperationContext,StepContext,RetainedCloneProgress,ValueError,ValueRefusalKind};
use store::os_store::snapshot_read::SnapshotRead;
/// 🎒️ Borrows original command read custody without duplicating the captured snapshot or issuer lease.
pub struct ArtifactCommandSnapshotRead<'a,T:?Sized>{pub(super) original:&'a RefCell<Option<SnapshotRead<T>>>,pub(super) operation:&'a AppOperationContext}
impl<T:?Sized> ArtifactCommandSnapshotRead<'_,T>{
 /// 🎟️ Moves the exact issuer read only after its initiating turn funds a metadata event and revalidates current custody.
 pub fn take_current(&self,generation:u64,revision:[u8;32],cx:&mut StepContext<'_>)->Result<Option<SnapshotRead<T>>,semio_framework_job::OriginalTurnFailure>{
  let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  cx.consume_fuel(1);
  if cx.operation().0!=self.operation.operation_id||cx.generation().0!=self.operation.generation{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read requires the admitted business identity").into())}
  let Ok(mut original)=self.original.try_borrow_mut()else{return Ok(None)};
  let read=original.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read has already transferred"))?;
  if revision!=self.operation.canonical_base_revision||generation!=self.operation.generation||!read.commit_authority_matches(generation,revision){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read is no longer current at its issuer").into())}
  let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;
  Ok(original.take())
 }
}
