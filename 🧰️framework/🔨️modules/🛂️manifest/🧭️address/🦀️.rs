//! 🧭️ Manifest addressing: action, tool and utility references, command addresses, keybindings and element ids.

use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use semio_framework_ui_declaration::layout::ActionDescriptor;

// #region 🛂️Manifest
//#region 🔖️Manifest

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// ⌨️ Operating system selector for a platform-specific keybinding.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Platform {
    MacOs,
    Windows,
    Linux,
}

/// ⌨️ One command chord, optionally restricted to a host platform.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PlatformKeybinding {
    pub chord: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
}

impl PlatformKeybinding {
    pub fn new(chord: impl Into<String>) -> Self {
        Self { chord: chord.into(), platform: None }
    }

    pub fn for_platform(chord: impl Into<String>, platform: Platform) -> Self {
        Self { chord: chord.into(), platform: Some(platform) }
    }
}

/// 📇️ A relative action id used by declarations nested beneath an owning window kind.
/// Distinct from `ActionAddress`, which qualifies a dispatched invocation down to a window instance.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct ActionRef(String);

// 🚫️async: E1 — `new` is a pure single-field wrapper, zero suspension points, same rationale
// already applied to `as_str` below; reverted per R9 (sync closure / catch_unwind consumers).
impl ActionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    // 🚫️async: E1 transitive — the only consumer is inside an `Iterator::find` (external trait)
    // closure, which must be sync; pure field access, no I/O (R9).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ActionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ActionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// 📍️ Fully qualified address of an action owned by one concrete window instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ActionAddress {
    pub plugin_id: String,
    pub app_id: String,
    pub mode_id: String,
    pub window_kind_id: String,
    pub window_instance_id: String,
    pub action_id: String,
}

/// 📨️ One addressed action invocation with named DSL-value arguments. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: `arguments` used to be
/// keyed to `serde_json::Value` (not a `ToValue`/`FromValue` target by design); `DslValue` carries
/// the exact same schema-less-JSON shape without the serde dependency.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub address: ActionAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

//#region 🔖️Utilities
/// 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
/// `ActionRef`, scoping utilities to window kinds/modes with a typed, resolvable id.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct UtilityRef(String);

impl UtilityRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for UtilityRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for UtilityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

//#endregion 🔖️Utilities
//#region 🔖️Commands
/// 📍️ Hierarchical owner of a command definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CommandOwnerAddress {
    Os,
    Plugin { plugin_id: String },
    App { plugin_id: String, app_id: String },
    Mode { plugin_id: String, app_id: String, mode_id: String },
}

/// 📍️ Fully qualified address of one command.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct CommandAddress {
    pub owner: CommandOwnerAddress,
    pub command_id: String,
}

/// 📨️ One addressed command invocation with named DSL-value arguments — see
/// `ActionInvocation`'s docstring for why `arguments` is keyed to `DslValue`, not
/// `serde_json::Value`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub address: CommandAddress,
    pub arguments: BTreeMap<String, DslValue>,
}

/// 👓️ Plugin handlers borrow the retained semantic fields instead of materializing a second argument map.
pub struct CommandInvocationView<'a>{pub address:&'a CommandAddress,pub arguments:&'a DslValue}

//#endregion 🔖️Commands
//#region 🔖️Tools
/// 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
/// `UtilityRef`, scoping tools to modes with a typed, resolvable id.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct ToolRef(String);

impl ToolRef {
    pub async fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    // 🚫️async: E1 transitive — the only consumer is inside an `Iterator::find` (external trait)
    // closure, which must be sync; pure field access, no I/O (R9).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ToolRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

//#endregion 🔖️Tools
//#region 🆔️ElementId

/// 🆔️ Whether `id` matches the renderer-agnostic UI element id grammar: dot-separated segments,
/// each starting with a lowercase letter and continuing with letters/digits only (camelCase, no
/// hyphens/underscores) — e.g. `framework.window.main.action.addLayer`. This id is the single
/// integration key across i18n, tooltips, hotkeys, command origin tracking, tutorials, E2E selectors,
/// and introduction anchors; each renderer maps it onto its own element (React → DOM `id` attribute,
/// wgpu → hit-target `control_id`), so no renderer-specific shape leaks into the grammar itself.
pub fn is_element_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.split('.').all(|segment| {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) if first.is_ascii_lowercase() => chars.all(|c| c.is_ascii_alphanumeric()),
            _ => false,
        }
    })
}

/// 🆔️ Normalizes arbitrary input (a domain object's own id, a free-text label, an already
/// grammar-safe word) into a single camelCase element-id segment: splits on `-`/`_`/` `/`.`, lowercases
/// the very first character, capitalizes the first character after each separator, and drops any other
/// non-alphanumeric character. Idempotent on input that is already a valid segment. Used as the last
/// resort by `child_element_id` when a child id is derived from something not already grammar-safe (e.g.
/// a runtime label) — prefer a real semantic key first, then this, then a numeric index.
pub fn element_id_segment(raw: &str) -> String {
    let mut segment = String::new();
    let mut capitalize_next = false;
    for ch in raw.chars() {
        if ch == '-' || ch == '_' || ch == ' ' || ch == '.' {
            capitalize_next = true;
            continue;
        }
        if !ch.is_ascii_alphanumeric() {
            continue;
        }
        if segment.is_empty() {
            segment.push(ch.to_ascii_lowercase());
        } else if capitalize_next {
            segment.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            segment.push(ch);
        }
    }
    segment
}

/// 🆔️ Derives a child element id by suffixing `parent` with one or more segments, each normalized
/// through `element_id_segment` — the hierarchical mechanism every composite element uses to name its
/// parts instead of a context/registry: `child_element_id("ui.chat", &["send"])` → `"ui.chat.send"`.
pub fn child_element_id(parent: &str, segments: &[&str]) -> String {
    let mut id = parent.to_string();
    for segment in segments {
        id.push('.');
        id.push_str(&element_id_segment(segment));
    }
    id
}

/// 🆔️ Element id of the app shell's navbar — singular, shell-owned chrome.
pub const UI_NAVBAR_ELEMENT_ID: &str = "ui.navbar";

/// 🆔️ Element id of the app shell's footer — singular, shell-owned chrome.
pub const UI_FOOTER_ELEMENT_ID: &str = "ui.footer";

/// 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`.
pub fn window_element_id(kind_id: &str) -> String {
    child_element_id("framework.window", &[kind_id])
}

/// 🆔️ Element id of a panel tab's uncollapsed panel body. `tab_id` is already a dotted
/// `PanelTabDefinition.id()` (e.g. `puzzle.catalogue`) — appended verbatim rather than through
/// `child_element_id`, which would collapse its dots into camelCase.
pub fn panel_tab_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}")
}

/// 🆔️ Alias id of the first draggable tree row inside a panel tab (document order within that
/// uncollapsed panel) — stamped via `data-element-alias` since no single tree row has a stable semantic
/// id at authoring time. Used to teach catalogue drag-and-drop without hardcoding a kind id.
pub fn panel_tab_first_draggable_element_id(tab_id: &str) -> String {
    format!("framework.panelTab.{tab_id}.firstDraggable")
}

//#endregion 🆔️ElementId
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
