//! 🪟️ Layout and window actions: layouts, display windows and conflict resolution. See [`super`].

use super::*;

/// 🔊️ Notice code of a focused or opened window announcement.
pub const WINDOW_FOCUS_ANNOUNCED_CODE: &str = "shell.window-focus.announced";

/// 🔊️ Notice code of a refused window focus.
pub const WINDOW_FOCUS_REFUSED_CODE: &str = "shell.window-focus.refused";

/// 🔊️ Notice code of a refused row control focus (the row holds no cell to type into, or no window presents it).
pub const ROW_CONTROL_FOCUS_REFUSED_CODE: &str = "shell.row-control-focus.refused";

impl<'ctx> ActionsCtx<'ctx> {
    /// 🎯️ Runs [`semio_framework::FOCUS_WINDOW_ACTION_ID`]: opens the window of the named kind when it is closed, raises it, takes the keyboard focus off any chrome control so the keyboard ring continues inside the window, and announces the result politely (the transient notice is a live status node of the accessibility tree) in the interface language. A kind the app never declared, a missing argument or a full topology queue is announced as a refusal and returned as the error.
    pub(super) fn framework_focus_window(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let is_de = self.prefs.locale_id == "de";
        let Some(window_kind_id) = action.args.as_ref().and_then(|args| args.get(semio_framework::FOCUS_WINDOW_ARG_KIND)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string) else {
            return Err(format!("{} needs the argument {}", semio_framework::FOCUS_WINDOW_ACTION_ID, semio_framework::FOCUS_WINDOW_ARG_KIND));
        };
        let fallback = action.args.as_ref().and_then(|args| args.get(semio_framework::FOCUS_WINDOW_ARG_FALLBACK)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        let kind_name = |ctx: &Self, kind_id: &str| ctx.docs.session.as_ref().and_then(|session| session.app.window_kinds.iter().find(|kind| kind.id == kind_id)).map_or_else(|| kind_id.to_string(), |kind| kind.label.resolve(ctx.prefs.active_terminology(), ctx.prefs.active_locale()).to_string());
        let device = self.focus_device();
        let Some(target_kind_id) = self.dock.resolve_window_kind_for_device(self.docs, &window_kind_id, fallback.as_deref(), device) else {
            let message = shell_chrome_string("window.focus.unavailable", is_de).replace("{name}", &kind_name(self, &window_kind_id)).replace("{device}", shell_chrome_string(device.vocab_key(), is_de));
            self.chrome.show_transient_notice(message, semio_framework::Severity::Warning, Some(WINDOW_FOCUS_REFUSED_CODE));
            return Err(format!("window kind {window_kind_id} is not part of the {} layout", device.id()));
        };
        let name = kind_name(self, &target_kind_id);
        match self.dock.focus_window_of_kind(self.docs, &target_kind_id, self.windows, self.settle) {
            Ok(outcome) => {
                self.input.accessibility_focused_control_id = None;
                let key = if matches!(outcome, WindowKindFocus::Opened { .. }) { "window.focus.opened" } else { "window.focus.raised" };
                self.chrome.show_transient_notice(shell_chrome_string(key, is_de).replace("{name}", &name), semio_framework::Severity::Info, Some(WINDOW_FOCUS_ANNOUNCED_CODE));
                Ok(())
            }
            Err(reason) => {
                self.chrome.show_transient_notice(shell_chrome_string("window.focus.refused", is_de).replace("{name}", &name), semio_framework::Severity::Warning, Some(WINDOW_FOCUS_REFUSED_CODE));
                Err(reason)
            }
        }
    }

    /// ✏️ Runs [`semio_framework::FOCUS_ROW_CONTROL_ACTION_ID`]: plans the first editable cell of the named tree row with the shared rule ([`semio_framework::plan_row_control_focus`]) and queues it as the pending keyboard focus, which the pointer context applies with the input state in hand. A row nobody presents, or one without an editable cell, is announced politely as a refusal in the interface language and returned as the error. A successful focus is not announced: the focused field announces itself through the accessibility tree.
    pub(super) fn framework_focus_row_control(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let is_de = self.prefs.locale_id == "de";
        let Some(row_id) = action.args.as_ref().and_then(|args| args.get(semio_framework::FOCUS_ROW_CONTROL_ARG_ROW)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string) else {
            return Err(format!("{} needs the argument {}", semio_framework::FOCUS_ROW_CONTROL_ACTION_ID, semio_framework::FOCUS_ROW_CONTROL_ARG_ROW));
        };
        let row = semio_renderer_wgpu_ui_engine::retained_ui_engine::retained_row_controls(&row_id);
        let chosen = row.as_ref().and_then(|row| {
            let controls: Vec<semio_framework::RowControl<'_>> = row.controls.iter().map(|control| semio_framework::RowControl { id: control.key.as_str(), kind: control.kind.as_str(), disabled: control.disabled }).collect();
            let key = semio_framework::plan_row_control_focus(&controls)?;
            row.controls.iter().find(|control| control.key == key).map(|control| PendingRowControlFocus { window_id: row.window_id.clone(), window_generation: row.window_generation, node_id: control.node_id, node_key: control.key.clone() })
        });
        match chosen {
            Some(target) => {
                self.input.pending_row_control_focus = Some(target);
                Ok(())
            }
            None => {
                self.chrome.show_transient_notice(shell_chrome_string("row.control.refused", is_de).to_string(), semio_framework::Severity::Warning, Some(ROW_CONTROL_FOCUS_REFUSED_CODE));
                Err(format!("row {row_id} holds no editable control"))
            }
        }
    }

    /// 📱️ The device class the shell presents right now: below the mobile breakpoint the one mobile layout, otherwise the user's layout preference (React `selectUiDevice`).
    fn focus_device(&self) -> FocusDevice {
        if self.chrome.mobile_panel_active() {
            FocusDevice::Mobile
        } else if self.chrome.chrome_build.preferences.ui_layout == "tablet" {
            FocusDevice::Tablet
        } else {
            FocusDevice::Desktop
        }
    }

    /// 🧱 Applies the chosen layout.
    pub(super) async fn framework_set_layout(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.chrome.chrome_build.preferences.ui_layout = if value == "tablet" { "tablet".to_string() } else { "desktop".to_string() };
            self.note_shell_setting_command("os.setLayout", Some(value)).await?;
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
        }
        Ok(())
    }

