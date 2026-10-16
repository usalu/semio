//! 🎛️ `shell-prefs` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl PrefsState {
    //#region 🏷️LabelResolution
    /// 🌐️ Active `Locale` derived from `locale_id`. This engine is not manifest-aware the way the
    /// React renderer is (no per-window locale/terminology context is threaded through render
    /// calls), so every `LocalizedLabel` in this file resolves against this shell-wide value.
    pub fn active_locale(&self) -> Locale {
        Locale::parse(&self.locale_id).expect("shell locale has explicit admitted authority")
    }

    /// 🗣️ Active `Terminology` derived from `terminology_id`. See `active_locale` doc.
    pub fn active_terminology(&self) -> Terminology {
        Terminology::parse(&self.terminology_id).expect("shell terminology has explicit admitted authority")
    }

    //#endregion 🏷️LabelResolution
    //#region 🏠️🧳️PluginHostConfig
    /// 🧭️ A shell surface alias resolves to its owner's concrete guest window address.
    pub fn session_action_invocation(&self, session: &ActiveSession, action: &ActionDescriptor, view: &ViewModel) -> Result<semio_framework::manifest::ActionInvocation, String> {
        let supplied_window = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str);
        let requested_window = if supplied_window == Some("spawned") { session.view_state.window_id.as_deref() } else { supplied_window };
        let panel_leaves: Vec<&str> = flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().map(|tab| tab.id()).collect();
        let window_instance_id = action_window_instance_id(requested_window, &panel_leaves, session.view_state.window_id.as_deref(), view.focused_window_id.as_deref(), &session.app.window_kinds.first().id);
        let window_kind_id = view.window_instances.iter().find(|instance| instance.id == window_instance_id).map(|instance| instance.window_kind_id.clone()).or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_instance_id).map(|kind| kind.id.clone())).ok_or_else(|| format!("action window instance {window_instance_id} has no declared kind"))?;
        let panel_origin = requested_window.is_some_and(|id| panel_leaves.contains(&id));
        let mut arguments: std::collections::BTreeMap<String, DslValue> = action.args.as_ref().and_then(semio_framework_value::DslValue::as_object).map(|entries| entries.iter().filter(|(key, _)| !(panel_origin && key == "windowId")).cloned().collect()).unwrap_or_default();
        if supplied_window == Some("spawned") {
            arguments.insert("windowId".into(), semio_framework_value::DslValue::from(serde_json::json!(window_instance_id)));
        }
        Ok(semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), mode_id: session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone()), window_kind_id, window_instance_id, action_id: action.action.clone() },
            arguments,
        })
    }

    //#endregion 🏠️🧳️PluginHostConfig
    /// 🌌️ TEMP(Wave 3): replace with workflow_palette() once the shell palette wiring lands. Reads
    /// `program.manifest.apps` directly (one `SpaceProgramEntry` per app) — `PluginManifest.workflows`/
    /// `WorkflowDefinition` were deleted in Wave 0 (WP-0.1); the real palette derivation moves to
    /// `semio-framework-os`'s `registry::workflow_palette()` (`AppIo`-driven) once the browser shell wires
    /// it in. `yields` is empty here (no registry lookup at this layer) — matches every other Wave-1
    /// `WorkflowNode.yields` derivation until Wave 2 populates apps' declared output ports.
    pub fn build_space_workflows(&self, plugins: &PluginState) -> Vec<SpaceProgramEntry> {
        plugins.plugins
            .iter()
            .flat_map(|program| {
                program.manifest.apps.iter().map(|app| SpaceProgramEntry {
                    plugin_id: program.plugin_id.clone(),
                    workflow_step_id: app.id.clone(),
                    app_id: app.id.clone(),
                    label: app.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                    breadcrumb: app.breadcrumb.clone(),
                    yields: String::new(),
                })
            })
            .collect()
    }

    /// 🧯 Records one plugin's boot activation fault and refreshes the shell's status line. The shell
    /// stays alive: a trapped guest is that plugin's failure, not the renderer's.
    pub fn record_plugin_fault(&mut self, plugin_id: &str, app_id: &str, detail: String, chrome: &mut ChromeState, plugins: &mut PluginState) {
        debug_log(&format!("[TRACE] wgpu shell boot: plugin {plugin_id} app {app_id} faulted: {detail}"));
        plugins.plugin_faults.push(ShellPluginFault { plugin_id: plugin_id.to_string(), app_id: app_id.to_string(), detail,#[cfg(not(target_arch="wasm32"))] frame:None,#[cfg(not(target_arch="wasm32"))] kernel:None,#[cfg(not(target_arch="wasm32"))] actor_call:None });
        chrome.error = self.fault_status(plugins);
    }

    /// 🧯 Records one surface's render fault and refreshes the shell's status line, WITHOUT failing the
    /// refresh. A surface repeats across refreshes, so the same surface never accumulates two entries.
    ///
    /// 📣️ `eprintln!` is a no-op in a `wasm32-unknown-unknown` Worker — a fault recorded there
    /// left no trace at all, which is why a blank body read as "nothing happened". `debug_log` is
    /// this type's ONE trace sink for exactly that reason; no diagnostic in this file may use
    /// `eprintln!` except `debug_log`'s own native arm.
    pub fn record_surface_fault(&mut self, surface_id: &str, body_key: &str, refusal: semio_framework_os_renderer_wgpu_program_bridge::ProgramFault, chrome: &mut ChromeState, plugins: &mut PluginState) {
        #[cfg(not(target_arch = "wasm32"))]
        let (frame,kernel,actor_call)=(refusal.frame,refusal.kernel,refusal.actor_call);
        #[cfg(not(target_arch = "wasm32"))]
        let detail = if frame.is_some()||kernel.is_some()||actor_call.is_some() { LocalizedLabel::native("The input could not be delivered.", "Die Eingabe konnte nicht zugestellt werden.").resolve(self.active_terminology(), self.active_locale()).to_string() } else { refusal.text };
        #[cfg(target_arch = "wasm32")]
        let detail = refusal.text;
        debug_log(&format!("[TRACE] wgpu-shell surface fault surface={surface_id} body={body_key} detail={detail}"));
        if let Some(existing) = plugins.surface_faults.iter_mut().find(|fault| fault.surface_id == surface_id) {
            existing.body_key = body_key.to_string();
            existing.detail = detail;
            #[cfg(not(target_arch = "wasm32"))]
            { existing.frame = frame;existing.kernel=kernel;existing.actor_call=actor_call; }
        } else {
            plugins.surface_faults.push(ShellSurfaceFault { surface_id: surface_id.to_string(), body_key: body_key.to_string(), detail, #[cfg(not(target_arch = "wasm32"))] frame,#[cfg(not(target_arch="wasm32"))] kernel,#[cfg(not(target_arch="wasm32"))] actor_call });
        }
        chrome.error = self.fault_status(plugins);
    }

    /// 🗣️ The per-plugin boot status, in English or German with no default language — `None` while
    /// every plugin the boot asked for activated.
    pub fn plugin_fault_status(&self, plugins: &PluginState) -> Option<String> {
        if plugins.plugin_faults.is_empty() {
            return None;
        }
        let is_de = self.locale_id == "de";
        let label = shell_chrome_string("plugin.faulted", is_de);
        Some(plugins.plugin_faults.iter().map(|fault| format!("{label}: {} ({}) — {}", fault.plugin_id, fault.app_id, fault.detail)).collect::<Vec<_>>().join(" · "))
    }

    /// 🗣️ The per-surface render status, in English or German with no default language — `None` while
    /// every surface this refresh asked for published.
    pub fn surface_fault_status(&self, plugins: &PluginState) -> Option<String> {
        if plugins.surface_faults.is_empty() {
            return None;
        }
        let is_de = self.locale_id == "de";
        let label = shell_chrome_string("surface.faulted", is_de);
        Some(plugins.surface_faults.iter().map(|fault| format!("{label}: {} ({}) — {}", fault.surface_id, fault.body_key, fault.detail)).collect::<Vec<_>>().join(" · "))
    }

    /// 🧯 Every live per-plugin and per-surface fault as one status line — the shell's typed fault card.
    pub fn fault_status(&self, plugins: &PluginState) -> Option<String> {
        let parts: Vec<String> = [self.plugin_fault_status(plugins), self.surface_fault_status(plugins)].into_iter().flatten().collect();
        (!parts.is_empty()).then(|| parts.join(" · "))
    }

    /// 🚪️ Opens one plugin's app for the boot session, isolating an activation fault as a per-plugin
    /// status. Returns `None` when the plugin is not loaded or its actor refused to open.
    pub async fn open_boot_instance(&mut self, plugin_id: &str, app_id: &str, chrome: &mut ChromeState, plugins: &mut PluginState) -> Option<u32> {
        let Some(program) = plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
            self.record_plugin_fault(plugin_id, app_id, "plugin is not loaded".to_string(), chrome, plugins);
            return None;
        };
        match program.create_app(app_id).await {
            Ok(instance_id) => Some(instance_id),
            Err(refusal) => {
                let detail = self.note_refused_open(refusal, chrome);
                self.record_plugin_fault(plugin_id, app_id, detail, chrome, plugins);
                None
            }
        }
    }

    /// 🧯 Replaces the per-surface fault set FOR THE SURFACES THIS PASS VISITED, so a surface that
    /// recovered stops showing a card and one that faulted keeps showing the same typed card. The
    /// refresh itself always succeeds: an unreadable surface is that surface's failure, never the
    /// renderer's — see [`ShellSurfaceFault`].
    ///
    /// ⚠️ `visited`, not `clear()`: a scoped pass re-renders only what its `UiDirtyScope` named, and
    /// clearing the whole set would silently dismiss the fault card of every surface it never looked
    /// at — a broken body that repairs itself the moment an unrelated window settles.
    pub fn settle_surface_faults(&mut self, faults: Vec<(String, String, semio_framework_os_renderer_wgpu_program_bridge::ProgramFault)>, visited: &[String], chrome: &mut ChromeState, plugins: &mut PluginState) {
        plugins.surface_faults.retain(|fault| !visited.contains(&fault.surface_id));
        for (surface_id, body_key, detail) in faults {
            self.record_surface_fault(&surface_id, &body_key, detail, chrome, plugins);
        }
        chrome.error = self.fault_status(plugins);
    }
}

//#endregion ShellLifecycle

