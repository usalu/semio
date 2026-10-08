//! 🧬️ Directed owned scene descriptors and snapshot records.

use serde::{Deserialize, Serialize};

pub use crate::infinite::board::ports::HandleDescriptor;
pub use crate::infinite::board::{CameraDescriptor, NodeDescriptor};

/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct EdgeDescriptor {
    pub id: String,
    pub source: String,
    pub target: String,
    /// 🧩️ Semantic edge-kind id for compatibility at `edge` specificity.
    #[serde(default)]
    #[value(default)]
    pub edge_kind: Option<String>,
    /// 🔺️ Per-instance source tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    #[value(default)]
    pub source_tip: Option<String>,
    /// 🔺️ Per-instance target tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    #[value(default)]
    pub target_tip: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
}

/// 🧵️ Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
///
/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WireDescriptor {
    pub id: String,
    pub source: String,
    /// 🧩️ Semantic wire-kind id (defaults from catalog when omitted in fixtures).
    #[serde(default)]
    #[value(default)]
    pub wire_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub target: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub end_x: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub end_y: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
}

/// 🎯️ One `targetRegions` row of the owning document, as the descriptor spells it. Derived, not
/// hand-written: unlike its node/edge siblings a region carries no free-form `userData`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct RegionDescriptor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    #[value(default)]
    pub label: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SceneDescriptor {
    pub nodes: Vec<NodeDescriptor>,
    pub handles: Vec<HandleDescriptor>,
    pub edges: Vec<EdgeDescriptor>,
    #[serde(default)]
    #[value(default)]
    pub wires: Vec<WireDescriptor>,
    /// 🎯️ Fill-constraining rectangles painted beneath every entity and hit-tested after
    /// all of them — absent on a board that declares none.
    #[serde(default)]
    #[value(default)]
    pub regions: Vec<RegionDescriptor>,
    /// 💠️ JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
    #[serde(default)]
    #[value(default)]
    pub selection_exit_highlight_ids: Vec<String>,
}

#[path="📸️snapshot/🦀️.rs"]
pub mod snapshot;
pub use snapshot::*;
