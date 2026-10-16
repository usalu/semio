//! 🚶️ `shell-session` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
/// 🔎️ The reserved body key of the per-window SEARCH pane. React mounts it as a second `Pane` fed by
/// the SAME `WindowEngagement` (`windowEngagementToSearchSpec`, `🛠️ShellHelpers/🟦️.tsx:1826`), so it
/// has no `UiRefreshSection` of its own and takes the engagements key with its own suffix.
pub(crate) const WINDOW_SEARCH_BODY_KEY: &str = "framework.section.engagements.search";

/// 🔀️ Reserved retained surface suffix for the World Projection pane's Tree body.
pub(crate) const WINDOW_PROJECTION_BODY_KEY: &str = "framework.worldOrbit.projection.body";

//#endregion 🎬️WindowActionsAndSearchPanes
//#region ⌨️WindowScope
/// 🪟️ One tab stack of a seeded mode layout, in layout order — the Rust twin of React's
/// `WindowScopeStackV1` (`🏛️ShellHost/⌨️window-scope/🟦️.ts`), driven by the same fixture
/// (`🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WindowScopeStackV1 {
    pub window_ids: Vec<String>,
    pub active_window_id: Option<String>,
}

/// 🪟️ Flattens a dock tree into its tab stacks, in layout order — twin of `modeLayoutStacksV1`.
pub(crate) fn mode_layout_stacks_v1(node: &crate::dock::DockNode) -> Vec<WindowScopeStackV1> {
    match node {
        crate::dock::DockNode::Stack { windows, active } => {
            let window_ids: Vec<String> = windows.iter().map(|tab| tab.window_id.clone()).collect();
            let active_window_id = if active.is_empty() { window_ids.first().cloned() } else { Some(active.clone()) };
            vec![WindowScopeStackV1 { window_ids, active_window_id }]
        }
        crate::dock::DockNode::Row(children) | crate::dock::DockNode::Column(children) => children.iter().flat_map(|(child, _)| mode_layout_stacks_v1(child)).collect(),
    }
}

/// 🪟️ The window a freshly seeded mode layout opens ACTIVE, or `None` when the seed must be left
/// alone — twin of `dockSeedActiveWindowIdV1`. A mode with windows always has exactly one active
/// window, so every window-scoped verb, chord badge and Actions rail has an owner before the user's
/// first click.
pub(crate) fn dock_seed_active_window_id_v1(stacks: &[WindowScopeStackV1], active_window_id: Option<&str>) -> Option<String> {
    let all: Vec<&String> = stacks.iter().flat_map(|stack| stack.window_ids.iter()).collect();
    if all.is_empty() {
        return None;
    }
    if let Some(active) = active_window_id {
        if all.iter().any(|id| id.as_str() == active) {
            return None;
        }
    }
    for stack in stacks {
        let candidate = stack.active_window_id.clone().or_else(|| stack.window_ids.first().cloned());
        if let Some(candidate) = candidate {
            if stack.window_ids.contains(&candidate) {
                return Some(candidate);
            }
        }
    }
    all.first().map(|id| (*id).clone())
}

//#endregion ⌨️WindowScope
//#region ⌨️ShellShortcutTable
/// ⌨️ The verb one [`SHELL_SHORTCUT_ROWS`] row asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellShortcut {
    ToggleSearch,
    ToggleFind,
    NavBack,
    NavForward,
    NavUp,
    TogglePanelAnchor(PanelAnchor),
    CloseWindow,
    FocusWindow,
    NewWindow,
    ModeStep(i32),
    SurfaceRole(semio_framework::manifest::AppRole),
    /// ⏪️ A reserved history-edit verb — answered only while `HistoryPatch.timeTravel` offers it (see `⏪️time-travel`).
    TimeTravel(TimeTravelVerb),
}

impl ShellShortcut {
    /// ⌨️ Whether this verb needs the async dispatch funnel (a guest round-trip or an os command) —
    /// `false` means `handle_keyboard`'s synchronous chrome-state arms answer it.
    pub(crate) fn is_async(self) -> bool {
        matches!(self, ShellShortcut::ModeStep(_) | ShellShortcut::SurfaceRole(_) | ShellShortcut::TimeTravel(_))
    }
}

