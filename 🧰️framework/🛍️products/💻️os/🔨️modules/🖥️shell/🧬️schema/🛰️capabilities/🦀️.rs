//! 🛰️ Capabilities of the shell schema: the capability record, the command catalog and the capability listing an MCP gateway advertises. See [`super`].

use super::*;

//#region 🛰️ShellCapability
/// 🛰️ One machine-readable descriptor per [`ShellCommand`] variant — what an MCP gateway (a later
/// packet) compiles into its tool catalog. Defined locally (packet §3: "do NOT depend on the
/// gateway crate or on `🛂️manifest`").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ShellCapability {
    /// 🧊️ Stable dotted id, e.g. `ui.window.focus`. Reuses the wgpu shell's existing informal
    /// `shell.*` verb strings where one already names this exact mutation (see
    /// `📓️terra-P9-report.md` "existing-verb → variant mapping" for the full table); coined fresh,
    /// domain-shaped, where no verb existed yet.
    pub id: String,
    pub title: String,
    pub description: String,
    /// 🧾️ JSON Schema for this variant's payload (schemars-derived from [`ShellCommand`]).
    #[value(with = "json_value_bridge")]
    pub schema: JsonValue,
    /// ✅️ True for commands that only stage/observe data (no mutation with externally-visible
    /// consequence beyond the field itself) — none of today's commands are observable-only, this
    /// flag exists for the gateway to distinguish future read-modeling commands if any are added.
    pub observable_only: bool,
}

struct CommandMeta {
    pub(super) id: &'static str,
    pub(super) title: &'static str,
    pub(super) description: &'static str,
    observable_only: bool,
}

/// 🆔 Returns the [`ShellCapability::id`] for a command instance — used by [`reduce`] to stamp
/// [`ShellEvent::Applied`]. Declaration order here MUST match [`ShellCommand`]'s declaration
/// order and [`SHELL_COMMAND_CATALOG`]'s order; `shell_capabilities_declaration_order_matches_enum`
/// guards this.
pub(crate) fn capability_id_for(command: &ShellCommand) -> String {
    let index = match command {
        ShellCommand::RegisterLoadedPlugin { .. } => 0,
        ShellCommand::UnregisterLoadedPlugin { .. } => 1,
        ShellCommand::SetPluginStatus { .. } => 2,
        ShellCommand::SetPluginSupervisorState { .. } => 3,
        ShellCommand::SetActiveSession { .. } => 4,
        ShellCommand::SetSessionError { .. } => 5,
        ShellCommand::SetAppLabelOverride { .. } => 6,
        ShellCommand::SetActionPaneFolded { .. } => 7,
        ShellCommand::SetActionPaneExpanded { .. } => 8,
        ShellCommand::StageActionArg { .. } => 9,
        ShellCommand::ResetActionArgs { .. } => 10,
        ShellCommand::SetActiveUtility { .. } => 11,
        ShellCommand::SetActiveTool { .. } => 12,
        ShellCommand::SetCommandExpanded { .. } => 13,
        ShellCommand::StageCommandArg { .. } => 14,
        ShellCommand::ResetCommandArgs { .. } => 15,
        ShellCommand::SetPanelVisible { .. } => 16,
        ShellCommand::SetPanelSize { .. } => 17,
        ShellCommand::SetPanelPath { .. } => 18,
        ShellCommand::SetDockOverride { .. } => 19,
        ShellCommand::SetPanelPathMemory { .. } => 20,
        ShellCommand::SetTreeOpenState { .. } => 21,
        ShellCommand::HydrateDockUi { .. } => 22,
        ShellCommand::ResetDock => 23,
        ShellCommand::FocusWindow { .. } => 24,
        ShellCommand::SetShellLayout { .. } => 25,
        ShellCommand::SetActiveExample { .. } => 26,
        ShellCommand::SetMobilePanelPath { .. } => 27,
        ShellCommand::SetMobilePanelVisible { .. } => 28,
        ShellCommand::SetExtraWindows { .. } => 29,
        ShellCommand::SetWindowTitle { .. } => 30,
        ShellCommand::SetWindowIcon { .. } => 31,
        ShellCommand::SetSearchOpen { .. } => 32,
        ShellCommand::SetFindOpen { .. } => 33,
        ShellCommand::AutoStartIntroduction { .. } => 34,
        ShellCommand::SetIntroductionStep { .. } => 35,
        ShellCommand::CompleteIntroductionInteraction { .. } => 36,
        ShellCommand::OpenDialog { .. } => 37,
        ShellCommand::CloseDialog { .. } => 38,
        ShellCommand::ShowTransientNotice { .. } => 39,
        ShellCommand::DismissTransientNotice => 40,
        ShellCommand::SetOpenWithFocusRole { .. } => 41,
        ShellCommand::SetActiveTutorial { .. } => 42,
        ShellCommand::SetUiDriverDraft { .. } => 43,
        ShellCommand::SetUiThemeDraft { .. } => 44,
        ShellCommand::SetSyncBackboneUri { .. } => 45,
        ShellCommand::SetSyncCardKind { .. } => 46,
        ShellCommand::SetSyncDraftPath { .. } => 47,
        ShellCommand::SetDocumentSyncStatus { .. } => 48,
        ShellCommand::SetDocumentInferencePort { .. } => 49,
        ShellCommand::ClearDocumentInferencePort { .. } => 50,
        ShellCommand::SetMergePolicy { .. } => 51,
        ShellCommand::SetConflicts { .. } => 52,
        ShellCommand::SelectConflict { .. } => 53,
        ShellCommand::SetStorageScope { .. } => 54,
        ShellCommand::SetOpeningPreference { .. } => 55,
    };
    SHELL_COMMAND_CATALOG[index].id.to_string()
}

