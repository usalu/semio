//! 🪈️ Staged slice of the `child-test-view` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧪️ Fixture seam observes actual child publication and retained store owners.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use crate::stores_components::InstanceStores;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> InstanceStores<A> {
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_parent_store_mut(&mut self)->&mut store::ArtifactStore<A::Snapshot,A::Mutation>{&mut self.store}
}
