//! 🪆️ Unmounted canonical typed child metadata and exact existing registry read owner.
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
use {semio_framework_artifact_reference::ArtifactRef,semio_framework_artifact_reference::ArtifactDialect};
use crate::ArtifactChildRead;

pub struct ArtifactChild<S> {
 pub child_id:String,
 pub target:ArtifactRef,
 read_owner:Option<ArtifactChildRead<S>>,
}
impl<S> ArtifactChild<S> {
 pub fn new(child_id:String,target:ArtifactRef)->Self {Self{child_id,target,read_owner:None}}
 pub fn bind_read(&mut self,slot:&str,read:ArtifactChildRead<S>)->Result<(),ValueError> {
  read.check_identity(slot,&self.child_id,&self.target)?;
  self.read_owner=Some(read);
  Ok(())
 }
 pub fn snapshot(&self)->Result<&S,ValueError> {
  let read=self.read_owner.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact child has no retained snapshot read"))?;
  read.check_member(&self.child_id,&self.target)?;
  read.snapshot()
 }
 pub fn read_owner(&self)->Result<ArtifactChildRead<S>,ValueError> {
  let read=self.read_owner.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact child has no retained snapshot read"))?;
  read.check_member(&self.child_id,&self.target)?;
  Ok(read.clone())
 }
 pub fn take_read_owner(&mut self)->Option<ArtifactChildRead<S>> {self.read_owner.take()}
 pub fn clone_controlled(&self,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError> {
  if let Some(read)=&self.read_owner{read.check_member(&self.child_id,&self.target)?;}
  control.step()?;
  let child_id=control.copy_text(&self.child_id)?;
  let artifact_id=control.copy_text(&self.target.artifact_id)?;
  let artifact_kind=control.copy_text(&self.target.dialect.artifact_kind)?;
  let standard=control.copy_text(&self.target.dialect.standard)?;
  let subset=control.copy_text(&self.target.dialect.subset)?;
  control.step()?;
  Ok(Self{child_id,target:ArtifactRef{artifact_id,dialect:ArtifactDialect{artifact_kind,standard,subset}},read_owner:self.read_owner.clone()})
 }
}
impl<S> std::fmt::Debug for ArtifactChild<S> {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.debug_struct("ArtifactChild").field("child_id",&self.child_id).field("target",&self.target).finish()}
}
impl<S> PartialEq for ArtifactChild<S> {
 fn eq(&self,other:&Self)->bool {self.child_id==other.child_id&&self.target==other.target}
}
