//! ⌨️ Command and action staging actions: keybindings, command and action staging, expansion, engagement input and search possibles. See [`super`].

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
    /// 🖥 Runs the os command the action names.
    pub(super) async fn framework_run_os_command(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let command_id = value.to_string();
            self.apply_os_command(&command_id, None).await?;
        }
        Ok(())
    }

    /// ⌨ Starts capturing a keybinding for a command.
    pub(super) async fn framework_capture_keybinding(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            let already = self.input.keybinding_capture_control_id.as_deref() == Some(value);
            self.input.keybinding_capture_control_id = (!already).then(|| value.to_string());
        }
        Ok(())
    }

    /// ↩ Resets the keybinding of a command.
    pub(super) async fn framework_reset_keybinding(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.chrome.chrome_build.preferences.keybinding_overrides.remove(value);
        }
        Ok(())
    }

    /// 🔽 Expands or collapses a command row.
    pub(super) async fn framework_set_command_expanded(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
            let live = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().any(|entry| entry.definition.in_palette && !entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key);
            if live {
                self.commands.expanded_command_id = (self.commands.expanded_command_id.as_deref() != Some(key)).then(|| key.to_string());
            }
        }
        Ok(())
    }

    /// ▶ Executes the command resolved from the action arguments.
    pub(super) async fn framework_execute_resolved_command(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key) {
                match &entry.address.owner {
                    semio_framework::manifest::CommandOwnerAddress::Os => {
                        self.apply_os_command(&entry.definition.id, None).await?;
                    }
                    _ => {
                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address, arguments: Default::default() }).await?;
                    }
                }
            }
        }
        Ok(())
    }

    /// 📝 Stages one command argument.
    pub(super) async fn framework_stage_command_arg(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(command), Some(arg)) = (args.get("command").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
            let staged = self.commands.staged_command_args.entry(command.to_string()).or_default();
            let value = staged_arg_value(&args, staged.get(arg));
            staged.insert(arg.to_string(), value);
        }
        Ok(())
    }

    /// 🧽 Clears the staged command.
    pub(super) async fn framework_reset_staged_command(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
            self.commands.staged_command_args.remove(value);
        }
        Ok(())
    }

    /// 🚀 Executes the staged command.
    pub(super) async fn framework_execute_staged_command(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
            let staged = self.commands.staged_command_args.get(&key).cloned().unwrap_or_default();
            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && command_address_stable_key(&entry.address) == key) {
                let Some(effective) = resolved_execute_args(&entry.definition.args, &staged) else { return Ok(()) };
                match &entry.address.owner {
                    semio_framework::manifest::CommandOwnerAddress::Os => {
                        let command_id = entry.definition.id.clone();
                        let option = entry.definition.args.first().and_then(|arg| effective.get(&arg.id)).and_then(|value| value.as_str().map(ToOwned::to_owned));
                        self.apply_os_command(&command_id, option.as_deref()).await?;
                    }
                    _ => {
                        let arguments = effective.into_iter().filter_map(|(id, value)| serde_json::from_value::<DslValue>(value).ok().map(|value| (id, value))).collect();
                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address.clone(), arguments }).await?;
                    }
                }
            }
            self.commands.staged_command_args.remove(&key);
            self.commands.expanded_command_id = None;
        }
        Ok(())
    }

    /// 📂 Expands or collapses an action row.
    pub(super) async fn framework_set_action_expanded(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
            if self.windows.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id) {
                self.windows.action_panel_expanded.remove(window_id);
            } else {
                self.windows.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
            }
        }
        self.as_window_actions().republish_window_action_panes()
    }

    /// ✍ Stages one action argument.
    pub(super) async fn framework_stage_action_arg(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(window_id), Some(action_id), Some(arg_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
            let (window_id, action_id, arg_id) = (window_id.to_string(), action_id.to_string(), arg_id.to_string());
            let value = staged_arg_value(&args, self.commands.staged_map_for(&window_id, &action_id).get(&arg_id));
            self.commands.stage_arg(&window_id, &action_id, &arg_id, value);
        }
        self.as_window_actions().republish_window_action_panes()
    }

    /// 🧼 Clears the staged action arguments.
    pub(super) async fn framework_reset_action_args(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
            self.commands.reset_staged_args(&window_id, &action_id);
        }
        self.as_window_actions().republish_window_action_panes()
    }

    /// 🎯 Executes the staged action.
    pub(super) async fn framework_execute_staged_action(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
            self.execute_staged_action(&window_id, &action_id).await?;
        }
        Ok(())
    }

    /// 👆 Stores the engagement input text.
    pub(super) async fn framework_set_engagement_input(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
            let value = args.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
            self.windows.engagement_inputs.insert(window_id.to_string(), value);
        }
        Ok(())
    }

    /// 🔍 Toggles the search possibles panel.
    pub(super) async fn framework_toggle_search_possibles(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
            let open = self.commands.search_possibles_open.get(window_id).copied().unwrap_or(false);
            self.commands.search_possibles_open.insert(window_id.to_string(), !open);
        }
        self.as_window_actions().republish_window_action_panes()
    }
}
