//! 🎴️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-app` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::VcsArtifactApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧪️ Direct access to the wrapped app — used to assert on app-private test fixtures (e.g.
    /// `TestApp::received_actions`) that a framework-owned interception must never populate.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub async fn test_app(&self) -> &A {
        &self.app
    }
}
