//! 🧵️ Unmounted immutable tool document keeps its existing parent registry read and child root together.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};

pub(crate) struct ArtifactOwnedToolDocument<A:ArtifactApp>{
 parent:Option<store::SnapshotRead<A::Snapshot>>,
 children:Option<std::sync::Arc<ChildContentView>>,
 generation:u64,
 revision:[u8;32],
}

impl<A:ArtifactApp> ArtifactOwnedToolDocument<A>{
 pub(crate) fn new(parent:store::SnapshotRead<A::Snapshot>,children:std::sync::Arc<ChildContentView>,generation:u64,revision:[u8;32])->Result<Self,ValueError>{
  if !parent.commit_authority_matches(generation,revision){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"tool document parent publication differs from its captured read authority"))}
  Ok(Self{parent:Some(parent),children:Some(children),generation,revision})
 }
 pub(crate) fn snapshot(&self)->Result<&A::Snapshot,ValueError>{
  self.parent.as_ref().map(store::SnapshotRead::get).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"tool document parent read was already transferred to retirement"))
 }
 pub(crate) fn generation(&self)->u64{self.generation}
 pub(crate) fn revision(&self)->[u8;32]{self.revision}
 pub(crate) fn capture_child<S:Send+Sync+'static,F>(&self,slot:&str,select:F)->Result<store::ArtifactChildRead<S>,ValueError>
 where F:for<'a> FnOnce(&'a A::Snapshot)->&'a store::ArtifactChild<S>{
  let handle=select(self.snapshot()?);
  let children=self.children.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"tool document child root was already transferred to retirement"))?;
  let read=children.capture_read::<S>(slot,&handle.child_id,&handle.target.dialect)?;
  read.check_identity(slot,&handle.child_id,&handle.target)?;
  Ok(read)
 }
}
