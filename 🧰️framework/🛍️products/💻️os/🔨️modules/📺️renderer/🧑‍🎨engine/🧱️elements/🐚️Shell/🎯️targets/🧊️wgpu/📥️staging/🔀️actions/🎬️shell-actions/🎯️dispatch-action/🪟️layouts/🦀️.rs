//! 🪟️ Layout and window actions: layouts, display windows and conflict resolution. See [`super`].

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
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
