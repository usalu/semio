//! 🚶️ `shell-session` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl<'ctx> SessionCtx<'ctx> {
    /// 🕸️ Mirrors the engine surfaces a paint pass attached into `node_graph_states` /
    /// `tiled_map_states` / `board2d_states`, the maps the OS event loop hit-tests pointer/wheel
    /// events against (`node_graph_pointer_down_into` and siblings). The engine host itself is owned
    /// by the worker's `ENGINE_SURFACES` registry; this side carries only the surface's screen bounds
    /// and the controller its observations are addressed to.
    ///
    /// A host constructed this frame is also the moment the app-static catalogue must land on it —
    /// `refresh_app_catalogue` runs on the UI refresh, which for a freshly attached surface has
    /// already happened.
    pub fn sync_engine_surface_states(&mut self) {
        let registrations = semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::take_engine_surface_registrations();
        let live_windows = self.dock.live_window_ids(&self.docs);
        let live_windows: Vec<&str> = live_windows.iter().map(String::as_str).collect();
        trace_engine_surface_sync(&self.surfaces.node_graph_states, &registrations, &live_windows);
        let created = mirror_engine_surface_states(&mut self.surfaces.node_graph_states, &mut self.surfaces.tiled_map_states, &mut self.surfaces.board2d_states, &mut self.surfaces.world3d_status, &mut self.surfaces.world3d_window_ids, registrations, &live_windows);
        if !self.plugins.app_catalogue_json.is_empty() {
            for surface_id in created {
                semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_set_catalogue_json(&surface_id, &self.plugins.app_catalogue_json);
            }
        }
        self.retire_closed_world3d_windows(&live_windows);
        self.dock.sync_world3d_declared_actions(&mut self.surfaces, &self.docs);
    }

    /// 🪟️ Retires a closed World3d window at the layout-to-input sync boundary, once its capture-lost cancel was driven
    /// ([`Self::world3d_close_cancel_settled`]). The scene owner and every shell-local projection keyed by that owner
    /// disappear in one bounded pass before input.
    pub fn retire_closed_world3d_windows(&mut self, live_window_ids: &[&str]) {
        let retired: Vec<String> = self.surfaces.world3d_window_ids.iter().filter(|(_, window_id)| !live_window_ids.contains(&window_id.as_str())).map(|(host_id, _)| host_id.clone()).collect();
        for host_id in retired {
            if !self.surfaces.world3d_close_cancel_settled(&host_id) {
                continue;
            }
            let Some(window_id) = self.surfaces.world3d_window_ids.remove(&host_id) else { continue };
            let Some(mut state) = self.surfaces.world3d_states.remove(&host_id) else { continue };
            begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
            self.surfaces.world3d_retirement_epoch = self.surfaces.world3d_retirement_epoch.wrapping_add(1).max(1);
            let retirement_id = format!("{host_id}#{}", self.surfaces.world3d_retirement_epoch);
            assert!(self.surfaces.retired_world3d_states.len() < SCENE_SURFACE_CAPACITY, "retired World3d owners share the fixed scene-owner grants");
            self.surfaces.retired_world3d_states.push_back((retirement_id, state));
            assert!(self.surfaces.world3d_states.set_external_reservations(self.surfaces.retired_world3d_states.len()), "retired World3d reservation follows active owner removal");
            self.surfaces.world3d_status.remove(&host_id);
            self.surfaces.world3d_status_pill_trace.remove(&host_id);
            self.settle.settle_pump.watches.remove(&host_id);
            self.windows.active_utility_by_window.remove(&window_id);
            self.windows.action_panel_folded.remove(&window_id);
            self.windows.search_panel_folded.remove(&window_id);
            self.windows.utility_bar_folded.remove(&window_id);
            self.windows.projection_pane_folded.remove(&window_id);
            self.windows.world_projection_template.remove(&window_id);
            self.windows.window_icon_overrides.remove(&window_id);
            self.windows.window_title_overrides.remove(&window_id);
            self.windows.action_panel_expanded.remove(&window_id);
            self.commands.search_possibles_open.remove(&window_id);
            self.windows.measures_folded.remove(&window_id);
            self.windows.measures_expanded.remove(&window_id);
            self.windows.measures_width.remove(&window_id);
            self.windows.engagement_expanded.remove(&window_id);
            self.windows.engagement_activated.remove(&window_id);
            self.windows.window_engagements.remove(&window_id);
            self.dock.window_content_rects.remove(&window_id);
            self.dock.window_silhouettes.remove(&window_id);
            let prefix = format!("{window_id}:");
            self.commands.staged_action_args.retain(|key, _| !key.starts_with(&prefix));
        }
        self.surfaces.world3d_status.retain(|host_id, _| self.surfaces.world3d_window_ids.contains_key(host_id));
    }

    /// 🛍️ The `framework.marketplace` leaf's body — the wgpu twin of React's `buildMarketplaceTree`: one section per plugin
    /// SOURCE, a `label · version · status` row per plugin, and this renderer's own install/reload/uninstall lane.
    ///
    /// 🎯️ Every row carries its verbs as ROW ACTIONS on ONE target (U6's row model): a plugin row targets
    /// `framework` with `{ pluginId }` and names `installPlugin` or `reloadPlugin`/`uninstallPlugin`; an extension row targets
    /// `{ extensionId, enabled: !enabled }` and names `setExtensionEnabled`/`uninstallExtension`. A verb the row cannot run
    /// paints and announces DISABLED (React's `disabled` buttons) instead of disappearing, and names why as its localized reason
    /// (the contract's `RowAction.reason`, announced as its description) — the session's own program is never uninstallable
    /// (React's `canUninstall`), a plugin mid-install offers nothing, a failed extension cannot be enabled.
    ///
    /// 🪟️ The roster is ONE windowed Tree section (`framework.marketplace.source.local`): the rows the tree window observer
    /// reported for it ([`tree_windows::TreeWindowScheduler::window_of`]) — one viewport of rows before the first report —
    /// materialise inside [`MARKETPLACE_WINDOW_NODE_BUDGET`], and the retained layout pitches the rest as spacers, so a roster of
    /// any length stays inside the retained node ceiling and scrolls to its last row. Store-owned extensions stay nested under
    /// their declared host, including failed loads, so install failures never erase inventory.
    pub fn build_marketplace_ui(&self) -> UiNode {
        let is_de = self.prefs.locale_id == "de";
        let session_plugin = self.docs.session.as_ref().map(|session| session.plugin_id.clone());
        let ids = self.plugins.marketplace_roster();
        let installing = self.plugins.plugin_install.as_ref().map(|install| install.plugin_id.clone());
        let verb = |action: &str, args: Option<DslValue>, icon: IconName, label_key: &'static str, disabled_because: Option<&'static str>| ui_wgpu::wgpu::component::ui::UiTreeItemAction {
            icon_id: icon,
            label: Some(Label::data(shell_chrome_string(label_key, is_de))),
            action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            placement: None,
            disabled: disabled_because.is_some(),
            reason: disabled_because.map(|reason_key| Label::data(shell_chrome_string(reason_key, is_de))),
        };
        let extension_item = |entry: &ShellExtensionProjection| {
            let extension_id = entry.record.extension_id.clone();
            let can_toggle = matches!(entry.load_status, ShellExtensionLoadStatus::Available | ShellExtensionLoadStatus::Loaded);
            let target = semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "extensionId": extension_id.clone(), "enabled": !entry.enabled });
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{}.extension.{extension_id}", entry.record.extends_host),
                label: Label::data(format!("{} · {} · {}", entry.record.label, entry.record.version, shell_chrome_string(if entry.enabled { "plugins.extension.enabled" } else { "plugins.extension.disabled" }, is_de))),
                actions: Some(vec![
                    verb("setExtensionEnabled", target.clone(), if entry.enabled { IconName::EyeOff } else { IconName::Eye }, if entry.enabled { "plugins.extension.disable" } else { "plugins.extension.enable" }, (!can_toggle).then_some("plugins.reason.extensionUnavailable")),
                    verb("uninstallExtension", target, IconName::Trash2, "plugins.action.uninstall", None),
                ]),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let plugin_item = |plugin_id: &str| {
            let resident = self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id);
            let in_flight = installing.as_deref() == Some(plugin_id);
            let status_key = if in_flight {
                "plugins.status.installing"
            } else if resident.is_some() {
                "plugins.status.loaded"
            } else {
                "plugins.status.available"
            };
            let label = resident.map(|entry| entry.manifest.label.clone()).unwrap_or_else(|| plugin_id.to_string());
            let version = resident.map(|entry| entry.manifest.version.clone()).unwrap_or_default();
            let head = if version.is_empty() { format!("{label} \u{b7} {}", shell_chrome_string(status_key, is_de)) } else { format!("{label} \u{b7} {version} \u{b7} {}", shell_chrome_string(status_key, is_de)) };
            let target = semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "pluginId": plugin_id });
            let installing_reason = in_flight.then_some("plugins.reason.installing");
            let uninstall_reason = installing_reason.or_else(|| (session_plugin.as_deref() == Some(plugin_id)).then_some("plugins.reason.inUse"));
            let actions = if resident.is_some() {
                vec![verb("reloadPlugin", target.clone(), IconName::RotateCcw, "plugins.action.reload", installing_reason), verb("uninstallPlugin", target, IconName::Trash2, "plugins.action.uninstall", uninstall_reason)]
            } else {
                vec![verb("installPlugin", target, IconName::Download, "plugins.action.install", installing_reason)]
            };
            let extensions: Vec<UiTreeItemNode> = self.plugins.extensions.iter().filter(|entry| entry.record.extends_host == plugin_id).map(&extension_item).collect();
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{plugin_id}"),
                label: Label::data(head),
                default_open: Some(!extensions.is_empty()),
                presence: UiPresence { status: if in_flight { ui_wgpu::wgpu::component::ui::UiStatus::Loading } else { ui_wgpu::wgpu::component::ui::UiStatus::Idle }, ..UiPresence::default() },
                actions: Some(actions),
                items: (!extensions.is_empty()).then_some(extensions),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let section_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
        let (offset, rows) = self.dock.tree_windows.window_of(FRAMEWORK_MARKETPLACE_TAB_ID, &section_key).unwrap_or((0, self.dock.tree_windows.viewport_rows_or_default()));
        let offset = (offset as usize).min(ids.len().saturating_sub(1));
        let mut credit = MARKETPLACE_WINDOW_NODE_BUDGET;
        let materialised: Vec<UiTreeItemNode> = ids
            .iter()
            .skip(offset)
            .take(rows as usize)
            .map(|plugin_id| plugin_item(plugin_id))
            .map_while(|item| {
                let cost = 1 + item.items.as_ref().map_or(0, Vec::len);
                (cost <= credit).then(|| {
                    credit -= cost;
                    item
                })
            })
            .collect();
        let (items, window) = if ids.is_empty() {
            (vec![UiTreeItemNode::base("framework.marketplace.empty", Label::data(shell_chrome_string("marketplace.unavailable", is_de)))], None)
        } else {
            (materialised, Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: ids.len() as u32, offset: offset as u32 }))
        };
        let mut missing_hosts: Vec<&str> = self.plugins.extensions.iter().map(|entry| entry.record.extends_host.as_str()).filter(|host| !ids.iter().any(|id| id.as_str() == *host)).collect();
        missing_hosts.sort();
        missing_hosts.dedup();
        let install_file = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.file".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromFile", is_de)),
            icon_id: Some(IconName::FileArchive),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionFile".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let install_url = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.url".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromUrl", is_de)),
            icon_id: Some(IconName::Download),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionUrl".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let mut sections = vec![
            UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.marketplace.extensions.install".into(),
                label: Some(Label::data(shell_chrome_string("plugins.extension.install", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![install_url, install_file],
                window: None,
            },
            UiTreeSectionNode {
                header_toolbar: None,
                id: MARKETPLACE_LOCAL_SECTION_ID.into(),
                label: Some(Label::data(format!("{}: local", shell_chrome_string("plugins.source", is_de)))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window,
            },
        ];
        sections.extend(missing_hosts.into_iter().map(|host| UiTreeSectionNode {
            header_toolbar: None,
            id: format!("framework.marketplace.missing-host.{host}"),
            label: Some(Label::data(host.to_string())),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: self.plugins.extensions.iter().filter(|entry| entry.record.extends_host.as_str() == host).map(&extension_item).collect(),
            window: None,
        }));
        display_panel_body("framework.marketplace.panel", sections)
    }

    //#region 🧭️PanelAnchorAccessors
    /// 🧭️ The dock arrangement this session's sources declare, before any persisted override — the wgpu
    /// twin of React's `defaultDock` (`🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly): app panel tabs routed to
    /// their `PanelGroup` corner, the framework Display branch and the sync leaf on bottom-left, the
    /// framework chat leaf on top-right, and the Tool/Command branches on bottom-middle.
    ///
    /// 🧭️ React's own bottom-left order (`🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly): the FRAMEWORK
    /// Display branch first — its children are the framework's two display leaves
    /// (`createFrameworkDisplayPanelTabs`, `📌️ChromePanels/🟦️.tsx:249`), never the app's tabs —
    /// then the app's own bottom-left tabs as SIBLINGS of that branch, then the sync leaf. The
    /// branch used to be built out of the app's tabs and was therefore absent altogether for an
    /// app that declares none (puzzle3d), which is why the first real wgpu boot's footer showed no
    /// "Display" chip at all while React always shows one.
    ///
    /// ⚙️ Bottom-right carries ONE Settings branch (the app's own Settings-group tabs first, then the
    /// five framework leaves — React's `integrateAppSettingsPanelTabsIntoFrameworkBranch`, which
    /// re-orders the framework children past the app's), then the Marketplace leaf, then the
    /// framework-owned History leaf — the guest's own `framework.body.history`, which React's dock
    /// partitions into the same leaf (`partitionFrameworkHistoryPanelTab`).
    pub fn default_dock(&self) -> ShellDock {
        let mut dock = ShellDock::default();
        let is_de = self.prefs.locale_id == "de";
        let Some(session) = &self.docs.session else { return dock };
        let terminology = self.prefs.active_terminology();
        let locale = self.prefs.active_locale();
        for tab in &session.app.panel_tabs {
            let anchor = PanelAnchor::from_group(tab.group);
            let order = dock.tabs(anchor).len() as i32;
            dock.tabs_mut(anchor).push(dock_tab_node_of(tab, terminology, locale, order));
        }
        let app_bottom_left: Vec<DockTabNode> = std::mem::take(dock.tabs_mut(PanelAnchor::BottomLeft));
        let mut bottom_left = vec![DockTabNode::branch(FRAMEWORK_CATEGORY_DISPLAY_ID, shell_chrome_string("panelToggle.display", is_de), "layout-grid", 0, framework_display_tabs(is_de))];
        let offset = bottom_left.len() as i32;
        bottom_left.extend(app_bottom_left.into_iter().enumerate().map(|(index, tab)| DockTabNode { order: offset + index as i32, ..tab }));
        let order = bottom_left.len() as i32;
        bottom_left.push(DockTabNode::leaf(FRAMEWORK_SYNC_PANEL_TAB_ID, shell_sync_pill_text(self.sync.sync_pill(), is_de), "refresh-cw", order));
        *dock.tabs_mut(PanelAnchor::BottomLeft) = bottom_left;
        let chat_order = dock.tabs(PanelAnchor::TopRight).len() as i32;
        dock.tabs_mut(PanelAnchor::TopRight).push(DockTabNode::leaf(FRAMEWORK_CHAT_PANEL_ID, shell_chrome_string("panelToggle.chat", is_de), "message-square", chat_order));
        let app_settings: Vec<DockTabNode> = std::mem::take(dock.tabs_mut(PanelAnchor::BottomRight));
        let (history, app_settings): (Vec<DockTabNode>, Vec<DockTabNode>) = app_settings.into_iter().partition(|tab| tab.id == FRAMEWORK_PANEL_TAB_HISTORY_ID);
        let mut settings_children = app_settings;
        let offset = settings_children.len() as i32;
        for (index, child) in self.prefs.framework_settings_tabs().into_iter().enumerate() {
            settings_children.push(DockTabNode { order: offset + index as i32, ..child });
        }
        let mut bottom_right = vec![DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, shell_chrome_string("panelToggle.settings", is_de), "settings", 0, settings_children)];
        bottom_right.push(DockTabNode::leaf(FRAMEWORK_MARKETPLACE_TAB_ID, shell_chrome_string("marketplace.tab", is_de), "circle-dot", 1));
        bottom_right.push(DockTabNode::leaf(FRAMEWORK_TASK_MANAGER_PANEL_ID, shell_chrome_string("panelToggle.taskManager", is_de), "cpu", 2));
        let history_offset = bottom_right.len() as i32;
        bottom_right.extend(history.into_iter().enumerate().map(|(index, tab)| DockTabNode { order: history_offset + index as i32, ..tab }));
        *dock.tabs_mut(PanelAnchor::BottomRight) = bottom_right;
        let tool_tabs = self.docs.tool_panel_tabs(&self.prefs);
        if !tool_tabs.is_empty() {
            dock.tabs_mut(PanelAnchor::BottomMiddle).push(DockTabNode::branch(FRAMEWORK_CATEGORY_TOOL_ID, shell_chrome_string("panelToggle.tool", is_de), "hammer", 0, tool_tabs));
        }
        let command_tabs = self.docs.command_category_tabs(&self.chrome, &self.plugins);
        if !command_tabs.is_empty() {
            dock.tabs_mut(PanelAnchor::BottomMiddle).push(DockTabNode::branch(FRAMEWORK_CATEGORY_COMMAND_ID, shell_chrome_string("panelToggle.command", is_de), "wrench", 1, command_tabs));
        }
        dock
    }

    /// 📱️ All eight anchors' root tabs flattened into the single mobile panel's tab list, in
    /// `PanelAnchor::ALL` order, plus the synthetic `framework.mobile.app` leaf React appends when the
    /// navbar's example/mode/role clusters have no room — React's `mobilePanelTabs`
    /// (`🏛️ShellHost/🟦️.tsx`), which flattens `defaultDock`, not the override-applied dock.
    pub fn mobile_panel_tabs(&self) -> Vec<DockTabNode> {
        let is_de = self.prefs.locale_id == "de";
        let dock = self.default_dock();
        let mut tabs: Vec<DockTabNode> = PanelAnchor::ALL.into_iter().flat_map(|anchor| dock.tabs(anchor).to_vec()).collect();
        let has_example = !self.docs.session_example_rows(&self.plugins, &self.prefs).is_empty() && semio_framework_os_renderer_wgpu_host_base::boot_locks().example_id.is_empty();
        let has_modes = self.docs.session.as_ref().is_some_and(|session| session.app.modes.len() > 1);
        if has_example || has_modes {
            tabs.push(DockTabNode::leaf(FRAMEWORK_MOBILE_APP_TAB_ID, shell_chrome_string("mobilePanel.app", is_de), "circle-dot", 99));
        }
        tabs
    }

    /// 🧭️ Recomputes `dock_tabs` from the declared sources plus the persisted override, and reconciles
    /// every anchor's remembered path onto what still exists — the wgpu twin of the `applyDockSkeleton`
    /// memo plus React's per-anchor `reconcileActivePath` effects.
    pub fn sync_dock_tabs(&mut self) {
        let default_dock = self.default_dock();
        self.dock.dock_tabs = apply_dock_skeleton(&default_dock, self.dock.dock_override.as_ref());
        for anchor in PanelAnchor::ALL {
            if self.dock.dock_tabs.tabs(anchor).is_empty() {
                let state = self.dock.anchor_state_mut(anchor);
                state.path.clear();
                state.visible = false;
                continue;
            }
            let path = self.dock.dock_tabs.reconcile_path(anchor, &self.dock.anchor_state(anchor).path.clone());
            self.dock.anchor_state_mut(anchor).path = path;
        }
    }

    /// 🎯️ Completes a tab drag onto `target`, persisting the arrangement whenever it stops matching the
    /// computed default — React's `handleTabDockDrop`.
    pub fn move_dock_tab(&mut self, tab_id: &str, target: &DockTabMoveTarget) -> bool {
        let mut dock = self.dock.dock_tabs.clone();
        if !move_tab_in_dock(&mut dock, tab_id, target) {
            return false;
        }
        let next = dock_skeleton_of(&dock);
        let default_skeleton = dock_skeleton_of(&self.default_dock());
        self.dock.dock_override = (!dock_skeletons_equal(Some(&next), Some(&default_skeleton))).then_some(next);
        self.sync_dock_tabs();
        self.dock.reveal_dock_tab(tab_id);
        true
    }

    /// 🗄️ Loads the persisted arrangement and chrome and applies both — called once from `ShellState::new()`
    /// and again whenever the session's app changes, since the per-app layer is keyed by app id. Seeds the
    /// owned present-state snapshots so the first frame doesn't re-persist what was just read back.
    pub fn load_persisted_dock(&mut self) {
        let app_id = self.docs.dock_store_app_id();
        self.dock.dock_override = load_dock_skeleton_from_store(app_id.as_deref());
        self.sync_dock_tabs();
        if let Some(state) = load_dock_ui_from_store(app_id.as_deref()) {
            self.dock.apply_dock_ui(&state);
            self.sync_dock_tabs();
        }
        self.chrome.chrome_present.last_persisted_dock_ui = Some(self.dock.dock_ui_snapshot());
        self.chrome.chrome_present.last_persisted_dock_skeleton = Some(self.dock.dock_override.clone());
        self.dock.load_persisted_named_layouts(&self.docs);
    }

    /// 🖥️ Applies a builtin or saved layout by id — React's `applyNamedLayout`, which searches
    /// `[...builtinLayouts, ...userLayouts]` and calls `onApplyLayout` with what it finds. `false`
    /// when no layout carries that id, so the caller journals nothing for a stale row.
    pub fn apply_named_layout(&mut self, layout_id: &str) -> bool {
        let Some(layout) = self.dock.named_layout_by_id(layout_id, &self.docs) else { return false };
        self.dock.layout_override = Some(layout);
        self.sync_dock();
        self.windows.active_window_id = self.dock.dock.active_window_id.clone();
        true
    }
    //#endregion 🧭️PanelAnchorAccessors
}

