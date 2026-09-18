//! @emoji 📑️ wgpu twin of the `Tabs` element — ticket `26/09/17/WGPU-RENDERER-REACT-PARITY` packet
//! W2k. `Tabs` was one of the elements the plugin/react-only census flagged as "no confirmed product
//! consumer"; the consumer-hunt this packet ran found a real one —
//! `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️.tsx:64-91` renders a live
//! `Tabs`/`TabsList`/`TabsTrigger`/`TabsContent` tree — so this is the one element in that group that
//! genuinely needed a twin (the rest are documented as not needing one in the packet report).
//!
//! Shaped like the `👥️PresenceBar` twin beside it: a declarative `UiNode` builder plus pure decision
//! functions, so it needs only the light `wgpu` feature, never `wgpu-engine`. React twin: `🟦️.tsx` in
//! this same folder.
//!
//! The keyboard table below is React's `moveTabFocus` (`🟦️.tsx:163-181`) rule for rule, including the
//! `dir === "rtl"` inversion of the horizontal pair — which is the whole reason this twin takes a
//! `FlowInline`: a mirrored tablist must step the other way for the same key.

use crate::wgpu::{ActionDescriptor, IconName, Label, UiButtonNode, UiNode, UiPresence, UiStackNode, UiState, UiTextNode};
use ui_contract::FlowInline;

//#region 📑️Vocabulary

/// 📑️ Which axis a tablist runs along — React's `TabsOrientation` (`🟦️.tsx`'s `Tabs` props). Decides
/// which arrow pair moves focus and which pair is inert.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// 📑️ Whether moving focus also activates the tab — React's `TabsActivationMode`. `Automatic` is
/// React's default: arrowing to a tab selects it. Under `Manual`, `Enter`/`Space` selects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsActivationMode {
    #[default]
    Automatic,
    Manual,
}

/// 📑️ One trigger in a tablist.
#[derive(Clone, Debug, PartialEq)]
pub struct TabRow {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl TabRow {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }

    pub fn disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: true }
    }
}

/// ⌨️ What one key did to a tablist — the wgpu twin of React's `moveTabFocus` plus the `Manual`
/// activation branch (`🟦️.tsx:225-230`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsKey {
    /// 🚫️ Not a key this tablist claims — the host's generic routing keeps it.
    Ignored,
    /// 🎯️ Move roving focus to this index, WITHOUT selecting it (`Manual` activation).
    Focus(usize),
    /// ✅️ Move roving focus to this index and select it (`Automatic` activation, or `Enter`/`Space`).
    Activate(usize),
}

//#endregion 📑️Vocabulary

//#region ⌨️Keyboard

/// ⌨️ The key that steps BACKWARD through this tablist, and the one that steps forward — React's
/// `previousKey`/`nextKey` (`🟦️.tsx:164-168`). A vertical tablist uses `ArrowUp`/`ArrowDown` and
/// carries no flow direction; a horizontal one mirrors its pair under [`FlowInline::Rtl`].
pub fn tabs_step_keys(orientation: TabsOrientation, flow: FlowInline) -> (&'static str, &'static str) {
    match orientation {
        TabsOrientation::Vertical => ("ArrowUp", "ArrowDown"),
        TabsOrientation::Horizontal if flow.is_rtl() => ("ArrowRight", "ArrowLeft"),
        TabsOrientation::Horizontal => ("ArrowLeft", "ArrowRight"),
    }
}

