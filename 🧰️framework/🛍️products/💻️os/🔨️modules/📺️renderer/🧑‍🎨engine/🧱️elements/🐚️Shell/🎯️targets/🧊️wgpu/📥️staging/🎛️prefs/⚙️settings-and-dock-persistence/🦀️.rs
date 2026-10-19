//! 🎛️ `shell-prefs` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl PrefsState {
    /// 🚗️ The document edited by General: the live draft, else the selected custom or builtin.
    pub fn driver_document(&self, chrome: &ChromeState) -> OsUiDriver {
        self.driver_draft.clone().unwrap_or_else(|| resolve_ui_driver_document(&self.driver_id, &chrome.chrome_build.preferences.custom_drivers))
    }

    pub fn clear_driver_draft(&mut self, chrome: &mut ChromeState) {
        self.driver_draft = None;
        chrome.chrome_build.driver = resolve_ui_driver_chrome(&self.driver_id, &chrome.chrome_build.preferences.custom_drivers);
    }

    pub fn set_driver_field(&mut self, key: &str, value: &str, chrome: &mut ChromeState) -> bool {
        if !valid_ui_driver_axis_value(key, value) {
            return false;
        }
        let mut draft = self.driver_document(chrome);
        if !matches!(draft.config,DslValue::Object(_)) {
            draft.config = builtin_ui_driver_document("default").config;
        }
        if let DslValue::Object(entries)=&mut draft.config {
            if let Some((_,axis))=entries.iter_mut().find(|(name,_)|name==key){*axis=DslValue::String(value.to_owned());}else{entries.push((key.to_owned(),DslValue::String(value.to_owned())));}
        }
        chrome.chrome_build.driver = ui_driver_document_chrome(&draft);
        self.driver_draft = Some(draft);
        true
    }

    pub fn build_settings_driver_editor(&self, is_de: bool, chrome: &ChromeState) -> UiTreeSectionNode {
        let driver = self.driver_document(chrome);
        let fallback = builtin_ui_driver_document("default");
        let mut items = Vec::new();
        for (key, label_key, options) in SHELL_DRIVER_AXES {
            let value = ui_driver_axis(&driver, key).or_else(|| ui_driver_axis(&fallback, key)).unwrap_or_default();
            items.push(settings_tree_select_item_with(
                &format!("framework.settings.driver.{key}"),
                shell_chrome_string(*label_key, is_de),
                value,
                options.iter().map(|(value, key)| ((*value).to_string(), shell_chrome_string(*key, is_de).to_string())).collect(),
                "setDriverField",
                semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": *key }),
            ));
        }
        items.push(UiTreeItemNode {
            id: "framework.settings.driver.save.label".into(),
            label: Label::data(shell_chrome_string("common.name", is_de)),
            control: Some(UiControlNode::Input(UiInputNode {
                id: "framework.settings.driver.saveLabel".into(),
                input_kind: "text".into(),
                value: self.driver_save_label.clone(),
                placeholder: Some(Label::data(shell_chrome_string("settings.driver.savePlaceholder", is_de))),
                accessibility_label: None,
                commit: Some("blur".into()),
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "setDriverSaveLabel".into(), args: None },
                on_submit: None,
                on_abort: None,
                on_repeat_last: None,
                presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        items.push(UiTreeItemNode {
            id: "framework.settings.driver.save.action".into(),
            label: Label::data(shell_chrome_string("settings.driver.save", is_de)),
            control: Some(UiControlNode::Button(UiButtonNode {
                id: Some("framework.settings.driver.save".into()),
                icon_id: IconName::Save,
                label: Label::data(shell_chrome_string("settings.driver.save", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "saveDriver".into(), args: None },
                style: None,
                presence: UiPresence { state: if self.driver_save_label.trim().is_empty() { semio_framework_ui_declaration::ui::UiState::Disabled } else { semio_framework_ui_declaration::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            })),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
        if self.driver_id.starts_with("custom.") {
            items.push(UiTreeItemNode {
                id: "framework.settings.driver.delete.action".into(),
                label: Label::data(shell_chrome_string("settings.driver.delete", is_de)),
                control: Some(UiControlNode::Button(UiButtonNode {
                    id: Some("framework.settings.driver.delete".into()),
                    icon_id: IconName::Trash2,
                    label: Label::data(shell_chrome_string("settings.driver.delete", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "deleteDriver".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": self.driver_id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            });
        }
        let label = if self.driver_draft.is_some() { format!("{} ({})", shell_chrome_string("settings.tab.driver", is_de), shell_chrome_string("settings.driver.dirty", is_de)) } else { shell_chrome_string("settings.tab.driver", is_de).to_string() };
        UiTreeSectionNode { header_toolbar: None, id: "framework.settings.driver.editor".into(), label: Some(Label::data(label)), default_open: Some(false), presence: UiPresence::default(), items, window: None }
    }

    /// 🚗️ Every selectable driver id with its display label — the two built-ins plus whatever the
    /// preferences carry, React's `uiDriverList = [...builtinUiDrivers(), ...customDrivers]`.
    pub fn driver_rows(&self, chrome: &ChromeState) -> Vec<(String, String)> {
        let is_de = self.locale_id == "de";
        let mut rows = vec![("default".to_string(), shell_chrome_string("settings.driver.default", is_de).to_string()), ("compact".to_string(), shell_chrome_string("settings.driver.compact", is_de).to_string())];
        for (id, driver) in &chrome.chrome_build.preferences.custom_drivers {
            if !rows.iter().any(|(existing, _)| existing == id) {
                rows.push((id.clone(), driver.label.clone()));
            }
        }
        rows
    }

    /// 👁️✏️ The `framework.settings.default-apps` leaf's body — the wgpu twin of React's
    /// `buildSettingsDefaultAppsTree`: one row per (dialect × role) the loaded plugins can open, whose
    /// `Select` pins that pairing's default app (`"none"` clears it, React's `DEFAULT_APP_NONE_VALUE`),
    /// value encoded as `"<pluginId> <appId>"` exactly like React's `encodeDefaultAppValue`.
    pub fn build_settings_default_apps_ui(&self, plugins: &PluginState) -> UiNode {
        let is_de = self.locale_id == "de";
        let rows = plugins.default_app_rows();
        if rows.is_empty() {
            return UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("framework.settings.defaultApps.empty".into()),
                children: vec![settings_text_row(shell_chrome_string("settings.unavailable", is_de))],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            });
        }
        let items: Vec<UiNode> = rows
            .into_iter()
            .map(|(dialect, role, value, options)| {
                let mut choices = vec![(DEFAULT_APP_NONE_VALUE.to_string(), shell_chrome_string("settings.defaultApps.none", is_de).to_string())];
                choices.extend(options);
                settings_select_row_with(&format!("framework.settings.defaultApps.{dialect}.{role}"), &format!("{dialect} — {role}"), &value, choices, "setDefaultApp", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "row": format!("{dialect}/{role}") }))
            })
            .collect();
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("framework.settings.defaultApps.panel".into()),
            children: vec![settings_section("framework.settings.defaultApps.table", shell_chrome_string("settings.tab.defaultApps", is_de), items)],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// ⚖️ The `framework.settings.conflicts` leaf's body — the wgpu twin of React's
    /// `buildConflictsTree` (`📌️ChromePanels/🟦️.tsx`), now over a REAL projection: one row per OPEN
    /// conflict in [`ShellState::open_conflicts`], labelled `<kind> — <code> — <message>` off the
    /// conflict's own `ConflictKind` tag and worst `MutationMessage` (never parsed from prose), and
    /// React's own unavailable row only when there genuinely is no open conflict.
    ///
    /// ⚖️ React nests Accept/Discard as two inline buttons in the row's single control slot plus a
    /// `ConflictDiffPreview` behind `🔺️DiffViewHost`. Authored here as action-only nested leaves under
    /// React's own `….accept`/`….discard` ids; panel projection lifts that strip into one horizontal
    /// `Toolbar` of `Button`s so schema `rowCount` stays one. The selected row also owns a
    /// semantic DiffView Surface in its explicit detail relation; the empty local snapshot still
    /// produces React's additions-only preview of the incoming/degraded side.
    pub fn build_settings_conflicts_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        if self.open_conflicts.is_empty() {
            return display_panel_body(
                "framework.settings.conflicts.panel",
                vec![UiTreeSectionNode {
                    header_toolbar: None,
                    id: "framework.settings.conflicts.empty".into(),
                    label: Some(Label::data(shell_chrome_string("conflict.panel", is_de))),
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items: vec![UiTreeItemNode::base("framework.settings.conflicts.empty.row", Label::data(shell_chrome_string("settings.unavailable", is_de)))],
                    window: None,
                }],
            );
        }
        let items: Vec<UiTreeItemNode> = self
            .open_conflicts
            .iter()
            .map(|conflict| {
                let row_id = format!("framework.settings.conflicts.{}", conflict.id);
                let kind = shell_chrome_string(if conflict.quarantined { "conflict.quarantined" } else { "conflict.degraded" }, is_de);
                let selected = self.selected_conflict_id.as_deref() == Some(conflict.id.as_str());
                let detail = selected.then(|| {
                    let UiNode::ComponentScene(scene) = semio_framework_ui_declaration::ui::build_diff_view_scene(
                        format!("{row_id}.preview"),
                        "framework",
                        semio_framework_ui_declaration::ui::DiffViewScene { before: String::new(), after: conflict.preview_after.clone(), language: Some("json".into()), mode: Some("unified".into()), domain_id: Some(conflict.id.clone()) },
                    ) else {
                        unreachable!("DiffView builder returns a ComponentScene")
                    };
                    scene
                });
                let verb = |suffix: &str, label_key: &'static str, icon: IconName, accept: bool| UiTreeItemNode {
                    id: format!("{row_id}.{suffix}"),
                    label: Label::data(shell_chrome_string(label_key, is_de)),
                    icon_id: Some(icon),
                    action: Some(ActionDescriptor { controller_id: "framework".into(), action: "resolveConflict".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "conflictId": conflict.id.clone(), "accept": accept }) }),
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                };
                UiTreeItemNode {
                    id: row_id.clone(),
                    label: Label::data(format!("{kind} \u{2014} {} \u{2014} {}", conflict.code, conflict.message)),
                    icon_id: Some(IconName::TriangleAlert),
                    default_open: None,
                    action: Some(ActionDescriptor { controller_id: "framework".into(), action: "selectConflict".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "conflictId": conflict.id.clone() }) }),
                    items: Some(vec![verb("accept", "conflict.accept", IconName::Check, true), verb("discard", "conflict.discard", IconName::X, false)]),
                    detail,
                    presence: UiPresence { selected, ..UiPresence::default() },
                    ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                }
            })
            .collect();
        display_panel_body(
            "framework.settings.conflicts.panel",
            vec![UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.settings.conflicts.table".into(),
                label: Some(Label::data(shell_chrome_string("conflict.panel", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window: None,
            }],
        )
    }

    //#region 🧭️PanelAnchorAccessors
    /// ⚙️ The five framework Settings leaves in React's own order (`createFrameworkSettingsPanelTab`):
    /// general, theme, keybindings, default-apps, conflicts. The theme leaf is omitted outright when
    /// the boot descriptor locks the theme id — React's `locks.themeId` render gate, which removes the
    /// tab rather than disabling it.
    pub fn framework_settings_tabs(&self) -> Vec<DockTabNode> {
        let is_de = self.locale_id == "de";
        let mut tabs = vec![DockTabNode::leaf(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, shell_chrome_string("settings.tab.general", is_de), "settings-2", 0)];
        if semio_framework_os_renderer_wgpu_host_base::boot_locks().theme_id.is_empty() {
            tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_THEME_TAB_ID, shell_chrome_string("settings.tab.theme", is_de), "paintbrush", 1));
        }
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, shell_chrome_string("settings.tab.keybindings", is_de), "circle-dot", 2));
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID, shell_chrome_string("settings.tab.defaultApps", is_de), "app-window", 3));
        tabs.push(DockTabNode::leaf(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, shell_chrome_string("conflict.panel", is_de), "triangle-alert", 4));
        tabs
    }
    //#endregion 🧭️PanelAnchorAccessors
}

//#endregion ShellLifecycle

