//! 🔌️ Port graph layer: handles and port descriptors on generic graph engine.

pub use crate::infinite::board::*;

use serde::{Deserialize, Serialize};

// #region 🔖️HandleDescJson
/// 🌉️ Hand-written, not derived: `user_data: Option<serde_json::Value>` has no `ToValue`/
/// `FromValue` for `serde_json::Value` (only the `DslValue <-> serde_json::Value` `From` bridges in
/// `🌱️value/🦀️.rs` exist) — same reason as `graph::manifest::KindDef`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleDescJson {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    #[serde(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub handle_kind: Option<String>,
    /// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
    #[serde(default)]
    pub color: Option<String>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    #[serde(default)]
    pub icon_kind: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    pub scale: Option<f64>,
}

impl semio_framework_value::ToValue for HandleDescJson {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("nodeId".to_string(), semio_framework_value::ToValue::to_value(&self.node_id)),
            ("angle".to_string(), semio_framework_value::ToValue::to_value(&self.angle)),
            ("radius".to_string(), semio_framework_value::ToValue::to_value(&self.radius)),
            ("selected".to_string(), semio_framework_value::ToValue::to_value(&self.selected)),
            ("style".to_string(), semio_framework_value::ToValue::to_value(&self.style)),
            ("handleKind".to_string(), semio_framework_value::ToValue::to_value(&self.handle_kind)),
            ("color".to_string(), semio_framework_value::ToValue::to_value(&self.color)),
            ("iconKind".to_string(), semio_framework_value::ToValue::to_value(&self.icon_kind)),
            ("userData".to_string(), match &self.user_data { Some(v) => semio_framework_value::DslValue::from(v), None => semio_framework_value::DslValue::Null }),
            ("visible".to_string(), semio_framework_value::ToValue::to_value(&self.visible)),
            ("locked".to_string(), semio_framework_value::ToValue::to_value(&self.locked)),
            ("scale".to_string(), semio_framework_value::ToValue::to_value(&self.scale)),
        ])
    }
}

impl semio_framework_value::FromValue for HandleDescJson {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for HandleDescJson, found {value:?}")));
        };
        let mut id = None;
        let mut node_id = None;
        let mut angle = None;
        let mut radius = None;
        let mut selected = None;
        let mut style = None;
        let mut handle_kind = None;
        let mut color = None;
        let mut icon_kind = None;
        let mut user_data = None;
        let mut visible = None;
        let mut locked = None;
        let mut scale = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "nodeId" => node_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("nodeId"))?),
                "angle" => angle = Some(<f64 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("angle"))?),
                "radius" => radius = <Option<f64> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("radius"))?,
                "selected" => selected = <Option<bool> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                "style" => style = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("style"))?,
                "handleKind" => handle_kind = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("handleKind"))?,
                "color" => color = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("color"))?,
                "iconKind" => icon_kind = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("iconKind"))?,
                "userData" => user_data = if matches!(entry, semio_framework_value::DslValue::Null) { None } else { Some(serde_json::Value::from(&entry)) },
                "visible" => visible = <Option<bool> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("visible"))?,
                "locked" => locked = <Option<bool> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("locked"))?,
                "scale" => scale = <Option<f64> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("scale"))?,
                _ => {}
            }
        }
        Ok(HandleDescJson {
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "HandleDescJson missing id"))?,
            node_id: node_id.ok_or_else(|| semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "HandleDescJson missing nodeId"))?,
            angle: angle.ok_or_else(|| semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "HandleDescJson missing angle"))?,
            radius,
            selected,
            style,
            handle_kind,
            color,
            icon_kind,
            user_data,
            visible,
            locked,
            scale,
        })
    }
}
// #endregion 🔖️HandleDescJson

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
