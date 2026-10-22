//! 🪇️ Staged slice of the `child-test-view` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🧪️ Fixture seam observes actual child publication and retained store owners.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::children_components::ChildComposition;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_content_view(&self)->ChildContentView{ChildContentView::clone(&self.child_content_root)}

 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_member_mut(&mut self,key:&(String,String))->Option<&mut M>{self.children.get_mut(key).map(|entry|&mut entry.member)}

 #[cfg(any(test,feature="artifact-app-testing"))]
 pub async fn test_publish_child_content(&mut self,slot:&str,child_id:&str)->Result<(),Fault>{
  let generation=self.admit_child_content_publication()?;
  self.publish_child_content_member(generation, slot, child_id).await
 }
}
