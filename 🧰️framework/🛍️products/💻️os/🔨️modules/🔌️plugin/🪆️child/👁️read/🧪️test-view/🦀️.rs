//! 🧪️ Fixture seam observes actual child publication and retained store owners.
use super::*;
impl<A:ArtifactApp,M:store::SpaceMember+store::MemberFactory+'static> VcsArtifactApp<A,M>{
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_content_view(&self)->ChildContentView{ChildContentView::clone(&self.child_content_root)}
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_parent_store_mut(&mut self)->&mut store::ArtifactStore<A::Snapshot,A::Mutation>{&mut self.store}
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_member_mut(&mut self,key:&(String,String))->Option<&mut M>{self.children.get_mut(key).map(|entry|&mut entry.member)}
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub async fn test_publish_child_content(&mut self,slot:&str,child_id:&str)->Result<(),Fault>{
  let generation=self.admit_child_content_publication()?;
  self.publish_child_content_member(generation,slot,child_id).await
 }
}
