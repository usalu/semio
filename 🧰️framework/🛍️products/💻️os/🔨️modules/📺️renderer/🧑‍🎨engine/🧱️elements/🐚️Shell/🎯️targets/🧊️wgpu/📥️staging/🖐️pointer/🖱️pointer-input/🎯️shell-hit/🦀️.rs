//! 🎯️ Shell hit: the long `handle_shell_hit` that routes one pointer hit through the shell. See [`super`].

use super::*;

//#region ShellInput
impl<'ctx> PointerCtx<'ctx> {
    /// 🪟️ The five pane chips answer FIRST and by decode, not by prefix: their ids carry the window
    /// instance in the middle (React's `framework.window.<segment>.<pane>.<verb>`), so a
    /// `starts_with` arm cannot name them. See [`WindowPaneChip::control_id`].
    ///
    /// 🔼️ A `Select` popup's scroll chevron (`{select}.scroll.up`/`.down`). Answered BEFORE the
    /// `match` for the same reason the pane chips are: the id is decoded, not prefixed, and the
    /// press handler cannot clamp — it only arms a direction that the popup's own painter prices
    /// in `select_scroll_step` pixels and clamps with `select_clamped_scroll`, because only the
    /// painter holds the item count and the collision-resolved popup height
    /// (ticket 26/09/17 packet W15a; the chevrons painted and did nothing before it).
    ///
    /// 👁️✏️ The surface-role group. Switching roles is a TRANSACTIONAL session switch, not a
    /// view-state flip: the successor instance is created before the predecessor is retired,
    /// so no render ever observes two live instances of one plugin.
    ///
    /// 🛑️ The World3d compute cancel — dispatches whatever id the surface's own status
    /// document named, never a verb this shell knows.
    ///
    /// 🔀️ The pane's own Projection chip — React's `framework.worldOrbit.projection.<segment>.pane.fold`,
    /// ONE id on both sides of the fold (`Pane`'s `chromeToggleId = toggleId ?? childElementId(id,
    /// "pane", "fold")`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10006`), flipping the pane's own local
    /// `folded` state and nothing else.
    ///
    /// 🪟️ React gates the cap on `canMaximize` — a mode with ONE window has nothing to
    /// maximize away from, exactly as `focus_active_window` already refuses there.
    ///
    /// 🖥️ React's `applyNamedLayout` searches `[...builtinLayouts, ...userLayouts]`
    /// (`📌️ChromePanels/🟦️.tsx`'s `useNamedLayoutHost`), so a layout the user SAVED resolves
    /// exactly like one the app declared. This arm used to search the app's declarations
    /// alone, which is why a restored `namedLayouts` entry was inert.
    ///
    /// 🎓️ The introduction's own four controls, answered HERE rather than by a rect test inside
    /// the painter — React wires them as ordinary `onClick`/`onPointerDown` handlers on real
    /// elements, and routing them through the one press funnel is what makes the veil able to
    /// swallow a press instead of letting it through to the surface below.
    ///
    /// 📱️ The mobile panel's flattened tab row selects a path in the ONE mobile panel instead
    /// of an anchor's, and a tool leaf arms its tool exactly as the desktop row does.
    ///
    /// 📑️ A panel tab is addressed by React's own BARE id (`framework.panel.artifact`), so the
    /// anchor is derived here from whichever anchor's tab tree carries it — exactly as React
    /// derives it from the tab's own `group` — instead of being carried in the control id.
    ///
    /// 🖱️ Group rows (`menu.group.<category>`, but any row with `children`) open their submenu on
    /// click, not just hover — matched by path (not a flat top-level id scan) so nested rows work.
    ///
    /// 🍿️ A row the surface answers ITSELF never reaches the guest — React's
    /// `dispatchTextEditorMenu` short-circuits its own `localActions` the same way.
    ///
    /// 🔀️ The picked value is MERGED into whatever the node authored, never
    /// substituted for it — see `merge_committed_args` (packet W15d).
    pub async fn handle_shell_hit(&mut self, hit: &HitTarget<ActionDescriptor>, input: &InputState<ActionDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        if let Some((window_id, chip)) = self.dock.window_pane_chip_target(id, &self.docs) {
            self.dock.toggle_window_pane_chip(&window_id, chip, &self.surfaces, &mut self.docs, &mut self.windows)?;
            return Ok(true);
        }
        if semio_framework_ui_wgpu_controls::select::arm_select_scroll(&mut self.input.scroll_offsets, id) {
            return Ok(true);
        }
        if let Some((kind, index)) = ShellPaletteKind::row_index(id) {
            match kind {
                ShellPaletteKind::Search => self.activate_search_item(index).await?,
                ShellPaletteKind::Find => self.activate_find_item(index).await?,
            }
            return Ok(true);
        }
        match id {
            "framework.hub.scrim" => return Ok(true),
            "framework.hub.signIn" => {
                self.opening.push_uri("/hub".to_string());
                self.as_actions().apply_shell_uri("/hub").await?;
                return Ok(true);
            }
            "ui.nav.back" => {
                if self.opening.uri_index > 0 {
                    self.opening.uri_index -= 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.opening.uri_index + 1 < self.opening.uri_history.len() {
                    self.opening.uri_index += 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.opening.shell_uri(&self.docs);
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.opening.push_uri(parent);
                    }
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                let open = matches!(&self.input.overlay_state, OverlayState::Dropdown(id) if id == "example");
                self.input.overlay_state = if open { OverlayState::None } else { OverlayState::Dropdown("example".to_string()) };
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.roles.") => {
                let requested = match id.trim_start_matches("playground.navbar.roles.") {
                    "editor" => semio_framework::manifest::AppRole::Editor,
                    "viewer" => semio_framework::manifest::AppRole::Viewer,
                    _ => return Ok(true),
                };
                self.as_actions().switch_to_session_role(requested).await?;
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.").to_string();
                self.as_window_actions().apply_navbar_mode(&mode_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.world3d.cancel::") => {
                let host_id = id.trim_start_matches("shell.world3d.cancel::").to_string();
                let action = world3d_cancel_affordance(self.surfaces.world3d_status.get(&host_id).map(String::as_str));
                let owner = self.surfaces.world3d_states.get(&host_id).map(|world| (world.controller_id.clone(), world.surface_id.clone()));
                if let (true, Some((controller_id, surface_id))) = (action.cancellable, owner) {
                    debug_log(&format!("[TRACE] shell world3d cancel {}", serde_json::json!({ "host": host_id, "surface": surface_id, "action": action.cancel_action })));
                    let mut args = action.cancel_args;
                    args.insert("surfaceId".to_string(), Value::String(surface_id));
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: action.cancel_action, args: Some(semio_framework::DslValue::from(Value::Object(args))) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("framework.utility.collection.") => {
                let collection_id = id.trim_start_matches("framework.utility.collection.");
                let expanded = self.windows.utility_collection_expanded.get(collection_id).copied().unwrap_or(false);
                self.windows.utility_collection_expanded.insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                if !self.docs.session_example_rows(&self.plugins, &self.prefs).iter().any(|row| row.control_id == id) {
                    return Ok(true);
                }
                let example_id = id.trim_start_matches("shell.example.");
                self.plugins.active_example_id = Some(example_id.to_string());
                self.input.overlay_state = OverlayState::None;
                if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "setActiveExample".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "exampleId": example_id }) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let activated = self.windows.engagement_activated.get(window_id).copied().unwrap_or(false);
                self.windows.engagement_activated.insert(window_id.to_string(), !activated);
                self.windows.engagement_expanded.insert(window_id.to_string(), !activated);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.windows.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.windows.measures_expanded.insert(window_id.to_string(), !expanded);
                if !expanded {
                    self.windows.engagement_activated.remove(window_id);
                    self.windows.engagement_expanded.insert(window_id.to_string(), false);
                }
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.commands.search_open = !self.commands.search_open;
                self.commands.find_open = false;
                self.input.overlay_state = if self.commands.search_open { OverlayState::Search } else { OverlayState::None };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.commands.find_open = !self.commands.find_open;
                self.commands.search_open = false;
                self.input.overlay_state = if self.commands.find_open { OverlayState::Find } else { OverlayState::None };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                self.dock.toggle_anchor(PanelAnchor::BottomLeft);
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                self.dock.toggle_anchor(PanelAnchor::TopLeft);
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                self.dock.toggle_anchor(PanelAnchor::TopRight);
                self.note_panel_toggle_command(id, "details").await?;
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                self.dock.toggle_anchor(PanelAnchor::BottomRight);
                self.note_panel_toggle_command(id, "settings").await?;
                return Ok(true);
            }
            "ui.panelToggle.chat" => {
                self.dock.toggle_anchor_tab(PanelAnchor::TopRight, FRAMEWORK_CHAT_PANEL_ID);
                self.note_panel_toggle_command(id, "chat").await?;
                return Ok(true);
            }
            "ui.panelToggle.command" => {
                self.dock.toggle_anchor_tab(PanelAnchor::BottomMiddle, FRAMEWORK_CATEGORY_COMMAND_ID);
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                self.as_actions().apply_os_command("os.toggleFullscreen", None).await?;
                return Ok(true);
            }
            "space.canvas.home" => {
                let controller_id = self.plugins.host_controller_id().unwrap_or_default();
                self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "goHome".into(), args: None }).await?;
                return Ok(true);
            }
            "space.canvas.back" => {
                let has_focused_instance = if let Some(session) = self.docs.session.as_ref() { panel_state_from_view(&session.view_state)?.is_some_and(|panel| panel.active_spawned_id.is_some()) } else { false };
                if has_focused_instance {
                    let controller_id = self.plugins.host_controller_id().unwrap_or_default();
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "closeFocusedInstance".into(), args: None }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".focus") => {
                if let Some((path, window_id)) = parse_dock_tab_action_id(id, "focus") {
                    self.dock.dock.set_stack_active(&path, window_id);
                    self.windows.active_window_id = Some(window_id.to_string());
                    let mut order = Vec::new();
                    dock_window_order(&self.dock.dock.root, &mut Vec::new(), &mut order);
                    if order.len() > 1 {
                        self.dock.dock.toggle_maximize(&path);
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".close") => {
                if let Some((_, window_id)) = parse_dock_tab_action_id(id, "close") {
                    let window_id = window_id.to_string();
                    self.as_session().close_dock_window_and_arm(&window_id, id);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(layout) = self.dock.named_layout_by_id(layout_id, &self.docs) {
                    self.dock.layout_override = Some(layout);
                    self.as_session().sync_dock();
                    self.windows.active_window_id = self.dock.dock.active_window_id.clone();
                    self.as_actions().note_control_command(id, Some(serde_json::json!({ "layoutId": layout_id }))).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                ({ let __arg0 = ActionDescriptor { controller_id: self.docs.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(), action: "setMode".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "modeId": mode_id }) }; self.as_actions().dispatch_action(__arg0) }).await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.appearance.") => {
                self.prefs.appearance_id = id.trim_start_matches("framework.settings.appearance.").to_string();
                return Ok(true);
            }
            UI_INTRODUCTION_VEIL_CONTROL_ID | UI_INTRODUCTION_SKIP_CONTROL_ID => {
                match self.docs.introduction_seen_key() {
                    Some(seen_key) => self.chrome.chrome_build.dismiss_introduction(&seen_key),
                    None => self.chrome.chrome_build.skip_introduction(),
                }
                return Ok(true);
            }
            UI_INTRODUCTION_NEXT_CONTROL_ID => {
                if let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) {
                    self.docs.chrome_tour_advance_current_step(&step, &mut self.chrome);
                }
                return Ok(true);
            }
            UI_INTRODUCTION_BACK_CONTROL_ID => {
                self.chrome.chrome_build.back_introduction();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.mobile.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.mobile.").to_string();
                let tabs = self.as_session().mobile_panel_tabs();
                self.dock.mobile_panel_path = mobile_panel_reconcile_path(&tabs, &[tab_id.clone()]);
                self.dock.mobile_panel_visible = true;
                self.arm_tool_for_panel_tab(&self.dock.mobile_panel_path.clone()).await?;
                return Ok(true);
            }
            "ui.mobilePanel.toggle" => {
                self.dock.mobile_panel_visible = !self.dock.mobile_panel_visible;
                return Ok(true);
            }
            id if self.dock.panel_tab_anchor(id).is_some() => {
                if let Some(anchor) = self.dock.panel_tab_anchor(id) {
                    let tab_id = id.to_string();
                    if hit.kind == HitKind::Toggle {
                        let was_visible = self.dock.anchor_open(anchor);
                        let was_path = self.dock.anchor_state(anchor).path.first().cloned();
                        self.dock.toggle_anchor_tab(anchor, &tab_id);
                        self.as_actions().journal_panel_selection(anchor, &tab_id, was_visible, was_path).await?;
                    } else {
                        self.select_panel_tab(anchor, &tab_id).await?;
                    }
                    self.arm_tool_for_panel_tab(&self.dock.anchor_state(anchor).path.clone()).await?;
                }
                return Ok(true);
            }
            id if self.input.context_menu.as_ref().is_some_and(|menu| {
                let mut prefix = Vec::new();
                context_menu_path_for_item_id(&menu.items, id, &mut prefix).is_some()
            }) =>
            {
                let disabled = self.input.context_menu.as_ref().is_some_and(|menu| {
                    let mut prefix = Vec::new();
                    context_menu_path_for_item_id(&menu.items, id, &mut prefix).and_then(|path| context_menu_item_at_path(&menu.items, &path)).is_some_and(|item| item.disabled)
                });
                if disabled {
                    return Ok(true);
                }
                let submenu_path = self.input.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    (!context_menu_item_at_path(&menu.items, &path)?.children.is_empty()).then_some(path)
                });
                if let Some(path) = submenu_path {
                    if let Some(menu) = self.input.context_menu.as_mut() {
                        menu.active = path;
                        menu.submenu_collapsed_at = None;
                    }
                    return Ok(true);
                }
                let action = self.input.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    context_menu_item_at_path(&menu.items, &path)?.action.clone()
                });
                let origin = self.input.context_menu.as_ref().map(|menu| (menu.x, menu.y));
                self.input.context_menu = None;
                if let Some(action) = action {
                    let claimed = origin.is_some_and(|(x, y)| self.input.claim_text_editor_menu_action(&action, x, y));
                    if !claimed {
                        self.as_actions().dispatch_action(action).await?;
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.input.collapsed_sections.get(&key).copied().unwrap_or(false);
                let next_open = !self.input.canonical_tree_open(section_id, !collapsed);
                self.input.collapsed_sections.insert(key, !next_open);
                self.input.set_canonical_tree_open(section_id, next_open);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                self.input.toggle_immediate_tree_item(item_id);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.input.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.input.open_selects.get(id).copied().unwrap_or(false);
                self.input.close_open_selects();
                self.input.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(action) = self.input.widget_maps.select_metas.get(select_id).cloned() {
                        semio_framework_ui_wgpu_controls::select::clear_select_scroll(&mut self.input.scroll_offsets, select_id);
                        self.input.open_selects.insert(select_id.to_string(), false);
                        let args = merge_committed_args(action.args.as_ref(), semio_framework_value::DslValue::String(value.to_string()));
                        self.as_actions().dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args }).await?;
                        return Ok(true);
                    }
                }
            }
            id if self.input.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, action)) = self.input.widget_maps.toggle_metas.get(id).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "pressed": !pressed }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "delta": -meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "delta": meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                if tree_row_has_chevron(input, item_id) {
                    self.input.toggle_immediate_tree_item(item_id);
                }
                self.input.queue_tree_selection(item_id, &mut self.settle);
                return Ok(false);
            }
            id if tree_drag_handle_item_id(id).is_some() => return Ok(true),
            _ => {}
        }
        Ok(false)
    }
}
//#endregion ShellInput
