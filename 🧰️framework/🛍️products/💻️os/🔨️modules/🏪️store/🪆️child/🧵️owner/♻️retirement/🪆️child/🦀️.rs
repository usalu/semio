//! ♻️ Unmounted inline child cursor pays literal backing before releasing the exact captured owner.
use super::super::ArtifactChild;
use crate::ArtifactChildRead;
use semio_framework_value::{SnapshotRetirementStep,ValueError,ValueRefusalKind};

pub struct ArtifactChildRetirement<S>{
 buffers:[Option<Vec<u8>>;5],
 remaining:[usize;5],
 read:Option<ArtifactChildRead<S>>,
}
impl<S> ArtifactChildRetirement<S>{
 pub fn new(child:ArtifactChild<S>)->Self{
  let ArtifactChild{child_id,target,read_owner}=child;
  let semio_framework_artifact_reference::ArtifactRef{artifact_id,dialect}=target;
  let semio_framework_artifact_reference::ArtifactDialect{artifact_kind,standard,subset}=dialect;
  let buffers=[child_id,artifact_id,artifact_kind,standard,subset].map(|text|Some(text.into_bytes()));
  let remaining=std::array::from_fn(|index|buffers[index].as_ref().map_or(0,Vec::capacity));
  Self{buffers,remaining,read:read_owner}
 }
 pub fn terminal_is_empty(&self)->bool{self.buffers.iter().all(Option::is_none)&&self.read.is_none()}
 pub fn next_close_byte_demand(&self)->Option<usize>{
  if let Some(index)=self.buffers.iter().position(Option::is_some){Some(usize::from(self.remaining[index]>0))}
  else{self.read.as_ref().map(|_|std::mem::size_of::<ArtifactChildRead<S>>())}
 }
 pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
  if self.terminal_is_empty(){return Ok(SnapshotRetirementStep::Complete)}
  if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}
  if let Some(index)=self.buffers.iter().position(Option::is_some){
   let paid=maximum_bytes.min(self.remaining[index]);
   self.remaining[index]-=paid;
   let released_items=usize::from(self.remaining[index]==0);
   if released_items>0{drop(self.buffers[index].take());}
   return Ok(SnapshotRetirementStep::Pending{released_items,released_bytes:paid})
  }
  let width=std::mem::size_of::<ArtifactChildRead<S>>();
  if maximum_bytes<width{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}
  let read=self.read.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"child captured owner disappeared before exact release"))?;
  drop(read);
  Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:width})
 }
}
impl<S> Drop for ArtifactChildRetirement<S>{
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"child retirement cursor dropped before terminal-empty");}
}