/// 🩺️ One line per chrome-walk sync, naming the surfaces a DRAIN-eviction rule would have retired
/// here — the ones that are live, whose window is live, and that this drain does not mention. A
/// non-empty `old-rule-would-evict` followed by a press that still reaches the graph is the
/// runtime witness that engine-surface state is owned by the window instance and not by a painted
/// frame (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`). One line per completed chrome walk —
/// tens per minute, not per frame.
pub fn trace_engine_surface_sync(states: &AdmittedSurfaceMap<NodeGraphSurface>, registrations: &[semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceRegistration], live_window_ids: &[&str]) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    use std::sync::atomic::{AtomicU64, Ordering};
    static SYNCS: AtomicU64 = AtomicU64::new(0);
    static DRAINS_WITH_GRAPH: AtomicU64 = AtomicU64::new(0);
    let painted = registrations.iter().any(|entry| matches!(entry.detail, semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::NodeGraph));
    let syncs = SYNCS.fetch_add(1, Ordering::Relaxed) + 1;
    let with_graph = if painted { DRAINS_WITH_GRAPH.fetch_add(1, Ordering::Relaxed) + 1 } else { DRAINS_WITH_GRAPH.load(Ordering::Relaxed) };
    let would_evict: Vec<&str> = states.iter().filter(|(id, _)| !registrations.iter().any(|entry| &entry.host_id == *id)).map(|(id, _)| id.as_str()).collect();
    let live: Vec<String> = states.iter().map(|(id, surface)| format!("{id}@{}x{}+{},{}", surface.bounds.w.round(), surface.bounds.h.round(), surface.bounds.x.round(), surface.bounds.y.round())).collect();
    debug_log(&format!("[TRACE] wgpu-shell engine surfaces syncs={syncs} drains-with-graph={with_graph} drain={} live={live:?} old-rule-would-evict={would_evict:?} windows={live_window_ids:?}", registrations.len()));
}

