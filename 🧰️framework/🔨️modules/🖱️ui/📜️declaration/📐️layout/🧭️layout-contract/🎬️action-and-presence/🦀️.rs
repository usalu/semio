//! 🎬️ Action and presence: the action descriptor, the style spec, the ui state and status, the peer mark and the `UiPresence` with their serde defaults. See [`super`].

use super::*;

// #region layout
//#region 🔖️Action
/// 🎬️ One action a control fires; the default (an empty action name) is the unbound marker `fired_action` answers `None`
/// for.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionDescriptor {
    pub controller_id: String,
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<DslValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
}
//#endregion 🔖️Action

//#region 🔖️Presence
/// 🧭️ The one shared, mandatory visual state every rendered UI element carries — orthogonal to
/// `status` and to the `hover`/`selected` flags. `Hidden` short-circuits everything else: a hidden
/// element is not rendered at all (no layout, no paint, no events) — renderers/reconcile must check
/// this before doing anything with the rest of an element's `UiPresence`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiState {
    Introducing,
    Celebrating,
    Previewed,
    #[default]
    Normal,
    Disabled,
    Hidden,
}

/// 🧭️ The activity lifecycle of a UI element, orthogonal to [`UiState`] and composable with it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiStatus {
    Waiting,
    Loading,
    #[default]
    Idle,
    Finished,
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde skip_serializing_if) — see R9
pub(super) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
/// mandatory `presence` field: `state` × `status` × `hover` × `selected`. All combinations are
/// visually distinguishable except `state == Hidden`, which makes the rest irrelevant — see
/// [`UiPresence::visible`]. Defaults to fully inert (`Normal`/`Idle`/`false`/`false`) and is omitted
/// from the wire format entirely at default (see `UiPresence::is_default`).
/// 👥️ One peer's mark on the element carrying this `UiPresence` — hover/selection dot plus
/// initials chip (contract-freeze §C7.6 of ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
/// SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`). `label` is the actor id
/// itself (no display name is carried this far down the stack — see `PeerPresence`'s own doc
/// comment in the plugin crate); a renderer that has the full roster may substitute a friendlier
/// name, but every renderer must always carry SOME text alongside color (never color alone).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiPeerMark {
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub hovered: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub selected: bool,
    pub label: String,
}

/// 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
/// mandatory `presence` field: `state` × `status` × `hover` × `selected` × own `color` × peer
/// `marks`. All combinations are visually distinguishable except `state == Hidden`, which makes
/// the rest irrelevant — see [`UiPresence::visible`]. Defaults to fully inert
/// (`Normal`/`Idle`/`false`/`false`/`None`/`[]`) and is omitted from the wire format entirely at
/// default (see `UiPresence::is_default`). `color`/`peers` (ticket 26/08/17/SHARED-PRESENCE-
/// SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) make `UiPresence` `Clone`-only — no
/// longer `Copy` — since `peers: Vec<UiPeerMark>` owns heap data; `UiNode::presence()`/
/// `UiControlNode::presence()` therefore return `&UiPresence`, not a by-value copy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct UiPresence {
    #[serde(skip_serializing_if = "is_default")]
    #[value(skip_serializing_if = "is_default")]
    pub state: UiState,
    #[serde(skip_serializing_if = "is_default")]
    #[value(skip_serializing_if = "is_default")]
    pub status: UiStatus,
    #[serde(skip_serializing_if = "is_default")]
    #[value(skip_serializing_if = "is_default")]
    pub hover: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[value(skip_serializing_if = "is_default")]
    pub selected: bool,
    /// 🎨️ This session's own hub-assigned palette index — stamped onto every `interaction_domain`-
    /// bound tree item by `ui_tree_stamp_presence`, `None` for a folder-only session with no hub.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
    /// 👥️ Every OTHER peer currently marking this element (hover and/or selection), sorted by
    /// actor.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<UiPeerMark>,
}

impl UiPresence {
    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde skip_serializing_if) — see R9
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    /// 🙈️ `false` for `state == Hidden` — callers must not render, lay out, or hit-test the element.
    pub fn visible(&self) -> bool {
        self.state != UiState::Hidden
    }
    pub fn state(state: UiState) -> Self {
        Self { state, ..Self::default() }
    }
    pub fn status(status: UiStatus) -> Self {
        Self { status, ..Self::default() }
    }
    pub fn selected(selected: bool) -> Self {
        Self { selected, ..Self::default() }
    }
    pub fn with_state(self, state: UiState) -> Self {
        Self { state, ..self }
    }
    pub fn with_status(self, status: UiStatus) -> Self {
        Self { status, ..self }
    }
    pub fn with_hover(self, hover: bool) -> Self {
        Self { hover, ..self }
    }
    pub fn with_selected(self, selected: bool) -> Self {
        Self { selected, ..self }
    }
    /// 🙈️ `Hidden` when `hidden`, else `Normal` — the one-line migration for today's `is_hidden`
    /// flags on elements that are genuinely not rendered (not to be confused with a domain "dimmed"
    /// prop, e.g. a tree item's eye-toggle, which must stay visible/clickable).
    pub fn hidden_if(hidden: bool) -> Self {
        Self::state(if hidden { UiState::Hidden } else { UiState::Normal })
    }
    /// 🚫️ `Disabled` when `disabled`, else `Normal` — the one-line migration for today's `disabled` flags.
    pub fn disabled_if(disabled: bool) -> Self {
        Self::state(if disabled { UiState::Disabled } else { UiState::Normal })
    }
    /// 🎉️ `Celebrating` when `celebrating`, else `Normal` — the transient completion emphasis fired e.g.
    /// when an introduction step advances by the user performing the taught action.
    pub fn celebrate_if(celebrating: bool) -> Self {
        Self::state(if celebrating { UiState::Celebrating } else { UiState::Normal })
    }
}

impl UiStatus {
    /// 🌀️ The one-line migration for today's independent `loading`/`waiting` flag pairs — `loading`
    /// wins precedence when both are set, matching the prior ad-hoc convention.
    pub fn busy(loading: bool, waiting: bool) -> Self {
        if loading {
            UiStatus::Loading
        } else if waiting {
            UiStatus::Waiting
        } else {
            UiStatus::Idle
        }
    }
}
//#endregion 🔖️Presence
// #endregion layout
