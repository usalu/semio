//! 🚶️ `shell-session` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> SessionCtx<'ctx> {
    /// 🔍️ Every palette row, in React's own declaration order (`🏛️ShellHost/🟦️.tsx`'s `searchItems`
    /// memo): panel tabs, window kinds, resolved commands, then the host-app verbs. Declaration order is
    /// what the ranker breaks ties on and what a cold (empty-query) palette shows, so it is part of the
    /// contract, not an accident.
    pub(super) fn build_search_items(&self) -> Vec<SearchPaletteItem> {
        let is_de = self.prefs.locale_id == "de";
        let Some(session) = &self.docs.session else {
            return Vec::new();
        };
        let mut items = Vec::new();
        for tab in flatten_panel_tab_leaves(&session.app.panel_tabs) {
            items.push(SearchPaletteItem {
                id: format!("panel.{}", tab.id()),
                label: tab.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string(),
                description: None,
                group: shell_chrome_string("palette.group.panels", is_de).into(),
                dispatch_action: Some(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab.id() }) }),
                action: None,
                category: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string(),
                description: None,
                group: shell_chrome_string("palette.group.windows", is_de).into(),
                dispatch_action: None,
                action: Some(format!("window:{}", kind.id)),
                category: None,
            });
        }
        items.extend(self.docs.command_search_items(&self.chrome, &self.plugins, &self.prefs));
        let panel = self.plugins.space_mode && panel_state_from_view(&session.view_state).ok().flatten().is_some();
        if panel {
            let controller_id = self.plugins.host_controller_id().unwrap_or_default();
            for program in self.prefs.build_space_workflows(&self.plugins) {
                let breadcrumb = self.plugins.plugins.iter().find_map(|plugin| plugin.manifest.apps.iter().find(|app| app.id == program.app_id)).map(|app| shell_navbar_title(app, &self.prefs.terminology_id)).unwrap_or_else(|| program.breadcrumb.join(" · "));
                items.push(SearchPaletteItem {
                    id: format!("spawn.{}", program.plugin_id),
                    label: format!("{} {breadcrumb}", shell_chrome_string("palette.spawnPrefix", is_de)),
                    description: None,
                    group: shell_chrome_string("palette.group.catalogue", is_de).into(),
                    dispatch_action: Some(ActionDescriptor { controller_id: controller_id.clone(), action: "spawnApp".into(), args: crate::action_args_json!({ "pluginId": program.plugin_id }) }),
                    action: None,
                    category: None,
                });
            }
            for (id, action, label_key, group_key) in
                [("studio.undo", "undo", "palette.undo", "palette.group.hostApp"), ("studio.redo", "redo", "palette.redo", "palette.group.hostApp"), ("studio.home", "goHome", "palette.goHome", "palette.group.navigation")]
            {
                items.push(SearchPaletteItem {
                    id: id.into(),
                    label: shell_chrome_string(label_key, is_de).into(),
                    description: None,
                    group: shell_chrome_string(group_key, is_de).into(),
                    dispatch_action: Some(ActionDescriptor { controller_id: controller_id.clone(), action: action.into(), args: None }),
                    action: None,
                    category: None,
                });
            }
        }
        items
    }

    /// 🔍️ The palette's visible rows — ranked by the SAME algorithm, fields, weights, threshold and
    /// limit React's `UISearch` passes `rankFuzzyItems` (`🔎️ShellSearch/🟦️.tsx`): `label` ×2,
    /// `description` ×1, the group heading ×0.5, threshold `0.4`, limit `20`.
    pub(crate) fn filtered_search_items(&self) -> Vec<SearchPaletteItem> {
        rank_fuzzy_items(
            self.build_search_items(),
            &self.commands.search_query,
            &[(|item: &SearchPaletteItem| Some(item.label.as_str()), 2.0), (|item: &SearchPaletteItem| item.description.as_deref(), 1.0), (|item: &SearchPaletteItem| Some(item.group.as_str()), 0.5)],
            FUZZY_SEARCH_THRESHOLD,
            FUZZY_SEARCH_LIMIT,
        )
    }

    /// 🔎️ One bounded command-palette layout authority, consumed unchanged by paint, hit
    /// publication, accessibility and input routing. Its dimensions are React's authored
    /// `CommandDialog` contract: `sm:max-w-lg`, `h-large`, `max-h-layout-command`, group `p-single`,
    /// heading `py-single`, and item `py-tiny`.
    pub(crate) fn command_palette_plan(&self, theme: &Theme, width: f32, height: f32) -> Option<ShellPalettePlan> {
        let kind = match self.input.overlay_state {
            OverlayState::Search => ShellPaletteKind::Search,
            OverlayState::Find => ShellPaletteKind::Find,
            _ => return None,
        };
        let (query, placeholder, empty, rows, selected) = match kind {
            ShellPaletteKind::Search => {
                let rows = self.filtered_search_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.group)).collect::<Vec<_>>();
                (self.commands.search_query.clone(), shell_chrome_string("search.placeholder", self.prefs.locale_id == "de").to_string(), shell_chrome_string("search.empty", self.prefs.locale_id == "de").to_string(), rows, self.commands.search_selected)
            }
            ShellPaletteKind::Find => {
                let rows = self.commands.filtered_find_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.category.unwrap_or_default())).collect::<Vec<_>>();
                (self.commands.find_query.clone(), shell_chrome_string("find.placeholder", self.prefs.locale_id == "de").to_string(), shell_chrome_string("find.empty", self.prefs.locale_id == "de").to_string(), rows, self.commands.find_selected)
            }
        };
        let inset = theme.control_height;
        let dialog_width = SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX.min((width - inset * 2.0).max(1.0));
        let input_height = theme.navbar_height;
        let body_line = theme.font_size_body * 1.5;
        let small_line = theme.font_size_small * (10.0 / 7.0);
        let row_padding = theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING;
        let list_max = theme.gap_standard * SHELL_PALETTE_LIST_MAX_UI_SPACING;
        let mut groups = Vec::<String>::new();
        for (_, _, _, _, group) in &rows {
            if !groups.iter().any(|existing| existing == group) {
                groups.push(group.clone());
            }
        }
        let mut specs = Vec::<(Option<(usize, String, String, Option<String>, bool)>, Option<String>, f32)>::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 2);
        if rows.is_empty() {
            specs.push((None, None, body_line + theme.gap_standard * SHELL_PALETTE_MEDIUM_UI_SPACING * 2.0));
        } else {
            for group in groups {
                if !group.is_empty() && specs.len() < SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 1 {
                    specs.push((None, Some(group.clone()), small_line + theme.gap_standard * 3.0));
                }
                for (index, item_id, label, description, row_group) in rows.iter().filter(|(_, _, _, _, row_group)| row_group == &group) {
                    if specs.len() >= SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 1 {
                        break;
                    }
                    let height = body_line + description.as_ref().map_or(0.0, |_| small_line) + row_padding * 2.0;
                    specs.push((Some((*index, item_id.clone(), label.clone(), description.clone(), *index == selected)), None, height));
                }
                if let Some(last) = specs.last_mut() {
                    last.2 += theme.gap_standard;
                }
            }
        }
        let natural_list_height = specs.iter().map(|(_, _, height)| *height).sum::<f32>();
        let viewport_list_height = list_max.min((height - inset * 2.0 - input_height).max(1.0)).min(natural_list_height);
        let dialog_height = input_height + viewport_list_height;
        let dialog = Rect::new((width - dialog_width) * 0.5, (height - dialog_height) * 0.5, dialog_width, dialog_height);
        let list_top = dialog.y + input_height;
        let list_bottom = dialog.y + dialog.h;
        let selected_bounds = specs
            .iter()
            .scan(0.0, |y, (row, _, height)| {
                let top = *y;
                *y += *height;
                Some((row.as_ref().is_some_and(|(_, _, _, _, selected)| *selected), top, *y))
            })
            .find(|(selected, _, _)| *selected);
        let scroll = selected_bounds
            .map(|(_, top, bottom)| {
                if bottom > viewport_list_height {
                    bottom - viewport_list_height
                } else if top < 0.0 {
                    top
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0)
            .clamp(0.0, (natural_list_height - viewport_list_height).max(0.0));
        let mut entries = Vec::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY);
        entries.push(ShellPalettePlanEntry::Input { rect: Rect::new(dialog.x, dialog.y, dialog.w, input_height), query, placeholder });
        let mut y = list_top - scroll;
        for (row, group, entry_height) in specs {
            let rect = Rect::new(dialog.x + theme.gap_standard, y, (dialog.w - theme.gap_standard * 2.0).max(1.0), entry_height);
            y += entry_height;
            if rect.y < list_top || rect.y + rect.h > list_bottom {
                continue;
            }
            match (row, group) {
                (Some((index, item_id, label, description, selected)), _) => entries.push(ShellPalettePlanEntry::Row { rect, index, item_id, label, description, selected }),
                (None, Some(label)) => entries.push(ShellPalettePlanEntry::Group { rect, label }),
                (None, None) => entries.push(ShellPalettePlanEntry::Empty { rect, label: empty.clone() }),
            }
        }
        let close_size = input_height.min(theme.control_height);
        entries.push(ShellPalettePlanEntry::Close { rect: Rect::new(dialog.x + dialog.w - close_size, dialog.y, close_size, close_size), label: shell_chrome_string("common.close", self.prefs.locale_id == "de").to_string() });
        Some(ShellPalettePlan { kind, dialog, list: Rect::new(dialog.x, list_top, dialog.w, viewport_list_height), entries })
    }

    //#region ⌨️ShellShortcutDispatch
    /// 🪟️ Closes one window wherever it sits — the ONE close lane React's `closeWindow(windowId)`
    /// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1475-1485`) is: the tab's own chip, the `mod+shift+w` chord
    /// and any future caller collapse the layout by the same rule, refocus `children[0]`, and tear
    /// down whatever the window was hosting.
    pub(crate) fn close_dock_window(&mut self, window_id: &str) -> bool {
        if self.dock.dock.window_kind_id(window_id).is_none() {
            return false;
        }
        if !crate::interpreter::request_ui_document_close(window_id) {
            return false;
        }
        if !self.dock.dock.close_window(window_id) {
            return false;
        }
        self.windows.window_icon_overrides.remove(window_id);
        self.windows.window_title_overrides.remove(window_id);
        self.windows.active_window_id = self.dock.dock.active_window_id.clone();
        crate::scenes::request_canvas_pointer_gesture_cancel_for_window(window_id);
        crate::scenes::cancel_scene_list_transfer_for_window(window_id);
        self.teardown_spawned_window(window_id);
        self.dock.persist_dock_layout();
        self.windows.owe_window_topology_refresh(&mut self.settle);
        true
    }

    /// 🪶️ Destroys the plugin instance a closed window was hosting and drops its panel entry —
    /// React's `onWindowClose` (`🏛️ShellHost/🟦️.tsx:10533-10555`). Without it the guest instance ran
    /// forever behind a window that no longer exists (the teardown gap
    /// REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT documented). A window that hosts no spawned app is
    /// simply nothing to tear down.
    pub(super) fn teardown_spawned_window(&mut self, window_id: &str) {
        let Some(session) = self.docs.session.as_ref() else {
            return;
        };
        let Ok(Some(mut panel)) = panel_state_from_view(&session.view_state) else {
            return;
        };
        let Some(closed) = take_spawned_entry(&mut panel, window_id) else {
            return;
        };
        if let Ok(json) = panel_json(&panel) {
            if let Some(session) = self.docs.session.as_mut() {
                session.view_state.panel_json = Some(json);
            }
        }
        if let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == closed.plugin_id) {
            program.destroy_app(closed.instance_id);
            debug_log(&format!("[TRACE] wgpu-shell closed window {window_id} destroyed spawned {} instance {}", closed.plugin_id, closed.instance_id));
        }
    }

    /// 🪟️ `mod+shift+w` — closes the active window through [`Self::close_dock_window`], last window of
    /// the mode included.
    pub(super) fn close_active_window(&mut self) -> bool {
        let Some(window_id) = self.windows.active_window_id.clone() else {
            return false;
        };
        self.close_dock_window_and_arm(&window_id, "ui.window.close")
    }

    pub(crate) fn close_dock_window_and_arm(&mut self, window_id: &str, control_id: &str) -> bool {
        if !self.close_dock_window(window_id) {
            return false;
        }
        self.docs.arm_control_command(control_id, Some(serde_json::json!({ "windowId": window_id })), &mut self.settle, &self.plugins, &self.prefs);
        true
    }

    /// ⌨️ Applies the shell verbs that are pure chrome-state transitions (no guest round-trip, no os
    /// command) — the synchronous half of the one [`SHELL_SHORTCUT_ROWS`] table. Returns whether the key
    /// was consumed; an unconsumed key falls through to the rest of `handle_keyboard` untouched.
    pub(super) fn apply_sync_shell_shortcut(&mut self, shortcut: ShellShortcut, palette_open: bool, input: &mut InputState<ActionDescriptor>) -> bool {
        match shortcut {
            ShellShortcut::ToggleSearch => {
                self.commands.search_open = !self.commands.search_open;
                self.commands.find_open = false;
                self.input.overlay_state = if self.commands.search_open { OverlayState::Search } else { OverlayState::None };
                self.commands.search_selected = 0;
                if self.commands.search_open {
                    let query = self.commands.search_query.clone();
                    self.commands.set_palette_query(ShellPaletteKind::Search, query, input, &mut self.input);
                } else {
                    self.commands.close_command_palette(input, &mut self.input);
                }
                debug_log(&format!("[TRACE] wgpu-shell palette chord search-open={} overlay={:?} focused={:?}", self.commands.search_open, self.input.overlay_state, input.focused_id));
                true
            }
            ShellShortcut::ToggleFind => {
                self.commands.find_open = !self.commands.find_open;
                self.commands.search_open = false;
                self.input.overlay_state = if self.commands.find_open { OverlayState::Find } else { OverlayState::None };
                self.commands.find_selected = 0;
                if self.commands.find_open {
                    let query = self.commands.find_query.clone();
                    self.commands.set_palette_query(ShellPaletteKind::Find, query, input, &mut self.input);
                } else {
                    self.commands.close_command_palette(input, &mut self.input);
                }
                true
            }
            ShellShortcut::NavBack => {
                if palette_open {
                    return false;
                }
                self.opening.uri_index = self.opening.uri_index.saturating_sub(1);
                self.opening.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::NavForward => {
                if palette_open {
                    return false;
                }
                if self.opening.uri_index + 1 < self.opening.uri_history.len() {
                    self.opening.uri_index += 1;
                }
                self.opening.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::NavUp => {
                if palette_open {
                    return false;
                }
                let uri = self.opening.shell_uri(&self.docs);
                if let Some(parent) = uri.rsplit_once('/').map(|(parent, _)| parent.to_string()) {
                    if !parent.is_empty() {
                        self.opening.push_uri(parent);
                    }
                }
                self.opening.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::TogglePanelAnchor(anchor) => {
                if palette_open {
                    return false;
                }
                self.dock.toggle_panel_anchor(anchor)
            }
            ShellShortcut::CloseWindow => !palette_open && self.close_active_window(),
            ShellShortcut::FocusWindow => !palette_open && self.dock.focus_active_window(&self.windows),
            ShellShortcut::NewWindow => !palette_open && self.dock.open_active_window_in_new_window(&mut self.settle, &self.docs, &mut self.windows),
            ShellShortcut::ModeStep(_) | ShellShortcut::SurfaceRole(_) | ShellShortcut::TimeTravel(_) => false,
        }
    }
    //#endregion ⌨️ShellShortcutDispatch
}

//#endregion ShellInput

