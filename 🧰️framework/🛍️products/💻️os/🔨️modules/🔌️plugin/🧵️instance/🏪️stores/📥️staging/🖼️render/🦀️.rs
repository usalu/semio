//! 🪠️ Staged slice of the `render` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use crate::stores_components::{InstanceStores, LocalInteraction};
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> LocalInteraction<A> {
    /// 🕹️ Combines the persisted-local `interaction_store` snapshot with the ephemeral
    /// `interaction_hover` map into one `InteractionState` — the "app-side source" a host's presence
    /// heartbeat reads for THIS app instance before calling `assemble_presence_interaction`
    /// (`🏪️store/🔄️sync/🦀️.rs`, wave 2a) with it plus this app's declared hover/selection
    /// specs (from `AppDefinition.interactions`, already available wherever a heartbeat is built).
    pub async fn interaction_state(&self, stores: &InstanceStores<A>) -> protocol::InteractionState {
        let mut state = self.interaction_selection_snapshot(stores);
        state.hover = self.interaction_hover.clone();
        state
    }

    /// 🕹️ The persisted-local selection half, with a store read failure NAMED instead of silently
    /// collapsing into an empty `InteractionState`. Every reader treats "no selection" and "the
    /// selection could not be read" identically, so a swallowed read reaches the user as a panel that
    /// renders the empty-document summary on a live pick — measured in the browser on
    /// `framework.panel.inspection` (ticket 26/09/02/PUZZLE-3D-END-TO-END, wave B6: 25 full refreshes
    /// over 73 s kept the pre-pick body hash while the same turn's leftover `InteractionView` carried
    /// the picked id).
    pub fn interaction_selection_snapshot(&self, stores: &InstanceStores<A>) -> protocol::InteractionState {
        let mut state = match stores.interaction_store.snapshot() {
            Ok(state) => state,
            Err(error) => {
                semio_framework_plugin_reactor_app::debug_runtime_line(format_args!("[TRACE] interaction-store snapshot unavailable, selection read as empty: {error}"));
                protocol::InteractionState::default()
            }
        };
        if let Some(overlay) = &self.interaction_leftover_selection {
            for (domain, selection) in &overlay.selection {
                let empty = state.selection.get(domain).map(|current| current.ids.is_empty()).unwrap_or(true);
                if empty && !selection.ids.is_empty() {
                    state.selection.insert(domain.clone(), selection.clone());
                    if let Some(mode) = overlay.active_mode.get(domain) {
                        state.active_mode.insert(domain.clone(), *mode);
                    }
                    if let Some(granularity) = overlay.active_granularity.get(domain) {
                        state.active_granularity.insert(domain.clone(), granularity.clone());
                    }
                }
            }
            let leftover_ids: Vec<String> = overlay.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
            let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
            if vortex_empty && !leftover_ids.is_empty() {
                let granularity = overlay
                    .active_granularity
                    .get("vortex")
                    .cloned()
                    .or_else(|| overlay.selection.get("vortex").map(|selection| selection.granularity.clone()))
                    .filter(|granularity| !granularity.is_empty())
                    .unwrap_or_else(|| "object".to_string());
                state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: leftover_ids, anchor_id: None });
                if let Some(mode) = overlay.active_mode.get("vortex") {
                    state.active_mode.insert("vortex".to_string(), *mode);
                }
                state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
            }
        }
        let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
        if vortex_empty && !self.interaction_leftover_ids.is_empty() {
            let granularity = state.active_granularity.get("vortex").cloned().filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        state
    }
}
