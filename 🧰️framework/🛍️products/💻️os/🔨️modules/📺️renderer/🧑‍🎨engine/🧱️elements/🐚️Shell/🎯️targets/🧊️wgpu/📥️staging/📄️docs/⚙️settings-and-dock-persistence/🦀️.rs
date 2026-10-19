//! 📄️ `shell-docs` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl DocumentState {
    pub fn retain_document_for_close(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        self.closing_documents.try_admit(document)
    }

    /// 🔑️ Lends the chrome walk the EXACT window-body owner for one step and takes it back with
    /// [`Self::restore_window_document`]. A retained document read used to be reached through
    /// `UiDocumentLease::try_alias`, which mints a new arena alias: the arena admits
    /// [`UI_DOCUMENT_LEASE_ALIASES`] of them per slot and only `close_read_step_with_grant` gives one
    /// back, so a per-frame-step read that simply dropped its alias exhausted the slot after seven
    /// steps and every later read answered `AliasCapacity`. The chrome step reads that as "this
    /// window has no document", advances past the body, and the surface's page ingress freezes
    /// forever — a booted shell with an empty `UI_ENGINE` tree and a black canvas
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`). Moving the one
    /// owner instead of aliasing it needs no credit at all, which is why it cannot run out.
    pub fn take_window_document(&mut self, window_id: &str) -> Option<UiDocumentLease> {
        if window_id == "spawned" {
            self.spawned_ui.take()
        } else {
            self.window_ui.remove(window_id)
        }
    }

    pub fn restore_window_document(&mut self, window_id: &str, document: UiDocumentLease) {
        if window_id == "spawned" {
            self.spawned_ui = Some(document);
        } else {
            self.window_ui.insert(window_id.to_string(), document);
        }
    }

    /// ♻️ Drives the retained-document retirement registry AND the `UiValue` arena it feeds to idle.
    ///
    /// ⚖️ One page per FRAME (`render_chrome_step`'s `CloseDocument` phase) is enough while the shell
    /// only refreshes on interaction. A converging guest chain is different: `flush_deferred_actions`
    /// runs many refreshes back to back inside ONE flush and no frame happens in between, so the
    /// registry only grows — and the `UiValue` collection slots a retired document's records hold are
    /// a FIXED process-wide arena. At `ui_value_headroom().collections == 0` every later `UiValue`
    /// deserialization fails as `data did not match any variant of untagged enum UiValue` and the
    /// shell stops painting entirely (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, measured on 6118 as
    /// `headroom collections=0`). A refresh therefore returns the previous refresh's arenas before it
    /// asks the guest for the next document.
    pub fn drain_retained_document_arenas(&mut self) {
        for _ in 0..SHELL_DOCUMENT_RETIREMENT_STEPS {
            let consumed = self.close_document_one();
            if !consumed && self.closing_documents.terminal_is_empty() {
                break;
            }
        }
        for _ in 0..SHELL_DOCUMENT_RETIREMENT_STEPS {
            if ui_contract::close_ui_value_page_one() {
                break;
            }
        }
    }

    /// ♻️ Retires ONE surface's previous document to terminal before its replacement is asked for.
    ///
    /// ⚖️ A refresh used to move EVERY live document into the retirement registry and only then ask
    /// the guest for the replacements, so the process-wide resident aggregate had to carry two full
    /// sets at once — and `UiResidentPermit::try_reserve` is a fixed aggregate, so the second set was
    /// refused `Capacity` on every surface and the shell kept the stale documents forever
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.2).
    /// One root at a time is the whole fix: the peak is the live surface count, never twice it.
    pub fn retire_one_surface_document(&mut self, document: Option<UiDocumentLease>) -> Result<(), String> {
        let Some(document) = document else { return Ok(()) };
        if self.retain_document_for_close(document).is_err() {
            return Err("shell: document retirement registry refused the exact prior owner".to_string());
        }
        self.drain_retained_document_arenas();
        Ok(())
    }

    /// 🧹 Retires every window (or panel) document whose surface the active layout no longer names.
    pub fn retire_documents_outside(&mut self, retained: &[String], windows: bool, chrome: &mut ChromeState) -> Result<(), String> {
        let stale: Vec<String> = if windows { self.window_ui.keys().filter(|id| !retained.contains(id)).cloned().collect() } else { self.panel_documents.keys().filter(|id| !retained.contains(id)).cloned().collect() };
        for id in stale {
            if !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(&id) {
                return Err(format!("shell: retained UI close registry refused surface '{id}'"));
            }
            chrome.chrome_build.clear_content_focus(&id);
            let document = if windows { self.window_ui.remove(&id) } else { self.panel_documents.remove(&id) };
            self.retire_one_surface_document(document)?;
        }
        Ok(())
    }

    pub fn close_document_one(&mut self) -> bool {
        if self.closing_documents.terminal_is_empty() {
            return !ui_contract::close_ui_document_page_one();
        }
        self.closing_documents.close_one()
    }

    /// ⚙️ The `framework.settings.general` leaf's body — the wgpu twin of React's
    /// `buildSettingsGeneralTree` (`📌️ChromePanels/🟦️.tsx`): the app-identity section, then the
    /// appearance / layout / driver / language / terminology selects and the Reset Dock button, each
    /// row's id and each option's value byte-identical to React's. Rows whose axis is locked by the
    /// boot descriptor are omitted entirely (React's own `locks.*` render gates), never disabled.
    ///
    /// ⚖️ React's `framework.settings.mergePolicy` row (`📌️ChromePanels/🟦️.tsx:467-486`) — the one
    /// General-leaf control with no wgpu twin (audit W14 §B2). Option VALUES are React's
    /// `MERGE_POLICY_OPTIONS` verbatim, which are `protocol::MergePolicy`'s own variant names.
    pub fn build_settings_general_ui(&self, chrome: &ChromeState, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let locks = semio_framework_os_renderer_wgpu_host_base::boot_locks();
        let mut sections = Vec::new();
        if let Some(session) = self.session.as_ref() {
            let app_label = session.app.label.resolve(prefs.active_terminology(), prefs.active_locale()).to_string();
            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.settings.app".into(),
                label: Some(Label::data(shell_chrome_string("settings.tab.app", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![
                    UiTreeItemNode::base("framework.settings.app.label", Label::data(format!("{}: {app_label}", shell_chrome_string("settings.app.name", is_de)))),
                    UiTreeItemNode::base("framework.settings.app.id", Label::data(format!("{}: {}", shell_chrome_string("settings.app.id", is_de), session.app.id))),
                    UiTreeItemNode::base("framework.settings.app.controller", Label::data(format!("{}: {}", shell_chrome_string("settings.app.controller", is_de), session.app.controller_id))),
                    UiTreeItemNode::base("framework.settings.app.plugin", Label::data(format!("{}: {}", shell_chrome_string("settings.app.plugin", is_de), session.plugin_id))),
                ],
                window: None,
            });
        }
        let mut general = Vec::new();
        if locks.appearance.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.appearance",
                shell_chrome_string("settings.tab.appearance", is_de),
                &prefs.appearance_id,
                vec![
                    ("system".to_string(), shell_chrome_string("settings.appearance.system", is_de).to_string()),
                    ("light".to_string(), shell_chrome_string("settings.appearance.light", is_de).to_string()),
                    ("dark".to_string(), shell_chrome_string("settings.appearance.dark", is_de).to_string()),
                ],
                "setAppearance",
            ));
        }
        general.push(settings_tree_select_item(
            "framework.settings.layout",
            shell_chrome_string("settings.tab.layout", is_de),
            &chrome.chrome_build.preferences.ui_layout,
            vec![("desktop".to_string(), shell_chrome_string("settings.layout.desktop", is_de).to_string()), ("tablet".to_string(), shell_chrome_string("settings.layout.tablet", is_de).to_string())],
            "setLayout",
        ));
        general.push(settings_tree_select_item("framework.settings.driver", shell_chrome_string("settings.tab.driver", is_de), &prefs.driver_id, prefs.driver_rows(chrome), "setDriver"));
        if locks.locale.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.language",
                shell_chrome_string("settings.tab.language", is_de),
                &prefs.locale_id,
                vec![("en".to_string(), shell_chrome_string("settings.language.en", is_de).to_string()), ("de".to_string(), shell_chrome_string("settings.language.de", is_de).to_string())],
                "setLocale",
            ));
        }
        if locks.terminology.is_empty() {
            general.push(settings_tree_select_item(
                "framework.settings.terminology",
                shell_chrome_string("settings.tab.terminology", is_de),
                &prefs.terminology_id,
                self.active_terminologies().into_iter().map(|id| (id.clone(), id)).collect(),
                "setTerminology",
            ));
        }
        general.push(settings_tree_select_item(
            "framework.settings.mergePolicy",
            shell_chrome_string("settings.mergePolicy", is_de),
            &prefs.merge_policy,
            SHELL_MERGE_POLICY_OPTIONS.iter().map(|(value, label_key)| ((*value).to_string(), shell_chrome_string(*label_key, is_de).to_string())).collect(),
            "setMergePolicy",
        ));
        general.push(UiTreeItemNode {
            id: "framework.settings.resetDock.action".into(),
            label: Label::data(shell_chrome_string("settings.resetDock", is_de)),
            control: Some(UiControlNode::Button(UiButtonNode {
                id: Some("framework.settings.resetDock".into()),
                icon_id: IconName::RotateCcw,
                label: Label::data(shell_chrome_string("settings.resetDock", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "runOsCommand".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": "os.resetDock" }) },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        sections.push(UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.settings.general".into(),
            label: Some(Label::data(shell_chrome_string("settings.tab.general", is_de))),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: general,
            window: None,
        });
        sections.push(prefs.build_settings_driver_editor(is_de, chrome));
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    /// 🖥️ The `framework.display.windows` leaf's body — the wgpu twin of React's
    /// `buildDisplayWindowsTree` (`📌️ChromePanels/🟦️.tsx`): one section per window KIND the app
    /// declares, whose rows are that kind's plain leaf plus — for a `world-3d` surface — the whole
    /// `createWorldProjectionTemplates` taxonomy, each row carrying React's own control id.
    ///
    /// 🔃️ React pre-reverses every template level to cancel the bottom-anchored `Tree`'s own
    /// per-level reversal, so what it RENDERS is: the plain kind leaf, then Parallel, then
    /// Perspective. The retained bottom-anchored tree uses that same up-flow layout and authored order.
    ///
    /// 🖱️ React's rows are drag SOURCES (`COMPOSE_WINDOW_TEMPLATE_MIME`) with no click handler. The
    /// retained Tree carries that same payload into the dock's tab/split/root-split drop resolver.
    pub fn build_display_windows_ui(&self, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let Some(session) = self.session.as_ref() else {
            return display_panel_body("framework.display.windows.panel", vec![display_unavailable_section("framework.display.windows.empty", is_de)]);
        };
        let terminology = prefs.active_terminology();
        let locale = prefs.active_locale();
        if session.app.window_kinds.iter().next().is_none() {
            return display_panel_body("framework.display.windows.panel", vec![display_unavailable_section("framework.display.windows.empty", is_de)]);
        }
        let mut sections: Vec<UiTreeSectionNode> = Vec::new();
        for kind in session.app.window_kinds.iter() {
            let label = kind.label.resolve(terminology, locale).to_string();
            let mut items = if kind.surface_kind == semio_framework_ui_declaration::ui::SurfaceKind::World3d {
                world_projection_template_rows(&semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "projection"]), &kind.id, 0, &mut 0)
            } else {
                Vec::new()
            };
            items.push(UiTreeItemNode {
                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "kind"]),
                label: Label::data(label.clone()),
                icon_id: Some(kind.icon_id.clone()),
                draggable: Some(true),
                drag_data: Some(window_template_drag_data(&kind.id, None)),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str()]),
                label: Some(Label::data(label)),
                default_open: Some(false),
                presence: UiPresence::default(),
                items,
                window: None,
            });
        }
        display_panel_body("framework.display.windows.panel", sections)
    }

    pub fn active_terminologies(&self) -> Vec<String> {
        let mut ids = vec!["native".to_string()];
        if let Some(session) = self.session.as_ref() {
            for id in &session.app.terminologies {
                if !ids.contains(id) {
                    ids.push(id.clone());
                }
            }
        }
        ids
    }

    //#region 🧭️PanelAnchorAccessors
    /// 🛠️ One `tool.<id>` leaf per tool the ACTIVE MODE declares, in declared order — React's
    /// `buildToolTabs` over `resolveModeTools(app, activeModeId)`. Label and icon come off the tool's
    /// own manifest entry through the live terminology and locale.
    pub fn tool_panel_tabs(&self, prefs: &PrefsState) -> Vec<DockTabNode> {
        let Some(session) = self.session.as_ref() else { return Vec::new() };
        let terminology = prefs.active_terminology();
        let locale = prefs.active_locale();
        let mode_id = session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        semio_framework::manifest::resolve_mode_tools(&session.app, &mode_id)
            .into_iter()
            .enumerate()
            .map(|(order, tool)| DockTabNode::leaf(format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}{}", tool.id), tool.label.resolve(terminology, locale).to_string(), tool.icon_id.as_str(), order as i32))
            .collect()
    }

    /// 🎛️ One leaf per command category, the wgpu twin of React's `buildCommandCategoryTabs` — the
    /// bottom-middle Command branch's children, whose bodies `build_command_panel_ui` paints.
    pub fn command_category_tabs(&self, chrome: &ChromeState, plugins: &PluginState) -> Vec<DockTabNode> {
        let resolved: Vec<_> = self.resolved_commands(chrome, plugins).into_iter().filter(|entry| entry.definition.in_palette).collect();
        command_categories(&resolved).into_iter().enumerate().map(|(order, (id, label))| DockTabNode::leaf(format!("{FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX}{id}"), label, "wrench", order as i32)).collect()
    }

    /// 🗄️ The app id whose per-app dock layer wins over the shared `os` one — React's `new DockLayoutStore(storage, session?.app.id)`.
    pub fn dock_store_app_id(&self) -> Option<String> {
        self.session.as_ref().map(|session| session.app.id.clone())
    }

    /// 🔌️ Drops a resident plugin — React's `uninstallPlugin`. The session's OWN program is never
    /// dropped (React's `canUninstall`), because a shell may not unload the program it is running.
    pub fn uninstall_plugin(&mut self, plugin_id: &str, plugins: &mut PluginState, directory: &mut DirectoryState) -> bool {
        if self.session.as_ref().is_some_and(|session| session.plugin_id == plugin_id) {
            return false;
        }
        let before = plugins.plugins.len();
        #[cfg(not(target_arch="wasm32"))]
        if directory.inference_port.as_ref().is_some_and(|runner|runner.owner==plugin_id) {directory.cancel_inference_port();}
        plugins.plugins.retain(|entry| entry.plugin_id != plugin_id);
        plugins.plugins.len() != before
    }

    /// ⚔️ Seeds the Conflicts leaf's roster once per session — React's `AppCommand::ReadConflicts`
    /// bootstrap (`🏛️ShellHost/🟦️.tsx`: "otherwise the panel would only ever show conflicts a later
    /// reply happened to carry"). Best-effort and native-only, the same posture — and the same
    /// backend limit — [`ShellState::refresh_history_snapshot`] already takes.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn seed_open_conflicts(&mut self, chrome: &mut ChromeState, plugins: &PluginState, prefs: &mut PrefsState) {
        if prefs.conflicts_seeded {
            return;
        }
        let Some(session) = self.session.as_ref() else { return };
        let Some(plugin) = plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let instance_id = session.instance_id;
        prefs.conflicts_seeded = true;
        match plugin.read_conflicts(instance_id).await {
            Ok(conflicts) => prefs.open_conflicts = conflict_rows(&conflicts),
            Err(error) => { let detail = prefs.note_refused_open(error, chrome); debug_log(&format!("[TRACE] wgpu shell read_conflicts failed: {detail}")); },
        }
    }

    /// ⚔️ Accept/discard one conflict and adopt the roster the guest answers with — React's
    /// `dispatchResolveConflict`, whose reply carries the surviving conflicts so the row leaves
    /// without a second read.
    pub async fn resolve_open_conflict(&mut self, conflict_id: &str, accept: bool, chrome: &mut ChromeState, plugins: &PluginState, prefs: &mut PrefsState) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some((plugin, instance_id)) = self.session.as_ref().and_then(|session| plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().map(|plugin| (plugin, session.instance_id))) {
                match plugin.resolve_conflict(instance_id, conflict_id, accept).await {
                    Ok(conflicts) => prefs.open_conflicts = conflict_rows(&conflicts),
                    Err(error) => { let detail = prefs.note_refused_open(error, chrome); debug_log(&format!("[TRACE] wgpu shell resolve_conflict failed: {detail}")); },
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = accept;
            prefs.open_conflicts.retain(|row| row.id != conflict_id);
        }
        if prefs.selected_conflict_id.as_deref() == Some(conflict_id) {
            prefs.selected_conflict_id = None;
        }
    }
    //#endregion 🧭️PanelAnchorAccessors
}

//#endregion ShellLifecycle

/// ⚔️ React's `conflictKindLabel`/`conflictMessageText` reduction of one wire conflict into the
/// row the Conflicts leaf paints — the kind's own tag, and the WORST message's code and text
/// (`conflict.messages[0]`, React's own pick), never prose-parsed.
#[cfg(not(target_arch = "wasm32"))]
pub fn conflict_rows(conflicts: &[protocol::Conflict]) -> Vec<ShellConflictRow> {
    conflicts
        .iter()
        .filter(|conflict| conflict.status == protocol::ConflictStatus::Open)
        .map(|conflict| {
            let worst = conflict.messages.first();
            ShellConflictRow {
                id: conflict.id.0.clone(),
                quarantined: matches!(conflict.kind, protocol::ConflictKind::Quarantined { .. }),
                code: worst.map(|message| message.code.0.clone()).unwrap_or_default(),
                message: worst.map(|message| message.message.clone()).unwrap_or_default(),
                preview_after: match &conflict.kind {
                    protocol::ConflictKind::Quarantined { envelopes } => {
                        let compact = semio_framework_pack_json::to_json_string(envelopes);
                        serde_json::from_str::<serde_json::Value>(&compact).ok().and_then(|value| serde_json::to_string_pretty(&value).ok()).unwrap_or(compact)
                    }
                    protocol::ConflictKind::Degraded { edit_ids } => format!("// degraded edits: {}", edit_ids.join(", ")),
                },
            }
        })
        .collect()
}
