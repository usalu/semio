//! 👥️ The presence channel: `PresenceUpdate`/`PeerMark`, TTL-scoped and coalesced. [Design notes](README.md#source-notes-1).

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Presence

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_false(value: &bool) -> bool {
    !*value
}

/// 🧭️ The activity lifecycle of a node, orthogonal to `disabled`/`transition` — was `UiStatus` on the
/// old wgpu target's `UiPresence`. Lives on the document (`crate::UiNodeRecord::activity`) because it
/// is genuinely part of what the node IS this revision, not an ephemeral input-frequency signal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub enum Activity {
    Waiting,
    Loading,
    #[default]
    Idle,
    Finished,
}

/// 🚩️ One OTHER peer's mark on a node — hover/selection dot plus initials chip. Ported faithfully
/// from the old wgpu target's `UiPeerMark` (contract-freeze §C7.6, ticket 26/08/17/SHARED-PRESENCE-
/// SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION) — `label` is still the actor id's display form, not
/// a free-text caption.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
#[derive(semio_framework_value::NativeReceive)]
pub struct PeerMark {
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    pub label: String,
}

/// 🙋️ This session's own hover/selection/preview state and palette color on a node — the local half
/// of the presence channel; every OTHER session's equivalent arrives as a [`PeerMark`] in `peers`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
#[derive(semio_framework_value::NativeReceive)]
pub struct OwnPresence {
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    /// 👁️ Mid-drag or mid-hover-preview emphasis distinct from `hovered` — e.g. previewing a drop
    /// target before release, or a `Trigger::HoverPreview` binding's target while armed.
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub previewed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
}

/// 📡️ One coalesced, TTL-scoped update on the presence channel, keyed `(surface, node_key)` — never [Design notes](README.md#source-notes-92).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
#[derive(semio_framework_value::NativeReceive)]
pub struct PresenceUpdate {
    pub surface: crate::SurfaceId,
    /// 🔑️ [`crate::UiNodeRecord::key`], not [`crate::UiNodeId`] — presence must still land on the
    /// right element across a reconciliation that reassigns ids but keeps keys stable.
    pub node_key: String,
    pub own: OwnPresence,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<PeerMark>,
    pub ttl_ms: u32,
}
//#endregion 🔖️Presence

//#region 🧪️Tests
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
