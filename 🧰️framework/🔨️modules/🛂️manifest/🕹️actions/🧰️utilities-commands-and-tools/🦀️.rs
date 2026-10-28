//! 🧰️ Declared utilities, commands and tools of the manifest: the utility, command, OS command catalog and mode tool definitions. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Utilities
/// 📺️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
/// an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
/// host-owned session view state (`ViewModel.active_utility_id`), never a document field or VCS operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct UtilityDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🧺️ Visual ribbon collection this utility groups into; `None` = a flat top-level ribbon entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    /// 🖱️ CSS/winit cursor name applied to the window body while this utility is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<semio_framework_ui_declaration::utilities::UtilityCategory>,
    /// 🚦️ Whether window-scoped actions stay enabled while this utility is active. Defaults to `false`
    /// (matching today's whitelist-based gating where an active utility suppresses the action panel);
    /// set `true` for passive view utilities (e.g. cad `cad.play.view.*`) that should not gate actions.
    #[serde(default)]
    #[value(default)]
    pub allows_actions_while_active: bool,
    /// ⏯️ Declares that activating this utility runs a visible, abortable algorithm (tool run contract §2.4);
    /// reserves the tool run actions through [`crate::tool_run::RESERVED`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ToolRunDefinition>,
}

impl UtilityDefinition {
    /// 🔐️ A utility with sensible defaults (no group/keys/cursor/category/run, gates actions while active).
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: false, run: None }
    }
}
//#endregion 🔖️Utilities

//#region 🔖️Commands
/// 🎛️ Declares one command: a categorized verb offered in the footer command panel.
/// Its owner and availability are derived from the containing OS, plugin, app, or mode definition.
/// Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
/// `ArtifactApp::handle_command`/`ActionEmit`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandDefinition {
    pub id: String,
    /// 🔖️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    /// 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
    pub category: String,
    pub icon_id: IconName,
    pub kind: ActionKind,
    /// 📝️ Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
    pub args: Vec<ActionArgDef>,
    #[serde(default)]
    #[value(default)]
    pub keybindings: Vec<PlatformKeybinding>,
    #[serde(default)]
    #[value(default)]
    pub in_palette: bool,
    /// 🎯️ See `ActionDefinition.semantics` — same D6/§3.1 field, same defaulting/inheritance story.
    #[serde(default)]
    #[value(default)]
    pub semantics: ActionSemantics,
}

impl CommandDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, icon_id: impl Into<IconName>, kind: ActionKind) -> Self {
        Self { id: id.into(), label: label.into(), category: category.into(), icon_id: icon_id.into(), kind, args: Vec::new(), keybindings: Vec::new(), in_palette: true, semantics: ActionSemantics::for_kind(kind) }
    }

    /// 📚️ Declares a command with the neutral icon for its action kind.
    pub fn new_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        Self::new(id, label, category, default_action_icon_id(kind), kind)
    }

    /// ⚡️ Builds a catalog row without granting UI execution authority. A factory registration or
    /// static bounded-first-step proof must classify the exact key separately.
    pub fn bounded_catalog(id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        Self::new_catalog(id, label, category, kind)
    }

    /// 📎️ Attaches typed argument declarations to this command.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    /// ⌨️ Attaches one platform-aware command keybinding.
    pub fn with_keybinding(mut self, keybinding: PlatformKeybinding) -> Self {
        self.keybindings.push(keybinding);
        self
    }

    /// 🧬️ Replaces this command's whole `ActionSemantics` wholesale.
    pub async fn semantics(mut self, semantics: ActionSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    /// ⚠️ Marks this command destructive — see `ActionDefinition::destructive`.
    pub fn destructive(mut self) -> Self {
        self.semantics.effects.destructive = true;
        if self.semantics.policy.approval == ApprovalMode::Never {
            self.semantics.policy.approval = ApprovalMode::WhenDestructive;
        }
        self
    }

    /// 🎬️ Sets `ActionSemantics.use_when` — see `ActionDefinition::use_when`.
    pub async fn use_when(mut self, phrases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.semantics.use_when = phrases.into_iter().map(Into::into).collect();
        self
    }

    /// 💬️ Sets `ActionSemantics.description` — see `ActionDefinition::describe`.
    pub fn describe(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.semantics.description = Some(description.into());
        self
    }

    /// 👥️ Declares this command's [`CapabilityAudience`] — see `ActionDefinition::audience`.
    pub fn audience(mut self, audience: CapabilityAudience) -> Self {
        self.semantics.audience = Some(audience);
        self
    }

    /// 📖️ Appends one `ActionSemantics.examples` entry — see `ActionDefinition::example`.
    pub async fn example(mut self, example: impl Into<String>) -> Self {
        self.semantics.examples.push(example.into());
        self
    }
}

/// 💻️ Operating-system command catalog shared by every renderer.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct OsDefinition {
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
}
//#endregion 🔖️Commands

//#region 🔖️Tools
/// 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
/// Distinct from `UtilityDefinition` (a per-window pointer mode — a utility is a tool for a specific
/// window) and `CommandDefinition` (a fire-once verb): exactly one tool is active per app at a time,
/// and activation is host-owned session view state (`ViewModel.active_tool_id`), never a document
/// field or VCS operation. A tool's live options are supplied dynamically via `ArtifactApp::tool_measures`,
/// keyed by tool id — not part of this static declaration.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub id: String,
    /// 🎚️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    /// ▶️ Declares that this tool runs a visible, abortable, finalizable algorithm (tool run contract §2.4);
    /// reserves the tool run actions through [`crate::tool_run::RESERVED`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ToolRunDefinition>,
}

impl ToolDefinition {
    /// 🔧️ A tool with sensible defaults (no keybinding, no run).
    pub async fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), keys: None, run: None }
    }
}
//#endregion 🔖️Tools
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
