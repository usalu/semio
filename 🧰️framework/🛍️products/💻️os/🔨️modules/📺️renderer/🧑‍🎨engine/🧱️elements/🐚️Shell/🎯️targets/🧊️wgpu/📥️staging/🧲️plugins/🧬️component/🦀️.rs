//! 🧲️ `shell-plugins` component structs: `PluginState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🧲️ `PluginState`: plugins, extensions, the host capability, panel-run ledger, faults and the app catalogue.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `plugins`.
pub struct PluginState {
    pub plugins: Vec<ProgramBridgeEntry>,
    pub extensions: Vec<ShellExtensionProjection>,
    pub plugin_filter: String,
    pub host_capability: Option<&'static PluginHostConfig>,
    pub space_mode: bool,
    pub tool_run_panel_runs: std::collections::BTreeSet<u64>,
    /// 🧯 One entry per plugin whose boot activation faulted — the per-plugin status that replaces
    /// the old "any `create_app` failure aborts the whole shell boot" rule. A trapped actor belongs
    /// to exactly one plugin: every OTHER plugin, and the requested app itself, must still boot.
    pub plugin_faults: Vec<ShellPluginFault>,
    /// 🧯 One entry per surface whose document could not be rendered this refresh. The shell keeps
    /// painting every OTHER surface and shows this one as a typed fault card, instead of failing the
    /// boot — see [`ShellSurfaceFault`].
    pub surface_faults: Vec<ShellSurfaceFault>,
    /// 🛍️ The active app instance's APP-STATIC operator/palette catalogue, fetched once per instance
    /// from the reserved `framework.section.catalogue` retained surface and pushed into every live
    /// node-graph engine host. Mirrors the React shell's `appCatalogue`
    /// (`🐚️Shell/🟦️.tsx`) and its `uiRefreshWantsCatalogue` "full scope only" rule: the payload is
    /// app-static, so an instance fetches it exactly once and never on a scene change.
    /// Never carried on a `NodeGraphScene` — see `ArtifactApp::app_catalogue_json`.
    pub app_catalogue_json: String,
    /// 🛍️ Which app instance [`Self::app_catalogue_json`] was fetched for — the cache key that makes
    /// the fetch once-per-instance rather than once-per-refresh.
    pub app_catalogue_instance: Option<u32>,
    pub active_example_id: Option<String>,
    /// 📥️ Extension invocations the guest asked for, parked until a flush that owns `&mut self`
    /// can run them AND fold their answers' own effects back in — see `flush_deferred_actions`.
    #[cfg(target_arch = "wasm32")]
    pub pending_extension_invocations: Vec<PendingExtensionInvocation>,
    /// 👁️✏️ Pinned default app per `"<dialectCoordinate>/<role>"` — the projection React reads
    /// off `OpeningPreferences`, each value `"<pluginId> <appId>"` or `"none"`.
    pub default_app_pins: HashMap<String, String>,
    /// 🎬️ The ONE lazy plugin install this shell runs at a time, retained for exactly as long as it
    /// takes — the wgpu twin of React's `pluginOpInFlightRef` + `SET_PLUGIN_STATUS` pair. Its phase is
    /// what chrome reads; its `cancel` is what [`Self::cancel_plugin_install`] fires. Never persisted,
    /// never a document command.
    pub plugin_install: Option<ShellPluginInstall>,
}
