//! 🕹️ `shell-input` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl ShellInputState {
    /// 🎨️ React-equivalent canonical theme tree with independently expandable virtualized groups.
    pub fn build_settings_theme_ui(&self, chrome: &ChromeState, prefs: &PrefsState) -> UiNode {
        self.build_settings_theme_editor_tree(&prefs.theme_document(chrome), prefs.locale_id == "de", chrome, prefs)
    }

    /// ⌨️ The `framework.settings.keybindings` leaf's body — the wgpu twin of React's
    /// `buildSettingsKeybindingsTree` (`📌️ChromePanels/🟦️.tsx`): one row per remappable shortcut,
    /// carrying the chord in force (through [`format_keybinding_shortcut`]), a conflict marker when a
    /// later row claims the same first chord, a capture control and a reset control.
    ///
    /// ⌨️ React captures the next chord from a live `keydown` on the capture button. The wgpu chord
    /// capture lives in {@link ShellState::keybinding_capture_control_id}: arming a row makes the next
    /// key event a chord write instead of a shell verb, which is the same one-gesture remap.
    pub fn build_settings_keybindings_ui(&self, chrome: &ChromeState, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let overrides = &chrome.chrome_build.preferences.keybinding_overrides;
        let mut rows = Vec::new();
        for row in shell_keybinding_rows(overrides) {
            let capturing = self.keybinding_capture_control_id.as_deref() == Some(row.control_id.as_str());
            let label = humanize_control_id(&row.control_id);
            let label = if row.conflicted { format!("{label} ({})", shell_chrome_string("settings.keybindings.conflict", is_de)) } else { label };
            let mut controls = vec![
                settings_text_row(&format!("{label} — {}", format_keybinding_shortcut(&row.keys))),
                UiNode::Button(UiButtonNode {
                    id: Some(format!("framework.settings.keybindings.capture.{}", row.control_id)),
                    icon_id: IconName::CircleDot,
                    label: Label::data(shell_chrome_string(if capturing { "settings.keybindings.pressKeys" } else { "settings.keybindings.capture" }, is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "captureKeybinding".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": row.control_id.clone() }) },
                    style: None,
                    presence: UiPresence { selected: capturing, ..UiPresence::default() },
                    menu: None,
                }),
            ];
            if row.overridden {
                controls.push(UiNode::Button(UiButtonNode {
                    id: Some(format!("framework.settings.keybindings.reset.{}", row.control_id)),
                    icon_id: IconName::RotateCcw,
                    label: Label::data(shell_chrome_string("settings.keybindings.reset", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "resetKeybinding".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": row.control_id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                }));
            }
            rows.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some(format!("framework.settings.keybindings.{}", row.control_id)),
                children: controls,
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("framework.settings.keybindings.panel".into()),
            children: vec![settings_section("framework.settings.keybindings.list", shell_chrome_string("settings.tab.keybindings", is_de), rows)],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }
}

//#endregion ShellLifecycle