/// 🗂️ One entry per [`ShellCommand`] variant, in declaration order. `id`s reuse the wgpu shell's
/// existing `shell.*` verb strings where one already exists for this exact mutation (see
/// `📓️terra-P9-report.md`), and are coined fresh in the same dotted-noun style otherwise.
const SHELL_COMMAND_CATALOG: [CommandMeta; 56] = [
    CommandMeta { id: "plugin.register", title: "Register Loaded Plugin", description: "Add or replace a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.unregister", title: "Unregister Loaded Plugin", description: "Remove a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.setStatus", title: "Set Plugin Status", description: "Set a plugin panel's open/collapsed/error status.", observable_only: false },
    CommandMeta { id: "plugin.setSupervisorState", title: "Set Plugin Supervisor State", description: "Update a plugin's resource/failure monitoring summary.", observable_only: false },
    CommandMeta { id: "plugin.setActiveSession", title: "Set Active Session", description: "Switch the active app instance binding.", observable_only: false },
    CommandMeta { id: "plugin.setSessionError", title: "Set Session Error", description: "Set or clear the top-level shell error message.", observable_only: false },
    CommandMeta { id: "ui.app.setLabelOverride", title: "Set App Label Override", description: "Set or clear one app-specific label customization.", observable_only: false },
    CommandMeta { id: "shell.action.fold", title: "Set Action Pane Folded", description: "Collapse or expand a window's action rail.", observable_only: false },
    CommandMeta { id: "shell.action.expand", title: "Set Action Pane Expanded", description: "Open or close a window's action arg form.", observable_only: false },
    CommandMeta { id: "shell.action.stageArg", title: "Stage Action Arg", description: "Buffer one action argument value.", observable_only: false },
    CommandMeta { id: "shell.action.reset", title: "Reset Action Args", description: "Clear all staged args for one action.", observable_only: false },
    CommandMeta { id: "ui.window.setActiveUtility", title: "Set Active Utility", description: "Switch a window's host-owned active utility.", observable_only: false },
    CommandMeta { id: "ui.tool.setActive", title: "Set Active Tool", description: "Switch the host-owned active mode tool.", observable_only: false },
    CommandMeta { id: "ui.command.expand", title: "Set Command Expanded", description: "Open or close a command palette entry's arg form.", observable_only: false },
    CommandMeta { id: "ui.command.stageArg", title: "Stage Command Arg", description: "Buffer one command palette argument value.", observable_only: false },
    CommandMeta { id: "ui.command.reset", title: "Reset Command Args", description: "Clear all staged args for one command.", observable_only: false },
    CommandMeta { id: "shell.panelToggle", title: "Set Panel Visible", description: "Show or hide a docked panel.", observable_only: false },
    CommandMeta { id: "ui.panel.setSize", title: "Set Panel Size", description: "Resize a docked panel.", observable_only: false },
    CommandMeta { id: "shell.panel.tab", title: "Set Panel Path", description: "Navigate a docked panel's breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.dock.setOverride", title: "Set Dock Override", description: "Persist a user dock rearrangement.", observable_only: false },
    CommandMeta { id: "ui.panel.setPathMemory", title: "Set Panel Path Memory", description: "Remember (or forget) a drill-down tab per panel.", observable_only: false },
    CommandMeta { id: "ui.tree.setOpenState", title: "Set Tree Open State", description: "Expand or collapse a tree section.", observable_only: false },
    CommandMeta { id: "ui.dock.hydrate", title: "Hydrate Dock UI", description: "Restore dock layout and panel visibility from a snapshot.", observable_only: false },
    CommandMeta { id: "ui.dock.reset", title: "Reset Dock", description: "Clear the dock override, returning to the default layout.", observable_only: false },
    CommandMeta { id: "ui.window.focus", title: "Focus Window", description: "Change the focused window instance.", observable_only: false },
    CommandMeta { id: "ui.shell.setLayout", title: "Set Shell Layout", description: "Change the window split/stack arrangement.", observable_only: false },
    CommandMeta { id: "ui.example.setActive", title: "Set Active Example", description: "Switch the active catalog example.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelPath", title: "Set Mobile Panel Path", description: "Change the mobile panel breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelVisible", title: "Set Mobile Panel Visible", description: "Show or hide the mobile panel.", observable_only: false },
    CommandMeta { id: "ui.window.setExtraWindows", title: "Set Extra Windows", description: "Replace the spawned extra window list.", observable_only: false },
    CommandMeta { id: "ui.window.setTitle", title: "Set Window Title", description: "Set a live window title override.", observable_only: false },
    CommandMeta { id: "ui.window.setIcon", title: "Set Window Icon", description: "Set a live window icon override.", observable_only: false },
    CommandMeta { id: "ui.search.setOpen", title: "Set Search Open", description: "Show or hide the global search panel.", observable_only: false },
    CommandMeta { id: "ui.find.setOpen", title: "Set Find Open", description: "Show or hide the find-in-window panel.", observable_only: false },
    CommandMeta { id: "ui.introduction.autoStart", title: "Auto-Start Introduction", description: "Mark a walkthrough as auto-started this session.", observable_only: false },
    CommandMeta { id: "ui.introduction.setStep", title: "Set Introduction Step", description: "Change the walkthrough step index.", observable_only: false },
    CommandMeta { id: "ui.introduction.completeInteraction", title: "Complete Introduction Interaction", description: "Mark one walkthrough step interaction done.", observable_only: false },
    CommandMeta { id: "ui.dialog.open", title: "Open Dialog", description: "Push a dialog onto the open-dialog stack.", observable_only: false },
    CommandMeta { id: "ui.dialog.close", title: "Close Dialog", description: "Pop a dialog off the open-dialog stack (top, or by id).", observable_only: false },
    CommandMeta { id: "ui.notice.show", title: "Show Transient Notice", description: "Show a non-blocking auto-dismiss notice.", observable_only: false },
    CommandMeta { id: "ui.notice.dismiss", title: "Dismiss Transient Notice", description: "Dismiss the current transient notice.", observable_only: false },
    CommandMeta { id: "ui.open.setFocusRole", title: "Set Open-With Focus Role", description: "Set which role group the Open panel focuses.", observable_only: false },
    CommandMeta { id: "ui.tutorial.setActive", title: "Set Active Tutorial", description: "Switch the active video tutorial.", observable_only: false },
    CommandMeta { id: "ui.driver.setDraft", title: "Set UI Driver Draft", description: "Set or clear the in-progress driver editor draft.", observable_only: false },
    CommandMeta { id: "ui.theme.setDraft", title: "Set UI Theme Draft", description: "Set or clear the in-progress theme editor draft.", observable_only: false },
    CommandMeta { id: "sync.setBackboneUri", title: "Set Sync Backbone URI", description: "Set or clear the hub document sync backbone URI.", observable_only: false },
    CommandMeta { id: "sync.setCardKind", title: "Set Sync Card Kind", description: "Set or clear the check-in target type.", observable_only: false },
    CommandMeta { id: "sync.setDraftPath", title: "Set Sync Draft Path", description: "Set the work-in-progress check-in path.", observable_only: false },
    CommandMeta { id: "sync.setDocumentStatus", title: "Set Document Sync Status", description: "Set one document's sync health.", observable_only: false },
    CommandMeta { id: "inference.setDocumentPort", title: "Set Document Inference Port", description: "Replace one document's ephemeral inference-port state.", observable_only: false },
    CommandMeta { id: "inference.clearDocumentPort", title: "Clear Document Inference Port", description: "Retire one document's ephemeral inference-port state.", observable_only: false },
    CommandMeta { id: "merge.setPolicy", title: "Set Merge Policy", description: "Set the conflict resolution strategy.", observable_only: false },
    CommandMeta { id: "merge.setConflicts", title: "Set Conflicts", description: "Replace the open conflict roster.", observable_only: false },
    CommandMeta { id: "merge.selectConflict", title: "Select Conflict", description: "Select a conflict for preview.", observable_only: false },
    CommandMeta { id: "host.setStorageScope", title: "Set Storage Scope", description: "Select the shell state storage backend.", observable_only: false },
    CommandMeta { id: "host.setOpeningPreference", title: "Set Opening Preference", description: "Set or clear the default app/dialect for a role.", observable_only: false },
];

/// 🛰️ Every [`ShellCommand`] variant's machine-readable descriptor, in declaration order.
pub fn shell_capabilities() -> Vec<ShellCapability> {
    let root_schema = schemars::schema_for!(ShellCommand);
    let root_value = serde_json::to_value(&root_schema).unwrap_or(serde_json::Value::Null);
    let one_of = root_value.get("oneOf").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    SHELL_COMMAND_CATALOG
        .iter()
        .enumerate()
        .map(|(index, meta)| ShellCapability {
            id: meta.id.to_string(),
            title: meta.title.to_string(),
            description: meta.description.to_string(),
            schema: one_of.get(index).cloned().unwrap_or(serde_json::Value::Null),
            observable_only: meta.observable_only,
        })
        .collect()
}
//#endregion 🛰️ShellCapability
