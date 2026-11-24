//! 🧩️ Plugin, extension and chat actions: install, reload and uninstall of plugins and extensions, chat drafts and tool call cancellation. See [`super`].

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
    /// 🔌 Installs or reloads a plugin.
    pub(super) async fn framework_install_or_reload_plugin(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            if action.action == "reloadPlugin" {
                self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory);
            }
            let installed = self.plugins.install_plugin(&plugin_id).await;
            if action.action == "reloadPlugin" {
                self.opening.plugin_recovery = installed.is_err().then(|| (plugin_id.clone(), false));
                self.settle.owe_settle();
            }
            self.note_shell_setting_command(if action.action == "reloadPlugin" { "os.reloadPlugin" } else { "os.installPlugin" }, Some(&plugin_id)).await?;
        }
        Ok(())
    }

    /// ⏳️ Answers a guest's typed activation request (`os.on-demand.request`, queued by the host-effect funnel): every deferred owner the frame Worker's activator resolves for it is installed through [`PluginState::install_plugin`], so the install band shows the progress and its cancel stops the load, then the contributions push moves the flow registry generation and the parked document resumes. An owner that cannot load tells the person in their language and leaves the rest of the document working; a cancelled install is silent.
    #[cfg(target_arch = "wasm32")]
    pub(super) async fn framework_activate_on_demand(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let Some(request) = action.args.as_ref().and_then(|args| args.get("request")).and_then(|value| value.as_str()).map(ToOwned::to_owned) else { return Ok(()) };
        let owners = match semio_renderer_wgpu_program_bridge::resolve_on_demand_owners(&request) {
            Ok(owners) => owners,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu shell on-demand activation request failed: {error}"));
                return Ok(());
            }
        };
        let mut installed = false;
        for plugin_id in owners {
            match self.plugins.install_plugin(&plugin_id).await {
                Ok(()) => installed = true,
                Err(error) => {
                    debug_log(&format!("[TRACE] wgpu shell on-demand install of {plugin_id} failed: {error}"));
                    if let Some((text, alert)) = semio_renderer_wgpu_program_bridge::on_demand_status_notice(&plugin_id, &self.prefs.locale_id) {
                        self.chrome.show_transient_notice(text, if alert { semio_framework::Severity::Error } else { semio_framework::Severity::Info }, Some(&format!("on-demand.{plugin_id}")));
                    }
                }
            }
        }
        if installed {
            self.as_window_actions().push_contributions().await?;
            self.settle.owe_refresh(UiDirtyScope::Full);
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) async fn framework_activate_on_demand(&mut self, _action: &ActionDescriptor) -> Result<(), String> {
        Ok(())
    }

    /// 🩺️ React's `recovery.disablePlugin`: the plugin is quarantined, and dropped unless it is the session's own program.
    pub(super) async fn framework_quarantine_plugin(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory);
            self.opening.plugin_recovery = Some((plugin_id, true));
            self.settle.owe_settle();
        }
        Ok(())
    }

    /// 🔧 Uninstalls a plugin.
    pub(super) async fn framework_uninstall_plugin(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            if self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory) {
                self.note_shell_setting_command("os.uninstallPlugin", Some(&plugin_id)).await?;
            }
        }
        Ok(())
    }

    /// 🌍 Installs an extension from a url.
    pub(super) async fn framework_install_extension_url(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let url = action.args.as_ref().and_then(|args| args.get("url")).and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(ToOwned::to_owned);
        let mut request = serde_json::json!({
            "op": "extension-store-install-url",
            "prompt": shell_chrome_string("plugins.extension.urlPrompt", self.prefs.locale_id == "de")
        });
        if let Some(url) = url.as_deref() {
            request["url"] = serde_json::Value::String(url.to_string());
        }
        if self.install_extension_from_store(request).await? {
            self.note_shell_setting_command("os.installExtensionUrl", url.as_deref()).await?;
        }
        Ok(())
    }

    /// 📦 Installs an extension from a file.
    pub(super) async fn framework_install_extension_file(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let mut request = serde_json::json!({ "op": "extension-store-install-file" });
        if let Some(path) = action.args.as_ref().and_then(|args| args.get("path")).and_then(|value| value.as_str()).filter(|path| !path.is_empty()) {
            request["path"] = serde_json::Value::String(path.to_string());
        }
        if self.install_extension_from_store(request).await? {
            self.note_shell_setting_command("os.installExtensionFile", None).await?;
        }
        Ok(())
    }

    /// 🔛 Enables or disables an extension.
    pub(super) async fn framework_set_extension_enabled(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let extension_id = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned);
        let enabled = action.args.as_ref().and_then(|args| args.get("enabled")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
        if let Some(extension_id) = extension_id {
            if self.set_extension_enabled(&extension_id, enabled).await? {
                self.note_shell_setting_command(if enabled { "os.enableExtension" } else { "os.disableExtension" }, Some(&extension_id)).await?;
            }
        }
        Ok(())
    }

    /// 🚮 Uninstalls an extension.
    pub(super) async fn framework_uninstall_extension(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(extension_id) = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
            if self.uninstall_extension(&extension_id).await? {
                self.note_shell_setting_command("os.uninstallExtension", Some(&extension_id)).await?;
            }
        }
        Ok(())
    }

    /// 💬 Stores the chat draft.
    pub(super) async fn framework_set_chat_draft(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.directory.agent_chat_draft = value.to_string();
        }
        Ok(())
    }

    /// 📨 Sends the chat draft.
    pub(super) async fn framework_send_chat_draft(&mut self) -> Result<(), String> {
        self.directory.send_agent_chat_draft(&mut self.chrome);
        Ok(())
    }

    /// 🛑 Cancels a running tool call.
    pub(super) async fn framework_cancel_tool_call(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(invocation_id) = action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
            self.chrome.cancel_agent_tool_call(&invocation_id);
        }
        Ok(())
    }
}
