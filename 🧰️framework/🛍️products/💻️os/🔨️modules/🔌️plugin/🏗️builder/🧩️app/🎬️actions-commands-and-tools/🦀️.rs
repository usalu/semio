//! 🎬️ Actions, commands and tools of the app builder: the mutation, view and shell actions with their descriptions, audience, destructive and interactive job flags, the commands, utilities, interactions and tools. See [`super`].

use super::*;

impl AppBuilder {
    /// ✏️ Declares a document-mutating action — dispatched as VCS operations with a true inverse.
    pub async fn mutation(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)).await
    }

    /// 👁️ Declares an ephemeral view action (camera, selection, hover, active utility) — not recorded in history.
    pub async fn view_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::View)).await
    }

    /// 🐚️ Declares a shell-only effect action (navigate, export, spawn) — no document mutation.
    pub async fn shell_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::Shell)).await
    }

    /// 📇️ Declares a fully specified action (icon, args, keybinding, palette visibility, category).
    pub async fn action_with(mut self, action: ActionDefinition) -> Self {
        self.actions.push(action);
        self
    }

    /// 📝️ Attaches typed argument declarations to an already-declared action (post-hoc, mirroring
    /// `window_kind_actions`). If the id isn't declared yet at call time the args are dropped; the
    /// mismatch surfaces in `build_definition`, which asserts every declared action's args are consistent.
    pub async fn action_args(mut self, action_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let action_id = action_id.as_ref();
        if let Some(action) = self.actions.iter_mut().find(|entry| entry.id == action_id) {
            action.args = args;
        }
        self
    }

    /// 💬️ Attaches a localized, agent-facing description to one already-declared action or
    /// command — the text `capabilities_search`/`capabilities_describe` show an agent choosing a
    /// tool. Searches every surface an id can be declared on, exactly like
    /// [`Self::action_interactive_job`], so a window-kind action is reached too.
    pub async fn action_describe(mut self, action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>) -> Self {
        let action_id = action_id.as_ref();
        let description = description.into();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.description = Some(description.clone());
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.description = Some(description.clone());
            }
        }
        self
    }

    /// 🗣️ Attaches the natural-language phrases a capability search should match one
    /// already-declared action or command against (`ActionSemantics.use_when`).
    pub async fn action_use_when(mut self, action_id: impl AsRef<str>, phrases: Vec<String>) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.use_when = phrases.clone();
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.use_when = phrases.clone();
            }
        }
        self
    }

    /// 🎯️ Declares one already-declared action's or command's [`CapabilityAudience`] — the
    /// override `manifest::derive_audience` cannot make on its own (a `Mutation`-kind pointer
    /// gesture). Also drops it out of the palette when it is not agent-facing.
    pub async fn action_audience(mut self, action_id: impl AsRef<str>, audience: CapabilityAudience) -> Self {
        let action_id = action_id.as_ref();
        let hidden = !matches!(audience, CapabilityAudience::Agent);
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.audience = Some(audience);
                if hidden {
                    action.in_palette = false;
                }
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.audience = Some(audience);
                if hidden {
                    command.in_palette = false;
                }
            }
        }
        self
    }

    /// ⚠️ Marks one already-declared action or command destructive — it discards user content
    /// that no later verb reconstructs (delete, clear, replace-the-whole-document). Sets
    /// `effects.destructive` and raises `policy.approval` to `WhenDestructive` unless it is
    /// already `Always`, which is what makes the MCP gateway pause for a human before committing
    /// the invocation (`🛡️policy`'s `ApprovalMode::WhenDestructive => effects.destructive`).
    /// Searches every surface an id can be declared on, exactly like [`Self::action_audience`].
    pub async fn action_destructive(mut self, action_id: impl AsRef<str>) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.effects.destructive = true;
                if action.semantics.policy.approval == semio_framework::ApprovalMode::Never {
                    action.semantics.policy.approval = semio_framework::ApprovalMode::WhenDestructive;
                }
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.effects.destructive = true;
                if command.semantics.policy.approval == semio_framework::ApprovalMode::Never {
                    command.semantics.policy.approval = semio_framework::ApprovalMode::WhenDestructive;
                }
            }
        }
        self
    }

    /// 🧵️ Explicitly changes one declared action's or command's Phase-8 execution disposition.
    /// Searches every surface an id can be declared on — bare actions, window-kind actions, bare
    /// commands and mode commands — because an id missed here stays
    /// [`InteractiveJobClassification::Unclassified`], which `validate_interactive_job_classification`
    /// rejects only once the guest is already running.
    pub async fn action_interactive_job(mut self, action_id: impl AsRef<str>, classification: semio_framework::InteractiveJobClassification) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.execution.interactive_job = classification;
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.execution.interactive_job = classification;
            }
        }
        self
    }

    /// 🧵️ Explicitly assigns one execution disposition to every action and command already declared by this app.
    pub async fn interactive_jobs(mut self, classification: semio_framework::InteractiveJobClassification) -> Self {
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            action.semantics.execution.interactive_job = classification;
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            command.semantics.execution.interactive_job = classification;
        }
        self
    }

    /// 🎛️ Declares a fully specified app-owned command.
    pub async fn command(mut self, command: CommandDefinition) -> Self {
        self.commands.push(command);
        self
    }

    /// 🎛️ Declares an app-scope command (applies whenever this app is focused, in any mode).
    pub async fn app_command(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        self.command(CommandDefinition::bounded_catalog(id, label, category, kind)).await
    }

    /// 📝️ Attaches typed argument declarations to an already-declared command (post-hoc,
    /// mirroring `action_args`).
    pub async fn command_args(mut self, command_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let command_id = command_id.as_ref();
        if let Some(command) = self.commands.iter_mut().find(|entry| entry.id == command_id) {
            command.args = args;
        }
        self
    }

    /// 🧰️ Declares an interactive utility this app exposes (referenced by `window_kind_utilities`).
    pub async fn utility(mut self, utility: UtilityDefinition) -> Self {
        self.utilities.push(utility);
        self
    }

    /// 🕹️ Declares a framework-managed hover and selection domain.
    pub async fn interaction(mut self, interaction: InteractionDefinition) -> Self {
        self.interactions.push(interaction);
        self
    }

    /// 🧰️ Declares a utility with default settings (no group/keys/cursor/category, gates actions while active).
    pub async fn utility_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.utility(UtilityDefinition::new(id, label, icon_id)).await
    }

    /// 🛠️ Declares a mode-level tool this app exposes (referenced by `.mode_tools()`). Distinct
    /// from `.utility()`: a tool is scoped to a whole mode, not a window kind, and its live options are
    /// supplied dynamically via `ArtifactApp::tool_measures`/`PluginApp::tool_measures`.
    pub async fn tool(mut self, tool: ToolDefinition) -> Self {
        self.tools.push(tool);
        self
    }

    /// 🛠️ Declares a tool with default settings (no keybinding).
    pub async fn tool_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.tool(ToolDefinition::new(id, label, icon_id).await).await
    }

}
