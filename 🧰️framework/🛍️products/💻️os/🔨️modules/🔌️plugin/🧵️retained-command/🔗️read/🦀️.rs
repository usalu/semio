//! 🔗️ One paid transfer lends the actual Store read retained by the original command owner.
use std::cell::RefCell;
use semio_framework_plugin_decl_views::AppOperationContext;
use semio_framework_job::StepContext;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::{ValueError, ValueRefusalKind};
use store::os_store::snapshot_read::SnapshotRead;
/// 🎒️ Borrows original command read custody without duplicating the captured snapshot or issuer lease.
pub struct ArtifactCommandSnapshotRead<'a,T:?Sized>{pub original:Option<&'a RefCell<Option<SnapshotRead<T>>>>,pub operation:&'a AppOperationContext,pub(super) source:Option<semio_framework::action_bus::OriginalToolSourceBinding>}
impl<'a,T:?Sized> ArtifactCommandSnapshotRead<'a,T>{
 /// 🕳️ Pure work inputs declare the absence of captured Store read custody explicitly.
 pub fn unavailable(operation:&'a AppOperationContext)->Self{Self{original:None,operation,source:None}}

 /// 🪪️ Borrows a genuine current issuer read under this initiating physical caller without minting authority.
 pub fn borrow_original(original:&'a RefCell<Option<SnapshotRead<T>>>,operation:&'a AppOperationContext,cx:&StepContext<'_>)->Result<Self,ValueError>{
  let read=original.try_borrow().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read custody is already borrowed"))?;
  let current=read.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read custody is absent"))?;
  if !current.commit_authority_matches(operation.generation,operation.canonical_base_revision){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read issuer differs from its business operation"))}
  Ok(Self{original:Some(original),operation,source:Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx))})
 }
 /// 🎟️ Moves the exact issuer read only after its initiating turn funds a metadata event and revalidates current custody.
 pub fn take_current(&self,generation:u64,revision:[u8;32],cx:&mut StepContext<'_>)->Result<Option<SnapshotRead<T>>,semio_framework_job::OriginalTurnFailure>{
  let original=self.original.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"this work input has no captured snapshot read"))?;
  let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  cx.consume_fuel(1);
  if !self.source.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read requires its admitted physical source binding").into())}
  let Ok(mut original)=original.try_borrow_mut()else{return Ok(None)};
  let read=original.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read has already transferred"))?;
  if revision!=self.operation.canonical_base_revision||generation!=self.operation.generation||!read.commit_authority_matches(generation,revision){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command read is no longer current at its issuer").into())}
  let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;
  Ok(original.take())
 }
}
