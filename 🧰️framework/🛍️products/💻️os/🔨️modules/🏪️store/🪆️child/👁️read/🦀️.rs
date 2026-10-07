//! 👁️ Unmounted typed child read authority retains an existing owner capability.
use std::sync::Arc;
use semio_framework_value::{ValueError,ValueRefusalKind};
use {semio_framework_artifact_reference::ArtifactRef,semio_framework_artifact_reference::ArtifactDialect};
pub trait ArtifactChildSnapshotOwner<S>:Send+Sync {
 fn slot(&self)->&str;
 fn child_id(&self)->&str;
 fn artifact_id(&self)->&str;
 fn dialect(&self)->&ArtifactDialect;
 fn revision(&self)->&[u8;32];
 fn snapshot(&self)->Result<&S,ValueError>;
}
pub struct ArtifactChildRead<S>{owner:Arc<dyn ArtifactChildSnapshotOwner<S>>}
impl<S> ArtifactChildRead<S>{
 pub fn from_owner<T:ArtifactChildSnapshotOwner<S>+'static>(owner:Arc<T>)->Self{Self{owner}}
 pub fn snapshot(&self)->Result<&S,ValueError>{self.owner.snapshot()}
 pub fn revision(&self)->&[u8;32]{self.owner.revision()}
 pub fn check_target(&self,target:&ArtifactRef)->Result<(),ValueError>{
  if self.owner.artifact_id()!=target.artifact_id||self.owner.dialect()!=&target.dialect{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained child target differs from its declared handle"))}
  self.snapshot()?;
  Ok(())
 }
 pub fn check_member(&self,child_id:&str,target:&ArtifactRef)->Result<(),ValueError>{
  if self.owner.child_id()!=child_id{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained child member key differs from its declared handle"))}
  self.check_target(target)
 }
 pub fn check_identity(&self,slot:&str,child_id:&str,target:&ArtifactRef)->Result<(),ValueError>{
  if self.owner.slot()!=slot{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained child slot differs from its declared parent field"))}
  self.check_member(child_id,target)
 }
}
impl<S> Clone for ArtifactChildRead<S>{
 fn clone(&self)->Self{Self{owner:Arc::clone(&self.owner)}}
}
