//! 🔁️ Host lifecycle of the plugin host: hot-swap events, loaded programs, the plugin host with its registry, quarantine and action kernel, the hot-swap rollback and the manifest validator. See [`super`].

use super::*;

// #region host
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramHotSwapEvent {
    pub plugin_id: String,
    pub version: String,
    pub added_apps: Vec<String>,
    pub removed_apps: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedProgram {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    pub artifact_uri: String,
}

pub struct PluginHost {
    pub(super) registry: PluginRegistry,
    instances: HashMap<u32, OsInstanceState>,
    next_instance_id: u32,
    pub(super) programs: HashMap<String, LoadedProgram>,
    //#region 🔖️Quarantine
    /// 🚧️ Plugin ids currently held out of rotation for `recovery_ui`. No producer in this crate
    /// sets this today (nothing here is wired to the actor kernel's `FailureStage::Quarantined` —
    /// see `📓️terra-C1-report.md`); `load_plugin`/`hot_swap_plugin` clear it on every successful
    /// or rolled-back load, matching a manual operator restart.
    pub(super) quarantined: HashSet<String>,
    //#endregion 🔖️Quarantine
    //#region 🔖️ExtensionInstall
    /// 🧩️ Installed `.sxt` extension descriptors, keyed by `extension_id` — see
    /// `install_extension_package`. `.sxt` install is native-host tooling (mirrors
    /// `store::extension`'s own `not(all(target_arch = "wasm32", target_env = "p2"))` gate — a
    /// guest component never installs an extension into itself).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(super) installed_extensions: HashMap<String, InstalledExtension>,
    //#endregion 🔖️ExtensionInstall
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            programs: HashMap::new(),
            quarantined: HashSet::new(),
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            installed_extensions: HashMap::new(),
        }
    }

    pub fn is_quarantined(&self, plugin_id: &str) -> bool {
        self.quarantined.contains(plugin_id)
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn load_plugin(&mut self, program: LoadedProgram) -> ProgramHotSwapEvent {
        let plugin_id = program.plugin_id.clone();
        let version = program.manifest.version.clone();
        let previous_apps: Vec<String> = self.programs.get(&plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        let next_apps: Vec<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();
        for app in &program.manifest.apps {
            self.registry.register_app(app.clone());
            // 🪐️ B1 ripple resolved: seeds `registry::APP_REGISTRATIONS` (`workflow_palette()`'s
            // backing store) at native/test load time — the wasm-hosted browser shell has no direct
            // call path here, so it pushes the equivalent over the wire instead (see the space app's
            // `SpaceCommand::SetAppRegistrations`).
            crate::registry::register_app_io(&plugin_id, app);
        }
        crate::registry::register_artifact_descriptors(&program.manifest);
        self.programs.insert(plugin_id.clone(), program);
        self.quarantined.remove(&plugin_id);
        ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
    }

    pub fn hot_swap_plugin(&mut self, program: LoadedProgram) -> ProgramHotSwapEvent {
        let plugin_id = program.plugin_id.clone();
        let rollback = HotSwapRollback { previous_plugin: self.programs.get(&plugin_id).cloned(), instance_generations: self.instances.iter().map(|(id, state)| (*id, state.generation)).collect() };

        if let Err(error) = validate_plugin_manifest(&program) {
            self.quarantined.remove(&plugin_id);
            return rollback.emit_failure(plugin_id, error);
        }

        let previous_apps: Vec<String> = rollback.previous_plugin.as_ref().map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        let next_apps: Vec<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();

        if let Err(error) = self.validate_swap_apps(&program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_instances(&plugin_id, &program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_app_retention(&program, rollback.previous_plugin.as_ref()) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_window_kinds(&program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }

        let controller_rebindings = self.plan_controller_rebindings(&plugin_id, &program);
        let version = program.manifest.version.clone();
        for app in &program.manifest.apps {
            self.registry.register_app(app.clone());
            // 🪐️ See the `load_plugin` sibling loop's comment above.
            crate::registry::register_app_io(&plugin_id, app);
        }
        crate::registry::register_artifact_descriptors(&program.manifest);
        self.programs.insert(plugin_id.clone(), program);
        for (instance_id, controller_id) in controller_rebindings {
            if let Some(instance) = self.instances.get_mut(&instance_id) {
                instance.controller_id = controller_id;
            }
        }
        for instance in self.instances.values_mut() {
            instance.generation += 1;
        }
        self.quarantined.remove(&plugin_id);
        ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.registry.apps()
    }

    pub fn contributions(&self) -> Vec<ProgramContributionEntry> {
        let mut entries = Vec::new();
        for loaded in self.programs.values() {
            for topic_contribution in &loaded.manifest.topic_contributions {
                entries.push(ProgramContributionEntry { plugin_id: loaded.plugin_id.clone(), topic_contribution: Some(topic_contribution.clone()) });
            }
        }
        entries
    }

    pub fn contributions_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.contributions())
    }

    pub fn create_instance(&mut self, app_id: &str, document_json: String, view_state: ViewModel) -> Option<u32> {
        let app = self.registry.find_app(app_id)?;
        let id = self.next_instance_id;
        self.next_instance_id += 1;
        self.instances.insert(id, OsInstanceState { id, app_id: app.id.clone(), controller_id: app.controller_id.clone(), document_json, view_state, generation: 0 });
        Some(id)
    }

    pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
        self.instances.get(&instance_id)
    }

    pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
        self.instances.get_mut(&instance_id)
    }

    //#region 🔖️ActionKernel

    /// 🩺️ Delegates to `ui_wgpu::wgpu::ui_recovery_panel`'s `🔖️StatusBuilders` builder — this host
    /// has no locale on hand at this call site (no `ViewModel` threaded into `recovery_ui`), so
    /// `is_de` is pinned to `false` (English) until a locale source is plumbed through.
    pub fn recovery_ui(&self, plugin_id: &str) -> UiNode {
        ui_recovery_panel(plugin_id, self.is_quarantined(plugin_id), false)
    }
    //#endregion 🔖️ActionKernel

    pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewModel) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.view_state = view_state;
            instance.generation += 1;
        }
    }

    pub fn render_body(&self, instance_id: u32, body_key: &str, ui: UiNode) -> UiNode {
        let _ = (instance_id, body_key);
        ui
    }

    fn hot_swap_failed(&mut self, plugin_id: String, error: String, rollback: HotSwapRollback) -> ProgramHotSwapEvent {
        rollback.restore(self);
        self.quarantined.remove(&plugin_id);
        rollback.emit_failure(plugin_id, error)
    }

    fn validate_swap_apps(&self, program: &LoadedProgram) -> Result<(), String> {
        for app in &program.manifest.apps {
            if app.id.trim().is_empty() {
                return Err("app id must not be empty".into());
            }
            if app.controller_id.trim().is_empty() {
                return Err(format!("app {} controller_id must not be empty", app.id));
            }
        }
        Ok(())
    }

    fn validate_swap_instances(&self, plugin_id: &str, program: &LoadedProgram) -> Result<(), String> {
        let next_app_ids: HashSet<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();
        let previous_app_ids: HashSet<String> = self.programs.get(plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        for instance in self.instances.values() {
            if !previous_app_ids.contains(&instance.app_id) {
                continue;
            }
            if !next_app_ids.contains(&instance.app_id) {
                return Err(format!("instance {} references removed app {}", instance.id, instance.app_id));
            }
        }
        Ok(())
    }

    /// 🔢️ Same-version hot-swaps must not silently drop apps — a version bump is required to
    /// shrink the app set, so a client relying on document/instance continuity can detect the change.
    fn validate_swap_app_retention(&self, program: &LoadedProgram, previous: Option<&LoadedProgram>) -> Result<(), String> {
        if let Some(previous) = previous {
            if previous.manifest.version == program.manifest.version && previous.manifest.apps.len() > program.manifest.apps.len() {
                return Err("cannot hot-swap to fewer apps within the same version".into());
            }
        }
        Ok(())
    }

    fn validate_swap_window_kinds(&self, program: &LoadedProgram) -> Result<(), String> {
        for app in &program.manifest.apps {
            if app.window_kinds.is_empty() {
                return Err(format!("app {} must declare at least one window kind", app.id));
            }
            for window_kind in &app.window_kinds {
                if window_kind.body_key.trim().is_empty() {
                    return Err(format!("app {} window kind {} body_key must not be empty", app.id, window_kind.id));
                }
            }
        }
        Ok(())
    }

    fn plan_controller_rebindings(&self, plugin_id: &str, program: &LoadedProgram) -> Vec<(u32, String)> {
        let apps_by_id: HashMap<&str, &AppDefinition> = program.manifest.apps.iter().map(|app| (app.id.as_str(), app)).collect();
        let previous_app_ids: HashSet<String> = self.programs.get(plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        self.instances.values().filter(|instance| previous_app_ids.contains(&instance.app_id)).filter_map(|instance| apps_by_id.get(instance.app_id.as_str()).map(|app| (instance.id, app.controller_id.clone()))).collect()
    }
}

struct HotSwapRollback {
    previous_plugin: Option<LoadedProgram>,
    instance_generations: HashMap<u32, u64>,
}

impl HotSwapRollback {
    fn emit_failure(self, plugin_id: String, _error: String) -> ProgramHotSwapEvent {
        let version = self.previous_plugin.as_ref().map(|previous| previous.manifest.version.clone()).unwrap_or_default();
        ProgramHotSwapEvent { plugin_id, version, added_apps: vec![], removed_apps: vec![] }
    }

    fn restore(&self, host: &mut PluginHost) {
        if let Some(previous) = &self.previous_plugin {
            for app in &previous.manifest.apps {
                host.registry.register_app(app.clone());
            }
            // 🚧️ B1 ripple: see `PluginHost::load_plugin`'s sibling loop comment.
            host.programs.insert(previous.plugin_id.clone(), previous.clone());
        }
        for (instance_id, generation) in &self.instance_generations {
            if let Some(instance) = host.instances.get_mut(instance_id) {
                instance.generation = *generation;
            }
        }
    }
}

fn validate_plugin_manifest(program: &LoadedProgram) -> Result<(), String> {
    if program.plugin_id.trim().is_empty() {
        return Err("plugin_id must not be empty".into());
    }
    if program.manifest.plugin_id.trim().is_empty() {
        return Err("manifest.plugin_id must not be empty".into());
    }
    if program.manifest.version.trim().is_empty() {
        return Err("manifest.version must not be empty".into());
    }
    if program.plugin_id != program.manifest.plugin_id {
        return Err("plugin_id must match manifest.plugin_id".into());
    }
    Ok(())
}
// #endregion host