/// 🧲️ The retention rule itself, stated once for all three vello-composited kinds: an engine
/// surface's pointer state belongs to its WINDOW INSTANCE, never to a painted frame. A drain
/// REFRESHES what it mentions and retires only the surfaces whose owning window has left
/// `live_window_ids`. Answers the surfaces whose engine host was constructed on this drain — the
/// ones the app-static catalogue must be installed on, and only those.
///
/// 🩸️ This used to evict every surface absent from the drain. Retained painting means an
/// unchanged document does not repaint, so the node graph fell out of `node_graph_states` in
/// nearly every frame — and a graph that is not in that map cannot be reached by a press, a
/// wheel, a context menu, a catalogue drop or the `Fit graph` control, because every one of those
/// paths starts by hit-testing this map. `world3d_states` was never a drain projection and its
/// input worked throughout, which is the asymmetry this closes (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-retained-controls-wires-2026-09-13.md` §7.3(b)).
///
/// `Self`-less so its law can drive the real body without a `ShellState` fixture —
/// `🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs` over
/// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
pub fn mirror_engine_surface_states(
    node_graph_states: &mut AdmittedSurfaceMap<NodeGraphSurface>,
    tiled_map_states: &mut AdmittedSurfaceMap<TiledMapSurface>,
    board2d_states: &mut AdmittedSurfaceMap<Board2dSurface>,
    world3d_status: &mut HashMap<String, String>,
    world3d_window_ids: &mut HashMap<String, String>,
    registrations: Vec<semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceRegistration>,
    live_window_ids: &[&str],
) -> Vec<String> {
    let retired: Vec<String> = node_graph_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
    for id in retired {
        node_graph_states.remove(&id);
    }
    let retired: Vec<String> = tiled_map_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
    for id in retired {
        tiled_map_states.remove(&id);
    }
    let retired: Vec<String> = board2d_states.iter().filter(|(_, surface)| !live_window_ids.contains(&surface.window_id.as_str())).map(|(id, _)| id.clone()).collect();
    for id in retired {
        board2d_states.remove(&id);
    }
    let mut constructed = Vec::new();
    for registration in registrations {
        let semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceRegistration { host_id, surface_id, engine_token: _, window_id, bounds, controller_id, detail, created } = registration;
        match detail {
            semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::NodeGraph => {
                if let Some(surface) = node_graph_states.get_or_insert_with(host_id.clone(), || NodeGraphSurface { surface_id: surface_id.clone(), bounds, controller_id: controller_id.clone(), window_id: window_id.clone() }) {
                    surface.surface_id = surface_id;
                    surface.bounds = bounds;
                    surface.controller_id = controller_id;
                    surface.window_id = window_id;
                }
                if created {
                    constructed.push(host_id);
                }
            }
            semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::TiledMap { selection_method } => {
                if let Some(surface) = tiled_map_states.get_or_insert_with(host_id.clone(), || TiledMapSurface {
                    surface_id: surface_id.clone(),
                    bounds,
                    controller_id: controller_id.clone(),
                    selection_method: selection_method.clone(),
                    window_id: window_id.clone(),
                }) {
                    surface.surface_id = surface_id;
                    surface.bounds = bounds;
                    surface.controller_id = controller_id;
                    surface.selection_method = selection_method;
                    surface.window_id = window_id;
                }
                if created {
                    constructed.push(host_id);
                }
            }
            semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::Board2d { snapshot_json } => {
                if let Some(surface) =
                    board2d_states.get_or_insert_with(host_id.clone(), || Board2dSurface { surface_id: surface_id.clone(), bounds, controller_id: controller_id.clone(), snapshot_json: snapshot_json.clone(), window_id: window_id.clone() })
                {
                    surface.surface_id = surface_id;
                    surface.bounds = bounds;
                    surface.controller_id = controller_id;
                    surface.snapshot_json = snapshot_json;
                    surface.window_id = window_id;
                }
                if created {
                    constructed.push(host_id);
                }
            }
            semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::World3d { status_json } => {
                world3d_window_ids.insert(host_id.clone(), window_id);
                match status_json {
                    Some(status_json) => world3d_status.insert(host_id, status_json),
                    None => world3d_status.remove(&host_id),
                };
            }
        }
    }
    constructed
}

/// 🎟️ The process-wide resident census, appended to every render's own log line so a `Capacity`
/// refusal is never the first thing anyone learns about the aggregate.
pub fn resident_census() -> String {
    ui_contract::UiResidentPermit::snapshot().map_or_else(|_| String::new(), |snapshot| format!(" resident-roots={} resident-bytes={}/{}", snapshot.used_slots, snapshot.bytes, ui_contract::UI_RESIDENT_AGGREGATE_BYTES))
}

/// 🖥️ The two framework Display leaves in React's own order — `framework.display.windows` then
/// `framework.display.layout` (`createFrameworkDisplayPanelTabs`, `📌️ChromePanels/🟦️.tsx:249`,
/// orders `-100`/`-99` so they always precede whatever an app adds). Framework-owned and
/// session-independent, exactly like `framework_settings_tabs`, which is why the Display chip is
/// unconditional chrome in both renderers.
pub fn framework_display_tabs(is_de: bool) -> Vec<DockTabNode> {
    vec![DockTabNode::leaf(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, shell_chrome_string("display.tab.windows", is_de), "layout-grid", 0), DockTabNode::leaf(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, shell_chrome_string("display.tab.layout", is_de), "layout", 1)]
}

//#endregion ShellLifecycle

