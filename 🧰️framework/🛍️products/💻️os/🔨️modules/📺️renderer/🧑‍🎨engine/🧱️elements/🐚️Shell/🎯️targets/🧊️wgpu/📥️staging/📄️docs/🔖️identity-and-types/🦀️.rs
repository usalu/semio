//! 📄️ `shell-docs` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID: &str = "framework.display.windows";

pub const COMPOSE_WINDOW_TEMPLATE_MIME: &str = "application/x-compose-window-template";

pub const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID: &str = "framework.display.layout";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowTemplateDragPayload {
    pub window_kind_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

pub fn window_template_drag_data(window_kind_id: &str, template_id: Option<&str>) -> HashMap<String, String> {
    let payload = WindowTemplateDragPayload { window_kind_id: window_kind_id.to_string(), template_id: template_id.map(str::to_string) };
    HashMap::from([(COMPOSE_WINDOW_TEMPLATE_MIME.to_string(), serde_json::to_string(&payload).expect("window template payload is infallible"))])
}

pub const FRAMEWORK_CHAT_PANEL_ID: &str = "framework.chat";

/// 🔄️ Byte-identical to React's `frameworkSyncTab` leaf id (`🏛️ShellHost/🟦️.tsx`'s `"s-sync-status"`),
/// the same id the footer check-in pill already registers.
pub const FRAMEWORK_SYNC_PANEL_TAB_ID: &str = "s-sync-status";

/// 🧵️ Byte-identical to React's Task Manager panel and row-action controller id.
pub const FRAMEWORK_TASK_MANAGER_PANEL_ID: &str = "os.task-manager";

/// 🛍️ Byte-identical to React's `FRAMEWORK_MARKETPLACE_TAB_ID` — the bottom-right Marketplace leaf.
pub const FRAMEWORK_MARKETPLACE_TAB_ID: &str = "framework.marketplace";

#[derive(Clone, Debug)]
pub struct SearchPaletteItem {
    pub id: String,
    pub label: String,
    /// 🔍️ The palette's secondary line — a command's resolved chords, exactly what React's `UISearch`
    /// item puts there (`description: commandKeybindingChords(...)`). Ranked at weight 1 (see
    /// `filtered_search_items`), so typing a chord finds its command.
    pub description: Option<String>,
    pub group: String,
    pub dispatch_action: Option<ActionDescriptor>,
    pub action: Option<String>,
    /// 🗂️ Coarse command-source tag (os/plugin/app/mode) — `None` for the pre-existing panel/window/
    /// keybinding/action/studio entries, `Some(..)` for entries derived from `shell::ActionPanelAndUtilities`'s
    /// `ResolvedCommand` aggregation (see `command_search_items`).
    pub category: Option<semio_framework::manifest::CommandOwnerAddress>,
}

//#region ShellTypes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub breadcrumb: Vec<String>,
}

/// 📌️ Host-owned panel state, serialized into `ViewModel::panel_json`.
///
/// 🪶️ It deliberately carries NO app roster. The roster is one [`SpaceProgramEntry`] per app of
/// every loaded plugin — it grew with the plugin closure, was written on every panel commit and was
/// never read back ([`Self::build_space_workflows`] recomputes it from `self.plugins` wherever it is
/// actually needed), so it was pure inflation of the one long string a view context may carry
/// (65 536 characters, `🪟️view-context/🧬️schema/🔣️.json`). What remains is bounded by the open
/// spawned instances (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpacePanelState {
    pub active_panel_tab: String,
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

//#endregion ShellTypes

/// 🛠️ One mode tool's leaf id under the Tool branch — React's `tool.<id>` (`🛠️ShellHelpers/🟦️.tsx`'s
/// `buildToolTabs`/`toolIdFromPanelTabId`). The leaf tab IS the activation control.
pub const FRAMEWORK_TOOL_PANEL_TAB_PREFIX: &str = "tool.";

/// 🎛️ One command category's leaf id under the Command branch — React's `command.category.<id>`.
pub const FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX: &str = "command.category.";
