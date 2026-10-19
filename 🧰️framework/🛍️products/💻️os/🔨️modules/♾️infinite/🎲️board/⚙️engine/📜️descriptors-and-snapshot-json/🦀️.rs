//! 📜️ Descriptors and snapshot JSON of the board engine: the edge, wire, region, scene and handle descriptors, the scene and snapshot codecs, the graph extension and the stroke scene encoder. See [`super`].

use super::*;

/// ➡️ Port graph engine with directed handle endpoints.
pub type DirectedPortGraphEngine = GraphEngine<Ported, Directed>;

/// ⚙️ Directed port board engine alias.
pub type BoardEngine = DirectedPortGraphEngine;

/// 🪢️ Cubic edge connecting two handles (legacy field names).
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub source_handle: HandleId,
    pub target_handle: HandleId,
}

// #region 🔖️EdgeEndpointResolution
/// 🔗️ Resolves a ported edge endpoint to a node id (handle lookup, then node id).
fn resolve_endpoint_node_id(endpoint_id: &str, handle_to_node: &std::collections::HashMap<String, String>) -> String {
    handle_to_node.get(endpoint_id).cloned().unwrap_or_else(|| endpoint_id.to_string())
}
// #endregion 🔖️EdgeEndpointResolution

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
    /// 🌈️ Parsed from descriptor `color` when set (overrides catalog fill).
    pub color_fill: Option<Color>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    pub icon_kind: Option<String>,
    pub properties: PropertyBag,
}

// #region 🔖️HandleDescriptor
/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HandleDescriptor {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    #[serde(default)]
    #[value(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub handle_kind: Option<String>,
    /// 🖌️ CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
    #[serde(default)]
    #[value(default)]
    pub color: Option<String>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    #[serde(default)]
    #[value(default)]
    pub icon_kind: Option<String>,
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
    pub scale: Option<f64>,
}

pub fn decode_board_scene_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<SceneDescriptor,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    let mut scene=SceneDescriptor::from_value_controlled(raw.get(),control)?.guard_decoded();
    macro_rules! normalize {($field:ident)=>{if let Some(rows)=raw.get().get(stringify!($field)).and_then(|value|value.as_array()){for (entry,row)in scene.get_mut().$field.iter_mut().zip(rows){control.step()?;entry.visible=board_visible_option(&BoardVisibility{hidden:row.get("hidden").and_then(|value|value.as_bool()),visible:row.get("visible").and_then(|value|value.as_bool()),locked:None});}}};}
    normalize!(nodes);normalize!(handles);normalize!(edges);normalize!(wires);
    Ok(scene.take())
}

/// 📸️ Both physical inputs are admitted before the pure redraw pipeline executes.
pub fn redraw_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=decode_board_snapshot_json(source,decode)?;
 let opts=decode_redraw_options_json(options,decode)?;
 redraw_layout(&mut snapshot,&opts,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}

/// ⚛️ Explicit physical force-only entry, with optional empty option text admitted as declared defaults.
pub fn force_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=decode_board_snapshot_json(source,decode)?;
 let opts:ForceGraphLayoutOptions=if options.trim().is_empty(){ForceGraphLayoutOptions::default()}else{{let admission:ForceAdmission=semio_framework_pack_json::from_json_str_controlled(options,semio_framework_pack_json::JsonMemberPolicy::Reject,decode)?;admission.admit(decode)?}};
 if snapshot.schema.as_str()=="board.ports.directed.v1"{apply_ported_force_layout(&mut snapshot,&opts,work)?;}else{apply_force_layout(&mut snapshot,&opts,work)?;}
 Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}

/// 🔗️ Physical snap entry shares the exclusive snapshot admission contract.
pub fn snap_snapshot_json(source:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{let mut snapshot=decode_board_snapshot_json(source,decode)?;snap_edge_handles(&mut snapshot,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)}

pub fn decode_board_snapshot_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardSnapshot,ValueError>{semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)}

// #region 🔖️GraphExtension
/// 🧩️ Extension hook for domain-specific graph behavior.
pub trait GraphExtension: canvas::CanvasExtension {}
// #endregion 🔖️GraphExtension

// #region 🔖️RenderScene
/// 🎨️ Encodes a batch of edge curves into a stroked vello scene.
pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> canvas::Scene {
    let mut scene = canvas::Scene::new();
    let stroke = canvas::Stroke::new(stroke_width);
    for curve in curves {
        scene.stroke(&stroke, canvas::Affine::IDENTITY, canvas::Color::new(ui_styling::CANVAS_LIGHT.icon_bg), None, curve);
    }
    scene
}
// #endregion 🔖️RenderScene
// #endregion 🔖️Kinds
