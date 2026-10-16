//! ⚓️ `shell-dock` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ShellDockState {
    /// 🪟️ Every declared window instance, including siblings hidden by a focused/maximized view.
    /// The committed dock is the retention authority; the paint plan deliberately omits hidden
    /// siblings and therefore cannot decide that their engine owners have closed.
    pub fn live_window_ids(&self, docs: &DocumentState) -> Vec<String> {
        self.dock.collect_window_ids().into_iter().chain(docs.panel_documents.keys().cloned()).collect()
    }

    /// 📇️ Republishes, onto every live World3d surface, the action ids its WINDOW KIND declares.
    ///
    /// A World3d surface is keyed by its window instance id (the same identity `setCamera`'s
    /// `windowId` argument addresses), so its owning kind is exactly what the dock says that instance
    /// is, and the kind's `actions` are `window_kind_action_refs` after `build_definition` resolved
    /// them. This is the ONE declaration of which verbs a surface may emit: the world module offers
    /// no gumball handle whose verb is absent from it
    /// (`infinite_world::world::world3d_offers_transform_gumball`), rather than guessing per role.
    pub fn sync_world3d_declared_actions(&mut self, surfaces: &mut SurfaceState, docs: &DocumentState) {
        let Some(session) = docs.session.clone() else { return };
        let declarations: Vec<(String, Vec<String>)> = surfaces
            .world3d_states
            .keys()
            .filter_map(|host_id| {
                let window_id = surfaces.world3d_window_ids.get(host_id)?;
                let kind_id = self.live_window_kind_id(&session, window_id);
                let action_ids = session.app.window_kinds.iter().find(|kind| Some(kind.id.as_str()) == kind_id).map(|kind| kind.actions.iter().map(|action| action.id.clone()).collect()).unwrap_or_default();
                Some((host_id.clone(), action_ids))
            })
            .collect();
        for (host_id, action_ids) in declarations {
            if let Some(state) = surfaces.world3d_states.get_mut(&host_id) {
                infinite_world::world::set_world3d_declared_actions(&mut state.session, &action_ids);
            }
        }
    }

    /// 🖥️ The `framework.display.layout` leaf's body — the wgpu twin of React's
    /// `buildDisplayLayoutTree`: a `framework.display.layout.save` section carrying the name box and
    /// the Save button (disabled on a blank name, React's own `!host.layoutSaveLabel.trim()` gate),
    /// then a `framework.display.layout.list` section carrying the app's builtin layouts folded by
    /// their `groupPath` and, when this session saved any, a `Saved` folder whose rows each offer
    /// React's `framework.display.delete.<id>` action.
    pub fn build_display_layout_ui(&self, prefs: &PrefsState, docs: &DocumentState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let builtin: Vec<ui_wgpu::wgpu::NamedLayout> = docs.session.as_ref().map(|session| session.app.named_layouts.iter().filter(|entry| entry.origin == "builtin").cloned().collect()).unwrap_or_default();
        let save_section = UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.display.layout.save".into(),
            label: Some(Label::data(shell_chrome_string("display.saveLayout", is_de))),
            default_open: Some(false),
            presence: UiPresence::default(),
            items: vec![
                UiTreeItemNode {
                    id: "framework.display.layout.save.label".into(),
                    label: Label::data(shell_chrome_string("common.name", is_de)),
                    control: Some(ui_wgpu::wgpu::component::ui::UiControlNode::Input(UiInputNode {
                        id: "framework.display.saveLabel".into(),
                        input_kind: "text".into(),
                        value: self.layout_save_label.clone(),
                        placeholder: Some(Label::data(shell_chrome_string("display.saveLayoutPlaceholder", is_de))),
                        accessibility_label: None,
                        commit: None,
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        precision: None,
                        snaps: Vec::new(),
                        on_change: ActionDescriptor { controller_id: "framework".into(), action: "setLayoutSaveLabel".into(), args: None },
                        on_submit: None,
                        on_abort: None,
                        on_repeat_last: None,
                        presence: UiPresence::default(),
                        menu: None,
                        ..Default::default()
                    })),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                },
                UiTreeItemNode {
                    id: "framework.display.layout.save.action".into(),
                    label: Label::data(shell_chrome_string("common.save", is_de)),
                    control: Some(ui_wgpu::wgpu::component::ui::UiControlNode::Button(UiButtonNode {
                        id: Some("framework.display.save".into()),
                        icon_id: IconName::Save,
                        label: Label::data(shell_chrome_string("display.saveCurrentLayout", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "saveCurrentLayout".into(), args: None },
                        style: None,
                        presence: UiPresence { state: if self.layout_save_label.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                        menu: None,
                    })),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                },
            ],
            window: None,
        };
        let mut items = named_layout_rows(&builtin, false, is_de);
        if !self.user_layouts.is_empty() {
            items.push(UiTreeItemNode {
                id: "framework.display.layout.group.saved".into(),
                label: Label::data(shell_chrome_string("display.saved", is_de)),
                default_open: Some(false),
                items: Some(named_layout_rows(&self.user_layouts, true, is_de)),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
        }
        let list_section = UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.display.layout.list".into(),
            label: Some(Label::data(shell_chrome_string("display.layouts", is_de))),
            default_open: Some(true),
            presence: UiPresence::default(),
            items,
            window: None,
        };
        display_panel_body("framework.display.layout.panel", vec![save_section, list_section])
    }

    //#region 🧭️PanelAnchorAccessors
    pub fn anchor_state(&self, anchor: PanelAnchor) -> &PanelAnchorState {
        &self.panel_anchors[anchor.index()]
    }

    pub fn anchor_state_mut(&mut self, anchor: PanelAnchor) -> &mut PanelAnchorState {
        &mut self.panel_anchors[anchor.index()]
    }

    /// 🧭️ Is this anchor actually painting — open, and carrying at least one tab to paint.
    pub fn anchor_open(&self, anchor: PanelAnchor) -> bool {
        self.anchor_state(anchor).visible && !self.dock_tabs.tabs(anchor).is_empty()
    }

    /// 🧭️ Every anchor this frame paints a floating panel at, in `PanelAnchor::ALL` order.
    pub fn open_anchors(&self) -> Vec<PanelAnchor> {
        PanelAnchor::ALL.into_iter().filter(|anchor| self.anchor_open(*anchor)).collect()
    }

    /// 🧭️ Projection kept for the shell's column-level reads (`⌘️B`, the overlay safe area, the tutorial
    /// snapshot): the left column is open while either of its corner anchors is.
    pub fn left_panel_open(&self) -> bool {
        column_anchors(true).into_iter().any(|anchor| self.anchor_open(anchor))
    }

    pub fn right_panel_open(&self) -> bool {
        column_anchors(false).into_iter().any(|anchor| self.anchor_open(anchor))
    }

    /// 🧭️ Opens `anchor` at `tab_id`'s path, or folds it shut when that tab is already the open one —
    /// the toggle every panel-toggle control and every tab button shares (React's `PanelChromeTabBar`).
    pub fn toggle_anchor_tab(&mut self, anchor: PanelAnchor, tab_id: &str) {
        let already = self.anchor_open(anchor) && self.anchor_state(anchor).path.iter().any(|id| id == tab_id);
        if already {
            self.anchor_state_mut(anchor).visible = false;
            return;
        }
        let path = self.dock_tabs.locate(tab_id).filter(|(located, _)| *located == anchor).map(|(_, path)| self.dock_tabs.reconcile_path(anchor, &path)).unwrap_or_else(|| self.dock_tabs.default_path(anchor));
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
    }

    /// 🧭️ Opens whichever anchor hosts `tab_id` today and selects it — the programmatic reveal every
    /// `setActivePanelTab` outcome, tour step and panel-tab hit funnels through.
    pub fn reveal_dock_tab(&mut self, tab_id: &str) -> Option<PanelAnchor> {
        let (anchor, path) = self.dock_tabs.locate(tab_id)?;
        let path = self.dock_tabs.reconcile_path(anchor, &path);
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
        Some(anchor)
    }

    /// 🌳️ Is this anchor's branch folded shut in its tab bar.
    pub fn dock_branch_collapsed(&self, anchor: PanelAnchor, branch_id: &str) -> bool {
        self.dock_collapsed_branches.get(&format!("{}/{branch_id}", anchor.as_str())).copied().unwrap_or(false)
    }

    pub fn toggle_dock_branch(&mut self, anchor: PanelAnchor, branch_id: &str) {
        let key = format!("{}/{branch_id}", anchor.as_str());
        let next = !self.dock_collapsed_branches.get(&key).copied().unwrap_or(false);
        self.dock_collapsed_branches.insert(key, next);
    }

    /// 🧭️ The per-anchor projection the tutorial snapshot and the anchor-model tests read.
    pub fn panel_anchor_snapshot(&self, anchor: PanelAnchor) -> PanelAnchorSnapshot {
        let state = self.anchor_state(anchor);
        if !self.anchor_open(anchor) {
            return PanelAnchorSnapshot::default();
        }
        PanelAnchorSnapshot { visible: true, size: state.size, active_tab: state.active_tab().map(ToOwned::to_owned) }
    }

    /// 🗄️ Flattens every anchor's chrome into the persisted shape, omitting whatever equals the default.
    ///
    /// 🗄️ Only what DIFFERS from the computed default is stored, React's `dockUiStateStore` rule —
    /// and every anchor's path is reconciled to its own default by `sync_dock_tabs`, so a path
    /// equal to that default is not a user choice and must not turn an untouched anchor into a
    /// persisted entry.
    pub fn dock_ui_snapshot(&self) -> DockUiState {
        let mut anchors = BTreeMap::new();
        for anchor in PanelAnchor::ALL {
            let state = self.anchor_state(anchor);
            let entry = DockUiPanelState {
                visible: state.visible.then_some(true),
                size: (state.size != DEFAULT_PANEL_WIDTH_PX).then_some(state.size),
                path: (!state.path.is_empty() && state.path != self.dock_tabs.default_path(anchor)).then(|| state.path.clone()),
            };
            if entry != DockUiPanelState::default() {
                anchors.insert(anchor.as_str().to_string(), entry);
            }
        }
        DockUiState { version: DOCK_SKELETON_VERSION, anchors }
    }

    /// 🗄️ Applies a persisted per-anchor chrome snapshot — used both on load and directly by tests.
    pub fn apply_dock_ui(&mut self, state: &DockUiState) {
        for anchor in PanelAnchor::ALL {
            let Some(entry) = state.anchors.get(anchor.as_str()) else {
                self.anchor_state_mut(anchor).visible = false;
                continue;
            };
            let target = self.anchor_state_mut(anchor);
            target.visible = entry.visible.unwrap_or(false);
            if let Some(size) = entry.size {
                target.size = size;
            }
            if let Some(path) = &entry.path {
                target.path = path.clone();
            }
        }
    }

    /// 🗄️ Persists the current dock chrome so it survives a reload — the twin of `persist_dock_layout`
    /// for the unrelated `dock`/Mode system.
    pub fn persist_dock_ui(&mut self, chrome: &mut ChromeState, docs: &DocumentState) {
        let snapshot = self.dock_ui_snapshot();
        let app_id = docs.dock_store_app_id();
        save_dock_ui_to_store(app_id.as_deref(), Some(&snapshot));
        chrome.chrome_present.last_persisted_dock_ui = Some(snapshot);
    }

    /// 🗄️ Persists the dock chrome only when it actually changed since the last persist/load — called
    /// once per frame from `render_chrome` (`ShellChrome`), the same dirty-checked render-loop hook
    /// `persist_ui_prefs_if_changed` (💾️PrefsSync) uses, so no `ui.panelToggle.*` arm has to persist itself.
    pub fn persist_dock_ui_if_changed(&mut self, chrome: &mut ChromeState, docs: &DocumentState) {
        let current = self.dock_ui_snapshot();
        if chrome.chrome_present.last_persisted_dock_ui.as_ref() != Some(&current) {
            self.persist_dock_ui(chrome, docs);
        }
        let skeleton = self.dock_override.clone();
        if chrome.chrome_present.last_persisted_dock_skeleton != Some(skeleton.clone()) {
            let app_id = docs.dock_store_app_id();
            save_dock_skeleton_to_store(app_id.as_deref(), skeleton.as_ref());
            chrome.chrome_present.last_persisted_dock_skeleton = Some(skeleton);
        }
    }

    /// 🖥️ Restores this app's SAVED layouts from the replayed `os.config.ui-preferences` log — React's Display
    /// host projection, re-run from [`Self::load_persisted_dock`] on every app change as React re-memoizes it on
    /// every focused-app change.
    pub fn load_persisted_named_layouts(&mut self, docs: &DocumentState) {
        let app_id = docs.dock_store_app_id();
        self.user_layouts = load_named_layouts_from_store(app_id.as_deref());
    }

    /// 🖥️ Commits this app's saved-layout roster as `setNamedLayout` events — React's `onSaveUserLayout`/
    /// `onDeleteUserLayout`. Called by whatever mutates [`ShellState::user_layouts`].
    pub fn persist_named_layouts(&self, docs: &DocumentState) {
        let app_id = docs.dock_store_app_id();
        save_named_layouts_to_store(app_id.as_deref(), &self.user_layouts);
    }

    /// 🖥️ The CURRENT dock as a saveable user layout — React's
    /// `createNamedLayout("user-" + Date.now(), label, currentLayout, "user")`
    /// (`📌️ChromePanels/🟦️.tsx`'s `saveCurrentLayout`), with the same `user-<millis>` id grammar so a
    /// layout saved on either renderer reads as a user layout on the other. `None` with no session,
    /// which is React's own `if (!options.currentLayout) return` gate.
    pub fn current_named_layout(&self, label: &str, docs: &DocumentState) -> Option<ui_wgpu::wgpu::NamedLayout> {
        docs.session.as_ref()?;
        let stamp = instant_now_ms().max(0.0) as u64;
        Some(ui_wgpu::wgpu::NamedLayout { id: format!("user-{stamp}"), label: label.to_string(), icon_id: None, layout: self.dock.to_window_layout(), origin: "user".into(), group_path: None })
    }

    /// 🖥️ Opens one more instance of `window_kind_id`, split right of the active stack — the same
    /// placement [`ShellState::open_active_window_in_new_window`] uses for `mod+shift+n`, which is
    /// React's `onWindowOpenInNewWindow`. A `template_id` seeds that instance's projection exactly as
    /// React's dropped `{windowKindId, templateId}` payload does.
    pub fn open_display_window(&mut self, window_kind_id: &str, template_id: Option<&str>, settle: &mut SettleState, docs: &DocumentState, windows: &mut WindowState) -> bool {
        let Some(session) = docs.session.as_ref() else { return false };
        let Some(body_key) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).map(|kind| kind.body_key.clone()) else {
            return false;
        };
        if !windows.window_topology_publication_has_credit() {
            return false;
        }
        let instance_id = self.allocate_window_instance_id(window_kind_id);
        let anchor = windows.active_window_id.clone().or_else(|| self.dock.collect_window_ids().first().cloned());
        let zone = anchor
            .as_deref()
            .and_then(|window_id| self.dock_stack_path_for(window_id))
            .map(|stack_path| DockDropZone::Split { stack_path, side: semio_framework_os_renderer_wgpu_dock::DockSide::Right })
            .unwrap_or(DockDropZone::RootSplit { side: semio_framework_os_renderer_wgpu_dock::DockSide::Right });
        let payload = DockDragPayload {
            kind: DockDragKind::NewWindow,
            window_id: instance_id.clone(),
            window_kind_id: window_kind_id.to_string(),
            template_id: template_id.map(str::to_string),
            source_path: Vec::new(),
            tab_index: 0,
            ghost_label: window_kind_id.to_string(),
        };
        if !self.dock.apply_drop(&payload, &zone) {
            return false;
        }
        self.seed_new_window_projection(&instance_id, template_id, docs, windows);
        windows.active_window_id = Some(instance_id.clone());
        self.persist_dock_layout();
        windows.commit_window_topology_publication(&instance_id, &body_key, None, settle);
        true
    }

    /// 🏷️ Seeds a newly transferred window's projection chrome, as React's ShellHost drop handler does.
    pub fn seed_new_window_projection(&mut self, window_id: &str, template_id: Option<&str>, docs: &DocumentState, windows: &mut WindowState) {
        let Some(template_id) = template_id else { return };
        windows.world_projection_template.insert(window_id.to_string(), template_id.to_string());
        let Some(selected) = world_projection_template_selection_id(template_id) else { return };
        let effective = match selected { "parallel" => "orthographic", "axonometric" => "axonometric-isometric", "oblique" => "oblique-cavalier", "perspective" => "three-point", other => other };
        let template = world_projection_template(effective);
        let _ = self.apply_window_title_host_command(window_id, template.label, docs, windows);
        let _ = self.apply_window_icon_host_command(window_id, template.icon_id, docs, windows);
    }

    /// 🖥️ The layout `layout_id` names, searching the app's declared layouts and then this app's saved
    /// ones — React's `[...builtinLayouts, ...userLayouts].find(…)`.
    pub fn named_layout_by_id(&self, layout_id: &str, docs: &DocumentState) -> Option<ui_wgpu::wgpu::WindowLayout> {
        if let Some(named) = docs.session.as_ref().and_then(|session| session.app.named_layouts.iter().find(|entry| entry.id == layout_id)) {
            return Some(named.layout.clone());
        }
        self.user_layouts.iter().find(|entry| entry.id == layout_id).map(|entry| entry.layout.clone())
    }
    //#endregion 🧭️PanelAnchorAccessors
}

//#endregion ShellLifecycle

/// 🧭️ `⌘️B`'s left column and `⌘️⇧️B`'s right column — the two corner anchors that column carries,
/// top first, the only place the former hardcoded left/right pair survives.
pub fn column_anchors(left: bool) -> [PanelAnchor; 2] {
    if left {
        [PanelAnchor::TopLeft, PanelAnchor::BottomLeft]
    } else {
        [PanelAnchor::TopRight, PanelAnchor::BottomRight]
    }
}
