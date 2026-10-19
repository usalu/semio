//! 🎨️ Theme actions: theme selection, token edits, saving, export, import and deletion. See [`super`].

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
    /// 🎭 Selects the theme by id.
    pub(super) async fn framework_set_theme_id(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let changed = self.chrome.chrome_build.preferences.theme_id != value || self.prefs.theme_draft.is_some();
            set_active_theme_id(value);
            self.prefs.clear_theme_draft();
            self.chrome.chrome_build.preferences.theme_id = value.to_string();
            self.note_shell_setting_command("os.setThemeId", Some(value)).await?;
            if changed {
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
            }
        }
        Ok(())
    }

    /// 🌈 Edits one theme token: color, spacing, font stack, stroke, radius, opacity or metric.
    pub(super) async fn framework_set_theme_token(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        let verb = action.action.clone();
        let before = self.prefs.theme_document(&self.chrome);
        self.prefs.apply_theme_editor_commit(&verb, &args, &self.chrome);
        if self.prefs.theme_document(&self.chrome) != before {
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
        }
        Ok(())
    }

    /// 🔖 Stores the label under which the theme is saved.
    pub(super) async fn framework_set_theme_save_label(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let value = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
        if self.prefs.theme_save_label != value {
            self.prefs.theme_save_label = value;
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
        }
        Ok(())
    }

    /// 🗄 Saves the edited theme.
    pub(super) async fn framework_save_theme(&mut self) -> Result<(), String> {
        let label = self.prefs.theme_save_label.trim().to_string();
        if let Some(id) = custom_theme_id_for_label(&label) {
            let mut document = self.prefs.theme_document(&self.chrome);
            document.id = id.clone();
            document.label = label;
            if let Some(raw) = save_custom_theme_document(&id, &document) {
                self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                self.chrome.chrome_build.preferences.theme_id = id.clone();
                self.prefs.theme_draft = None;
                self.prefs.theme_save_label.clear();
                self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
            }
        }
        Ok(())
    }

    /// 📤 Exports the theme.
    pub(super) async fn framework_export_theme(&mut self) -> Result<(), String> {
        let document = self.prefs.theme_document(&self.chrome);
        if let Some(text) = document.canonical_json() {
            download_media_export(&format!("{}.theme.dsl", document.id), "text/plain", &text, None);
        }
        Ok(())
    }

    /// 📥 Imports a theme.
    pub(super) async fn framework_import_theme(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            self.prefs.pending_theme_import = true;
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.settle.submit_shell_io_future(async move {
            let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
            ShellIoCompletion::Actions(
                opened.into_iter().take(1).map(|file| ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": file.contents }) }).collect(),
            )
        });
        Ok(())
    }

    /// ✅ Applies the imported theme.
    pub(super) async fn framework_apply_imported_theme(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(text) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            if let Some(document) = ThemeDocument::parse(text) {
                let label = if document.label.is_empty() { document.id.clone() } else { document.label.clone() };
                if let Some(id) = custom_theme_id_for_label(&label) {
                    let saved = ThemeDocument { id: id.clone(), label, ..document };
                    if let Some(raw) = save_custom_theme_document(&id, &saved) {
                        self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                        self.chrome.chrome_build.preferences.theme_id = id.clone();
                        self.prefs.theme_draft = None;
                        self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                        self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// 🔄 Resets the theme selection.
    pub(super) async fn framework_reset_theme_id(&mut self) -> Result<(), String> {
        let changed = self.chrome.chrome_build.preferences.theme_id != "semio" || self.prefs.theme_draft.is_some();
        set_active_theme_id("semio");
        self.prefs.clear_theme_draft();
        self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
        self.note_shell_setting_command("os.resetThemeId", Some("semio")).await?;
        if changed {
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
        }
        Ok(())
    }

    /// 🧯 Deletes a custom theme.
    pub(super) async fn framework_delete_theme_id(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            delete_custom_theme(value);
            let removed = self.chrome.chrome_build.preferences.custom_themes.remove(value).is_some();
            let selected = self.chrome.chrome_build.preferences.theme_id == value;
            if selected {
                self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
            }
            self.note_shell_setting_command("os.deleteThemeId", Some(value)).await?;
            if removed || selected {
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
            }
        }
        Ok(())
    }
}
