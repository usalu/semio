//! ♻️ Unmounted inline read-owner release cursor; the registry keeps the full typed snapshot.
use semio_framework_value::{ValueError,ValueRefusalKind,SnapshotRetirementStep};
use crate::ArtifactChildRead;

pub struct ArtifactChildReadRetirement<S> {owner:Option<ArtifactChildRead<S>>}
impl<S> ArtifactChildReadRetirement<S> {
 pub fn new(owner:ArtifactChildRead<S>)->Self {Self{owner:Some(owner)}}
 pub fn terminal_is_empty(&self)->bool {self.owner.is_none()}
 pub fn next_close_byte_demand(&self)->Option<usize> {self.owner.as_ref().map(|_|std::mem::size_of::<ArtifactChildRead<S>>())}
 pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError> {
  if self.owner.is_none(){return Ok(SnapshotRetirementStep::Complete);}
  let width=std::mem::size_of::<ArtifactChildRead<S>>();
  if maximum_items==0||maximum_bytes<width {return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
  let owner=self.owner.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"child read release owner disappeared"))?;
  drop(owner);
  Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:width})
 }
}
