//! 🎛️ General settings actions: appearance, driver, locale, terminology, default app, projection template and merge policy of the framework settings panel. See [`super`].

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
    /// 🌓 Stores the appearance id and republishes the general settings tab.
    pub(super) async fn framework_set_appearance(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.prefs.appearance_id = value.to_string();
            self.note_shell_setting_command("os.setAppearance", Some(value)).await?;
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
        }
        Ok(())
    }

    /// 🚗 Stores the ui driver id, clears the driver draft and republishes the general settings tab.
    pub(super) async fn framework_set_driver(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.prefs.driver_id = value.to_string();
            self.prefs.clear_driver_draft(&mut self.chrome);
            self.note_shell_setting_command("os.setDriver", Some(value)).await?;
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
        }
        Ok(())
    }

    /// 🛠 Edits one field of the driver draft.
    pub(super) async fn framework_set_driver_field(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let (Some(key), Some(value)) = (action.args.as_ref().and_then(|args| args.get("key")).and_then(|value| value.as_str()), action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str())) {
            if self.prefs.set_driver_field(key, value, &mut self.chrome) {
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
            }
        }
        Ok(())
    }

    /// 🏷 Stores the label under which the driver draft is saved.
    pub(super) async fn framework_set_driver_save_label(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
            if self.prefs.driver_save_label != value {
                self.prefs.driver_save_label = value.to_string();
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
            }
        }
        Ok(())
    }

    /// 💾 Saves the driver draft as a custom driver.
    pub(super) async fn framework_save_driver(&mut self) -> Result<(), String> {
        let label = self.prefs.driver_save_label.trim().to_string();
        if let Some(id) = custom_ui_driver_id(&label) {
            let mut driver = self.prefs.driver_document(&self.chrome);
            driver.driver_id = id.clone();
            driver.label = label;
            self.chrome.chrome_build.preferences.custom_drivers.insert(id.clone(), driver);
            self.prefs.driver_id = id;
            self.prefs.driver_save_label.clear();
            self.prefs.clear_driver_draft(&mut self.chrome);
            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
        }
        Ok(())
    }

    /// 🗑 Deletes one custom driver.
    pub(super) async fn framework_delete_driver(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(id) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).filter(|id| id.starts_with("custom.")) {
            let changed = self.chrome.chrome_build.preferences.custom_drivers.remove(id).is_some() || self.prefs.driver_id == id || self.prefs.driver_draft.is_some();
            if self.prefs.driver_id == id {
                self.prefs.driver_id = "default".into();
            }
            self.prefs.clear_driver_draft(&mut self.chrome);
            if changed {
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
            }
        }
        Ok(())
    }

    /// 🌐 Installs the chosen locale and republishes the settings panels.
    pub(super) async fn framework_set_locale(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let changed = self.prefs.locale_id != value;
            self.prefs.locale_id = value.to_string();
            semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::install_ui_engine_locale(self.prefs.active_locale());
            if let Some(status) = self.prefs.plugin_fault_status(&self.plugins) {
                self.chrome.error = Some(status);
            }
            self.docs.trace_resolved_locale("setLocale", &self.chrome, &self.prefs);
            self.note_shell_setting_command("os.setLocale", Some(value)).await?;
            if changed {
                self.as_session().sync_dock_tabs();
                self.settle.owe_refresh(UiDirtyScope::Full);
                self.settle.owe_settle();
            }
        }
        Ok(())
    }

    /// 📖 Stores the terminology id.
    pub(super) async fn framework_set_terminology(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let changed = self.prefs.terminology_id != value;
            self.prefs.terminology_id = value.to_string();
            self.note_shell_setting_command("os.setTerminology", Some(value)).await?;
            if changed {
                self.as_session().sync_dock_tabs();
                self.settle.owe_refresh(UiDirtyScope::Full);
                self.settle.owe_settle();
            }
        }
        Ok(())
    }

    /// 📌 Stores the default app of a document kind.
    pub(super) async fn framework_set_default_app(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let key = action.args.as_ref().and_then(|args| args.get("row")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
            if value == DEFAULT_APP_NONE_VALUE {
                self.plugins.default_app_pins.remove(&key);
            } else {
                self.plugins.default_app_pins.insert(key, value.to_string());
            }
        }
        Ok(())
    }

    /// 🗺 Stores the world projection template.
    pub(super) async fn framework_set_world_projection_template(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(window_id), Some(template_id)) = (args.get("windowId").and_then(Value::as_str), args.get("templateId").and_then(Value::as_str)) {
            let (window_id, template_id) = (window_id.to_string(), template_id.to_string());
            self.dock.select_world_projection_template(&window_id, &template_id, &mut self.surfaces, &mut self.docs, &mut self.windows)?;
        }
        Ok(())
    }

    /// 🔀 Stores the merge policy.
    pub(super) async fn framework_set_merge_policy(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            if SHELL_MERGE_POLICY_OPTIONS.iter().any(|(option, _)| *option == value) {
                self.prefs.merge_policy = value.to_string();
                self.note_shell_setting_command("os.setMergePolicy", Some(value)).await?;
                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
            }
        }
        Ok(())
    }
}