//#endregion ⌨️ShellShortcutTable
impl<'ctx> SessionCtx<'ctx> {
    /// 🎛️ One command-category leaf's retained document — the shell's OWN publication, minted through
    /// the same `publish_window_measures` ingress every window overlay uses (revision-keyed, so an
    /// unchanged category pays no ingress). This is what makes the `bottom-middle` Command anchor a real
    /// painted panel rather than a builder with no reader.
    pub(super) fn publish_command_panel_document(&mut self, tab_id: &str, category_id: &str) -> Result<Option<UiDocumentLease>, String> {
        let node = self.build_command_category_ui(category_id);
        let records = panel_ui_records(tab_id, &node)?;
        self.windows.publish_surface_records(tab_id, records).map(Some)
    }

    /// 🧾️ One shell-owned leaf's retained document, routed by leaf id to its builder and minted through
    /// {@link panel_ui_records} + {@link ShellState::publish_surface_records} — the same revision-keyed
    /// ingress every window overlay uses, so an unchanged body pays no ingress. `None` for a leaf whose
    /// builder has nothing to show (React's own "no rows, no tree" case).
    pub(crate) fn publish_shell_panel_document(&mut self, tab_id: &str) -> Result<Option<UiDocumentLease>, String> {
        if let Some(category) = tab_id.strip_prefix(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX).map(ToOwned::to_owned) {
            return self.publish_command_panel_document(tab_id, &category);
        }
        let node = match tab_id {
            FRAMEWORK_DISPLAY_WINDOWS_TAB_ID => self.docs.build_display_windows_ui(&self.prefs),
            FRAMEWORK_DISPLAY_LAYOUT_TAB_ID => self.dock.build_display_layout_ui(&self.prefs, &self.docs),
            FRAMEWORK_SETTINGS_GENERAL_TAB_ID => self.docs.build_settings_general_ui(&self.chrome, &self.prefs),
            FRAMEWORK_SETTINGS_THEME_TAB_ID => self.input.build_settings_theme_ui(&self.chrome, &self.prefs),
            FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID => self.input.build_settings_keybindings_ui(&self.chrome, &self.prefs),
            FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID => self.prefs.build_settings_default_apps_ui(&self.plugins),
            FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID => self.prefs.build_settings_conflicts_ui(),
            FRAMEWORK_MARKETPLACE_TAB_ID => self.build_marketplace_ui(),
            FRAMEWORK_SYNC_PANEL_TAB_ID => self.sync.build_sync_attach_ui(&self.prefs),
            FRAMEWORK_TASK_MANAGER_PANEL_ID => self.opening.build_task_manager_ui(&self.prefs),
            FRAMEWORK_CHAT_PANEL_ID => self.directory.build_agent_chat_ui(&self.chrome, &self.prefs),
            crate::hub_connection::FRAMEWORK_HUB_PANEL_ID => crate::hub_connection::build_hub_workspace_ui(&self.directory.hub_workspace, self.prefs.active_locale()),
            other => match other.strip_prefix(FRAMEWORK_TOOL_PANEL_TAB_PREFIX) {
                Some(tool_id) => self.windows.build_tool_panel_ui(tool_id, &self.prefs, &self.docs),
                None => return Ok(None),
            },
        };
        let records = if ui_node_is_windowed(&node) { panel_ui_scroll_records(tab_id, &node)? } else { panel_ui_records(tab_id, &node)? };
        self.windows.publish_surface_records(tab_id, records).map(Some)
    }

    /// ♻️ Replaces one mounted host-owned retained panel after its local authority changes, without
    /// asking the guest to refresh unrelated bodies. Retirement admission is reserved before the
    /// successor is published; the exact prior lease enters that reservation before the map swaps.
    pub(crate) fn republish_shell_panel_document(&mut self, tab_id: &str) -> Result<(), String> {
        if !self.docs.panel_documents.contains_key(tab_id) {
            return Ok(());
        }
        let Some(admission) = self.docs.closing_documents.try_reserve_admission() else {
            return Err("shell: document retirement registry refused the exact prior owner".to_string());
        };
        let Some(document) = self.publish_shell_panel_document(tab_id)? else {
            let previous = self.docs.panel_documents.remove(tab_id);
            if let Some(previous) = previous {
                self.docs.closing_documents.admit_reserved(admission, previous);
                self.docs.drain_retained_document_arenas();
            }
            return Ok(());
        };
        let previous = self.docs.panel_documents.remove(tab_id).expect("mounted shell panel retains its exact prior owner");
        self.docs.closing_documents.admit_reserved(admission, previous);
        self.docs.panel_documents.insert(tab_id.to_string(), document);
        self.docs.drain_retained_document_arenas();
        Ok(())
    }