/// ⌨️ One key against a tablist whose roving focus sits on `focused`. React skips DISABLED triggers
/// entirely (`'[role="tab"]:not(:disabled)'`, `🟦️.tsx:170`), wraps at both ends, and answers
/// `Home`/`End` with the first and last ENABLED trigger — so this indexes the enabled subsequence and
/// maps back, rather than stepping the raw list.
pub fn tabs_key(rows: &[TabRow], focused: usize, key: &str, orientation: TabsOrientation, activation: TabsActivationMode, flow: FlowInline) -> TabsKey {
    let enabled: Vec<usize> = rows.iter().enumerate().filter(|(_, row)| !row.disabled).map(|(index, _)| index).collect();
    if enabled.is_empty() {
        return TabsKey::Ignored;
    }
    if matches!(activation, TabsActivationMode::Manual) && matches!(key, "Enter" | " ") {
        return if rows.get(focused).is_some_and(|row| !row.disabled) { TabsKey::Activate(focused) } else { TabsKey::Ignored };
    }
    let (previous_key, next_key) = tabs_step_keys(orientation, flow);
    let Some(position) = enabled.iter().position(|&index| index == focused) else { return TabsKey::Ignored };
    let target = if key == "Home" {
        0
    } else if key == "End" {
        enabled.len() - 1
    } else if key == previous_key {
        (position + enabled.len() - 1) % enabled.len()
    } else if key == next_key {
        (position + 1) % enabled.len()
    } else {
        return TabsKey::Ignored;
    };
    let index = enabled[target];
    match activation {
        TabsActivationMode::Automatic => TabsKey::Activate(index),
        TabsActivationMode::Manual => TabsKey::Focus(index),
    }
}

//#endregion ⌨️Keyboard

//#region 📑️Build

/// 📑️ The `role="tab"` id React stamps as `data-tabs-value` — the join a host addresses a trigger by.
pub fn tab_trigger_id(id: &str, value: &str) -> String {
    format!("{id}.trigger.{value}")
}

/// 📑️ The `role="tabpanel"` id.
pub fn tab_content_id(id: &str, value: &str) -> String {
    format!("{id}.content.{value}")
}

/// 📑️ The tablist as a declarative `UiNode` tree: a horizontal `Stack` of `Button` triggers, each
/// carrying `on_activate`'s descriptor so a click reaches the same verb React's `context.activate`
/// does. The SELECTED trigger is `UiState::Selected` and a disabled one `UiState::Disabled`, so paint
/// resolves React's `data-state="active"` (`presence.selected`) and `disabled:opacity-50`
/// (`UiState::Disabled`) from presence rather than from a bespoke flag. Only the active panel is materialised — React's `TabsContent` returns `null` for an
/// inactive value (`🟦️.tsx:236`), so an inactive subtree must not exist here either.
pub fn build_tabs(id: &str, rows: &[TabRow], value: &str, orientation: TabsOrientation, content: Option<UiNode>, action: ActionDescriptor) -> UiNode {
    let triggers: Vec<UiNode> = rows
        .iter()
        .map(|row| {
            let mut descriptor = action.clone();
            descriptor.args = Some(dsl::DslValue::Object(vec![("value".to_string(), dsl::DslValue::String(row.value.clone()))]));
            UiNode::Button(UiButtonNode {
                id: Some(tab_trigger_id(id, &row.value)),
                icon_id: IconName::CircleDot,
                label: Label::data(row.label.clone()),
                action: descriptor,
                style: None,
                presence: UiPresence { state: if row.disabled { UiState::Disabled } else { UiState::Normal }, selected: !row.disabled && row.value == value, ..UiPresence::default() },
                menu: None,
            })
        })
        .collect();
    let list = stack(format!("{id}.list"), matches!(orientation, TabsOrientation::Horizontal), triggers);
    let panel = content.unwrap_or_else(|| stack(tab_content_id(id, value), false, vec![UiNode::Text(UiTextNode { value: Label::data(String::new()), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));
    stack(id.to_string(), false, vec![list, panel])
}

/// 📑️ A plain `Stack` wrapper — `direction` is the string vocabulary the contract's own stack carries.
fn stack(id: String, horizontal: bool, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: if horizontal { "row".to_string() } else { "column".to_string() }, gap: None, padding: None, id: Some(id), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None, children })
}

//#endregion 📑️Build

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
