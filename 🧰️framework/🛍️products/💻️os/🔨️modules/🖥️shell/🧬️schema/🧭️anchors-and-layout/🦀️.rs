//! 🧭️ Anchors and layout of the shell schema: panel anchors, per-anchor values, the dock layout tree, dock state, extra window instances and icon names. See [`super`].

use super::*;

//#region 🧭️Anchor
/// 🧭️ The four docking edges a panel can attach to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Anchor {
    Left,
    Right,
    Top,
    Bottom,
}

impl Anchor {
    pub const ALL: [Anchor; 4] = [Anchor::Left, Anchor::Right, Anchor::Top, Anchor::Bottom];
}

/// 🧭️ One value per [`Anchor`] — `Anchor` is a small closed set (4 edges), so a fixed struct
/// avoids the "enum as HashMap key" JSON-serialization pitfall (serde_json map keys must reduce
/// to strings; a 4-field struct sidesteps the question entirely and is simpler to reason about).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ByAnchor<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Clone> ByAnchor<T> {
    pub fn uniform(value: T) -> Self {
        Self { left: value.clone(), right: value.clone(), top: value.clone(), bottom: value }
    }

    pub fn get(&self, anchor: Anchor) -> &T {
        match anchor {
            Anchor::Left => &self.left,
            Anchor::Right => &self.right,
            Anchor::Top => &self.top,
            Anchor::Bottom => &self.bottom,
        }
    }

    pub fn set(&mut self, anchor: Anchor, value: T) {
        match anchor {
            Anchor::Left => self.left = value,
            Anchor::Right => self.right = value,
            Anchor::Top => self.top = value,
            Anchor::Bottom => self.bottom = value,
        }
    }
}
//#endregion 🧭️Anchor

//#region 🧱️LayoutNode
/// 🧱️ How two child regions of a split are arranged.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// 🧱️ Minimal local mirror of the real `WindowLayoutNode`/`DockSkeleton` tree types (audit
/// `📓️luna-shellstate-audit.md` §1, `shellLayout`/`dockOverride` rows). The canonical types live
/// in the React/wgpu shells' own modules, which this crate must not depend on (§4 of the packet:
/// "if depending on `semio-framework` drags in a mid-rewrite crate, define the minimal local
/// mirror and say so"). `dockOverride` and `shellLayout` are structurally the same kind of tree
/// (window ids at the leaves, nested splits), so one recursive type serves both; reconciling this
/// with the real tree types is later-packet adoption work.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum LayoutNode {
    Leaf { window_id: String },
    Split { orientation: SplitOrientation, children: Vec<LayoutNode>, sizes: Vec<f32> },
}

/// 🗂️ Restored dock UI state (`HYDRATE_DOCK_UI` row) — a layout tree plus per-anchor visibility,
/// the shape the audit's `DockUiState` payload carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct DockUiState {
    pub layout: Option<LayoutNode>,
    pub panels_visible: ByAnchor<bool>,
}
//#endregion 🧱️LayoutNode

//#region 🪟️Windows
/// 🪟️ One spawned/extra window instance (audit `ExtraWindowInstance`). `params` mirrors the same
/// free-form seed shape `OpenWindow`'s kernel effect carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ExtraWindowInstance {
    pub window_id: String,
    pub kind: String,
    #[value(with = "json_value_option_bridge")]
    pub params: Option<JsonValue>,
}

/// 🖼️ Stable icon identifier. Local newtype mirror — the real `IconName` lives in the UI token
/// crate this module must not depend on (§4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[value(crate = "::protocol::value", transparent)]
pub struct IconName(pub String);
//#endregion 🪟️Windows
