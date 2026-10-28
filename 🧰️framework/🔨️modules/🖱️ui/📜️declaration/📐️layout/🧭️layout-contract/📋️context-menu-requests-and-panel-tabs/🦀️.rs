//! 📋️ Context menu requests and panel tabs: the context menu hit, selection group, text context, surface target, point, request and response and the framework panel tab ids, labels and icons. See [`super`].

use super::*;

// #region layout
//#region 🔖️ContextMenu
/// 🖱️ Payload of the WIT `context-menu` export's request — mirrors `context-menu-request-json`'s
/// `json` string. `surface` carries scene-target info (`World3D`/`nodeGraph`/`tiledMap`/... hit-test
/// results); `menu` is the `UiMenuRef` the host resolved from `data-menu-id`/a scene surface
/// convention id (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuHit {
    pub domain: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuSelectionGroup {
    pub domain: String,
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuTextContext {
    pub caret: usize,
    pub has_selection: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    pub can_rename: bool,
    pub has_completions: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuSurfaceTarget {
    pub surface_id: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<ContextMenuHit>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<ContextMenuSelectionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<ContextMenuTextContext>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuPoint {
    pub x: f64,
    pub y: f64,
}

/// 📬️ The plugin-facing on-demand menu request — deliberately does NOT carry view state (this crate
/// must never reference `semio_framework`'s `ViewModel`, same boundary as every other type
/// here). Mirrors `handle_action`/`render`/`tool_measures`, which all take `view_state: &ViewModel`
/// as a separate `ArtifactApp` method parameter rather than embedding it in the request payload; the
/// plugin SDK's `plugin_context_menu` free function parses the WIT-level combined JSON (which DOES
/// carry `viewState`, matching the TS `PluginContextMenuRequest` wire shape) and splits it into this
/// smaller struct plus a typed `ViewModel` before calling `ArtifactApp::context_menu`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuRequest {
    pub menu: UiMenuRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub surface: Option<ContextMenuSurfaceTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub window_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub point: Option<ContextMenuPoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuResponse {
    pub items: Vec<ContextMenuItemSpec>,
}
//#endregion 🔖️ContextMenu

//#region 🔖️PanelTabConstants
pub const FRAMEWORK_PANEL_TAB_ARTIFACT_ID: &str = "framework.panel.artifact";

pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";

pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";

pub const FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL: &str = "Artifact";

pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";

pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";

pub const FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID: &str = "framework.panel.artifact";

pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: &str = "framework.panel.catalogue";

pub const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: &str = "framework.panel.inspection";

pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ID: &str = "framework.panel.parameters";

pub const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: &str = "Parameters";

pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: &str = "framework.panel.parameters";

/// 🕰️ Auto-injected into every app's `panel_tabs` by `AppBuilder::build_definition` — unlike
/// Document/Catalogue/Inspection/Parameters (per-app content, opt-in), the command-history panel's
/// content is framework-generic (`HistoryView`), so every app gets it unconditionally.
pub const FRAMEWORK_PANEL_TAB_HISTORY_ID: &str = "framework.panel.history";

pub const FRAMEWORK_PANEL_TAB_HISTORY_LABEL: &str = "History";

pub const FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID: &str = "framework.panel.history";

/// 🔑️ Reserved `body_key` intercepted first in `VcsArtifactApp::render`, before any app-specific
/// body-key match — both renderers fetch it like any other panel-tab body.
pub const FRAMEWORK_HISTORY_BODY_KEY: &str = "framework.body.history";

/// ⏯️ Auto-injected into the `panel_tabs` of every app that declares a tool run (`ToolRunDefinition` on a tool or
/// utility) by `AppBuilder::build_definition`: the framework ToolRun panel, one group per run, fetched as the
/// reserved body `framework.body.toolRun` and revealed by the shell when a run starts.
pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_ID: &str = "framework.panel.toolRun";

pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL: &str = "Tool runs";

pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_ICON_ID: &str = "framework.panel.toolRun";

/// 🗣️ Resolves a well-known framework panel-tab id to its native English/German label; unknown ids resolve to None so app-specific panel tabs are left untouched.
pub fn framework_panel_tab_label(id: &str, is_de: bool) -> Option<&'static str> {
    match (id, is_de) {
        (FRAMEWORK_PANEL_TAB_ARTIFACT_ID, false) => Some(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL),
        (FRAMEWORK_PANEL_TAB_ARTIFACT_ID, true) => Some("Dokument"),
        (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, false) => Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL),
        (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, true) => Some("Katalog"),
        (FRAMEWORK_PANEL_TAB_INSPECTION_ID, false) => Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        (FRAMEWORK_PANEL_TAB_INSPECTION_ID, true) => Some("Inspektion"),
        (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, false) => Some(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL),
        (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, true) => Some("Parameter"),
        (FRAMEWORK_PANEL_TAB_HISTORY_ID, false) => Some(FRAMEWORK_PANEL_TAB_HISTORY_LABEL),
        (FRAMEWORK_PANEL_TAB_HISTORY_ID, true) => Some("Verlauf"),
        (FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, false) => Some(FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL),
        (FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, true) => Some("Werkzeugläufe"),
        _ => None,
    }
}
//#endregion 🔖️PanelTabConstants
// #endregion layout