    /// 📛 Stores the label under which the current layout is saved.
    pub(super) async fn framework_set_layout_save_label(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.dock.layout_save_label = value.to_string();
        }
        Ok(())
    }

    /// 📋 Saves the current layout under the stored label.
    pub(super) async fn framework_save_current_layout(&mut self) -> Result<(), String> {
        let label = self.dock.layout_save_label.trim().to_string();
        if label.is_empty() {
            return Ok(());
        }
        if let Some(layout) = self.dock.current_named_layout(&label, &self.docs) {
            let id = layout.id.clone();
            self.dock.user_layouts.retain(|entry| entry.id != id);
            self.dock.user_layouts.push(layout);
            self.dock.layout_save_label.clear();
            self.dock.persist_named_layouts(&self.docs);
            self.note_shell_setting_command("os.saveNamedLayout", Some(&id)).await?;
        }
        Ok(())
    }

    /// 📐 Applies a saved layout by name.
    pub(super) async fn framework_apply_named_layout(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            if self.as_session().apply_named_layout(&layout_id) {
                self.note_shell_setting_command("os.applyNamedLayout", Some(&layout_id)).await?;
            }
        }
        Ok(())
    }

    /// ❌ Deletes a saved user layout.
    pub(super) async fn framework_delete_user_layout(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            self.dock.user_layouts.retain(|entry| entry.id != layout_id);
            self.dock.persist_named_layouts(&self.docs);
            self.note_shell_setting_command("os.deleteNamedLayout", Some(&layout_id)).await?;
        }
        Ok(())
    }

    /// 🖼 Opens a display window.
    pub(super) async fn framework_open_display_window(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let window_kind_id = action.args.as_ref().and_then(|args| args.get("windowKindId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
        let template_id = action.args.as_ref().and_then(|args| args.get("templateId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
        if let Some(window_kind_id) = window_kind_id {
            self.dock.open_display_window(&window_kind_id, template_id.as_deref(), &mut self.settle, &self.docs, &mut self.windows);
        }
        Ok(())
    }

    /// ⚖ Selects a conflict to resolve.
    pub(super) async fn framework_select_conflict(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(conflict_id) = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()) {
            self.prefs.selected_conflict_id = (self.prefs.selected_conflict_id.as_deref() != Some(conflict_id)).then(|| conflict_id.to_string());
        }
        Ok(())
    }

    /// 🤝 Resolves the selected conflict.
    pub(super) async fn framework_resolve_conflict(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let conflict_id = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
        let accept = action.args.as_ref().and_then(|args| args.get("accept")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
        if let Some(conflict_id) = conflict_id {
            self.docs.resolve_open_conflict(&conflict_id, accept, &mut self.chrome, &self.plugins, &mut self.prefs).await;
        }
        Ok(())
    }
}
