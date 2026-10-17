//! 🪼️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_tool_run::ToolRunSettingsValues;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use std::future::Future;
use crate::stores_components::InstanceStores;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::*;
use semio_framework_tool_run::{tool_run_pointer_value, ToolRunSettingsReads};
use semio_framework_os_kernel::{MemberFactory, SpaceMember};

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🎚️ What the run's declared settings reads name right now. A declared window kind that is the starting
    /// window's reads only that window's config, the one its jobs receive; any other declared kind reads every
    /// partition of that kind.
    pub fn tool_run_settings_values(&self, reads: &ToolRunSettingsReads, window: Option<&(String, String)>) -> ToolRunSettingsValues {
        let config = if reads.config.is_empty() {
            Vec::new()
        } else {
            let document = semio_framework_value::ToValue::to_value(self.config_store.snapshot_owner().as_ref());
            reads.config.iter().map(|pointer| tool_run_pointer_value(&document, pointer).cloned()).collect()
        };
        let window_config = reads
            .window_config
            .iter()
            .flat_map(|(window_kind_id, pointers)| {
                let started = window.filter(|(_, kind)| kind == window_kind_id).map(|(window_id, _)| window_id.as_str());
                self.window_config_store.pointer_values(window_kind_id, pointers).into_iter().filter(move |(window_id, _)| started.is_none_or(|started| started == window_id)).map(move |(window_id, values)| (window_kind_id.clone(), window_id, values))
            })
            .collect();
        ToolRunSettingsValues { config, window_config }
    }
}