    /// 🎛️ One category leaf's body — the wgpu twin of React's `buildCommandCategoryTree`
    /// (`🛠️ShellHelpers/🟦️.tsx`): the STAGED FORM section for the expanded command first (so a bottom
    /// anchor's reversed section order paints it above the list, exactly as React's does), then the
    /// list of every other command in the category.
    ///
    /// 🎛️ This is [`ShellState::expanded_command_id`]'s reader — the gap `📓️w1c` recorded. React's
    /// `autoExpandedSingleton` rule is honoured too: a category with exactly one arg-carrying command
    /// opens that command's form with no expansion write at all.
    pub(crate) fn build_command_category_ui(&self, category_id: &str) -> UiNode {
        let is_de = self.prefs.locale_id == "de";
        let terminology = self.prefs.active_terminology();
        let locale = self.prefs.active_locale();
        let entries: Vec<ResolvedCommand> = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().filter(|entry| entry.definition.in_palette && entry.definition.category == category_id).collect();
        let arg_carrying: Vec<&ResolvedCommand> = entries.iter().filter(|entry| !entry.definition.args.is_empty()).collect();
        let expanded_key = match self.commands.expanded_command_id.as_deref() {
            Some(key) if arg_carrying.iter().any(|entry| command_address_stable_key(&entry.address) == key) => Some(key.to_string()),
            _ => (arg_carrying.len() == 1).then(|| command_address_stable_key(&arg_carrying[0].address)),
        };
        let mut sections: Vec<UiTreeSectionNode> = Vec::new();
        if let Some(expanded) = expanded_key.as_ref().and_then(|key| entries.iter().find(|entry| &command_address_stable_key(&entry.address) == key)) {
            let key = command_address_stable_key(&expanded.address);
            let staged = self.commands.staged_command_args.get(&key).cloned().unwrap_or_default();
            let staged_dsl = semio_framework_value::DslValue::from(Value::Object(staged.clone()));
            let effective_dsl = semio_framework::effective_action_args(&expanded.definition.args, &staged_dsl, None);
            let missing = !semio_framework::unresolved_action_args(&expanded.definition.args, &effective_dsl).is_empty();
            let effective = Value::from(&effective_dsl).as_object().cloned().unwrap_or_default();
            let mut rows = Vec::new();
            for arg in &expanded.definition.args {
                rows.push(staged_command_arg_row(&key, arg, effective.get(&arg.id), &self.input.interaction_selection, terminology, locale));
            }
            let element_key = key.replace(':', ".");
            let toolbar = UiStackNode {
                direction: "horizontal".into(),
                gap: None,
                padding: None,
                id: Some(format!("command.category.{category_id}.form.toolbar")),
                children: vec![
                    UiNode::Button(UiButtonNode {
                        id: Some(format!("command-{element_key}-execute")),
                        icon_id: IconName::Check,
                        label: Label::data(shell_chrome_string("common.execute", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "executeStagedCommand".into(), args: crate::action_args_json!({ "value": key.clone() }) },
                        style: None,
                        presence: UiPresence { state: if missing { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                        menu: None,
                    }),
                    UiNode::Button(UiButtonNode {
                        id: Some(format!("command-{element_key}-reset")),
                        icon_id: IconName::Undo,
                        label: Label::data(shell_chrome_string("common.reset", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "resetStagedCommand".into(), args: crate::action_args_json!({ "value": key.clone() }) },
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }),
                ],
                activate: None,
                drop_action: None,
                drop_overlay: None,
                presence: UiPresence::default(),
                menu: None,
            };
            sections.push(UiTreeSectionNode {
                id: format!("command.category.{category_id}.form"),
                label: Some(Label::data(expanded.definition.label.resolve(terminology, locale))),
                default_open: None,
                header_toolbar: Some(toolbar),
                presence: UiPresence::default(),
                items: rows,
                window: None,
            });
        }
        let list: Vec<UiTreeItemNode> = entries.iter().filter(|entry| expanded_key.as_deref() != Some(command_address_stable_key(&entry.address).as_str())).map(|entry| self.prefs.build_command_panel_row(entry)).collect();
        if !list.is_empty() {
            sections.push(UiTreeSectionNode { id: "command.category.list".into(), label: None, default_open: None, header_toolbar: None, presence: UiPresence::default(), items: list, window: None });
        }
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }
}

//#endregion ActionPanelAndUtilities

