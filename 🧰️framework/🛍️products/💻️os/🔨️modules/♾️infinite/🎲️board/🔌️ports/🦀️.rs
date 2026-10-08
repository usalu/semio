//! 🔌️ Port graph layer: handles and port descriptors on generic graph engine.

pub use crate::infinite::board::*;

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
pub use schema::HandleDescriptor;

// #region 🔖️HandleKinds
use canvas::Color;

#[derive(Clone, Debug)]
pub struct HandleKindDef {
    pub name: String,
    pub color: Color,
    pub default_wire_kind: Option<String>,
    pub scale: f64,
}

#[derive(Clone, Debug)]
pub struct NodeKindHandleTemplate {
    pub handle_kind: String,
    pub angle: f64,
    pub radius: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct HandleData {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    pub radius: f64,
    pub scale: f64,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub style: Option<String>,
    pub handle_kind: String,
    /// Parsed from descriptor `color` when set (overrides catalog fill).
    pub color_fill: Option<Color>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    pub icon_kind: Option<String>,
    pub properties: PropertyBag,
}
// #endregion 🔖️HandleKinds
