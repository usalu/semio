//! 🧵️ Unmounted host capture retains the exact immutable entry and its existing snapshot lease.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn capture_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
impl<S:Send+Sync+'static> store::ArtifactChildSnapshotOwner<S> for ChildContentEntry{
 fn slot(&self)->&str{&self.slot}
 fn child_id(&self)->&str{&self.child_id}
 fn artifact_id(&self)->&str{&self.reference.artifact_id}
 fn dialect(&self)->&ArtifactDialect{&self.reference.dialect}
 fn revision(&self)->&[u8;32]{&self.revision}
 fn snapshot(&self)->Result<&S,ValueError>{
  self.snapshot.get::<S>().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"captured child snapshot exact type changed"))
 }
}
impl ChildContentView{
 pub fn capture_read<S:Send+Sync+'static>(&self,slot:&str,child_id:&str,dialect:&ArtifactDialect)->Result<store::ArtifactChildRead<S>,ValueError>{
  if slot.len()>CHILD_CONTENT_ID_BYTES||child_id.len()>CHILD_CONTENT_ID_BYTES{return Err(capture_invalid("child capture identity exceeds fixed metadata bounds"))}
  let root=self.root.as_deref().ok_or_else(||capture_invalid("no immutable child root to capture"))?;
  let entry=root.pages.iter().flatten().flat_map(|page|page.entries.iter().flatten()).find(|entry|entry.slot==slot&&entry.child_id==child_id).ok_or_else(||capture_invalid("no exact child entry to capture"))?;
  if &entry.dialect!=dialect{return Err(capture_invalid("captured child dialect differs from the declared handle"))}
  if entry.snapshot.get::<S>().is_none(){return Err(capture_invalid("captured child snapshot exact type differs"))}
  Ok(store::ArtifactChildRead::from_owner(std::sync::Arc::clone(entry)))
 }
}
