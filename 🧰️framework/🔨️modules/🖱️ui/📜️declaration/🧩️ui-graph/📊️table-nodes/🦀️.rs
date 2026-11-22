//! 📊️ Table nodes: the retained column-headed table and its rows, projected to `Component::Table`/`Component::TableRow`. See [`super`].

use super::*;

// #region ui
//#region 🔖️TableNodes
/// 📊️ One row of a [`UiTableNode`]: positional text `cells`, one activation and trailing row `actions` on one target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTableRowNode {
    pub id: String,
    pub cells: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<UiTreeItemAction>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
}

/// 📊️ A column-headed data table: React's `Table` (`label` is its accessible name, `actions_label` heads the trailing actions column).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTableNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub label: Label,
    pub columns: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub actions_label: Option<Label>,
    pub rows: Vec<UiTableRowNode>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}
//#endregion 🔖️TableNodes
// #endregion ui
