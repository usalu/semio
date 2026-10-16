//! 🧲️ `shell-plugins` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl PluginState {
    /// 🛍️ Installs the cached catalogue on every node-graph surface whose engine host does not
    /// already carry it — the wgpu twin of React's `AppCatalogueContext` provider.
    pub fn publish_app_catalogue(&self, surfaces: &SurfaceState) {
        if self.app_catalogue_json.is_empty() {
            return;
        }
        for (host_id, _) in surfaces.node_graph_states.iter() {
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_set_catalogue_json(host_id, &self.app_catalogue_json);
        }
    }

    /// 👁️✏️ `(dialect, role, pinned value, options)` per opening pairing the loaded manifests declare —
    /// React's `defaultAppsRows` memo over `AppRouter.entriesFor` plus the persisted `OpeningPreferences`.
    pub fn default_app_rows(&self) -> Vec<(String, String, String, Vec<(String, String)>)> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut dialects: Vec<String> = Vec::new();
        for entry in &self.plugins {
            for app in &entry.manifest.apps {
                let dialect = app.dialect.to_coordinate();
                if !dialects.contains(&dialect) {
                    dialects.push(dialect);
                }
            }
        }
        dialects.sort();
        let mut rows = Vec::new();
        for dialect in dialects {
            for role in ["viewer", "editor"] {
                let options: Vec<(String, String)> = self
                    .plugins
                    .iter()
                    .flat_map(|entry| {
                        entry.manifest.apps.iter().filter(|app| app.dialect.to_coordinate() == dialect && app.role.as_str() == role).map(|app| (format!("{} {}", entry.plugin_id, app.id), format!("{} · {}", entry.manifest.label, app.id)))
                    })
                    .collect();
                if options.is_empty() {
                    continue;
                }
                let pinned = self.default_app_pins.get(&format!("{dialect}/{role}")).cloned().unwrap_or_else(|| DEFAULT_APP_NONE_VALUE.to_string());
                rows.push((dialect.clone(), role.to_string(), pinned, options));
            }
        }
        rows
    }

    /// 🛍️ Every plugin id the `framework.marketplace` leaf lists, in its row order: the plugins this shell HOLDS (`loaded`,
    /// extensions excluded — they nest under their host) and every plugin id the generated activation catalogue claims
    /// (`available`) — the same table [`ShellState::install_plugin`]'s lazy install reads, which is what makes `Install` a real
    /// verb rather than a decoration. Sorted and unique.
    pub fn marketplace_roster(&self) -> Vec<String> {
        let extension_ids: Vec<&str> = self.extensions.iter().map(|entry| entry.record.extension_id.as_str()).collect();
        let mut ids: Vec<String> = self.plugins.iter().filter(|entry| !extension_ids.contains(&entry.plugin_id.as_str())).map(|entry| entry.plugin_id.clone()).collect();
        for (_, plugin_id) in semio_framework_os_renderer_wgpu_program_bridge::plugin_artifact_kind_activations().iter() {
            if !ids.iter().any(|id| id.as_str() == *plugin_id) {
                ids.push((*plugin_id).to_string());
            }
        }
        ids.sort();
        ids.dedup();
        ids
    }
}

//#endregion ShellLifecycle

