//! 📥️ Input, select, toggle and group nodes: the input, select item and select, toggle, group and key value nodes. See [`super`].

use super::*;

// #region ui
//#region 🔖Primitives
/// ⌨️ A retained text, number, date, colour or file field — the contract's `InputProps`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiInputNode {
    pub id: String,
    pub input_kind: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// 🎯️ Fraction digits a number field commits — the contract's `InputProps.precision`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    /// 📌️ A number field's detents — the contract's `InputProps.snaps`: its page keys jump between them and a
    /// committed detent is kept off the step ladder.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub snaps: Vec<f64>,
    /// 🪁️ A number field shows and reads `stored × display_factor` — the contract's `InputProps.display_factor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    /// 🚧️ The hard range a typed number must keep — the contract's `InputProps.limits`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ui_contract::UiNumberLimits>,
    pub on_change: ActionDescriptor,
    /// ⏎️ React's `SearchInput.onSubmit` — the line the user CONFIRMS with Enter, dispatched
    /// through [`ui_contract::Trigger::Submit`]. Distinct from `on_change`, which React fires on
    /// every keystroke of the same field (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`: `onChange` →
    /// `applyDraft`, `onKeyDown` Enter → `input.onSubmit(draft.trim())`), so a command line that
    /// feeds autocomplete while typing and runs the verb on Enter needs BOTH bound at once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<ActionDescriptor>,
    /// ⎋️ React's `SearchInput.onAbort` — Escape inside the field cancels the active engagement
    /// session and hands the line back to the program ([`ui_contract::Trigger::Abort`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub on_abort: Option<ActionDescriptor>,
    /// 🔁️ React's `SearchInput.onRepeatLast` — Space on an EMPTY idle line restarts the last
    /// finalized engagement (`applySearchSpaceAction`, [`ui_contract::Trigger::RepeatLast`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub on_repeat_last: Option<ActionDescriptor>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiSelectItem {
    pub value: String,
    pub label: Label,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiSelectNode {
    pub id: String,
    pub value: String,
    pub items: Vec<UiSelectItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    pub on_change: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiToggleNode {
    #[serde(default, skip_serializing_if = "ui_contract::ToggleAppearance::is_button")]
    #[value(default, skip_serializing_if = "ui_contract::ToggleAppearance::is_button")]
    pub appearance: ui_contract::ToggleAppearance,
    pub id: String,
    pub icon_id: IconName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<Label>,
    pub on_change: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

/** 🌿️ A nestable labeled container of `UiNode` children — the declarative-tree mechanism for
     * subtrees like `Origin > X/Y/Z`: `ui_declarative_child_to_tree_item` expands a `Group` into a
     * `UiTreeItemNode` whose `items` are its recursively-converted children, so depth composes to any
     * level (`Plane > Origin > X/Y/Z`). Unlike `UiSectionNode` (top-level tree sections only, see
     * `assertNoNestedTreeSections` on the TS side), a `Group` may itself appear as another `Group`'s or
     * `UiFieldNode`'s child. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiGroupNode {
    pub id: String,
    pub label: Label,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiKeyValueEntry {
    pub label: Label,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiKeyValueNode {
    pub entries: Vec<UiKeyValueEntry>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}
//#endregion 🔖️Primitives
// #endregion ui
