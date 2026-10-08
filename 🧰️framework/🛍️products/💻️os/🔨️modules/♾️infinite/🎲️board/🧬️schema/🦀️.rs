//! 🧬️ Port-agnostic owned board descriptors.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct CameraDescriptor {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NodeDescriptor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    #[value(default)]
    pub draggable: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub text: Option<String>,
    /// 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
    #[serde(default)]
    #[value(default)]
    pub icon_kind: Option<String>,
    /// 🧩️ Semantic node-kind id for compatibility rows at `node` specificity.
    #[serde(default)]
    #[value(default)]
    pub node_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub root: Option<bool>,
    #[value(default)]
    pub shape: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub width: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub height: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub scale: Option<f64>,
}

#[path = "👁️visibility/🦀️.rs"]
pub mod visibility;
pub use visibility::{BoardVisibility, board_visible_option, board_visible_or_true, board_locked_option};
#[path = "🎨️palette/🦀️.rs"]
pub mod palette;
pub use palette::{BoardPalette, BoardPaletteOverlay};

/// 🔗️ Requires both semantic endpoint identities without serialized object lookup.
pub fn board_edge_handle_ids<'a>(source:Option<&'a str>,target:Option<&'a str>)->Option<(&'a str,&'a str)>{Some((source?,target?))}

#[path="🔺️edge-tip/🦀️.rs"]
pub mod edge_tip;

#[path="📐️layout/🦀️.rs"]
pub mod layout;

#[path="💡️inferences/📐️layout/🦀️.rs"]
pub mod layout_inferences;

#[path="🎯️dag-input/🦀️.rs"]
pub mod dag_input;
