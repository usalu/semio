//! 🪸️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_os_kernel as store;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::Mutation;
use protocol::OpText;
use std::future::Future;
use crate::stores_components::InstanceStores;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel::*;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🖋️ The actor this replica authors history transitions as — the store's own rule.
    pub fn supersede_author(&self) -> &str {
        &self.store.local_actor_id().0
    }
}
